//! implementation of the `textDocument/codeAction` request
//! 
//! # Usage
//! The [`Server::on_code_action`] endpoint is used to compute commands
//! for a range of a text document. Commonly these commands are displayed
//! in the user interface and may for example represent code fixes or
//! refactoring options.

use crate::TypeProvider;
use crate::workspace::execute_command::CommandContainer;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use serde::Deserialize;
use super::publish_diagnostics::Diagnostic;
use super::{TextDocumentIdentifer, Range};

#[derive(Default, Clone)]
pub(crate) struct CodeActionOptions;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CodeActionParams {
    text_document: TextDocumentIdentifer,
    range: Range,
    context: CodeActionContext
}

/// Contains additional diagnostic information about the context in which a code action is run.
#[derive(Deserialize, Debug)]
pub struct CodeActionContext {
    /// An array of diagnostics as defined by the server.
    pub diagnostics: Vec<Diagnostic>
}

impl CodeActionOptions {

    pub(crate) const METHOD: &'static str = "textDocument/codeAction";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, CodeActionOptions> {
        Endpoint::new(Callback::request(|_, _: CodeActionParams| Vec::<()>::new()))
    }
}

impl<T: TypeProvider> Server<T> {
    
    /// Sets the callback that will be called to [compute code actions](self).
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as code actions are requested:
    ///     * The server instance receiving the response.
    ///     * The [`TextDocumentIdentifer`] of the document for which code actions are requested.
    ///     * The [`Range`] of the document for which code actions are requested.
    ///     * The [`CodeActionContext`] for which code actions are requested.
    ///     * `return` - A list of commands to execute.

    pub fn on_code_action(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, Range, CodeActionContext) -> Vec<T::Command>) {
        self.text_document.code_action.set_callback(Callback::request(move |server, params: CodeActionParams| {
            callback(server, params.text_document, params.range, params.context).into_iter()
                .map(|command| CommandContainer(command))
                .collect::<Vec<_>>()
        }))
    }
}