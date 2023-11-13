use crate::Connection;
use crate::connection::Callback;
use serde::Deserialize;
use super::{VersionedTextDocumentIdentifier, Range};

pub(crate) struct DidChange<T: 'static>
    (pub(crate) fn(&mut Connection<T>, text_document: VersionedTextDocumentIdentifier, content_changes: Vec<TextDocumentContentChangeEvent>));

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentChangeEvent {
    pub range: Option<Range>,
    pub range_length: Option<i32>,
    pub text: String,
}

#[derive(Deserialize)]
struct DidChangeTextDocumentParams {
    text_document: VersionedTextDocumentIdentifier,
    content_changes: Vec<TextDocumentContentChangeEvent>
}

impl<T> DidChange<T> {

    pub(crate) const METHOD: &'static str = "textDocument/didChange";
    
    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        let DidChange(callback) = *self;
        Callback::notification(move |connection, params: DidChangeTextDocumentParams| callback(connection, params.text_document, params.content_changes))
    }
}

impl<T> Connection<T> {
    pub fn on_did_change(&mut self, callback: fn(&mut Connection<T>, VersionedTextDocumentIdentifier, Vec<TextDocumentContentChangeEvent>)) {
        self.text_document.did_change = DidChange(callback);
    }
}

impl<T> Default for DidChange<T> {
    fn default() -> Self {
        DidChange(|_, _, _| {})
    }
}