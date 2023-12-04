//! implementation of the `textDocument/onTypeFormatting` notification.
//! 
//! # Usage
//! Whenever a character is typed, [`Server::on_type_formatting`] is invoked. This could for
//! example be used to fix indentation after a bracket was typed. Additionally, [`Server::set_on_type_formatting_first_trigger_character`]
//! and [`Server::set_on_type_formatting_more_trigger_characters`] can be used to set the characters that trigger formatting.

use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use super::formatting::FormattingOptions;
use super::{TextDocumentIdentifer, TextEdit, Position};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DocumentOnTypeFormattingOptions {
    first_trigger_character: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    more_trigger_character: Vec<String>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentOnTypeFormattingParams {
    text_document: TextDocumentIdentifer,
    position: Position,
    ch: String,
    options: FormattingOptions
}

impl DocumentOnTypeFormattingOptions {

    pub(crate) const METHOD: &'static str = "textDocument/onTypeFormatting";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, DocumentOnTypeFormattingOptions> {
        Endpoint::new(Callback::request(|_, _: DocumentOnTypeFormattingParams| Vec::<TextEdit>::new()))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called to implement [on type formatting](self).
    ///
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as a document is formatted:
    ///     * `server` - The server on which the request was received.
    ///     * `document` - The [`TextDocumentIdentifer`] of the target document.
    ///     * `position` - The [`Position`] of the cursor.
    ///     * `options` - The [`FormattingOptions`] that specify how the document should be formatted.
    ///     * `return` - A list of edits to apply to the document.


    pub fn on_type_formatting(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, Position, String, FormattingOptions) -> Vec<TextEdit>) {
        self.text_document.on_type_formatting.set_callback(Callback::request(move |server, params: DocumentOnTypeFormattingParams | {
            callback(server, params.text_document, params.position, params.ch, params.options)
        }))
    }
    
    /// Sets the first trigger character that triggers [on type formatting](self).
    /// 
    /// # Argument
    /// * `value` - The first trigger character that triggers formatting.

    pub fn set_on_type_formatting_first_trigger_character(&mut self, value: String) {
        self.text_document.on_type_formatting.options_mut().first_trigger_character = value;
    }

    /// Sets the additional characters that trigger [on type formatting](self).
    ///     
    /// # Argument
    /// * `value` - Additional characters that trigger formatting.

    pub fn set_on_type_formatting_more_trigger_characters(&mut self, value: Vec<String>) {
        self.text_document.on_type_formatting.options_mut().more_trigger_character = value;
    }
}