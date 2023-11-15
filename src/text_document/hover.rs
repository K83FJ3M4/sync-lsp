use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use serde::{Deserialize, Serialize};
use super::will_save::TextDocumentSaveReason;
use super::{TextDocumentIdentifer, TextEdit, TextDocumentPositionParams, Range, Position};

#[derive(Default, Clone)]
pub(crate) struct HoverOptions;

#[derive(Serialize, Debug, Default)]
pub struct Hover {
    pub contents: Vec<MarkedString>,
    pub range: Option<Range>
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum MarkedString {
    String(String),
    LanguageString {
        language: String,
        value: String
    }
}

impl HoverOptions {

    pub(crate) const METHOD: &'static str = "textDocument/hover";
    
    pub(super) fn endpoint<T>() -> Endpoint<T, HoverOptions> {
        Endpoint::new(Callback::request(|_, _: TextDocumentPositionParams| Hover::default()))
    }
}

impl<T> Connection<T> {
    pub fn on_hover(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer, Position) -> Hover) {
        self.text_document.hover.set_callback(Callback::request(move |connection, params: TextDocumentPositionParams| {
            callback(connection, params.text_document, params.position)
        }))
    }
}