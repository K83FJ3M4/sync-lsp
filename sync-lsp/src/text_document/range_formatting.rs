//! implementation of the `textDocument/rangeFormatting` reques.
//! 
//! # Usage
//! Via the [`Server::on_range_formatting`] callback a client is able to request the formatting
//! of a specific range in a document.

use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
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

impl<T: TypeProvider> Server<T> {


    /// Sets the callback that will be called to implement [range formatting](self).
    ///
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as a document range is formatted:
    ///     * `server` - The server on which the request was received.
    ///     * `document` - The [`TextDocumentIdentifer`] of the target document.
    ///     * `range` - The [`Range`] that should be formatted.
    ///     * `options` - The [`FormattingOptions`] that specify how the document should be formatted.
    ///     * `return` - A list of edits to apply to the document.

    pub fn on_range_formatting(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, Range, FormattingOptions) -> Vec<TextEdit>) {
        self.text_document.range_formatting.set_callback(Callback::request(move |server, params: DocumentRangeFormattingParams | {
            callback(server, params.text_document, params.range, params.options)
        }))
    }
}