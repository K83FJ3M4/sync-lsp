use crate::connection::Endpoint;
use crate::{connection::Callback, Connection};
use self::completion::{CompletionOptions, ResolveCompletionOptions};
use self::publish_diagnostics::PublishDiagnosticsOptions;
use self::{did_open::DidOpenOptions, did_change::DidChangeOptions, will_save::WillSaveOptions, will_save_wait_until::WillSaveWaitUntilOptions, did_save::DidSaveOptions, did_close::DidCloseOptions};
use serde::{Serialize, Deserialize};
use serde_repr::Serialize_repr;

pub mod did_open;
pub mod did_change;
pub mod will_save;
mod will_save_wait_until;
mod did_save;
mod did_close;
pub mod publish_diagnostics;
pub mod completion;

pub type DocumentUri = String;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TextDocumentPositionParams {
    pub text_document: TextDocumentIdentifer,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

#[derive(Serialize, Debug)]
pub struct Location {
    pub uri: DocumentUri,
    pub range: Range,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentIdentifer {
    pub uri: DocumentUri,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Position {
    pub line: i32,
    pub character: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Deserialize, Debug)]
pub struct VersionedTextDocumentIdentifier {
    pub uri: DocumentUri,
    pub version: i32,
}

pub(super) struct TextDocumentService<T: 'static> {
    pub(super) sync_kind: TextDocumentSyncKind,
    did_open: Endpoint<T, DidOpenOptions>,
    did_change: Endpoint<T, DidChangeOptions>,
    will_save: Endpoint<T, WillSaveOptions>,
    will_save_wait_until: Endpoint<T, WillSaveWaitUntilOptions>,
    pub(super) did_save: Endpoint<T, DidSaveOptions>,
    did_close: Endpoint<T, DidCloseOptions>,
    #[allow(unused)]
    publish_diagnostics: PublishDiagnosticsOptions,
    pub(super) completion: Endpoint<T, CompletionOptions>,
    resolve_completion: Endpoint<T, ResolveCompletionOptions>
}

#[repr(i32)]
#[derive(Serialize_repr, Default, Clone, Copy)]
pub enum TextDocumentSyncKind {
    None = 0,
    Full = 1,
    #[default]
    Incremental = 2
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TextDocumentSyncOptions {
    pub open_close: bool,
    pub change: TextDocumentSyncKind,
    pub will_save: bool,
    pub will_save_wait_until: bool,
    pub save: DidSaveOptions
}

impl<T> TextDocumentService<T> {
    pub(super) fn resolve(&self, method: &str) -> Option<Callback<Connection<T>>> {
        match method {
            DidOpenOptions::METHOD => Some(self.did_open.callback()),
            DidChangeOptions::METHOD => Some(self.did_change.callback()),
            WillSaveOptions::METHOD => Some(self.will_save.callback()),
            WillSaveWaitUntilOptions::METHOD => Some(self.will_save_wait_until.callback()),
            DidSaveOptions::METHOD => Some(self.did_save.callback()),
            DidCloseOptions::METHOD => Some(self.did_close.callback()),
            CompletionOptions::METHOD => Some(self.completion.callback()),
            ResolveCompletionOptions::METHOD => Some(self.resolve_completion.callback()),
            _ => None
        }
    }
}

impl<T> Default for TextDocumentService<T> {
    fn default() -> Self {
        TextDocumentService {
            sync_kind: Default::default(),
            did_open: DidOpenOptions::endpoint(),
            did_change: DidChangeOptions::endpoint(),
            will_save: WillSaveOptions::endpoint(),
            will_save_wait_until: WillSaveWaitUntilOptions::endpoint(),
            did_save: DidSaveOptions::endpoint(),
            did_close: DidCloseOptions::endpoint(),
            publish_diagnostics: Default::default(),
            completion: CompletionOptions::endpoint(),
            resolve_completion: ResolveCompletionOptions::endpoint()
        }
    }
}

impl<T> Connection<T> {
    pub fn set_document_sync(&mut self, sync_kind: TextDocumentSyncKind) {
        self.text_document.sync_kind = sync_kind;
    }
}