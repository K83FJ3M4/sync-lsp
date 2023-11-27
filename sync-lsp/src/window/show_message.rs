//! impls for the `window/showMessage` notification
//!
//! # Usage
//! The main difference between [`Connection::show_message`] and [`Connection::log_message`] is
//! that this notification will shown as some kind of popup to the user, whereas
//! [`Connection::log_message`] will only be logged to the console.

use serde::Serialize;
use crate::{Connection, TypeProvider};
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

impl<T: TypeProvider> Connection<T> {

    /// This notification may be used to [show a message](self) to the user.
    /// 
    /// # Arguments
    /// * `r#type` - The type of message to show.
    /// * `message` - The message to show.
    
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