use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use super::{TextDocumentIdentifer, TextDocumentPositionParams, Location, Position};

#[derive(Default, Clone)]
pub(crate) struct DefinitionOptions;

impl DefinitionOptions {

    pub(crate) const METHOD: &'static str = "textDocument/definition";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DefinitionOptions> {
        Endpoint::new(Callback::request(|_, _: TextDocumentPositionParams| Vec::<Location>::new()))
    }
}

impl<T: TypeProvider> Server<T> {
    pub fn on_definition(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, Position) -> Vec<Location>) {
        self.text_document.definition.set_callback(Callback::request(move |server, params: TextDocumentPositionParams | {
            callback(server, params.text_document, params.position)
        }))
    }
}