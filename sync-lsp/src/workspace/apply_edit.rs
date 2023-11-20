use std::collections::HashMap;
use crate::text_document::{DocumentUri, TextEdit};
use serde::{Serialize, Deserialize};
use crate::connection::{RpcConnection, Callback};
use crate::{Connection, TypeProvider};

#[derive(Serialize, Debug, Default)]
pub struct WorkspaceEdit {
    pub changes: HashMap<DocumentUri, Vec<TextEdit>>
}

pub(super) struct ApplyWorkspaceRequest<T: TypeProvider> {
    callback: Callback<Connection<T>>
}

#[derive(Serialize)]
struct ApplyWorkspaceEditParams {
    edit: WorkspaceEdit
}

#[derive(Deserialize, Debug, Default)]
pub struct ApplyWorkspaceEditResponse {
    pub applied: bool
}

impl<T: TypeProvider> Connection<T> {
    pub fn apply_edit(&mut self, tag: T::ApplyEditData, edit: WorkspaceEdit) {
        self.request(
            ApplyWorkspaceRequest::<T>::METHOD,
            tag,
            ApplyWorkspaceEditParams { edit }
        );
    }

    pub fn on_apply_edit_response(&mut self, f: fn(&mut Connection<T>, T::ApplyEditData, ApplyWorkspaceEditResponse)) {
        self.workspace.apply_edit.callback = Callback::response(f);
    }
}

impl<T: TypeProvider> Default for ApplyWorkspaceRequest<T> {
    fn default() -> Self {
        Self {
            callback: Callback::response(|_, _: T::ApplyEditData, _: ApplyWorkspaceEditResponse| ())
        }
    }
}

impl<T: TypeProvider> ApplyWorkspaceRequest<T> {
    pub(super) const METHOD: &'static str = "workspace/applyEdit";

    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        self.callback.clone()
    }
}