use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use serde::Deserialize;
use super::{VersionedTextDocumentIdentifier, Range};

#[derive(Default, Clone)]
pub(crate) struct DidChangeOptions;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentChangeEvent {
    pub range: Option<Range>,
    pub range_length: Option<i32>,
    pub text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DidChangeTextDocumentParams {
    text_document: VersionedTextDocumentIdentifier,
    content_changes: Vec<TextDocumentContentChangeEvent>
}

impl DidChangeOptions {

    pub(crate) const METHOD: &'static str = "textDocument/didChange";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DidChangeOptions> {
        Endpoint::new(Callback::notification(|_, _: DidChangeTextDocumentParams| ()))
    }
}

impl<T: TypeProvider> Server<T> {
    pub fn on_change(&mut self, callback: fn(&mut Server<T>, VersionedTextDocumentIdentifier, Vec<TextDocumentContentChangeEvent>)) {
        self.text_document.did_change.set_callback(Callback::notification(move |server, params: DidChangeTextDocumentParams| {
            callback(server, params.text_document, params.content_changes)
        }));
    }
}