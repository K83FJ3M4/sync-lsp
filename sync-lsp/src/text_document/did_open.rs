//! implementation of the `textDocument/didOpen` notification
//! 
//! # Usage
//! Whenever a document is opened, [`Server::on_open`] is invoked.

use crate::{Server, TypeProvider};
use crate::connection::{Callback, Endpoint};
use serde::Deserialize;
use super::DocumentUri;

#[derive(Default, Clone)]
pub(crate) struct DidOpenOptions;

/// A document that has been opened.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentItem {
    /// The text document's URI.
    pub uri: DocumentUri,
    /// A language identifier.
    pub language_id: String,
    /// The initial version number of the document.
    pub version: i32,
    /// The entire content of the document.
    pub text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DidOpenParams {
    text_document: TextDocumentItem,
}

impl DidOpenOptions {

    pub(crate) const METHOD: &'static str = "textDocument/didOpen";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DidOpenOptions> {
        Endpoint::new(Callback::notification(|_, _: DidOpenParams| ()))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called if a [file is opened](self).
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as a file is opened:
    ///     * The server instance receiving the response.
    ///     * The [`TextDocumentItem`] of the document that has been opened.
    ///

    pub fn on_open(&mut self, callback: fn(&mut Server<T>, TextDocumentItem)) {
        self.text_document.did_open.set_callback(Callback::notification(move |server, params: DidOpenParams| {
            callback(server, params.text_document)
        }))
    }
}