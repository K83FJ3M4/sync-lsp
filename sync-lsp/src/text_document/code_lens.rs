//! implementation of the `textDocument/codeLens` request
//! 
//! # Usage
//! [`Server::on_code_action`] is called to compute commands, which are
//! commonly displayed in the user interface as some kind of button
//! in the source code. Optionally a command can be attached later on
//! via [`Server::on_resolve_code_action`].

use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use serde::{Deserialize, Serialize};
use super::{TextDocumentIdentifer, Range};
use crate::workspace::execute_command::{Command, serialize_opt_command, deserialize_opt_command};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CodeLensOptions {
    resolve_provider: bool
}

#[derive(Clone, Default)]
pub(crate) struct CodeLensResolveOptions;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CodeLensParams  {
    text_document: TextDocumentIdentifer,
}

/// A code lens represents a command.
#[derive(Serialize, Deserialize, Debug)]
pub struct CodeLens<C: Command, V> {
    /// The range in which this code lens is valid. Should only span a single line.
    pub range: Range,
    /// A server command as defined in [`TypeProvider::Command`]
    #[serde(default = "Option::default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_opt_command")]
    #[serde(deserialize_with = "deserialize_opt_command")]
    pub command: Option<C>,
    /// Arbitrary data as defined in [`TypeProvider::CodeLensData`]
    /// used to identify this code lens when [`Server::on_resolve_code_lens`] is invoked.
    pub data: V,
}

impl CodeLensOptions {
    pub(crate) const METHOD: &'static str = "textDocument/codeLens";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, CodeLensOptions> {
        Endpoint::new(Callback::request(|_, _: CodeLensParams| {
            Vec::<CodeLens<T::Command, T::CodeLensData>>::new()
        }))
    }
}

impl CodeLensResolveOptions {
    pub(crate) const METHOD: &'static str = "codeLens/resolve";
        
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, CodeLensResolveOptions> {
        Endpoint::new(Callback::request(|_, lens: CodeLens<T::Command, T::CodeLensData>| lens))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called to [compute code lenses](self).
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as code lenses are requested:
    ///     * The server instance receiving the response.
    ///     * The [`TextDocumentIdentifer`] of the document for which code actions are requested.
    ///     * `return` - A list of code lenses to display.

    pub fn on_code_lens(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer) -> Vec<CodeLens<T::Command, T::CodeLensData>>) {
        self.text_document.code_lens.set_callback(Callback::request(move |server, params: CodeLensParams| {
            callback(server, params.text_document)
        }));
    }

    /// Sets the callback that will be called to [compute commands of code lenses](self), which were previously returned by [`Server::on_code_lens`].
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon a code lens can be resolved:
    ///     * The server instance receiving the response.
    ///     * The [`CodeLens`] to resolve with `command` set to `None`.
    ///    * `return` - The resolved code lens.

    pub fn on_resolve_code_lens(&mut self, callback: fn(&mut Server<T>, CodeLens<T::Command, T::CodeLensData>) -> CodeLens<T::Command, T::CodeLensData>) {
        self.text_document.resolve_code_lens.set_callback(Callback::request(move |server, params| {
            callback(server, params)
        }));
    }
}

impl Default for CodeLensOptions {
    fn default() -> Self {
        CodeLensOptions {
            resolve_provider: true
        }
    }
}