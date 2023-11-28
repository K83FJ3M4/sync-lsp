pub use connection::{Transport, Connection, Server, ErrorCode};
/// This macro provides default implementations for all required types in [`TypeProvider`].
/// 
/// # Example
/// ```
/// use sync_lsp::{TypeProvider, type_provider};
/// 
/// struct MyServerState;
/// 
/// #[type_provider]
/// impl TypeProvider for MyServerState {
///     type ShowMessageRequestData = u32;
///     // All other types will be set to `Option<()>`
/// }
/// ```
pub use sync_lsp_derive::type_provider;
use serde::de::DeserializeOwned;
use serde::Serialize;
use workspace::execute_command::Command;
use std::fmt::Debug;

mod connection;
mod lifecycle;
pub mod text_document;
pub mod window;
pub mod workspace;

/// This trait is used to set type definitions for requests and notifications
/// with dynamic parameters.
/// 
/// For simplicity, it is recommended to use the
/// [`type_provider`] macro instead of implementing the default values manually.
/// Even tough technically allowed by the spec, it is not recommended to use
/// `()` as default types as some lsp clients may return undefined instead of null
/// in their responses causing the a deserialisation error on the server.
pub trait TypeProvider: 'static {
    type Command: Command;
    type CodeLensData: Serialize + DeserializeOwned;
    type CompletionData: Serialize + DeserializeOwned + Debug;
    type Configuration: DeserializeOwned;
    type InitializeOptions: DeserializeOwned;
    type ShowMessageRequestData: Serialize + DeserializeOwned + Default;
    type ApplyEditData: Serialize + DeserializeOwned + Default;
}

//TODO
//Implement dynamic registration support
//hide request cancellation beghin a feature flag

//Add Documentation
//Add Readme
//Publish