use crate::{Connection, TypeProvider};
use crate::lifecycle::initialize::{InitializeParams, InitializeResult, ServerCapabilities};
use crate::lifecycle::{LifecycleService, Initialized, Initialize, Shutdown, Exit, Cancel};
use crate::text_document::TextDocumentSyncOptions;
use super::ErrorCode;
use serde_json::from_value;
use log::error;

impl<T: TypeProvider> Default for LifecycleService<T> {
    fn default() -> Self {
        Self {
            initialize: Initialize(initialize),
            initialized: Initialized(initialized_error),
            shutdown: Shutdown(shutdown_error),
            exit: Exit(exit_error),
            cancel: Cancel(|_| ())
        }
    }
}

fn initialize(connection: &mut Connection<impl TypeProvider>, params: InitializeParams) -> InitializeResult {
    connection.lifecycle.initialize = Initialize(initialize_error);
    connection.lifecycle.initialized = Initialized(initialized);

    if let Some(options) = params.initialization_options {
        connection.initialization_options = match from_value(options) {
            Ok(options) => Some(options),
            Err(error) => {
                error!("Failed to deserialize initialization options: {}", error);
                None
            }
        };
    };
    
    connection.process_id = params.process_id;
    connection.root_uri = params.root_uri
        .or(params.root_path);

    InitializeResult {
        capabilities: ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncOptions {
                open_close: true,
                change: connection.text_document.sync_kind,
                will_save: true,
                will_save_wait_until: true,
                save: connection.text_document.did_save.options()
            }),
            completion_provider: Some(connection.text_document.completion.options()),
            execute_command_provider: Some(connection.workspace.execute_command.options()),
            signature_help_provider: Some(connection.text_document.signature_help.options()),
            document_on_type_formatting_provider: Some(connection.text_document.on_type_formatting.options()),
            code_lens_provider: Some(connection.text_document.code_lens.options()),
            document_link_provider: Some(connection.text_document.document_link.options()),
            hover_provider: true,
            definition_provider: true,
            references_provider: true,
            document_highlight_provider: true,
            document_symbol_provider: true,
            workspace_symbol_provider: true,
            code_action_provider: true,
            document_formatting_provider: true,
            document_range_formatting_provider: true,
            rename_provider: true
        }
    }
}

fn initialized(connection: &mut Connection<impl TypeProvider>) {
    connection.lifecycle.initialized = Initialized(initialized_error);
    connection.lifecycle.shutdown = Shutdown(shutdown);
}

fn shutdown(connection: &mut Connection<impl TypeProvider>) {
    connection.lifecycle.shutdown = Shutdown(shutdown_error);
    connection.lifecycle.exit = Exit(exit);
}

fn exit(connection: &mut Connection<impl TypeProvider>) {
    connection.lifecycle.exit = Exit(exit_error);
}

fn initialize_error(connection: &mut Connection<impl TypeProvider>, _: InitializeParams) -> InitializeResult {
    connection.error(
        ErrorCode::InvalidRequest,
        "Server has already been initialized".to_string()
    )
}

fn initialized_error(connection: &mut Connection<impl TypeProvider>) {
    connection.error::<()>(
        ErrorCode::InvalidRequest,
        "Only an uninitialized server may be initialized".to_string()
    );
}

fn shutdown_error(connection: &mut Connection<impl TypeProvider>) {
    connection.error::<()>(
        ErrorCode::ServerNotInitialized,
        "Only an initialized server may be shut down".to_string()
    );
}

fn exit_error(connection: &mut Connection<impl TypeProvider>) {
    connection.error::<()>(
        ErrorCode::InvalidRequest,
        "Only a shut down server may be exited".to_string()
    );
}