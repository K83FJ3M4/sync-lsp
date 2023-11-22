use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use super::{TextDocumentIdentifer, Position, Location};
use serde::Deserialize;

#[derive(Default, Clone)]
pub struct ReferenceOptions;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReferenceParams {
    text_document: TextDocumentIdentifer,
    position: Position,
    context: ReferenceContext
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceContext {
    pub include_declaration: bool
}

impl ReferenceOptions {

    pub(crate) const METHOD: &'static str = "textDocument/references";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, ReferenceOptions> {
        Endpoint::new(Callback::request(|_, _: ReferenceParams| Vec::<Location>::new()))
    }
}

impl<T: TypeProvider> Server<T> {
    pub fn on_references(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, Position, context: ReferenceContext) -> Vec<Location>) {
        self.text_document.references.set_callback(Callback::request(move |server, params: ReferenceParams| {
            callback(server, params.text_document, params.position, params.context)
        }))
    }
}