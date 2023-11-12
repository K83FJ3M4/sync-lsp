use serde::{Serialize, Deserialize};
use crate::Connection;
use crate::connection::{RpcConnection, Callback};

use super::MessageType;

pub(super) struct ShowMessageRequest<T: 'static>
    (fn(&mut Connection<T>, String, Option<MessageActionItem>));

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageActionItem {
    pub title: String
}

#[derive(Serialize)]
struct ShowMessageRequestParams {
    #[serde(rename = "type")]
    r#type: MessageType,
    message: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    actions: Vec<MessageActionItem>
}

impl<T> Connection<T> {
    pub fn show_message_request(&mut self, tag: &str, r#type: MessageType, message: String, actions: Vec<MessageActionItem>) {
        self.request(
            ShowMessageRequest::<T>::METHOD,
            tag,
            ShowMessageRequestParams {
                r#type,
                message,
                actions
            }
        );
    }

    pub fn on_show_message_response(&mut self, f: fn(&mut Connection<T>, String, Option<MessageActionItem>)) {
        self.window.show_message_request = ShowMessageRequest(f)
    }
}

impl<T> Default for ShowMessageRequest<T> {
    fn default() -> Self {
        Self(|_, _, _| {})
    }
}

impl<T> ShowMessageRequest<T> {
    pub(super) const METHOD: &'static str = "window/showMessageRequest";

    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        let ShowMessageRequest(callback) = *self;
        Callback::response(callback)
    }
}