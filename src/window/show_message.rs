use serde::Serialize;
use crate::Connection;
use crate::connection::RpcConnection;

use super::MessageType;

#[derive(Default)]
pub(super) struct ShowMessage;

#[derive(Serialize)]
struct ShowMessageParams {
    #[serde(rename = "type")]
    r#type: MessageType,
    message: String,
}

impl<T> Connection<T> {
    pub fn show_message(&mut self, r#type: MessageType, message: String) {
        self.notify(
            ShowMessage::METHOD, 
            ShowMessageParams {
                r#type,
                message
            }
        );
    }
}

impl ShowMessage {
    const METHOD: &'static str = "window/showMessage";
}