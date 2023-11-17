use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

use crate::{Connection, TypeProvider};
use crate::connection::RpcConnection;

use super::{DocumentUri, Range};

#[derive(Default, Clone)]
pub(super) struct PublishDiagnosticsOptions;

#[derive(Deserialize, Serialize, Debug)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: Option<DiagnosticSeverity>,
    pub code: Option<String>,
    pub source: Option<String>,
    pub message: String,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4
}

#[derive(Serialize)]
struct PublishDiagnosticsParams {
    uri: DocumentUri,
    diagnostics: Vec<Diagnostic>
}

impl PublishDiagnosticsOptions {
    const METHOD: &'static str = "textDocument/publishDiagnostics";
}

impl<T: TypeProvider> Connection<T> {
    pub fn publish_diagnostics(&mut self, uri: DocumentUri, diagnostics: Vec<Diagnostic>) {
        self.notify(
            PublishDiagnosticsOptions::METHOD,
            PublishDiagnosticsParams {
                uri,
                diagnostics
            }
        );
    }
}