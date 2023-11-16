use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize, Debug)]
pub struct CodeActionContext {
    pub diagnostics: Vec<Diagnostic>
}

impl CodeActionOptions {

    pub(crate) const METHOD: &'static str = "textDocument/codeAction";
    
    pub(super) fn endpoint<T>() -> Endpoint<T, CodeActionOptions> {
        Endpoint::new(Callback::request(|_, _: CodeActionParams| Vec::<()>::new()))
    }
}

impl<T> Connection<T> {
    pub fn on_code_action<C: 'static + Serialize>(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer, Range, CodeActionContext) -> Vec<C>) {
        self.text_document.code_action.set_callback(Callback::request(move |connection, params: CodeActionParams| {
            callback(connection, params.text_document, params.range, params.context)
        }))
    }
}