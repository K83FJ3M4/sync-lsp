use std::{io::{
    BufRead,
    Write,
    StdinLock,
    StdoutLock,
    Error,
    stdin,
    stdout
}, time::Duration};

use mio::{
    Events,
    Interest,
    Poll,
    Token, unix::SourceFd
};

use log::{
    warn,
    error
};

pub struct Transport {
    raw: RawTransport,
    error: Option<Error>,
    poll: Option<Poll>,
    events: Events,
    buffer: Vec<Vec<u8>>
}

enum RawTransport {
    Stdio {
        input: StdinLock<'static>,
        output: StdoutLock<'static>,
    }
}

impl RawTransport {
    fn input(&mut self) -> &mut dyn BufRead {
        match self {
            Self::Stdio { input, .. } => input
        }
    }

    fn output(&mut self) -> &mut dyn Write {
        match self {
            Self::Stdio { output, .. } => output
        }
    }
}

impl Transport {
    pub fn stdio() -> Transport {
        let poll = Poll::new().ok();
        let input = stdin().lock();

        #[cfg(unix)]
        if let Some(poll) = poll.as_ref() {
            use std::os::fd::AsRawFd;
            poll.registry().register(
                &mut SourceFd(&input.as_raw_fd()),
                Token(0),
                Interest::READABLE
            ).ok();
        }
        
        Transport {
            raw: RawTransport::Stdio {
                output: stdout().lock(),
                input
            },
            error: None,
            events: Events::with_capacity(1),
            buffer: Vec::new(),
            poll
        }
    }

    pub(crate) fn error(&mut self) -> &mut Option<Error> {
        &mut self.error
    }

    pub(crate) fn send(&mut self, message: String) {
        if self.error().is_some() { return }
        *self.error() = write!(self.raw.output(), "Content-Length: {}\r\n", message.len())
            .or(write!(self.raw.output(), "Content-Type: {}\r\n", "application/vscode-jsonrpc; charset=utf-8"))
            .or(write!(self.raw.output(), "\r\n{message}"))
            .or(self.raw.output().flush()).err();
    }

    pub(crate) fn recv(&mut self) -> Option<Vec<u8>> {
        if let Some(data) = self.buffer.pop() {
            return Some(data)
        }

        if self.error().is_some() { return None }
        match self.try_recv() {
            Ok(message) => Some(message),
            Err(error) => {
                *self.error() = Some(error);
                None
            }
        }
    }


    pub(crate) fn peek(&mut self) -> Option<Vec<u8>> {
        if self.poll() && self.buffer.len() < 10192 {
            let data = self.recv();
            if let Some(data) = data.clone() {
                self.buffer.push(data)
            }
            data
        } else {
            None
        }
    }

    fn poll(&mut self) -> bool {
        self.events.clear();
        if let Some(poll) = self.poll.as_mut() {
            poll.poll(&mut self.events, Some(Duration::from_millis(1))).ok();
        }
        !self.events.is_empty()
    }

    fn try_recv(&mut self) -> Result<Vec<u8>, Error> {
        loop {
            let mut content_length: Option<usize> = None;
    
            for line in self.raw.input().lines() {

                let line = line?;
                if line.is_empty() { break }

                match line.split_once(": ") {
                    Some(("Content-Length", value)) => content_length = Some(
                        if let Ok(content_length) = value.parse() {
                            content_length
                        } else {
                            error!("Failed to parse Content-Length");
                            continue
                        }
                    ),
                    Some(("Content-Type", value)) => {
                        if value != "application/vscode-jsonrpc; charset=utf-8" {
                            error!("Invalid Content-Type: {value}");
                            continue
                        }
                    },
                    None => warn!("Invalid header: {line}"),
                    Some((header, ..)) => warn!("Unknown header: {header}")
                }
            }

            let Some(content_length) = content_length else {
                error!("Received a message without a Content-Length");
                continue
            };

            let mut buffer = vec![0; content_length];

            self.raw.input()
                .read_exact(&mut buffer)?;

            //eprintln!("Received: {message}", message = String::from_utf8_lossy(&buffer));

            return Ok(buffer)
        }
    }
}