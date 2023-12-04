//! implementation of the `textDocument/willSave` notification.
//! 
//! # Usage
//! Whenever a document is about to be saved, [`Server::on_will_save`] is called
//! to notify the server.

use crate::{Server, TypeProvider};
use crate::connection::{Callback, Endpoint};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use super::TextDocumentIdentifer;

#[derive(Default, Clone)]
pub(crate) struct WillSaveOptions;

/// The reason why a text document is saved.
#[repr(i32)]
#[derive(Deserialize_repr, Debug)]
pub enum TextDocumentSaveReason {
    Manual = 1,
    AfterDelay = 2,
    FocusOut = 3
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WillSaveTextDocumentParams {
    text_document: TextDocumentIdentifer,
    reason: TextDocumentSaveReason
}

impl WillSaveOptions {

    pub(crate) const METHOD: &'static str = "textDocument/willSave";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, WillSaveOptions> {
        Endpoint::new(Callback::notification(|_, _: WillSaveTextDocumentParams| ()))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called to [notify the server that a document is about to be saved](self).
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as a document is about to be saved:
    ///     * The server instance receiving the response.
    ///     * The [`TextDocumentIdentifer`] of the target document.
    ///     * The [`TextDocumentSaveReason`] that specifies why the document is saved. 
    
    pub fn on_will_save(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, TextDocumentSaveReason)) {
        self.text_document.will_save.set_callback(Callback::notification(move |server, params: WillSaveTextDocumentParams| {
            callback(server, params.text_document, params.reason)
        }))
    }
}