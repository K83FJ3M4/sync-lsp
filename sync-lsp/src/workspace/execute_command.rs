use crate::TypeProvider;
use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;

#[derive(Serialize, Default, Clone)]
pub(crate) struct ExecuteCommandOptions {
    commands: Vec<String>
}

pub trait Command: Serialize + DeserializeOwned + Debug {
    fn commands() -> Vec<String>;
}

impl ExecuteCommandOptions {

    pub(crate) const METHOD: &'static str = "workspace/executeCommand";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, ExecuteCommandOptions> {
        let mut endpoint = Endpoint::<T, ExecuteCommandOptions>::new(Callback::request(|_, _: Value| ()));
        endpoint.options_mut().commands = T::Command::commands();
        endpoint
    }
}

impl<T: TypeProvider> Connection<T> {
    pub fn on_execute_command<R: 'static + Serialize>(&mut self, callback: fn(&mut Connection<T>, T::Command) -> R) {
        self.workspace.execute_command.set_callback(Callback::request(move |connection, params| {
            callback(connection, params)
        }))
    }
}