//! mplementation of `window/logMessage` notification.
//!
//! # Usage
//! When using this crate a logger for the `log` crate will be set up automatically.
//! User of this library should never use `println!()` or `eprintln!()` as they will
//! interfere with the stdio transport. All log macros are forwarded to [`Connection::log_message`]
//! ```
//! use log::{debug, info, warn, error};
//! error!("This is an error message");
//! warn!("This is a warning message");
//! info!("This is an info message");
//! debug!("This is a debug message and will therefore not be shown on release builds");
//! ```

use serde::Serialize;
use crate::{Connection, TypeProvider};
use crate::connection::RpcConnection;

use super::MessageType;

#[derive(Default)]
pub(super) struct LogMessage;

#[derive(Serialize)]
struct LogMessageParams {
    #[serde(rename = "type")]
    r#type: MessageType,
    message: String,
}

impl<T: TypeProvider> Connection<T> {

    /// This notification may be used to [log](self) to a console on the client side.
    /// 
    /// # Arguments
    /// * `r#type` - The type of log message.
    /// * `message` - The message to log.
    
    pub fn log_message(&mut self, r#type: MessageType, message: String) {
        self.notify(
            LogMessage::METHOD,
            LogMessageParams {
                r#type,
                message
            }
        );
    }
}

impl LogMessage {
    const METHOD: &'static str = "window/logMessage";
}