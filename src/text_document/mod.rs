use crate::{connection::Callback, Connection};
use self::{did_open::DidOpen, did_change::DidChange};
use serde::{Serialize, Deserialize};
use serde_repr::Serialize_repr;

pub mod did_open;
mod did_change;

pub type DocumentUri = String;

#[derive(Deserialize, Debug)]
pub struct Position {
    pub line: i32,
    pub character: i32,
}

#[derive(Deserialize, Debug)]
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
    pub(super) save_options: SaveOptions,
    did_open: DidOpen<T>,
    did_change: DidChange<T>
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
    pub save: SaveOptions
}

#[derive(Serialize, Default, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct SaveOptions {
    pub include_text: bool
}

impl<T> TextDocumentService<T> {
    pub(super) fn resolve(&self, method: &str) -> Option<Callback<Connection<T>>> {
        match method {
            DidOpen::<T>::METHOD => Some(self.did_open.callback()),
            DidChange::<T>::METHOD => Some(self.did_change.callback()),
            _ => None
        }
    }
}

impl<T> Default for TextDocumentService<T> {
    fn default() -> Self {
        TextDocumentService {
            sync_kind: Default::default(),
            save_options: Default::default(),
            did_open: Default::default(),
            did_change: Default::default()
        }
    }
}

impl<T> Connection<T> {
    pub fn set_document_sync(&mut self, sync_kind: TextDocumentSyncKind) {
        self.text_document.sync_kind = sync_kind;
    }

    pub fn set_save_options(&mut self, save_options: SaveOptions) {
        self.text_document.save_options = save_options;
    }
}