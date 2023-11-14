use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use super::{TextDocumentIdentifer, TextEdit};

#[derive(Default, Clone)]
pub(crate) struct WillSaveWaitUntilOptions;

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

impl WillSaveWaitUntilOptions {

    pub(crate) const METHOD: &'static str = "textDocument/willSaveWaitUntil";
    
    pub(super) fn endpoint<T>() -> Endpoint<T, WillSaveWaitUntilOptions> {
        Endpoint::new(Callback::request(|_, _: WillSaveWaitUntilTextDocumentParams| Vec::<TextEdit>::new()))
    }
}

impl<T> Connection<T> {
    pub fn on_will_save_wait_until(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer, TextDocumentSaveReason) -> Vec<TextEdit>) {
        self.text_document.will_save_wait_until.set_callback(Callback::request(move |connection, params: WillSaveWaitUntilTextDocumentParams| {
            callback(connection, params.text_document, params.reason)
        }))
    }
}