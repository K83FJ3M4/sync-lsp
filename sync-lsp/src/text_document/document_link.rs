//! implementation of the `textDocument/documentLink` request
//! 
//! # Usage
//! When a file is opened, the client may ask the server to provide links to other files via [`Server::on_document_link`].
//! These links can then be used to navigate to related files. Additionally, the server can choose not to
//! include the target of the link immediately. Instead [`Server::on_document_link_resolve`] can be used to
//! compute it separately.

use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use serde::{Deserialize, Serialize};
use super::{TextDocumentIdentifer, Range, DocumentUri};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DocumentLinkOptions {
    resolve_provider: bool
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DocumentLinkResolveOptions;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentLinkParams {
    text_document: TextDocumentIdentifer,
}

/// Specifies a link to another file.
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentLink {
    /// The range inside the current document, that will redirect to the target.
    pub range: Range,
    /// The uri of the file to link to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<DocumentUri>
}

impl DocumentLinkOptions {
    pub(crate) const METHOD: &'static str = "textDocument/documentLink";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DocumentLinkOptions> {
        Endpoint::new(Callback::request(|_, _: DocumentLinkParams| Vec::<DocumentLink>::new()))
    }
}

impl DocumentLinkResolveOptions {
    pub(crate) const METHOD: &'static str = "documentLink/resolve";
        
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DocumentLinkResolveOptions> {
        Endpoint::new(Callback::request(|_, lens: DocumentLink| lens))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called to [compute document links](self).
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as links is requested:
    ///     * The server instance receiving the response.
    ///     * The [`TextDocumentIdentifer`] of the document that has been opened.
    ///     * `return` - A list of links to display.

    pub fn on_document_link(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer) -> Vec<DocumentLink>) {
        self.text_document.document_link.set_callback(Callback::request(move |server, params: DocumentLinkParams| {
            callback(server, params.text_document)
        }));
    }

    /// Sets the callback that will be called to compute missing information on [document links](self), which were previously returned by [`Server::on_document_link`].
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon a link can be resolved:
    ///     * The server instance receiving the response.
    ///     * The [`DocumentLink`] to resolve.
    ///     * `return` - The resolved link.

    pub fn on_document_link_resolve(&mut self, callback: fn(&mut Server<T>, DocumentLink) -> DocumentLink) {
        self.text_document.resolve_document_link.set_callback(Callback::request(move |server, params| {
            callback(server, params)
        }));
    }
}

impl Default for DocumentLinkOptions {
    fn default() -> Self {
        Self {
            resolve_provider: false
        }
    }
}