use crate::Connection;
use super::jsonrpc::{RpcConnection, Callback, RpcError};

pub(crate) struct Endpoint<T: 'static, O: Clone + Default> {
    callback: Callback<Connection<T>>,
    options: O
}

impl<T> RpcConnection for Connection<T> {
    fn transport(&mut self) -> &mut crate::Transport {
        &mut self.transport
    }

    fn take_error(&mut self) -> Option<RpcError> {
        self.error.take()
    }

    fn resolve(&self, method: &str) -> Option<Callback<Self>> {
        self.lifecycle.resolve(method)
            .or(self.window.resolve(method))
            .or(self.text_document.resolve(method))
            .or(self.workspace.resolve(method))
    }
}

impl<T: 'static, O: Clone + Default> Endpoint<T, O> {
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