use crate::Connection;
use crate::connection::Callback;
use serde::Deserialize;
use super::DocumentUri;

pub(crate) struct DidOpen<T: 'static>
    (pub(crate) fn(&mut Connection<T>, text_document: TextDocumentItem));

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentItem {
    pub uri: DocumentUri,
    pub language_id: String,
    pub version: i32,
    pub text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DidOpenParams {
    text_document: TextDocumentItem,
}

impl<T> DidOpen<T> {

    pub(crate) const METHOD: &'static str = "textDocument/didOpen";
    
    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        let DidOpen(callback) = *self;
        Callback::notification(move |connection, params: DidOpenParams| callback(connection, params.text_document))
    }
}

impl<T> Connection<T> {
    pub fn on_did_open(&mut self, callback: fn(&mut Connection<T>, TextDocumentItem)) {
        self.text_document.did_open = DidOpen(callback);
    }
}

impl<T> Default for DidOpen<T> {
    fn default() -> Self {
        DidOpen(|_, _| {})
    }
}