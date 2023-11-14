use log::error;
use std::io::Error;
use std::ops::{Deref, DerefMut};
use serde::de::DeserializeOwned;
use serde_json::{Value, from_value};
use jsonrpc::RpcError;

pub use transport::Transport;
pub(crate) use jsonrpc::{Callback, ErrorCode, EmptyParams, RpcConnection};
pub(crate) use rpc::Endpoint;

use crate::lifecycle::LifecycleService;
use crate::text_document::TextDocumentService;
use crate::window::WindowService;
use crate::workspace::WorkspaceService;

use self::jsonrpc::RpcConnectionImpl;

mod rpc;
mod jsonrpc;
mod transport;
mod lifecycle;

pub struct Connection<T: 'static> {
    state: T,
    transport: Transport,
    error: Option<RpcError>,
    process_id: Option<u32>,
    root_uri: Option<String>,
    initialization_options: Option<Value>,
    lifecycle: LifecycleService<T>,
    pub(crate) window: WindowService<T>,
    pub(crate) text_document: TextDocumentService<T>,
    pub(crate) workspace: WorkspaceService<T>
}

impl<T> Connection<T> {
    pub fn new(state: T, transport: Transport) -> Connection<T> {
        Connection {
            state,
            transport,
            error: None,
            process_id: None,
            root_uri: None,
            initialization_options: None,
            lifecycle: Default::default(),
            window: Default::default(),
            text_document: Default::default(),
            workspace: Default::default()
        }
    }

    pub fn serve(self) -> Result<(), Error> {
        RpcConnectionImpl::serve(self)
    }

    pub fn error<R: Default>(&mut self, code: ErrorCode, message: String) -> R {
        self.error = Some(RpcError {
            code,
            message
        });
        R::default()
    }

    pub fn process_id(&self) -> Option<u32> {
        self.process_id
    }

    pub fn root_uri(&self) -> Option<&str> {
        self.root_uri.as_ref().map(|uri| uri.as_str())
    }

    pub fn initialization_options<O: DeserializeOwned>(&self) -> Option<O> {
        let Some(options) = self.initialization_options.as_ref() else { return None };
        match from_value(options.clone()) {
            Ok(options) => Some(options),
            Err(error) => {
                error!("Failed to deserialize initialization options: {}", error);
                None
            }
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