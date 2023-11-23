//! Implementation of the `workspace/symbol` request.
//! 
//! # Usage
//! In the language server protocol symbols are defined as any kind of symbol used in a
//! programming language. These symbols are commonly used resolve their specific
//! location in a source file. They can either be queried for a specific file or,
//! like in this case, for the whole workspace using [`Server::on_symbol`].

use crate::TypeProvider;
use crate::text_document::Location;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

/// This struct can be unsed in an [`Endpoint`].
#[derive(Default, Clone)]
pub(crate) struct SymbolOptions;

/// The parameters of a [`SymbolOptions::METHOD`] request.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkspaceSymbolParams  {
    /// A possibly empty query string to filter symbols.
    query: String
}

/// Denotes any kind of symbol used in a programming language, for example variables, functions, classes, interfaces etc.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInformation {
    /// The name of this symbol as it should appear in a user interface.
    pub name: String,
    /// A variant of [`SymbolKind`]
    pub kind: SymbolKind,
    /// The location of this symbol in the source code. Optionally this location
    /// may contain more than just the symbol itself, like visibility modifiers.
    pub location: Location,
    /// A optional name of another symbol containing this one.
    /// For example the name of a class containing this symbol as a method.
    pub container_name: Option<String>
}

/// This enum contain various kinds of symbols, which are mainly used
/// in the user interface and attached to [`SymbolInformation`]
#[repr(i32)]
#[derive(Serialize_repr, Debug)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
}

impl SymbolOptions {

    pub(crate) const METHOD: &'static str = "workspace/symbol";

    /// Creates a new [`Endpoint`] with the default options for this request.
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, SymbolOptions> {
        Endpoint::new(Callback::request(|_, _: WorkspaceSymbolParams| Vec::<SymbolInformation>::new()))
    }
}

impl<T: TypeProvider> Server<T> {
    /// Sets the callback that will be used to resolve all symbols in a workspace.
    /// 
    /// # Arguments
    /// * `callback` - A function that takes in a query string and returns a vector of [`SymbolInformation`] elements.
    /// This first argument is the server instance that received the request.
    /// The second argument is a possibly empty query string to filter symbols.
    pub fn on_symbol(&mut self, callback: fn(&mut Server<T>, String) -> Vec<SymbolInformation>) {
        self.workspace.symbol.set_callback(Callback::request(move |server, params: WorkspaceSymbolParams| {
            callback(server, params.query)
        }))
    }
}