use crate::TypeProvider;
use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use super::formatting::FormattingOptions;
use super::{TextDocumentIdentifer, TextEdit, Range};
use serde::Deserialize;

#[derive(Default, Clone)]
pub(crate) struct RangeFormattingOptions;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentRangeFormattingParams {
    text_document: TextDocumentIdentifer,
    range: Range,
    options: FormattingOptions
}

impl RangeFormattingOptions {

    pub(crate) const METHOD: &'static str = "textDocument/rangeFormatting";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, RangeFormattingOptions> {
        Endpoint::new(Callback::request(|_, _: DocumentRangeFormattingParams| Vec::<TextEdit>::new()))
    }
}

impl<T: TypeProvider> Connection<T> {
    pub fn on_range_formatting(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer, Range, FormattingOptions) -> Vec<TextEdit>) {
        self.text_document.range_formatting.set_callback(Callback::request(move |connection, params: DocumentRangeFormattingParams | {
            callback(connection, params.text_document, params.range, params.options)
        }))
    }
}