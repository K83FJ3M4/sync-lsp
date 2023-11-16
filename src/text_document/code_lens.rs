use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::{TextDocumentIdentifer, Range};

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensOptions {
    pub resolve_provider: bool
}

#[derive(Clone, Default)]
pub(crate) struct CodeLensResolveOptions;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CodeLensParams  {
    text_document: TextDocumentIdentifer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeLens<C, V> {
    pub range: Range,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<C>,
    pub data: V,
}

impl CodeLensOptions {
    pub(crate) const METHOD: &'static str = "textDocument/codeLens";
    
    pub(super) fn endpoint<T>() -> Endpoint<T, CodeLensOptions> {
        Endpoint::new(Callback::request(|_, _: CodeLensParams| Vec::<()>::new()))
    }
}

impl CodeLensResolveOptions {
    pub(crate) const METHOD: &'static str = "codeLens/resolve";
        
    pub(super) fn endpoint<T>() -> Endpoint<T, CodeLensResolveOptions> {
        Endpoint::new(Callback::request(|_, lens: CodeLens<Value, Value>| lens))
    }
}

impl<T> Connection<T> {
    pub fn on_code_lens<C: 'static + Serialize + DeserializeOwned, V: 'static + Serialize + DeserializeOwned>(
            &mut self,
            callback: fn(&mut Connection<T>, TextDocumentIdentifer) -> Vec<CodeLens<C, V>>,
            resolve: fn(&mut Connection<T>, CodeLens<C, V>) -> CodeLens<C, V>
        ) {
        self.text_document.code_lens.set_callback(Callback::request(move |connection, params: CodeLensParams| {
            callback(connection, params.text_document)
        }));
        self.text_document.resolve_code_lens.set_callback(Callback::request(move |connection, params| {
            resolve(connection, params)
        }));
    }

    pub fn set_code_lens_options(&mut self, options: CodeLensOptions) {
        self.text_document.code_lens.set_options(options);
    }
}