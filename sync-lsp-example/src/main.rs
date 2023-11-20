use sync_lsp::{Transport, Connection, TypeProvider, UnitType};
use sync_lsp::workspace::execute_command::{Command as CommandDescriptor, UnitCommand};
use log::info;

struct LanguageServer;

#[derive(Clone, Debug, CommandDescriptor)]
struct Command<T>(T);

impl TypeProvider for LanguageServer {
    type Command = UnitCommand;
    type CodeLensData = UnitType;
    type CompletionData = UnitType;
    type Configuration = UnitType;
    type InitializeOptions = UnitType;
}

fn main() {
    let transport = Transport::stdio();
    let mut connection = Connection::new(LanguageServer, transport);

    connection.on_execute_command(|_, command| {
        info!("Command executed: {:?}", command);
    });

    connection.serve().unwrap();
}