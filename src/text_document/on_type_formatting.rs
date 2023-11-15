use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use super::formatting::FormattingOptions;
use super::{TextDocumentIdentifer, TextEdit, Position};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DocumentOnTypeFormattingOptions {
    pub first_trigger_character: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub more_trigger_character: Vec<String>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentOnTypeFormattingParams {
    text_document: TextDocumentIdentifer,
    position: Position,
    ch: String,
    options: FormattingOptions
}

impl DocumentOnTypeFormattingOptions {

    pub(crate) const METHOD: &'static str = "textDocument/onTypeFormatting";
    
    pub(super) fn endpoint<T>() -> Endpoint<T, DocumentOnTypeFormattingOptions> {
        Endpoint::new(Callback::request(|_, _: DocumentOnTypeFormattingParams| Vec::<TextEdit>::new()))
    }
}

impl<T> Connection<T> {
    pub fn on_type_formatting(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer, Position, String, FormattingOptions) -> Vec<TextEdit>) {
        self.text_document.on_type_formatting.set_callback(Callback::request(move |connection, params: DocumentOnTypeFormattingParams | {
            callback(connection, params.text_document, params.position, params.ch, params.options)
        }))
    }

    pub fn set_on_type_formatting_options(&mut self, options: DocumentOnTypeFormattingOptions) {
        self.text_document.on_type_formatting.set_options(options)
    }
}