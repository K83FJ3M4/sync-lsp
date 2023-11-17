pub use connection::{Transport, Connection};
use serde::de::DeserializeOwned;
use serde::Serialize;

mod connection;
mod lifecycle;
pub mod text_document;
pub mod window;
pub mod workspace;

pub trait TypeProvider: 'static {
    type Command: Serialize + DeserializeOwned;
    type CodeLensData: Serialize + DeserializeOwned;
    type CompletionData: Serialize + DeserializeOwned;
    type Configuration: DeserializeOwned;
    type InitializeOptions: DeserializeOwned;
    //MessageActionRequest
}