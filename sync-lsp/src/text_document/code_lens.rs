use crate::TypeProvider;
use crate::{Connection, connection::Endpoint};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeLens<C: Command, V> {
    pub range: Range,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_opt_command")]
    #[serde(deserialize_with = "deserialize_opt_command")]
    pub command: Option<C>,
    pub data: V,
}

impl CodeLensOptions {
    pub(crate) const METHOD: &'static str = "textDocument/codeLens";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, CodeLensOptions> {
        Endpoint::new(Callback::request(|_, _: CodeLensParams| Vec::<CodeLens<T::Command, T::CodeLensData>>::new()))
    }
}

impl CodeLensResolveOptions {
    pub(crate) const METHOD: &'static str = "codeLens/resolve";
        
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, CodeLensResolveOptions> {
        Endpoint::new(Callback::request(|_, lens: CodeLens<T::Command, T::CodeLensData>| lens))
    }
}

impl<T: TypeProvider> Connection<T> {
    pub fn on_code_lens(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer) -> Vec<CodeLens<T::Command, T::CodeLensData>>) {
        self.text_document.code_lens.set_callback(Callback::request(move |connection, params: CodeLensParams| {
            callback(connection, params.text_document)
        }));
    }

    pub fn on_code_lens_resolve(&mut self, callback: fn(&mut Connection<T>, CodeLens<T::Command, T::CodeLensData>) -> CodeLens<T::Command, T::CodeLensData>) {
        self.text_document.resolve_code_lens.set_callback(Callback::request(move |connection, params| {
            callback(connection, params)
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