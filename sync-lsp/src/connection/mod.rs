use std::io::Error;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use jsonrpc::RpcError;

pub use jsonrpc::ErrorCode;
pub use transport::Transport;
pub(crate) use jsonrpc::{Callback, EmptyParams, RpcConnection, CancelParams};
pub(crate) use rpc::Endpoint;

use crate::TypeProvider;
use crate::lifecycle::LifecycleService;
use crate::lifecycle::initialize::ClientCapabilities;
use crate::text_document::TextDocumentService;
use crate::window::WindowService;
use crate::workspace::WorkspaceService;

use self::jsonrpc::{RpcConnectionImpl, MessageID};

mod rpc;
mod jsonrpc;
mod transport;
mod lifecycle;

/// This struct is a wrapper around the server state, which provides
/// type via the [`TypeProvider`] trait. It also contains the connection
/// to the client and all callbacks for the different endpoints.
/// 
/// # Example
/// ```
/// use sync_lsp::{Transport, TypeProvider, Server};
/// 
/// // For this example, we don't need any state.
/// struct MyServerState;
/// 
/// // This macro provides default implementations for all required types.
/// #[sync_lsp::type_provider]
/// impl TypeProvider for MyServerState {}
/// 
/// fn main() {
///     let transport = Transport::stdio();
///     let mut server = Server::new(MyServerState, transport);
/// 
///    //Callbacks can be registered here via the `server.on_*` methods.
///    //Note that callbacks may also be registered after the server has been started.
/// 
///     server.serve().unwrap();
/// }
/// ```
pub struct Server<T: TypeProvider> {
    pub connection: Connection<T>,
    state: T,
    process_id: Option<u32>,
    root_uri: Option<String>,
    initialization_options: Option<T::InitializeOptions>,
    
    lifecycle: LifecycleService<T>,
    pub(crate) window: WindowService<T>,
    pub(crate) text_document: TextDocumentService<T>,
    pub(crate) workspace: WorkspaceService<T>,
    pub(crate) capabilities: ClientCapabilities,
}

/// The connection to the client can be obtained in one of two way:
/// * By calling [`Server::split`] and taking the first element of the tuple
/// * By referencing the `connection` field of the [`Server`] struct.
/// The connection is used to send notifications and requests to the client.
/// 
/// # Example
/// ```
/// use sync_lsp::{Transport, TypeProvider, Server};
/// 
/// // For this example, we don't need any state.
/// struct MyServerState;
/// 
// This macro provides default implementations for all required types.
/// #[sync_lsp::type_provider]
/// impl TypeProvider for MyServerState {}
/// 
/// fn main() {
///     let transport = Transport::stdio();
///     let mut server = Server::new(MyServerState, transport);
///     server.on_open(|server, _| {
///         #[allow(unused)]
///         let (connection, state) = server.split();
///     });
///     server.serve().unwrap();
/// }
/// ```
pub struct Connection<T: TypeProvider> {
    transport: Transport,
    error: Option<RpcError>,
    current_request: Option<MessageID>,
    marker: PhantomData<T>
}

impl<T: TypeProvider> Server<T> {
    /// Creates a new server with the given state and transport.
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
            workspace: Default::default(),
            capabilities: ClientCapabilities::default()
        }
    }

    /// Returns the process id of the server, if one is provided by the client.
    pub fn process_id(&self) -> Option<u32> {
        self.process_id
    }

    /// Returns the root uri of the workspace, if one is provided by the client.
    pub fn root_uri(&self) -> Option<&str> {
        self.root_uri.as_ref().map(|uri| uri.as_str())
    }

    /// Returns the initialization options as defined in [`TypeProvider::InitializeOptions`] if available and parsed correctly.
    pub fn initialization_options(&self) -> Option<&T::InitializeOptions> {
        self.initialization_options.as_ref()
    }

    /// Splits the server into its connection and state.
    pub fn split(&mut self) -> (&mut Connection<T>, &mut T) {
        (&mut self.connection, &mut self.state)
    }

    /// Starts the server. This will block the current thread.
    /// Until there is either an error or the client sends a shutdown request.
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

    /// This will send a error if called from a request and log it,
    /// if called from a notification. For usability reasons it also
    /// returns a default value of the type `R`, which makes it possible
    /// to write
    /// ```
    /// return self.error(ErrorCode::InvalidParams, "Test Error".to_string());
    /// ```
    /// instead of
    /// ```
    /// self.error(ErrorCode::InvalidParams, "Test Error".to_string());
    /// return SomeType::default();
    /// ```
    pub fn error<R: Default>(&mut self, code: ErrorCode, message: String) -> R {
        self.error = Some(RpcError {
            code,
            message
        });
        R::default()
    }

    /// Check whether the current request has been cancelled.
    /// If this method has been called in a cancelled request,
    /// a error with code [`ErrorCode::RequestCancelled`] will be returned to the client,
    /// regardless of what the request handler returns.
    /// 
    /// # Example
    /// ```
    /// use sync_lsp::{Transport, TypeProvider, Server, text_document::completion::CompletionList};
    /// 
    /// // For this example, we don't need any state.
    /// struct MyServerState;
    /// 
    /// // This macro provides default implementations for all required types.
    /// #[sync_lsp::type_provider]
    /// impl TypeProvider for MyServerState {}
    /// 
    /// fn main() {
    ///     let transport = Transport::stdio();
    ///     let mut server = Server::new(MyServerState, transport);
    ///     
    ///     server.on_completion(|server, _, _| {
    ///         let result = Vec::new();
    /// 
    ///         while !server.connection.cancelled() {
    ///             // Do expensive work here
    ///         }
    /// 
    ///         CompletionList {
    ///             is_incomplete: false,
    ///             items: result
    ///         }
    ///     });
    /// 
    ///     server.serve().unwrap();
    /// }
    /// ```
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