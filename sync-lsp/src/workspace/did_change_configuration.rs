use crate::{Connection, TypeProvider};
use crate::connection::{Callback, Endpoint};
use serde::Deserialize;
use serde_json::Value;

#[derive(Default, Clone)]
pub(crate) struct DidChangeConfigurationOptions;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DidChangeConfigurationParams<T> {
    settings: T
}

impl DidChangeConfigurationOptions {

    pub(crate) const METHOD: &'static str = "workspace/didChangeConfiguration";
    
    pub(crate) fn endpoint<T: TypeProvider>() -> Endpoint<T, DidChangeConfigurationOptions> {
        Endpoint::new(Callback::notification(|_, _: DidChangeConfigurationParams<Value>| ()))
    }
}

impl<T: TypeProvider> Connection<T> {
    pub fn on_change_configuration(&mut self, callback: fn(&mut Connection<T>, T::Configuration)) {
        self.workspace.did_change_configuration.set_callback(Callback::notification(move |connection, params: DidChangeConfigurationParams<T::Configuration>| {
            callback(connection, params.settings)
        }))
    }
}