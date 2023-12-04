//! implementation of the `textDocument/onSave` notification
//! 
//! # Usage
//! Whenever a document is saved, [`Server::on_save`] is invoked. Optionally,
//! [`Server::set_save_include_text`] can be used to include the content of the
//! file on save.


use crate::{Server, TypeProvider};
use crate::connection::{Callback, Endpoint};
use serde::Deserialize;
use super::TextDocumentIdentifer;
use serde::Serialize;

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DidSaveOptions {
    include_text: bool
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DidSaveTextDocumentParams {
    text_document: TextDocumentIdentifer,
    text: Option<String>,
}


impl DidSaveOptions {

    pub(crate) const METHOD: &'static str = "textDocument/didSave";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DidSaveOptions> {
        Endpoint::new(Callback::notification(|_, _: DidSaveTextDocumentParams| ()))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called if a [file is saved](self).
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as a file is saved:
    ///     * The server instance receiving the response.
    ///     * The [`TextDocumentIdentifer`] of the saved document.
    ///     * The content of the file, if enabled via [`Server::set_save_include_text`].

    pub fn on_save(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, Option<String>)) {
        self.text_document.did_save.set_callback(Callback::notification(move |server, params: DidSaveTextDocumentParams| {
            callback(server, params.text_document, params.text)
        }))
    }

    /// Sets whether the content of the file should be included in the [`Server::on_save`] callback.
    /// 
    /// # Argument
    /// * `value` - If `true`, the content of the file will be included in the callback.

    pub fn set_save_include_text(&mut self, value: bool) {
        self.text_document.did_save.options_mut().include_text = value;
    }
}