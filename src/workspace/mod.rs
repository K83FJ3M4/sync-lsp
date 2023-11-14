use crate::{connection::{Callback, Endpoint}, Connection};

use self::did_change_configuration::DidChangeConfigurationOptions;

mod did_change_configuration;

pub(crate) struct WorkspaceService<T: 'static> {
    did_change_configuration: Endpoint<T, DidChangeConfigurationOptions>
}

impl<T> WorkspaceService<T> {
    pub(super) fn resolve(&self, method: &str) -> Option<Callback<Connection<T>>> {
        match method {
            DidChangeConfigurationOptions::METHOD => Some(self.did_change_configuration.callback()),
            _ => None
        }
    }
}

impl<T: 'static> Default for WorkspaceService<T> {
    fn default() -> Self {
        WorkspaceService {
            did_change_configuration: DidChangeConfigurationOptions::endpoint()
        }
    }
}