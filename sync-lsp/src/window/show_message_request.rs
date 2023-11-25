//! implements the `window/showMessageRequest` request.

use std::collections::HashMap;
use std::mem::replace;
use serde::{Serialize, Deserialize};
use crate::{Connection, Server, TypeProvider};
use crate::connection::{RpcConnection, Callback, CancellationToken};

use super::MessageType;

/// This struct provides a callback, but doesn't need to be used with an [`Endpoint`].
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

/// The parameters passed to the [`Connection::show_message_request`] request.
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
    /// This request will trigger a query to the user.
    /// The result resulting choice can be retrieved using the corresponding [`Server::on_show_message_response`] method.
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
    /// Retrieves the result from calls to [`Connection::show_message_request`].
    /// 
    /// # Arguments
    /// * `f` - A function to handle a [`MessageActionItem`].
    /// The first argument is the server instance that received the response.
    /// The second argument is the [`MessageActionItem`] including the same data as specified in the request.
    pub fn on_show_message_response(&mut self, f: fn(&mut Server<T>, MessageActionItem<T::ShowMessageRequestData>)) {
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

    pub(crate) fn callback(&self) -> Callback<Server<T>> {
        self.callback.clone()
    }
}