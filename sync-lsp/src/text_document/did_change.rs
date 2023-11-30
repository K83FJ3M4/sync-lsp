//! implementation of the `textDocument/didChange` notification
//! 
//! # Usage
//! Whenever a document is changed, [`Server::on_change`] is invoked.
//! The client should only send this if it claimed ownership of the document
//! via [`Server::on_open`] before.

use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use serde::Deserialize;
use super::{VersionedTextDocumentIdentifier, Range};

#[derive(Default, Clone)]
pub(crate) struct DidChangeOptions;

/// A change to a text document.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentChangeEvent {
    /// If range is omitted, the new text is considered to be the full content of the document.
    pub range: Option<Range>,
    /// The length of the range that got replaced.
    pub range_length: Option<i32>,
    /// The new text of the range/document.
    pub text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DidChangeTextDocumentParams {
    text_document: VersionedTextDocumentIdentifier,
    content_changes: Vec<TextDocumentContentChangeEvent>
}

impl DidChangeOptions {

    pub(crate) const METHOD: &'static str = "textDocument/didChange";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DidChangeOptions> {
        Endpoint::new(Callback::notification(|_, _: DidChangeTextDocumentParams| ()))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called if a [change to a file](self) is detected.
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as a change is detected:
    ///     * The server instance receiving the response.
    ///     * The [`VersionedTextDocumentIdentifier`] of the document that changed.
    ///     * The [`Vec<TextDocumentContentChangeEvent>`] that contains the changes to the document.
    
    pub fn on_change(&mut self, callback: fn(&mut Server<T>, VersionedTextDocumentIdentifier, Vec<TextDocumentContentChangeEvent>)) {
        self.text_document.did_change.set_callback(Callback::notification(move |server, params: DidChangeTextDocumentParams| {
            callback(server, params.text_document, params.content_changes)
        }));
    }
}