pub use connection::{Transport, Connection};
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
    //MessageActionRequest
}