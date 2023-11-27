//! implements the `window/showMessageRequest` request.
//!
//! # Usage
//! This request may be used to query the user for a choice.
//! The result can be retrieved using [`Server::on_show_message_response`] and therefore
//! [`Connection::show_message_request`] won't block the current thread.

use std::collections::HashMap;
use std::mem::replace;
use serde::{Serialize, Deserialize};
use crate::{Connection, Server, TypeProvider};
use crate::connection::{RpcConnection, Callback, CancellationToken};

use super::MessageType;

pub(super) struct ShowMessageRequest<T: TypeProvider> {
    callback: Callback<Server<T>>
}

/// This Item is beeing sent along every show message request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "")]
pub struct MessageActionItem<T: Default> {
    /// The title of the message action which will be shown to the user.
    pub title: String,
    /// This field is preserved and will be sent back in the response.
    /// It's type can be specified using the [`TypeProvider`] trait.
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

    /// This request will [trigger a query](self) to the user.
    /// 
    /// # Arguments
    /// * `r#type` - The type of message to show.
    /// * `message` - The message to show.
    /// * `actions` - The actions to show.
    /// * `result` - A optional cancellation token that can be used to cancel the request.
    
    pub fn show_message_request(&mut self, r#type: MessageType, message: String, mut actions: Vec<MessageActionItem<T::ShowMessageRequestData>>) -> Option<CancellationToken> {
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
        ).map(|id| id.into())
    }
}

impl<T: TypeProvider> Server<T> {

    /// Set the response handler for [showing a message request](self)
    ///
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters if the result of a query is received:
    ///     * The server instance receiving the response.
    ///     * A tag of type [`TypeProvider::ShowMessageRequestData`] that was passed to the request.
    
    pub fn on_show_message_response(&mut self, callback: fn(&mut Server<T>, MessageActionItem<T::ShowMessageRequestData>)) {
        self.window.show_message_request.callback = Callback::response(move |connection, mut tag: HashMap<String, T::ShowMessageRequestData>, params: Option<MessageActionItem<T::ShowMessageRequestData>>| {
            if let Some(mut action) = params {
                action.data = tag.remove(action.title.as_str()).unwrap_or_default();
                callback(connection, action)
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

    pub(crate) fn callback(&self) -> Callback<Server<T>> {
        self.callback.clone()
    }
}