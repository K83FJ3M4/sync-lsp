use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use serde::{Deserialize, Serialize};
use super::{TextDocumentIdentifer, Range, DocumentUri};

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DocumentLinkOptions {
    pub resolve_provider: bool
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub(super) struct DocumentLinkResolveOptions;

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
    
    pub(super) fn endpoint<T>() -> Endpoint<T, DocumentLinkOptions> {
        Endpoint::new(Callback::request(|_, _: DocumentLinkParams| Vec::<DocumentLink>::new()))
    }
}

impl DocumentLinkResolveOptions {
    pub(crate) const METHOD: &'static str = "documentLink/resolve";
        
    pub(super) fn endpoint<T>() -> Endpoint<T, DocumentLinkResolveOptions> {
        Endpoint::new(Callback::request(|_, lens: DocumentLink| lens))
    }
}

impl<T> Connection<T> {
    pub fn on_document_link(
            &mut self,
            callback: fn(&mut Connection<T>, TextDocumentIdentifer) -> Vec<DocumentLink>,
            resolve: fn(&mut Connection<T>, DocumentLink) -> DocumentLink
        ) {
        self.text_document.document_link.set_callback(Callback::request(move |connection, params: DocumentLinkParams| {
            callback(connection, params.text_document)
        }));
        self.text_document.resolve_document_link.set_callback(Callback::request(move |connection, params| {
            resolve(connection, params)
        }));
    }

    pub fn set_document_link_options(&mut self, options: DocumentLinkOptions) {
        self.text_document.document_link.set_options(options)
    }
}