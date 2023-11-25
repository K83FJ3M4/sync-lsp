//! mplementation of `window/logMessage` notification.

use serde::Serialize;
use crate::{Connection, TypeProvider};
use crate::connection::RpcConnection;

use super::MessageType;

/// This struct only exists so it can provide a mehtod string and should not be used with an [`Endpoint`].
#[derive(Default)]
pub(super) struct LogMessage;

/// The parameters passed to the [`Connection::log_message`] notification.
#[derive(Serialize)]
struct LogMessageParams {
    #[serde(rename = "type")]
    r#type: MessageType,
    message: String,
}

impl<T: TypeProvider> Connection<T> {
    /// This notification logs to a console on the client side.
    /// println!() should not be used in a server implementation,
    /// as it will interfere with the stdio transport.
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