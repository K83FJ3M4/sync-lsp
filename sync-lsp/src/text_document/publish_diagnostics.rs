//! impl of the `textDocument/publishDiagnostics` notification
//! 
//! # Usage
//! A server can publish diagnostics for a specific document via [`Server::publish_diagnostics`]
//! in any callback. The client will then display these diagnostics in the editor.

use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

use crate::{Connection, TypeProvider};
use crate::connection::RpcConnection;

use super::{DocumentUri, Range};

#[derive(Default, Clone)]
pub(super) struct PublishDiagnostics;

/// The diagnostic information.
#[derive(Deserialize, Serialize, Debug)]
pub struct Diagnostic {
    /// A range in the document that contains the diagnostic message.
    pub range: Range,
    /// The severity of the diagnostic.
    pub severity: Option<DiagnosticSeverity>,
    /// A optional code to identify the diagnostic.
    pub code: Option<String>,
    /// A string describing the source of this diagnostic.
    pub source: Option<String>,
    /// A human-readable string describing the diagnostic.
    pub message: String,
}

/// The diagnostic severity.
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

impl PublishDiagnostics {
    const METHOD: &'static str = "textDocument/publishDiagnostics";
}

impl<T: TypeProvider> Connection<T> {

    /// [Publishes diagnostics](self) for a specific document.
    /// 
    /// # Arguments
    /// * `uri` - The [`DocumentUri`] of the document to publish diagnostics for.
    /// * `diagnostics` - A list of diagnostics to publish.

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