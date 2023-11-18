use crate::TypeProvider;
use crate::workspace::symbol::SymbolInformation;
use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use super::TextDocumentIdentifer;
use serde::Deserialize;

#[derive(Default, Clone)]
pub struct DocumentSymbolOptions;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DocumentSymbolParams {
    text_document: TextDocumentIdentifer,
}

impl DocumentSymbolOptions {

    pub(crate) const METHOD: &'static str = "textDocument/documentSymbol";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DocumentSymbolOptions> {
        Endpoint::new(Callback::request(|_, _: DocumentSymbolParams| Vec::<SymbolInformation>::new()))
    }
}

impl<T: TypeProvider> Connection<T> {
    pub fn on_document_symbol(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer) -> Vec<SymbolInformation>) {
        self.text_document.document_symbol.set_callback(Callback::request(move |connection, params: DocumentSymbolParams| {
            callback(connection, params.text_document)
        }))
    }
}