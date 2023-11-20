pub use connection::{Transport, Connection, UnitType};
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
//Implement calcellation support
//Implement dynamic registration support

//Add License
//Add Documentation
//Add Readme
//Publish