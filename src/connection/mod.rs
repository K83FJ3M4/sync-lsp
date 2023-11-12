use std::ops::{Deref, DerefMut};
pub use transport::Transport;

mod jsonrpc;
mod transport;

pub struct Connection<T> {
    state: T,
    transport: Transport
}

impl<T> Connection<T> {
    pub fn new(state: T, transport: Transport) -> Connection<T> {
        Connection {
            state,
            transport
        }
    }

}

impl<T> Deref for Connection<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<T> DerefMut for Connection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}