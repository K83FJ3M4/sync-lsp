use crate::Connection;
use crate::lifecycle::initialize::{InitializeParams, InitializeResult, ServerCapabilities};
use crate::lifecycle::{LifecycleService, Initialized, Initialize, Shutdown, Exit};
use crate::text_document::TextDocumentSyncOptions;
use super::ErrorCode;

impl<T> Default for LifecycleService<T> {
    fn default() -> Self {
        Self {
            initialize: Initialize(initialize),
            initialized: Initialized(initialized_error),
            shutdown: Shutdown(shutdown_error),
            exit: Exit(exit_error)
        }
    }
}

fn initialize<T>(connection: &mut Connection<T>, params: InitializeParams) -> InitializeResult {
    connection.lifecycle.initialize = Initialize(initialize_error);
    connection.lifecycle.initialized = Initialized(initialized);

    connection.initialization_options = params.initialization_options;
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
                save: connection.text_document.save_options
            }),
            completion_provider: Some(connection.text_document.completion.options()),
            hover_provider: false,
            definition_provider: false,
            references_provider: false,
            document_highlight_provider: false,
            document_symbol_provider: false,
            workspace_symbol_provider: false,
            code_action_provider: false,
            document_formatting_provider: false,
            document_range_formatting_provider: false,
            rename_provider: false
        }
    }
}

fn initialized<T>(connection: &mut Connection<T>) {
    connection.lifecycle.initialized = Initialized(initialized_error);
    connection.lifecycle.shutdown = Shutdown(shutdown);
}

fn shutdown<T>(connection: &mut Connection<T>) {
    connection.lifecycle.shutdown = Shutdown(shutdown_error);
    connection.lifecycle.exit = Exit(exit);
}

fn exit<T>(connection: &mut Connection<T>) {
    connection.lifecycle.exit = Exit(exit_error);
}

fn initialize_error<T>(connection: &mut Connection<T>, _: InitializeParams) -> InitializeResult {
    connection.error(
        ErrorCode::InvalidRequest,
        "Server has already been initialized".to_string()
    )
}

fn initialized_error<T>(connection: &mut Connection<T>) {
    connection.error::<()>(
        ErrorCode::InvalidRequest,
        "Only an uninitialized server may be initialized".to_string()
    );
}

fn shutdown_error<T>(connection: &mut Connection<T>) {
    connection.error::<()>(
        ErrorCode::ServerNotInitialized,
        "Only an initialized server may be shut down".to_string()
    );
}

fn exit_error<T>(connection: &mut Connection<T>) {
    connection.error::<()>(
        ErrorCode::InvalidRequest,
        "Only a shut down server may be exited".to_string()
    );
}