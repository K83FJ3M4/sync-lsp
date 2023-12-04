//! implementation of the hover request
//! 
//! # Usage
//! Additional information about a specific symbol in the document can be requested via [`Server::on_hover`].
//! by the client.

use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use serde::Serialize;
use super::{TextDocumentIdentifer, TextDocumentPositionParams, Range, Position};

#[derive(Default, Clone)]
pub(crate) struct HoverOptions;

/// A hover represents additional information for a symbol.
#[derive(Serialize, Debug, Default)]
pub struct Hover {
    /// The information to display.
    pub contents: Vec<MarkedString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// This is used to highlight the specific range in the editor.
    pub range: Option<Range>
}

/// This may either be a markdown string or a language snippet.
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum MarkedString {
    /// A markdown string.
    String(String),
    /// A language snippet.
    LanguageString {
        language: String,
        value: String
    }
}

impl HoverOptions {

    pub(crate) const METHOD: &'static str = "textDocument/hover";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, HoverOptions> {
        Endpoint::new(Callback::request(|_, _: TextDocumentPositionParams| Hover::default()))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called to [compute hover information](self).
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as hover information is requested:
    ///     * The server instance receiving the response.
    ///     * The [`TextDocumentIdentifer`] of the target document.
    ///     * The [`Position`] of the cursor.
    ///     * `return` - The hover information to display.

    pub fn on_hover(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, Position) -> Hover) {
        self.text_document.hover.set_callback(Callback::request(move |server, params: TextDocumentPositionParams| {
            callback(server, params.text_document, params.position)
        }))
    }
}