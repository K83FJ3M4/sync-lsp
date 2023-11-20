use std::io::Error;
use std::ops::{Deref, DerefMut};
use jsonrpc::RpcError;

pub use transport::Transport;
pub(crate) use jsonrpc::{Callback, ErrorCode, EmptyParams, RpcConnection};
pub(crate) use rpc::Endpoint;

use crate::TypeProvider;
use crate::lifecycle::LifecycleService;
use crate::text_document::TextDocumentService;
use crate::window::WindowService;
use crate::workspace::WorkspaceService;

use serde::{Serialize, Deserialize};
use serde::ser::Serializer;
use serde::de::Deserializer;

use self::jsonrpc::RpcConnectionImpl;

mod rpc;
mod jsonrpc;
mod transport;
mod lifecycle;

pub struct Connection<T: TypeProvider> {
    state: T,
    transport: Transport,
    error: Option<RpcError>,
    process_id: Option<u32>,
    root_uri: Option<String>,
    initialization_options: Option<T::InitializeOptions>,
    lifecycle: LifecycleService<T>,
    pub(crate) window: WindowService<T>,
    pub(crate) text_document: TextDocumentService<T>,
    pub(crate) workspace: WorkspaceService<T>
}

#[derive(Clone, Copy, Debug)]
pub struct UnitType;

impl<T: TypeProvider> Connection<T> {
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

    pub fn initialization_options(&self) -> Option<&T::InitializeOptions> {
        self.initialization_options.as_ref()
    }
}

impl<T: TypeProvider> Deref for Connection<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<T: TypeProvider> DerefMut for Connection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl Serialize for UnitType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(99)
    }
}

impl<'a> Deserialize<'a> for UnitType {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        u32::deserialize(deserializer)
            .map(|_| UnitType)
    }
}