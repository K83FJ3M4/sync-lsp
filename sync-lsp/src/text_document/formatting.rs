//! implementation of the `textDocument/formatting` request
//! 
//! # Usage
//! Some server may be capable of formatting a document. If so, [`Server::on_formatting`] can be used to
//! provide this functionality to the client. It can be triggered either manually or automatically.

use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
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

/// Specifies how a document should be formatted.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FormattingOptions<T = ()> {
    /// The size of tabs in spaces.
    pub tab_size: u32,
    /// Specifies whether tabs should be used instead of spaces.
    pub insert_spaces: bool,
    /// Additional options can be handled by the server if a struct with the corresponding fields is provided here.
    #[serde(flatten)]
    pub properties: T
}

impl DocumentFormattingOptions {

    pub(crate) const METHOD: &'static str = "textDocument/formatting";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DocumentFormattingOptions> {
        Endpoint::new(Callback::request(|_, _: DocumentFormattingParams| Vec::<TextEdit>::new()))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called to [format a document](self).
    ///
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as a document is formatted:
    ///     * `server` - The server on which the request was received.
    ///     * `document` - The [`TextDocumentIdentifer`] of the target document.
    ///     * `options` - The [`FormattingOptions`] that specify how the document should be formatted.
    ///     * `return` - A list of edits to apply to the document.

    pub fn on_formatting(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, FormattingOptions) -> Vec<TextEdit>) {
        self.text_document.formatting.set_callback(Callback::request(move |server, params: DocumentFormattingParams | {
            callback(server, params.text_document, params.options)
        }))
    }
}