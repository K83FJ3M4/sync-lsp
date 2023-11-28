//! Implementation of the `workspace/applyEdit` request.
//! 
//! # Usage
//! Many `Command` implementations will want to apply edits to the workspace and 
//! may do so by using this request via [`Connection::apply_edit`].
//! The server shouldn't assume that the edit will be immediately applied.
//! Instead, the [`Server::on_apply_edit_response`] callback should be used to check whether
//! the edit was applied or not.

use std::collections::HashMap;
use crate::text_document::{DocumentUri, TextEdit};
use serde::{Serialize, Deserialize};
use crate::connection::{RpcConnection, Callback};
use crate::{Server, Connection, TypeProvider};

/// A workspace edit represents changes to many resources managed in the workspace.
#[derive(Serialize, Debug, Default)]
pub struct WorkspaceEdit {
    /// Holds changes to existing resources.
    pub changes: HashMap<DocumentUri, Vec<TextEdit>>
}

pub(super) struct ApplyEdit<T: TypeProvider> {
    callback: Callback<Server<T>>
}

#[derive(Serialize)]
struct ApplyWorkspaceEditParams {
    edit: WorkspaceEdit
}

/// This response is sent when the [`Connection::apply_edit`] request has been completed
/// and the edit has been applied.
#[derive(Deserialize, Debug, Default)]
pub struct ApplyWorkspaceEditResponse {
    /// Indicates whether the edit was applied or not.
    pub applied: bool
}

impl<T: TypeProvider> Connection<T> {

    /// Request the [application of a workspace edit](self)
    ///
    /// # Arguments
    /// * `tag` - A tag of type [`TypeProvider::ApplyEditData`] preserved throughout the request.
    /// * `edit` - The workspace edit to apply.
    /// * `result` - A boolean indicating whether the request was sent.
    
    pub fn apply_edit(&mut self, tag: T::ApplyEditData, edit: WorkspaceEdit) -> bool {
        self.request(
            ApplyEdit::<T>::METHOD,
            tag,
            ApplyWorkspaceEditParams { edit }
        )
    }
}

impl<T: TypeProvider> Server<T> {
    
    /// Set the response handler for [applying a workspace edit](self)
    ///
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as a response from [`Connection::apply_edit`] is received:
    ///     * The server instance receiving the response.
    ///     * A tag of type [`TypeProvider::ApplyEditData`] that was passed to the request.
    ///     * The response data of the client.

    pub fn on_apply_edit_response(&mut self, callback: fn(&mut Server<T>, T::ApplyEditData, ApplyWorkspaceEditResponse)) {
        self.workspace.apply_edit.callback = Callback::response(callback);
    }
}

impl<T: TypeProvider> Default for ApplyEdit<T> {
    fn default() -> Self {
        Self {
            callback: Callback::response(|_, _: T::ApplyEditData, _: ApplyWorkspaceEditResponse| ())
        }
    }
}

impl<T: TypeProvider> ApplyEdit<T> {
    pub(super) const METHOD: &'static str = "workspace/applyEdit";

    pub(crate) fn callback(&self) -> Callback<Server<T>> {
        self.callback.clone()
    }
}