//! implementation of the `textDocument/signatureHelp` request
//! 
//! # Usage
//! A client can request the signature of a item using [`Server::on_signature_help`].
//! Additionally, the specific characters that trigger signature help can be set via
//! [`Server::set_signature_help_trigger_characters`].

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

/// The signature of some item.
#[derive(Serialize, Debug, Default)]
pub struct SignatureHelp {
    /// One or more signatures.
    pub signatures: Vec<SignatureInformation>,
    /// The index of the active signature.
    pub active_signature: Option<u32>,
    /// The index of the active parameter.
    pub active_parameter: Option<u32>
}

/// Represents the signature of some item.
#[derive(Serialize, Debug)]
pub struct SignatureInformation {
    /// A string representing the name of the item.
    pub label: String,
    /// Documentation of the item.
    pub documentation: Option<String>,
    /// The parameters of this signature.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterInformation>
}

/// Represents a parameter of a callable-signature.
#[derive(Serialize, Debug)]
pub struct ParameterInformation {
    /// The label of this parameter.
    pub label: String,
    /// The markdown documentation of this parameter.
    pub documentation: Option<String>
}

impl SignatureHelpOptions {

    pub(crate) const METHOD: &'static str = "textDocument/signatureHelp";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, SignatureHelpOptions> {
        Endpoint::new(Callback::request(|_, _: TextDocumentPositionParams| SignatureHelp::default()))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called to [compute signature help](self).
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as signature help is requested:
    ///     * The server instance receiving the response.
    ///     * The [`TextDocumentIdentifer`] of the target document.
    ///     * The [`Position`] of the cursor.
    ///     * `return` - The signature help to display.

    pub fn on_signature_help(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, Position) -> SignatureHelp) {
        self.text_document.signature_help.set_callback(Callback::request(move |server, params: TextDocumentPositionParams| {
            callback(server, params.text_document, params.position)
        }))
    }

    /// Sets the characters that trigger [signature help](self).
    /// 
    /// # Argument
    /// * `value` - The characters that trigger signature help.

    pub fn set_signature_help_trigger_characters(&mut self, value: Vec<String>) {
        self.text_document.signature_help.options_mut().trigger_characters = value;
    }
}