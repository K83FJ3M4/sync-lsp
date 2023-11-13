use serde::Serialize;
use serde_repr::Serialize_repr;

use crate::Connection;
use crate::connection::RpcConnection;

use super::{DocumentUri, Range};

#[derive(Default)]
pub(super) struct PublishDiagnostics;

#[derive(Serialize, Debug)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: Option<DiagnosticSeverity>,
    pub code: Option<String>,
    pub source: Option<String>,
    pub message: String,
}

#[repr(i32)]
#[derive(Serialize_repr, Debug)]
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

impl PublishDiagnostics {
    const METHOD: &'static str = "textDocument/publishDiagnostics";
}

impl<T> Connection<T> {
    pub fn publish_diagnostics(&mut self, uri: DocumentUri, diagnostics: Vec<Diagnostic>) {
        self.notify(
            PublishDiagnostics::METHOD,
            PublishDiagnosticsParams {
                uri,
                diagnostics
            }
        );
    }
}