use crate::{connection::{Callback, Endpoint}, Server, TypeProvider};

use self::{did_change_configuration::DidChangeConfigurationOptions, did_change_watched_files::DidChangeWatchedFilesOptions, symbol::SymbolOptions, execute_command::ExecuteCommandOptions, apply_edit::ApplyWorkspaceRequest};

mod did_change_configuration;
mod did_change_watched_files;
pub mod symbol;
pub mod execute_command;
pub mod apply_edit;

pub(crate) struct WorkspaceService<T: TypeProvider> {
    did_change_configuration: Endpoint<T, DidChangeConfigurationOptions>,
    did_change_watched_files: Endpoint<T, DidChangeWatchedFilesOptions>,
    symbol: Endpoint<T, SymbolOptions>,
    pub(crate) execute_command: Endpoint<T, ExecuteCommandOptions>,
    apply_edit: ApplyWorkspaceRequest<T>
}

impl<T: TypeProvider> WorkspaceService<T> {
    pub(super) fn resolve(&self, method: &str) -> Option<Callback<Server<T>>> {
        match method {
            DidChangeConfigurationOptions::METHOD => Some(self.did_change_configuration.callback()),
            SymbolOptions::METHOD => Some(self.symbol.callback()),
            ExecuteCommandOptions::METHOD => Some(self.execute_command.callback()),
            ApplyWorkspaceRequest::<T>::METHOD => Some(self.apply_edit.callback()),
            DidChangeWatchedFilesOptions::METHOD => Some(self.did_change_watched_files.callback()),
            _ => None
        }
    }
}

impl<T: TypeProvider> Default for WorkspaceService<T> {
    fn default() -> Self {
        WorkspaceService {
            did_change_configuration: DidChangeConfigurationOptions::endpoint(),
            did_change_watched_files: DidChangeWatchedFilesOptions::endpoint(),
            symbol: SymbolOptions::endpoint(),
            execute_command: ExecuteCommandOptions::endpoint(),
            apply_edit: ApplyWorkspaceRequest::default(),           
        }
    }
}