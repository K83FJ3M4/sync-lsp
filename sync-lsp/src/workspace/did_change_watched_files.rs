//! implementation of the `workspace/didChangeWatchedFiles` notification
//! 
//! # Usage
//! The [`Server::on_change_watched_files`] notification signals changes to files or folders watched by the server.
//! The recomended way for servers to collect file changes is this notification, 
//! tough it's not the only way. Servers may use other ways like polling file system for changes.

use serde_repr::Deserialize_repr;
use serde::Deserialize;
use crate::{Server, TypeProvider};
use crate::connection::{Endpoint, Callback};

/// Options for an [`Endpoint`] struct.
#[derive(Default, Clone)]
pub(super) struct DidChangeWatchedFilesOptions;

/// A file event that is sent by the client when a file is created, changed or deleted.
#[derive(Deserialize, Debug)]
pub struct FileEvent {
    /// A file URI.
    pub uri: String,
    /// The file change type as defined in [`FileChangeType`].
    #[serde(rename = "type")]
    pub r#type: FileChangeType
}

/// The file change type attached to every [`FileEvent`].
#[repr(i32)]
#[derive(Deserialize_repr, Debug)]
pub enum FileChangeType {
    /// Denotes the creation of a file.
    Created = 1,
    /// Denotes a change to a file.
    Changed = 2,
    /// Denotes the deletion of a file.
    Deleted = 3
}

/// The parameters of a [`DidChangeWatchedFilesOptions::METHOD`] notification.
#[derive(Deserialize)]
struct DidChangeWatchedFilesParams {
    changes: Vec<FileEvent>
}

impl<T: TypeProvider> Server<T> {
    
    /// Sets the callback that will be called when the client sends a change watched files notification.
    /// 
    /// # Arguments
    /// * `callback` - A function for handling the change of watched files.
    /// The first argument is the server, the second one is a vector of [`FileEvent`] elements.

    pub fn on_change_watched_files(&mut self, callback: fn (&mut Server<T>, Vec<FileEvent>)) {
        self.workspace.did_change_watched_files.set_callback(Callback::notification(move |server, params: DidChangeWatchedFilesParams| {
            callback(server, params.changes)
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