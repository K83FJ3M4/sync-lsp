use std::collections::HashMap;
use std::mem::replace;
use serde::{Serialize, Deserialize};
use crate::{Connection, TypeProvider};
use crate::connection::{RpcConnection, Callback};

use super::MessageType;

pub(super) struct ShowMessageRequest<T: TypeProvider> {
    callback: Callback<Connection<T>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "")]
pub struct MessageActionItem<T: Default> {
    pub title: String,
    #[serde(skip)]
    #[serde(default)]
    pub data: T
}

#[derive(Serialize)]
#[serde(bound = "")]
struct ShowMessageRequestParams<T: Default> {
    #[serde(rename = "type")]
    r#type: MessageType,
    message: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    actions: Vec<MessageActionItem<T>>
}

impl<T: TypeProvider> Connection<T> {
    pub fn show_message_request(&mut self, r#type: MessageType, message: String, mut actions: Vec<MessageActionItem<T::ShowMessageRequestData>>) {
        self.request(
            ShowMessageRequest::<T>::METHOD,
            {
                let mut hashmap = HashMap::new();
                for action in actions.iter_mut() {
                    let data = replace(&mut action.data, Default::default());
                    hashmap.insert(action.title.clone(), data);
                }
                hashmap
            },
            ShowMessageRequestParams {
                r#type,
                message,
                actions
            }
        );
    }

    pub fn on_show_message_response(&mut self, f: fn(&mut Connection<T>, MessageActionItem<T::ShowMessageRequestData>)) {
        self.window.show_message_request.callback = Callback::response(move |connection, mut tag: HashMap<String, T::ShowMessageRequestData>, params: Option<MessageActionItem<T::ShowMessageRequestData>>| {
            if let Some(mut action) = params {
                action.data = tag.remove(action.title.as_str()).unwrap_or_default();
                f(connection, action)
            }
        })
    }
}

impl<T: TypeProvider> Default for ShowMessageRequest<T> {
    fn default() -> Self {
        Self {
            callback: Callback::response(|_, _: HashMap<String, T::ShowMessageRequestData>, _: Option<MessageActionItem<T::ShowMessageRequestData>>| {})
        }
    }
}

impl<T: TypeProvider> ShowMessageRequest<T> {
    pub(super) const METHOD: &'static str = "window/showMessageRequest";

    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        self.callback.clone()
    }
}