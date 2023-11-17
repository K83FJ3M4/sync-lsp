use crate::TypeProvider;
use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use super::{TextDocumentIdentifer, Position, Range, TextDocumentPositionParams};
use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Default, Clone)]
pub struct DocumentHighlightOptions;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DocumentHighlight  {
    pub range: Range,
    pub kind: Option<DocumentHighlightKind>
}

#[repr(i32)]
#[derive(Serialize_repr, Debug)]
pub enum DocumentHighlightKind {
    Text = 1,
    Read = 2,
    Write = 3
}

impl DocumentHighlightOptions {

    pub(crate) const METHOD: &'static str = "textDocument/documentHighlight";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DocumentHighlightOptions> {
        Endpoint::new(Callback::request(|_, _: TextDocumentPositionParams| Vec::<DocumentHighlight>::new()))
    }
}

impl<T: TypeProvider> Connection<T> {
    pub fn on_document_highlight(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer, Position) -> Vec<DocumentHighlight>) {
        self.text_document.document_highlight.set_callback(Callback::request(move |connection, params: TextDocumentPositionParams| {
            callback(connection, params.text_document, params.position)
        }))
    }
}