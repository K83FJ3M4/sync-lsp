//! Implementation for the `workspace/executeCommand` request.
//! 
//! # Usage
//! Commands can be attached to certain actions on the client side and then be executed on the server side.
//! This module provides a [`Command`] trait that can be implemented through a macro to define commands that
//! can be executed by the server via [`Server::on_execute_command`].
//! 
//! # Example
//! ```
//!use sync_lsp::text_document::{Range, Position};
//!use sync_lsp::text_document::code_lens::CodeLens;
//!use sync_lsp::{Transport, TypeProvider, Server, UnitType};
//!use sync_lsp::workspace::execute_command::Command;
//!use log::info;
//!
//!// This enum defines all commands that can be executed by the server.
//!// It could also be a struct or a tuple struct.
//!// Even unit structs and enum variants are supported.
//!#[derive(Clone, Command, Debug)]
//!enum MyCommand {
//!    #[command(title = "My first command")]
//!    MyCommand,
//!    #[command(title = "My command with arguments")]
//!    MyCommandWithArguments(u32),
//!}
//!
//!// For this example, we don't need any state.
//!struct MyServerState;
//!
//!// This macro provides default implementations for all required types.
//!#[sync_lsp::type_provider]
//!impl TypeProvider for MyServerState {
//!    type Command = MyCommand;
//!}
//!
//!fn main() {
//!    let transport = Transport::stdio();
//!    let mut server = Server::new(MyServerState, transport);
//!
//!    // One example for a way to send commands to the client is the code lens request.
//!    server.on_code_lens(|_, _| {
//!        vec![
//!            CodeLens {
//!                // For this example, we just return a code lens at the beginning of the document.
//!                range: Range {
//!                    start: Position { line: 0, character: 0 },
//!                    end: Position { line: 0, character: 0 }
//!                },
//!                // This command will be executed when the user clicks on the code lens.
//!                command: Some(MyCommand::MyCommandWithArguments(1)),
//!                // Since we didn't override TypeProvider::CodeLensData, we have to use UnitType here.
//!                data: UnitType
//!            }
//!        ]
//!    });
//!
//!    server.on_execute_command(|_, command| {
//!        // Instead of executing the command here, we just log it.
//!        info!("Received command: {:?}", command);
//!    });
//!
//!    server.serve().unwrap();
//!}
//! ```

use crate::TypeProvider;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use serde::{Serialize, Serializer, Deserializer, Deserialize};
/// This macro implements the [`Command`] trait for a given type.
/// the `#[command(title = "...")]` attribute can be used to define the title of the command
/// on enum variants or structs.
/// 
/// # Example
/// ```
/// use sync_lsp::workspace::execute_command::Command;
/// 
/// #[derive(Clone, Command)]
/// #[command(title = "My command without variants or arguments")]
/// struct MyCommand;
/// ```
/// ```
/// use sync_lsp::workspace::execute_command::Command;
/// 
/// #[derive(Clone, Command)]
/// enum MyCommand {
///     #[command(title = "My first command")]
///     MyCommand,
///     #[command(title = "My command with arguments")]
///     MyCommandWithArguments(u32),
/// }
/// ```
pub use sync_lsp_derive::Command;

/// This struct can be unsed in an [`Endpoint`] to list all available commands on the server.
#[derive(Serialize, Default, Clone)]
pub(crate) struct ExecuteCommandOptions {
    commands: Vec<String>
}

/// A wrapper struct to make it easier to deserialize and serialize commands.
pub(crate) struct CommandContainer<C: Command>(pub C);

/// A unit command is a command that does not take any arguments.
/// Command arguments are always optional, which is why this type
/// never needs to be instantiated and is therefore an empty enum.
#[derive(Debug, Clone, Deserialize)]
pub enum UnitCommand {}

/// Defines a command that can be executed by this server.
/// Instead of implementing this trait manually, you can use the `Command` derive macro.
/// If you do so, you must also derive [`Clone`].
/// Implementing this trait manually might break the connection if not done properly.
pub trait Command: Clone {
    /// Returns a vector of commands that can be executed by this server.
    fn commands() -> Vec<String>;
    /// This is beeing forwared to the `serde::Serialize::serialize` implementation.
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error>;
    /// This is beeing forwared to the `serde::Deserialize::deserialize` implementation.
    fn deserialize<'de, T: Deserializer<'de>>(deserializer: T) -> Result<Self, T::Error> where Self: Sized;
}

impl ExecuteCommandOptions {

    pub(crate) const METHOD: &'static str = "workspace/executeCommand";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, ExecuteCommandOptions> {
        let mut endpoint = Endpoint::<T, ExecuteCommandOptions>::new(Callback::request(|_, _: CommandContainer<T::Command>| ()));
        endpoint.options_mut().commands = T::Command::commands();
        endpoint
    }
}

impl<T: TypeProvider> Server<T> {
    /// Sets the callback that will be called to execute a certain `Command` defined in [`TypeProvider`].
    /// It's common for language servers to make a ['WorkspaceEdit'] in this callback.
    /// 
    /// # Arguments
    /// * `callback` - A function that is supposed to execute the command.
    /// The first argument is the server instance that received the command.
    /// The second argument is the command to be executed.
    pub fn on_execute_command<R: 'static + Serialize>(&mut self, callback: fn(&mut Server<T>, T::Command) -> R) {
        self.workspace.execute_command.set_callback(Callback::request(move |server, params: CommandContainer<T::Command>| {
            callback(server, params.0)
        }))
    }
}

/// A helper function that can be used to deserialize an optional command via `#[serde(deserialize_with)]`.
pub(crate) fn deserialize_opt_command<'de, D: Deserializer<'de>, C: Command>(deserializer: D) -> Result<Option<C>, D::Error> {
    Ok(Some(Command::deserialize(deserializer)?))
}

/// A helper function that can be used to serialize an optional command via `#[serde(serialize_with)]`.
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