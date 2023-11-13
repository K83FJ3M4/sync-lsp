use crate::Connection;
use crate::connection::Callback;
use serde::Deserialize;
use super::TextDocumentIdentifer;

pub(crate) struct DidClose<T: 'static>
    (pub(crate) fn(&mut Connection<T>, text_document: TextDocumentIdentifer));

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DidCloseTextDocumentParams {
    text_document: TextDocumentIdentifer,
}


impl<T> DidClose<T> {

    pub(crate) const METHOD: &'static str = "textDocument/didClose";
    
    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        let DidClose(callback) = *self;
        Callback::notification(move |connection, params: DidCloseTextDocumentParams| callback(connection, params.text_document))
    }
}

impl<T> Connection<T> {
    pub fn on_did_close(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer)) {
        self.text_document.did_close = DidClose(callback);
    }
}

impl<T> Default for DidClose<T> {
    fn default() -> Self {
        DidClose(|_, _| {})
    }
}