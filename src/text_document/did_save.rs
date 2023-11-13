use crate::Connection;
use crate::connection::Callback;
use serde::Deserialize;
use super::TextDocumentIdentifer;

pub(crate) struct DidSave<T: 'static>
    (pub(crate) fn(&mut Connection<T>, text_document: TextDocumentIdentifer, text: Option<String>));

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DidSaveTextDocumentParams {
    text_document: TextDocumentIdentifer,
    text: Option<String>,
}


impl<T> DidSave<T> {

    pub(crate) const METHOD: &'static str = "textDocument/didSave";
    
    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        let DidSave(callback) = *self;
        Callback::notification(move |connection, params: DidSaveTextDocumentParams| callback(connection, params.text_document, params.text))
    }
}

impl<T> Connection<T> {
    pub fn on_did_save(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer, Option<String>)) {
        self.text_document.did_save = DidSave(callback);
    }
}

impl<T> Default for DidSave<T> {
    fn default() -> Self {
        DidSave(|_, _, _| {})
    }
}