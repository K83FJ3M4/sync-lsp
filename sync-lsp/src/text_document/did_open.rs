use crate::{Server, TypeProvider};
use crate::connection::{Callback, Endpoint};
use serde::Deserialize;
use super::DocumentUri;

#[derive(Default, Clone)]
pub(crate) struct DidOpenOptions;

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

impl DidOpenOptions {

    pub(crate) const METHOD: &'static str = "textDocument/didOpen";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DidOpenOptions> {
        Endpoint::new(Callback::notification(|_, _: DidOpenParams| ()))
    }
}

impl<T: TypeProvider> Server<T> {
    pub fn on_open(&mut self, callback: fn(&mut Server<T>, TextDocumentItem)) {
        self.text_document.did_open.set_callback(Callback::notification(move |server, params: DidOpenParams| {
            callback(server, params.text_document)
        }))
    }
}