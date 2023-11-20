use serde_repr::Deserialize_repr;
use serde::Deserialize;
use crate::{Connection, TypeProvider};
use crate::connection::{Endpoint, Callback};

#[derive(Default, Clone)]
pub(super) struct DidChangeWatchedFilesOptions;

#[derive(Deserialize, Debug)]
pub struct FileEvent {
    pub uri: String,
    #[serde(rename = "type")]
    pub r#type: FileChangeType
}

#[repr(i32)]
#[derive(Deserialize_repr, Debug)]
pub enum FileChangeType {
    Created = 1,
    Changed = 2,
    Deleted = 3
}

#[derive(Deserialize)]
struct DidChangeWatchedFilesParams {
    changes: Vec<FileEvent>
}

impl<T: TypeProvider> Connection<T> {
    pub fn on_change_watched_files(&mut self, callback: fn (&mut Connection<T>, Vec<FileEvent>)) {
        self.workspace.did_change_watched_files.set_callback(Callback::notification(move |connection, params: DidChangeWatchedFilesParams| {
            callback(connection, params.changes)
        }))
    }
}

impl DidChangeWatchedFilesOptions {
    pub(super) const METHOD: &'static str = "workspace/didChangeWatchedFiles";

    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DidChangeWatchedFilesOptions> {
        Endpoint::<T, DidChangeWatchedFilesOptions>::new(
            Callback::notification(|_, _: DidChangeWatchedFilesParams| ())
        )
    }
}