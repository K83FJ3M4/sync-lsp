use std::io::Error;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use jsonrpc::RpcError;

pub use transport::Transport;
pub(crate) use jsonrpc::{Callback, ErrorCode, EmptyParams, RpcConnection, CancelParams};
pub(crate) use rpc::Endpoint;

use crate::TypeProvider;
use crate::lifecycle::LifecycleService;
use crate::text_document::TextDocumentService;
use crate::window::WindowService;
use crate::workspace::WorkspaceService;

use serde::{Serialize, Deserialize};
use serde::ser::Serializer;
use serde::de::Deserializer;

use self::jsonrpc::{RpcConnectionImpl, MessageID};

mod rpc;
mod jsonrpc;
mod transport;
mod lifecycle;

pub struct Server<T: TypeProvider> {
    pub connection: Connection<T>,
    state: T,
    process_id: Option<u32>,
    root_uri: Option<String>,
    initialization_options: Option<T::InitializeOptions>,
    
    lifecycle: LifecycleService<T>,
    pub(crate) window: WindowService<T>,
    pub(crate) text_document: TextDocumentService<T>,
    pub(crate) workspace: WorkspaceService<T>
}

pub struct Connection<T: TypeProvider> {
    transport: Transport,
    error: Option<RpcError>,
    current_request: Option<MessageID>,
    marker: PhantomData<T>
}

#[derive(Clone, Copy, Debug, Default)]
pub struct UnitType;

#[derive(Clone)]
pub struct CancellationToken(MessageID);

impl<T: TypeProvider> Server<T> {
    pub fn new(state: T, transport: Transport) -> Server<T> {
        Server {
            state,
            connection: Connection::new(transport),
            process_id: None,
            root_uri: None,
            initialization_options: None,
            lifecycle: Default::default(),
            window: Default::default(),
            text_document: Default::default(),
            workspace: Default::default()
        }
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

    pub fn split(&mut self) -> (&mut Connection<T>, &mut T) {
        (&mut self.connection, &mut self.state)
    }

    pub fn serve(self) -> Result<(), Error> {
        RpcConnectionImpl::serve(self)
    }
}

impl<T: TypeProvider> Connection<T> {
    fn new(transport: Transport) -> Connection<T> {
        Connection {
            transport,
            error: None,
            current_request: None,
            marker: PhantomData
        }
    }

    pub fn error<R: Default>(&mut self, code: ErrorCode, message: String) -> R {
        self.error = Some(RpcError {
            code,
            message
        });
        R::default()
    }

    pub fn cancel(&mut self, token: CancellationToken) {
        self.notify("$/cancelRequest", CancelParams {
            id: token.0
        })
    }

    pub fn cancelled(&mut self) -> bool {
        let Some(id) = self.current_request.clone() else { return false; };
        while let Some(params) = self.peek_notification::<CancelParams>("$/cancelRequest") {
            if params.id == id {
                self.current_request = self.error(
                    ErrorCode::RequestCancelled,
                    "Request cancelled".to_string()
                );
                return true;
            }
        }
        false
    }
}

impl<T: TypeProvider> Deref for Server<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<T: TypeProvider> DerefMut for Server<T> {
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

impl From<MessageID> for CancellationToken {
    fn from(id: MessageID) -> Self {
        CancellationToken(id)
    }
}