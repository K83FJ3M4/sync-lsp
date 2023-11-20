use std::io::{
    BufRead,
    Write,
    StdinLock,
    StdoutLock,
    Error,
    stdin,
    stdout
};

use log::{
    warn,
    error
};

pub enum Transport {
    Stdio {
        input: StdinLock<'static>,
        output: StdoutLock<'static>,
        error: Option<Error>
    }
}

impl Transport {
    pub fn stdio() -> Transport {
        Transport::Stdio {
            input: stdin().lock(),
            output: stdout().lock(),
            error: None
        }
    }

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

    pub(crate) fn error(&mut self) -> &mut Option<Error> {
        match self {
            Self::Stdio { error, .. } => error
        }
    }

    pub(crate) fn send(&mut self, message: String) {
        if self.error().is_some() { return }
        *self.error() = write!(self.output(), "Content-Length: {}\r\n", message.len())
            .or(write!(self.output(), "Content-Type: {}\r\n", "application/vscode-jsonrpc; charset=utf-8"))
            .or(write!(self.output(), "\r\n{message}"))
            .or(self.output().flush()).err();
    }

    pub(crate) fn recv(&mut self) -> Option<Vec<u8>> {
        if self.error().is_some() { return None }
        match self.try_recv() {
            Ok(message) => Some(message),
            Err(error) => {
                *self.error() = Some(error);
                None
            }
        }
    }

    fn try_recv(&mut self) -> Result<Vec<u8>, Error> {
        loop {
            let mut content_length: Option<usize> = None;
    
            for line in self.input().lines() {

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

            self.input()
                .read_exact(&mut buffer)?;

            return Ok(buffer)
        }
    }
}