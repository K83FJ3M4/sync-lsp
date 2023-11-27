//! Implementation of the `worksapce/didChangeConfiguration` notification.
//! 
//! # Usage
//! A server might require certain settings to be transferred from the client.
//! These include settings like the format of a file or the location of a specific resource.
//! Usually there is a ui provided for the user to change these settings. The
//! [`Server::on_change_configuration`] notification is only triggered when the user
//! changes these settings. Note that in the current version of this library,
//! there is no way to get the current settings from the client by polling. 

use crate::{Server, TypeProvider};
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

impl<T: TypeProvider> Server<T> {

    /// Set the request handler for [changing the server configuration](self)
    ///
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters if a change in configuration is received:
    ///     * The server instance receiving the response.
    ///     * The updated configuration of type [`TypeProvider::Configuration`].

    pub fn on_change_configuration(&mut self, callback: fn(&mut Server<T>, T::Configuration)) {
        self.workspace.did_change_configuration.set_callback(Callback::notification(move |server, params: DidChangeConfigurationParams<T::Configuration>| {
            callback(server, params.settings)
        }))
    }
}