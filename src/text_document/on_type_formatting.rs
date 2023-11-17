use crate::TypeProvider;
use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use super::formatting::FormattingOptions;
use super::{TextDocumentIdentifer, TextEdit, Position};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DocumentOnTypeFormattingOptions {
    first_trigger_character: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    more_trigger_character: Vec<String>
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
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DocumentOnTypeFormattingOptions> {
        Endpoint::new(Callback::request(|_, _: DocumentOnTypeFormattingParams| Vec::<TextEdit>::new()))
    }
}

impl<T: TypeProvider> Connection<T> {
    pub fn on_type_formatting(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer, Position, String, FormattingOptions) -> Vec<TextEdit>) {
        self.text_document.on_type_formatting.set_callback(Callback::request(move |connection, params: DocumentOnTypeFormattingParams | {
            callback(connection, params.text_document, params.position, params.ch, params.options)
        }))
    }
    
    pub fn set_on_type_formatting_first_trigger_character(&mut self, value: String) {
        self.text_document.on_type_formatting.options_mut().first_trigger_character = value;
    }

    pub fn set_on_type_formatting_more_trigger_characters(&mut self, value: Vec<String>) {
        self.text_document.on_type_formatting.options_mut().more_trigger_character = value;
    }
}