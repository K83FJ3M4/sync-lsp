use crate::TypeProvider;
use crate::{Connection, connection::Endpoint};
use crate::connection::Callback;
use serde::{Serialize, Serializer, Deserializer, Deserialize};
use serde_json::Value;
pub use sync_lsp_derive::Command;

#[derive(Serialize, Default, Clone)]
pub(crate) struct ExecuteCommandOptions {
    commands: Vec<String>
}

pub(crate) struct CommandContainer<C: Command>(pub C);

#[derive(Debug, Clone, Deserialize)]
pub enum UnitCommand {}

pub trait Command: Clone {
    fn commands() -> Vec<String>;
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error>;
    fn deserialize<'de, T: Deserializer<'de>>(deserializer: T) -> Result<Self, T::Error> where Self: Sized;
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
        self.workspace.execute_command.set_callback(Callback::request(move |connection, params: CommandContainer<T::Command>| {
            callback(connection, params.0)
        }))
    }
}

pub(crate) fn deserialize_opt_command<'de, D: Deserializer<'de>, C: Command>(deserializer: D) -> Result<Option<C>, D::Error> {
    Ok(Some(Command::deserialize(deserializer)?))
}


pub(crate) fn serialize_opt_command<S: Serializer, C: Command>(command: &Option<C>, serializer: S) -> Result<S::Ok, S::Error> {
    if let Some(command) = command {
        command.serialize(serializer)
    } else {
        serializer.serialize_none()
    }
}

impl<C: Command> Serialize for CommandContainer<C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de, C:Command> Deserialize<'de> for CommandContainer<C> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(CommandContainer(C::deserialize(deserializer)?))
    }
}

impl Command for UnitCommand {
    fn commands() -> Vec<String> {
        vec![]
    }

    fn serialize<T: Serializer>(&self, _: T) -> Result<T::Ok, T::Error> {
        match *self {}
    }

    fn deserialize<'de, T: Deserializer<'de>>(deserializer: T) -> Result<Self, T::Error> where Self: Sized {
        <Self as Deserialize>::deserialize(deserializer)
    }
}