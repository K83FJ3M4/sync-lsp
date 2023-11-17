use serde_repr::Serialize_repr;
use serde::Serialize;
use crate::{Connection, TypeProvider};
use crate::connection::RpcConnection;

#[derive(Default)]
pub(super) struct DidChangeWatchedFilesOptions;

#[derive(Serialize, Debug)]
pub struct FileEvent {
    pub uri: String,
    #[serde(rename = "type")]
    pub r#type: FileChangeType
}

#[repr(i32)]
#[derive(Serialize_repr, Debug)]
pub enum FileChangeType {
    Created = 1,
    Changed = 2,
    Deleted = 3
}

#[derive(Serialize)]
struct DidChangeWatchedFilesParams {
    changes: Vec<FileEvent>
}

impl<T: TypeProvider> Connection<T> {
    pub fn did_change_watched_files(&mut self, changes: Vec<FileEvent>) {
        self.notify(
            DidChangeWatchedFilesOptions::METHOD,
            DidChangeWatchedFilesParams {
                changes
            }
        );
    }
}

impl DidChangeWatchedFilesOptions {
    const METHOD: &'static str = "workspace/didChangeWatchedFiles";
}