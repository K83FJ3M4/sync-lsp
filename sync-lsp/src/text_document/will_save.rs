use crate::{Server, TypeProvider};
use crate::connection::{Callback, Endpoint};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use super::TextDocumentIdentifer;

#[derive(Default, Clone)]
pub(crate) struct WillSaveOptions;

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

impl WillSaveOptions {

    pub(crate) const METHOD: &'static str = "textDocument/willSave";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, WillSaveOptions> {
        Endpoint::new(Callback::notification(|_, _: WillSaveTextDocumentParams| ()))
    }
}

impl<T: TypeProvider> Server<T> {
    pub fn on_will_save(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, TextDocumentSaveReason)) {
        self.text_document.will_save.set_callback(Callback::notification(move |server, params: WillSaveTextDocumentParams| {
            callback(server, params.text_document, params.reason)
        }))
    }
}