use crate::Connection;
use crate::connection::Callback;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use super::TextDocumentIdentifer;

pub(crate) struct WillSave<T: 'static>
    (pub(crate) fn(&mut Connection<T>, text_document: TextDocumentIdentifer, reason: TextDocumentSaveReason));

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

impl<T> WillSave<T> {

    pub(crate) const METHOD: &'static str = "textDocument/willSave";
    
    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        let WillSave(callback) = *self;
        Callback::notification(move |connection, params: WillSaveTextDocumentParams| callback(connection, params.text_document, params.reason))
    }
}

impl<T> Connection<T> {
    pub fn on_will_save(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer, TextDocumentSaveReason)) {
        self.text_document.will_save = WillSave(callback);
    }
}

impl<T> Default for WillSave<T> {
    fn default() -> Self {
        WillSave(|_, _, _| {})
    }
}