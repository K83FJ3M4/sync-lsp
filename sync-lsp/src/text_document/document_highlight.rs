use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use super::{TextDocumentIdentifer, Position, Range, TextDocumentPositionParams};
use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Default, Clone)]
pub(crate) struct DocumentHighlightOptions;

/// A highlighted region of a document.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DocumentHighlight  {
    /// The range to highlight.
    pub range: Range,
    /// The kind of highlight, defaults to [`DocumentHighlightKind::Text`]
    pub kind: Option<DocumentHighlightKind>
}

/// This enum changes the way highlights are rendered.
#[repr(i32)]
#[derive(Serialize_repr, Debug)]
pub enum DocumentHighlightKind {
    /// A textual symbol.
    Text = 1,
    /// A immutable symbol, like a constant variable.
    Read = 2,
    /// A mutable symbol, like a variable.
    Write = 3
}

impl DocumentHighlightOptions {

    pub(crate) const METHOD: &'static str = "textDocument/documentHighlight";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DocumentHighlightOptions> {
        Endpoint::new(Callback::request(|_, _: TextDocumentPositionParams| Vec::<DocumentHighlight>::new()))
    }
}

impl<T: TypeProvider> Server<T> {
    pub fn on_document_highlight(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, Position) -> Vec<DocumentHighlight>) {
        self.text_document.document_highlight.set_callback(Callback::request(move |server, params: TextDocumentPositionParams| {
            callback(server, params.text_document, params.position)
        }))
    }
}