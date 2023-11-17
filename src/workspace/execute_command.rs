use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Default, Clone)]
pub(crate) struct ExecuteCommandOptions {
    commands: Vec<String>
}

impl ExecuteCommandOptions {

    pub(crate) const METHOD: &'static str = "workspace/executeCommand";
    
    pub(super) fn endpoint<T>() -> Endpoint<T, ExecuteCommandOptions> {
        Endpoint::new(Callback::request(|_, _: Value| ()))
    }
}

impl<T> Connection<T> {
    pub fn on_execute_command<C: 'static + DeserializeOwned, R: 'static + Serialize>(&mut self, callback: fn(&mut Connection<T>, C) -> R) {
        self.workspace.execute_command.set_callback(Callback::request(move |connection, params: C| {
            callback(connection, params)
        }))
    }

    pub fn set_commands(&mut self, commands: Vec<String>) {
        self.workspace.execute_command.options_mut().commands = commands;
    }
}