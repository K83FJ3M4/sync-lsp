use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use serde::Serialize;
use super::{TextDocumentIdentifer, TextDocumentPositionParams, Position};

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SignatureHelpOptions {
    trigger_characters: Vec<String>
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
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, SignatureHelpOptions> {
        Endpoint::new(Callback::request(|_, _: TextDocumentPositionParams| SignatureHelp::default()))
    }
}

impl<T: TypeProvider> Server<T> {
    pub fn on_signature_help(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, Position) -> SignatureHelp) {
        self.text_document.signature_help.set_callback(Callback::request(move |server, params: TextDocumentPositionParams| {
            callback(server, params.text_document, params.position)
        }))
    }

    pub fn set_signature_help_trigger_characters(&mut self, value: Vec<String>) {
        self.text_document.signature_help.options_mut().trigger_characters = value;
    }
}