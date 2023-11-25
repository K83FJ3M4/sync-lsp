//! impls for the `window/showMessage` notification

use serde::Serialize;
use crate::{Connection, TypeProvider};
use crate::connection::RpcConnection;

use super::MessageType;

/// This struct only exists so it can provide a mehtod string and should not be used with an [`Endpoint`].
#[derive(Default)]
pub(super) struct ShowMessage;

/// The parameters passed to the [`Connection::show_message`] notification.
#[derive(Serialize)]
struct ShowMessageParams {
    #[serde(rename = "type")]
    r#type: MessageType,
    message: String,
}

impl<T: TypeProvider> Connection<T> {
    /// The main difference between this notification and [`Connection::log_message`] is
    /// that this notification will shown as some kind of popup to the user, whereas
    /// [`Connection::log_message`] will only be logged to the console.
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