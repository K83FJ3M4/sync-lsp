use crate::Connection;
use super::jsonrpc::{RpcConnection, Callback, RpcError};

impl<T> RpcConnection for Connection<T> {
    fn transport(&mut self) -> &mut crate::Transport {
        &mut self.transport
    }

    fn take_error(&mut self) -> Option<RpcError> {
        self.error.take()
    }

    fn resolve(&self, method: &str) -> Option<Callback<Self>> {
        self.lifecycle.resolve(method)
    }
}