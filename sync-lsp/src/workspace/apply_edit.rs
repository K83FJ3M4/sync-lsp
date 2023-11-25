//! Implementation of the `workspace/applyEdit` request.
//! 
//! # Usage
//! Many `Command` implementations will want to apply edits to the workspace and 
//! may do so by using this request via [`Connection::apply_edit`]. The server shouldn't assume that the edit will be applied.
//! Instead, the [`Server::on_apply_edit_response`] callback should be used to check whether
//! the edit was applied or not.

use std::collections::HashMap;
use crate::text_document::{DocumentUri, TextEdit};
use serde::{Serialize, Deserialize};
use crate::connection::{RpcConnection, Callback, CancellationToken};
use crate::{Server, Connection, TypeProvider};

/// A workspace edit represents changes to many resources managed in the workspace.
#[derive(Serialize, Debug, Default)]
pub struct WorkspaceEdit {
    /// Holds changes to existing resources.
    pub changes: HashMap<DocumentUri, Vec<TextEdit>>
}

/// This struct provides a callback, but doesn't need to be used with an [`Endpoint`].
pub(super) struct ApplyEdit<T: TypeProvider> {
    callback: Callback<Server<T>>
}

/// The parameters passed to the [`Connection::apply_edit`] request.
#[derive(Serialize)]
struct ApplyWorkspaceEditParams {
    edit: WorkspaceEdit
}

#[derive(Deserialize, Debug, Default)]
pub struct ApplyWorkspaceEditResponse {
    /// Indicates whether the edit was applied or not.
    pub applied: bool
}

impl<T: TypeProvider> Connection<T> {

    /// This request is used to apply a [`WorkspaceEdit`] to resources on the client side.
    /// The result and tag of the request can be retrieved using the corresponding [`Server::on_apply_edit_response`] method.
    ///
    /// # Arguments
    /// * `tag` - A tag that will be passed to the [`Server::on_apply_edit_response`] callback.
    /// * `edit` - The workspace edit to apply.
    /// * `result` - A cancellation token that can be used to cancel the request.
    
    pub fn apply_edit(&mut self, tag: T::ApplyEditData, edit: WorkspaceEdit) -> Option<CancellationToken> {
        self.request(
            ApplyEdit::<T>::METHOD,
            tag,
            ApplyWorkspaceEditParams { edit }
        ).map(|id| id.into())
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called when the client responds to an [`Connection::apply_edit`] request.
    ///
    /// # Arguments
    /// * `f` - A function that will be called when the client responds to an [`Connection::apply_edit`] request.
    /// The first argument is the server instance that received the response.
    /// The second argument is the tag that was passed to the [`Connection::apply_edit`] request.
    /// The third argument is the response from the client.

    pub fn on_apply_edit_response(&mut self, f: fn(&mut Server<T>, T::ApplyEditData, ApplyWorkspaceEditResponse)) {
        self.workspace.apply_edit.callback = Callback::response(f);
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