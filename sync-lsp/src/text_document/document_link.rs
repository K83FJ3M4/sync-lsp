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

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentLink {
    pub range: Range,
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
    pub fn on_document_link(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer) -> Vec<DocumentLink>) {
        self.text_document.document_link.set_callback(Callback::request(move |server, params: DocumentLinkParams| {
            callback(server, params.text_document)
        }));
    }

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