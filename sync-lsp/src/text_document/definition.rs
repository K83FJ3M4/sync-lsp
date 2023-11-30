//! implementation of the `textDocument/definition` request
//! 
//! # Usage
//! [`Server::on_definition`] is invoked, to compute the definition
//! of a symbol at a given cursor position.

use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use super::{TextDocumentIdentifer, TextDocumentPositionParams, Location, Position};

#[derive(Default, Clone)]
pub(crate) struct DefinitionOptions;

impl DefinitionOptions {

    pub(crate) const METHOD: &'static str = "textDocument/definition";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DefinitionOptions> {
        Endpoint::new(Callback::request(|_, _: TextDocumentPositionParams| Vec::<Location>::new()))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called to locate a  [definition](self).
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters to resolve a definition:
    ///     * The server instance receiving the response.
    ///     * The [`TextDocumentIdentifer`] of the document for which a definition is requested.
    ///    * The [`Position`] at which a definition is requested.
    ///     * `return` - A list of [`Location`]s to display.

    pub fn on_definition(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, Position) -> Vec<Location>) {
        self.text_document.definition.set_callback(Callback::request(move |server, params: TextDocumentPositionParams | {
            callback(server, params.text_document, params.position)
        }))
    }
}