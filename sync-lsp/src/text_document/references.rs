//! implementation of the `textDocument/references` request
//! 
//! # Usage
//! A client can resolve references to a symbol via [`Server::on_references`], which could
//! be used to implement "Find all references" functionality.

use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use super::{TextDocumentIdentifer, Position, Location};
use serde::Deserialize;

#[derive(Default, Clone)]
pub(crate) struct ReferenceOptions;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReferenceParams {
    text_document: TextDocumentIdentifer,
    position: Position,
    context: ReferenceContext
}

/// Additional information about which references should be returned.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceContext {
    /// Include the declaration of the current symbol.
    pub include_declaration: bool
}

impl ReferenceOptions {

    pub(crate) const METHOD: &'static str = "textDocument/references";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, ReferenceOptions> {
        Endpoint::new(Callback::request(|_, _: ReferenceParams| Vec::<Location>::new()))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called to [resolve references](self).
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as references are requested:
    ///    * The server instance receiving the response.
    ///    * The [`TextDocumentIdentifer`] of the target document.
    ///    * The [`Position`] of the cursor.
    ///    * The [`ReferenceContext`] that specifies which references should be returned.
    ///    * `return` - A list of locations that reference the symbol at the given position.

    pub fn on_references(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, Position, context: ReferenceContext) -> Vec<Location>) {
        self.text_document.references.set_callback(Callback::request(move |server, params: ReferenceParams| {
            callback(server, params.text_document, params.position, params.context)
        }))
    }
}