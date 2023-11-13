use crate::Connection;
use crate::connection::Callback;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use super::{TextDocumentIdentifer, TextEdit};

pub(crate) struct WillSaveWaitUntil<T: 'static>
    (pub(crate) fn(&mut Connection<T>, text_document: TextDocumentIdentifer, reason: TextDocumentSaveReason) -> Vec<TextEdit>);

#[repr(i32)]
#[derive(Deserialize_repr, Debug)]
pub enum TextDocumentSaveReason {
    Manual = 1,
    AfterDelay = 2,
    FocusOut = 3
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WillSaveWaitUntilTextDocumentParams {
    text_document: TextDocumentIdentifer,
    reason: TextDocumentSaveReason
}

impl<T> WillSaveWaitUntil<T> {

    pub(crate) const METHOD: &'static str = "textDocument/willSaveWaitUntil";
    
    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        let WillSaveWaitUntil(callback) = *self;
        Callback::request(move |connection, params: WillSaveWaitUntilTextDocumentParams| callback(connection, params.text_document, params.reason))
    }
}

impl<T> Connection<T> {
    pub fn on_will_save_wait_until(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer, TextDocumentSaveReason) -> Vec<TextEdit>) {
        self.text_document.will_save_wait_until = WillSaveWaitUntil(callback);
    }
}

impl<T> Default for WillSaveWaitUntil<T> {
    fn default() -> Self {
        WillSaveWaitUntil(|_, _, _| Vec::new())
    }
}