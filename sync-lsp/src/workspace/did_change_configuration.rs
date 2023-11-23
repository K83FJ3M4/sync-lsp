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

/// Options for an [`Endpoint`] struct.
#[derive(Default, Clone)]
pub(crate) struct DidChangeConfigurationOptions;

/// The parameters of a [`DidChangeConfigurationOptions::METHOD`] notification.
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

    /// Sets the callback that will be called when the client sends a change configuration notification.
    /// 
    /// # Arguments
    /// * `callback` - A function for handling changes of the configuration.
    /// The first argument is the server, the second one is the settings value as defined in [`TypeProvider`].

    pub fn on_change_configuration(&mut self, callback: fn(&mut Server<T>, T::Configuration)) {
        self.workspace.did_change_configuration.set_callback(Callback::notification(move |server, params: DidChangeConfigurationParams<T::Configuration>| {
            callback(server, params.settings)
        }))
    }
}