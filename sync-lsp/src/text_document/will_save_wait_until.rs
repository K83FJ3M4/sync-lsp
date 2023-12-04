//! implementation of the `textDocument/willSaveWaitUntil` notification.
//! 
//! # Usage
//! Whenever a document is about to be saved, [`Server::on_will_save_wait_until`]
//! gives the server a chance to modify the document before it is saved.

use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use serde::Deserialize;
use super::will_save::TextDocumentSaveReason;
use super::{TextDocumentIdentifer, TextEdit};

#[derive(Default, Clone)]
pub(crate) struct WillSaveWaitUntilOptions;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WillSaveWaitUntilTextDocumentParams {
    text_document: TextDocumentIdentifer,
    reason: TextDocumentSaveReason
}

impl WillSaveWaitUntilOptions {

    pub(crate) const METHOD: &'static str = "textDocument/willSaveWaitUntil";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, WillSaveWaitUntilOptions> {
        Endpoint::new(Callback::request(|_, _: WillSaveWaitUntilTextDocumentParams| Vec::<TextEdit>::new()))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called to [modify a document before it is saved](self).
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as a document is about to be saved:
    ///     * The server instance receiving the response.
    ///     * The [`TextDocumentIdentifer`] of the target document.
    ///     * The [`TextDocumentSaveReason`] that specifies why the document is saved.
    ///     * `return` - A list of edits to apply to the document.

    pub fn on_will_save_wait_until(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, TextDocumentSaveReason) -> Vec<TextEdit>) {
        self.text_document.will_save_wait_until.set_callback(Callback::request(move |server, params: WillSaveWaitUntilTextDocumentParams| {
            callback(server, params.text_document, params.reason)
        }))
    }
}