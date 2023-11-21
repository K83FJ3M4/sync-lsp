use log::Level;

use crate::window::MessageType;
use crate::{Connection, TypeProvider};
use super::jsonrpc::{RpcConnection, Callback, RpcError, MessageID};

pub(crate) struct Endpoint<T: TypeProvider, O: Clone + Default> {
    callback: Callback<Connection<T>>,
    options: O
}

impl<T: TypeProvider> RpcConnection for Connection<T> {
    fn transport(&mut self) -> &mut crate::Transport {
        &mut self.transport
    }

    fn take_error(&mut self) -> Option<RpcError> {
        self.error.take()
    }

    fn log(&mut self, level: Level, message: String) {
        let r#type = match level {
            Level::Error => MessageType::Error,
            Level::Warn => MessageType::Warning,
            Level::Info => MessageType::Info,
            Level::Debug => MessageType::Log,
            Level::Trace => MessageType::Log
        };

        self.log_message(r#type, message)
    }

    fn resolve(&self, method: &str) -> Option<Callback<Self>> {
        self.lifecycle.resolve(method)
            .or(self.window.resolve(method))
            .or(self.text_document.resolve(method))
            .or(self.workspace.resolve(method))
    }

    fn set_current_request(&mut self, id: Option<MessageID>) {
        self.current_request = id;
    }
}

impl<T: TypeProvider, O: Clone + Default> Endpoint<T, O> {
    pub(crate) fn new(callback: Callback<Connection<T>>,) -> Self {
        Endpoint {
            callback,
            options: O::default()
        }
    }

    pub(crate) fn options_mut(&mut self) -> &mut O {
        &mut self.options
    }

    pub(crate) fn set_callback(&mut self, callback: Callback<Connection<T>>) {
        self.callback = callback;
    }

    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        self.callback.clone()
    }

    pub(crate) fn options(&self) -> O {
        self.options.clone()
    }
}