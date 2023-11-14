use crate::{connection::{Callback, Endpoint}, Connection};

use self::{did_change_configuration::DidChangeConfigurationOptions, did_change_watched_files::DidChangeWatchedFilesOptions, symbol::SymbolOptions};

mod did_change_configuration;
mod did_change_watched_files;
pub mod symbol;

pub(crate) struct WorkspaceService<T: 'static> {
    did_change_configuration: Endpoint<T, DidChangeConfigurationOptions>,
    #[allow(unused)]
    did_change_watched_files: DidChangeWatchedFilesOptions,
    symbol: Endpoint<T, SymbolOptions>
}

impl<T> WorkspaceService<T> {
    pub(super) fn resolve(&self, method: &str) -> Option<Callback<Connection<T>>> {
        match method {
            DidChangeConfigurationOptions::METHOD => Some(self.did_change_configuration.callback()),
            SymbolOptions::METHOD => Some(self.symbol.callback()),
            _ => None
        }
    }
}

impl<T: 'static> Default for WorkspaceService<T> {
    fn default() -> Self {
        WorkspaceService {
            did_change_configuration: DidChangeConfigurationOptions::endpoint(),
            did_change_watched_files: DidChangeWatchedFilesOptions::default(),
            symbol: SymbolOptions::endpoint()
        }
    }
}