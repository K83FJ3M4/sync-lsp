use std::{io::{
    BufRead,
    Write,
    StdinLock,
    StdoutLock,
    Error,
    stdin,
    stdout, BufReader
}, time::Duration, net::{ToSocketAddrs, TcpListener}};

use mio::net::TcpStream;

use mio::{
    Events,
    Poll,
    Token,
    Interest
};

#[cfg(unix)]
use mio::unix::SourceFd;

use log::{
    warn,
    error
};

/// The transport defines how data is sent and received from the client.
/// The langauge server protocol commonly uses stdio and ipc, but
/// tcp and custom transports are also supported.
/// All errors that occur during sending and receiving will cause the
/// [Server::serve](crate::Server::serve) method to immediately return with an error variant.
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
    },
    Tpc {
        input: BufReader<TcpStream>,
        output: TcpStream
    },
    Custom {
        input: Box<dyn BufRead>,
        output: Box<dyn Write>
    }
}

impl RawTransport {
    fn input(&mut self) -> &mut dyn BufRead {
        match self {
            Self::Stdio { input, .. } => input,
            Self::Tpc { input, .. } => input,
            Self::Custom { input, .. } => input
        }
    }

    fn output(&mut self) -> &mut dyn Write {
        match self {
            Self::Stdio { output, .. } => output,
            Self::Tpc { output, .. } => output,
            Self::Custom { output, .. } => output
        }
    }
}

impl Transport {

    /// Creates a new transport from the given input and output streams.
    /// This transport will not support polling and therefore will not be able to
    /// support request cancellation.
    /// 
    /// # Arguments
    /// * `input` - The input stream to read from.
    pub fn custom(input: impl BufRead + 'static, output: impl Write + 'static) -> Transport {
        Transport {
            raw: RawTransport::Custom {
                input: Box::new(input),
                output: Box::new(output)
            },
            error: None,
            events: Events::with_capacity(1),
            buffer: Vec::new(),
            poll: None
        }
    }

    /// Opens a tcp connection to the given address and returns a transport.
    /// 
    /// # Argument
    /// * `addr` - The address to connect to.
    pub fn tcp<T: ToSocketAddrs>(addr: T) -> Result<Transport, Error> {
        let mut poll = Poll::new().ok();
        let listener = TcpListener::bind(addr)?;
        let (stream, ..) = listener.accept()?;
        let input = stream.try_clone()?;
        let mut input = TcpStream::from_std(input);

        if let Some(poll) = poll.as_mut() {
            poll.registry().register(
                &mut input,
                Token(0),
                Interest::READABLE
            ).ok();
        }

        let input = BufReader::new(input);
        let output = TcpStream::from_std(stream);

        Ok(Transport {
            raw: RawTransport::Tpc {
                output,
                input
            },
            error: None,
            events: Events::with_capacity(1),
            buffer: Vec::new(),
            poll
        })
    }

    /// Locks the standard input and output streams and returns a transport.
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