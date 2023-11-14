use crate::Connection;
use crate::connection::{Callback, Endpoint};
use serde::Deserialize;
use super::TextDocumentIdentifer;

#[derive(Default, Clone)]
pub(crate) struct DidCloseOptions;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DidCloseTextDocumentParams {
    text_document: TextDocumentIdentifer,
}


impl DidCloseOptions {

    pub(crate) const METHOD: &'static str = "textDocument/didClose";

    pub(crate) fn endpoint<T>() -> Endpoint<T, DidCloseOptions> {
        Endpoint::new(Callback::notification(|_, _: DidCloseTextDocumentParams| {

        }))
    }
}

impl<T> Connection<T> {
    pub fn on_did_close(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer)) {
        self.text_document.did_close.set_callback(Callback::notification(move |connection, params: DidCloseTextDocumentParams| {
            callback(connection, params.text_document)
        }))
    }
}