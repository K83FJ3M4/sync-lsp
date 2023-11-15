use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use super::{TextDocumentIdentifer, TextEdit};
use serde::Deserialize;

#[derive(Default, Clone)]
pub(crate) struct DocumentFormattingOptions;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentFormattingParams {
    text_document: TextDocumentIdentifer,
    options: FormattingOptions
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FormattingOptions<T = ()> {
    pub tab_size: u32,
    pub insert_spaces: bool,
    #[serde(flatten)]
    pub properties: T
}

impl DocumentFormattingOptions {

    pub(crate) const METHOD: &'static str = "textDocument/formatting";
    
    pub(super) fn endpoint<T>() -> Endpoint<T, DocumentFormattingOptions> {
        Endpoint::new(Callback::request(|_, _: DocumentFormattingParams| Vec::<TextEdit>::new()))
    }
}

impl<T> Connection<T> {
    pub fn on_formatting(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer, FormattingOptions) -> Vec<TextEdit>) {
        self.text_document.formatting.set_callback(Callback::request(move |connection, params: DocumentFormattingParams | {
            callback(connection, params.text_document, params.options)
        }))
    }
}