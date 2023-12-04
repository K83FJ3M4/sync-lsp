//! implementation of the `textDocument/rename` request
//! 
//! # Usage
//! If a user chooses to rename a symbol, the server can be notified via [`Server::on_rename`]
//! and also rename all references to that symbol.

use crate::TypeProvider;
use crate::workspace::apply_edit::WorkspaceEdit;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use super::{TextDocumentIdentifer, Position};
use serde::Deserialize;

#[derive(Default, Clone)]
pub(crate) struct RenameOptions;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RenameParams {
    text_document: TextDocumentIdentifer,
    position: Position,
    new_name: String
}

impl RenameOptions {
    pub(crate) const METHOD: &'static str = "textDocument/rename";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, RenameOptions> {
        Endpoint::new(Callback::request(|_, _: RenameParams| WorkspaceEdit::default()))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called to [rename symbols](self).
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as a symbol is renamed:
    ///     * The server instance receiving the response.
    ///     * The [`TextDocumentIdentifer`] of the target document.
    ///     * The [`Position`] of the cursor.
    ///     * The new name of the symbol.
    ///     * `return` - A [`WorkspaceEdit`] that contains the changes to apply to the workspace.

    pub fn on_rename(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, Position, String) -> WorkspaceEdit) {
        self.text_document.rename.set_callback(Callback::request(move |server, params: RenameParams | {
            callback(server, params.text_document, params.position, params.new_name)
        }))
    }
}