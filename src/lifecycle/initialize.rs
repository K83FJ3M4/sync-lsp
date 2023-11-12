use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::Connection;
use crate::connection::Callback;

//use crate::text_document::{TextDocumentClientCapabilities, TextDocumentSyncKind};

pub(crate) struct Initialize<T: 'static>
    (pub(crate) fn(&mut Connection<T>, params: InitializeParams) -> InitializeResult);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InitializeParams {
    pub(crate) process_id: Option<u32>,
    pub(crate) root_path: Option<String>,
    pub(crate) root_uri: Option<String>,
    pub(crate) initialization_options: Option<Value>,
    #[allow(unused)]
    pub(crate) capabilities: ClientCapabilities,
    //pub(crate) trace: Option<TraceValue>,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InitializeResult {
    pub(crate) capabilities: ServerCapabilities,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub(crate) struct ClientCapabilities {
    //pub(crate) workspace: Option<WorkspaceClientCapabilities>,
    //pub(crate) text_document: TextDocumentClientCapabilities,
    //pub(crate) experimental: Option<Value>,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub text_document_sync: Option<TextDocumentSyncKind>,
    pub hover_provider: bool,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub completion_provider: Option<CompletionOptions>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub signature_help_provider: Option<SignatureHelpOptions>,
    pub definition_provider: bool,
    pub references_provider: bool,
    pub document_highlight_provider: bool,
    pub document_symbol_provider: bool,
    pub workspace_symbol_provider: bool,
    pub code_action_provider: bool,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub code_lens_provider: Option<CodeLensOptions>,
    pub document_formatting_provider: bool,
    pub document_range_formatting_provider: bool,
   // #[serde(skip_serializing_if = "Option::is_none")]
    //pub document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,
    pub rename_provider: bool,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub document_link_provider: Option<DocumentLinkOptions>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub execute_command_provider: Option<ExecuteCommandOptions>,
}
#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub(crate) struct DynamicRegistration {
    pub(crate) dynamic_registration: bool,
}

impl<T> Initialize<T> {

    pub(crate) const METHOD: &'static str = "initialize";

    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        let Initialize(callback) = *self;
        Callback::request(callback)
    }
}