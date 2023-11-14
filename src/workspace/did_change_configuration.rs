use crate::Connection;
use crate::connection::{Callback, Endpoint};
use serde::Deserialize;
use serde::de::DeserializeOwned;
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
    
    pub(crate) fn endpoint<T>() -> Endpoint<T, DidChangeConfigurationOptions> {
        Endpoint::new(Callback::notification(|_, _: DidChangeConfigurationParams<Value>| ()))
    }
}

impl<T> Connection<T> {
    pub fn on_did_change_configuration<S: DeserializeOwned + 'static>(&mut self, callback: fn(&mut Connection<T>, S)) {
        self.workspace.did_change_configuration.set_callback(Callback::notification(move |connection, params: DidChangeConfigurationParams<S>| {
            callback(connection, params.settings)
        }))
    }
}