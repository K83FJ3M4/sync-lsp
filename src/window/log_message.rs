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