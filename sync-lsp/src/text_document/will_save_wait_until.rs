use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use serde::Deserialize;
use super::will_save::TextDocumentSaveReason;
use super::{TextDocumentIdentifer, TextEdit};

#[derive(Default, Clone)]
pub(crate) struct WillSaveWaitUntilOptions;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WillSaveWaitUntilTextDocumentParams {
    text_document: TextDocumentIdentifer,
    reason: TextDocumentSaveReason
}

impl WillSaveWaitUntilOptions {

    pub(crate) const METHOD: &'static str = "textDocument/willSaveWaitUntil";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, WillSaveWaitUntilOptions> {
        Endpoint::new(Callback::request(|_, _: WillSaveWaitUntilTextDocumentParams| Vec::<TextEdit>::new()))
    }
}

impl<T: TypeProvider> Server<T> {
    pub fn on_will_save_wait_until(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, TextDocumentSaveReason) -> Vec<TextEdit>) {
        self.text_document.will_save_wait_until.set_callback(Callback::request(move |server, params: WillSaveWaitUntilTextDocumentParams| {
            callback(server, params.text_document, params.reason)
        }))
    }
}