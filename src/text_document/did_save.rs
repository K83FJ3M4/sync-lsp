use crate::Connection;
use crate::connection::{Callback, Endpoint};
use serde::Deserialize;
use super::TextDocumentIdentifer;
use serde::Serialize;

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DidSaveOptions {
    pub include_text: bool
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DidSaveTextDocumentParams {
    text_document: TextDocumentIdentifer,
    text: Option<String>,
}


impl DidSaveOptions {

    pub(crate) const METHOD: &'static str = "textDocument/didSave";
    
    pub(super) fn endpoint<T>() -> Endpoint<T, DidSaveOptions> {
        Endpoint::new(Callback::notification(|_, _: DidSaveTextDocumentParams| ()))
    }
}

impl<T> Connection<T> {
    pub fn on_did_save(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer, Option<String>)) {
        self.text_document.did_save.set_callback(Callback::notification(move |connection, params: DidSaveTextDocumentParams| {
            callback(connection, params.text_document, params.text)
        }))
    }

    pub fn set_save_options(&mut self, save_options: DidSaveOptions) {
        self.text_document.did_save.set_options(save_options);
    }
}