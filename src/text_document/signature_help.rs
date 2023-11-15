use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use serde::Serialize;
use super::{TextDocumentIdentifer, TextDocumentPositionParams, Position};

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelpOptions {
    pub trigger_characters: Vec<String>
}

#[derive(Serialize, Debug, Default)]
pub struct SignatureHelp {
    pub signatures: Vec<SignatureInformation>,
    pub active_signature: Option<u32>,
    pub active_parameter: Option<u32>
}

#[derive(Serialize, Debug)]
pub struct SignatureInformation {
    pub label: String,
    pub documentation: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterInformation>
}

#[derive(Serialize, Debug)]
pub struct ParameterInformation {
    pub label: String,
    pub documentation: Option<String>
}

impl SignatureHelpOptions {

    pub(crate) const METHOD: &'static str = "textDocument/signatureHelp";
    
    pub(super) fn endpoint<T>() -> Endpoint<T, SignatureHelpOptions> {
        Endpoint::new(Callback::request(|_, _: TextDocumentPositionParams| SignatureHelp::default()))
    }
}

impl<T> Connection<T> {
    pub fn on_signature_help(&mut self, callback: fn(&mut Connection<T>, TextDocumentIdentifer, Position) -> SignatureHelp) {
        self.text_document.signature_help.set_callback(Callback::request(move |connection, params: TextDocumentPositionParams| {
            callback(connection, params.text_document, params.position)
        }))
    }

    pub fn set_signature_help_options(&mut self, options: SignatureHelpOptions) {
        self.text_document.signature_help.set_options(options)
    }
}