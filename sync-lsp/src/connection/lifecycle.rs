use crate::TypeProvider;
use crate::lifecycle::initialize::{InitializeParams, InitializeResult, ServerCapabilities};
use crate::lifecycle::{LifecycleService, Initialized, Initialize, Shutdown, Exit, Cancel};
use crate::text_document::TextDocumentSyncOptions;
use super::{ErrorCode, Server};
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

fn initialize(server: &mut Server<impl TypeProvider>, params: InitializeParams) -> InitializeResult {
    server.lifecycle.initialize = Initialize(initialize_error);
    server.lifecycle.initialized = Initialized(initialized);

    if let Some(options) = params.initialization_options {
        server.initialization_options = match from_value(options) {
            Ok(options) => Some(options),
            Err(error) => {
                error!("Failed to deserialize initialization options: {}", error);
                None
            }
        };
    };
    
    server.process_id = params.process_id;
    server.root_uri = params.root_uri
        .or(params.root_path);

    InitializeResult {
        capabilities: ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncOptions {
                open_close: true,
                change: server.text_document.sync_kind,
                will_save: true,
                will_save_wait_until: true,
                save: server.text_document.did_save.options()
            }),
            completion_provider: Some(server.text_document.completion.options()),
            execute_command_provider: Some(server.workspace.execute_command.options()),
            signature_help_provider: Some(server.text_document.signature_help.options()),
            document_on_type_formatting_provider: Some(server.text_document.on_type_formatting.options()),
            code_lens_provider: Some(server.text_document.code_lens.options()),
            document_link_provider: Some(server.text_document.document_link.options()),
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

fn initialized(server: &mut Server<impl TypeProvider>) {
    server.lifecycle.initialized = Initialized(initialized_error);
    server.lifecycle.shutdown = Shutdown(shutdown);
}

fn shutdown(server: &mut Server<impl TypeProvider>) {
    server.lifecycle.shutdown = Shutdown(shutdown_error);
    server.lifecycle.exit = Exit(exit);
}

fn exit(server: &mut Server<impl TypeProvider>) {
    server.lifecycle.exit = Exit(exit_error);
}

fn initialize_error(server: &mut Server<impl TypeProvider>, _: InitializeParams) -> InitializeResult {
    server.connection.error(
        ErrorCode::InvalidRequest,
        "Server has already been initialized".to_string()
    )
}

fn initialized_error(server: &mut Server<impl TypeProvider>) {
    server.connection.error::<()>(
        ErrorCode::InvalidRequest,
        "Only an uninitialized server may be initialized".to_string()
    );
}

fn shutdown_error(server: &mut Server<impl TypeProvider>) {
    server.connection.error::<()>(
        ErrorCode::ServerNotInitialized,
        "Only an initialized server may be shut down".to_string()
    );
}

fn exit_error(server: &mut Server<impl TypeProvider>) {
    server.connection.error::<()>(
        ErrorCode::InvalidRequest,
        "Only a shut down server may be exited".to_string()
    );
}