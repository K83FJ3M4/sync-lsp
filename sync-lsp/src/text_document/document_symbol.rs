//! implementation of the `textDocument/documentSymbol` request
//! 
//! # Usage
//! The client can request a list of symbols from the server via [`Server::on_document_symbol`]
//! at any time. This is useful for the user to navigate to a specific symbol in a file.

use crate::TypeProvider;
use crate::workspace::symbol::SymbolInformation;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use super::TextDocumentIdentifer;
use serde::Deserialize;

#[derive(Default, Clone)]
pub struct DocumentSymbolOptions;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DocumentSymbolParams {
    text_document: TextDocumentIdentifer,
}

impl DocumentSymbolOptions {

    pub(crate) const METHOD: &'static str = "textDocument/documentSymbol";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DocumentSymbolOptions> {
        Endpoint::new(Callback::request(|_, _: DocumentSymbolParams| Vec::<SymbolInformation>::new()))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called to [compute document symbols](self).
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as document symbols are requested:
    ///     * The server instance receiving the response.
    ///     * The [`TextDocumentIdentifer`] of the document that has been opened.
    ///     * `return` - A list of symbols to display.

    pub fn on_document_symbol(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer) -> Vec<SymbolInformation>) {
        self.text_document.document_symbol.set_callback(Callback::request(move |server, params: DocumentSymbolParams| {
            callback(server, params.text_document)
        }))
    }
}