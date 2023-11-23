use sync_lsp::text_document::{Range, Position};
use sync_lsp::text_document::code_lens::CodeLens;
use sync_lsp::{Transport, TypeProvider, Server, UnitType};
use sync_lsp::workspace::execute_command::Command;
use log::info;

// This enum defines all commands that can be executed by the server.
// It could also be a struct or a tuple struct.
// Even unit structs and enum variants are supported.
#[derive(Clone, Command, Debug)]
enum MyCommand {
    #[command(title = "My first command")]
    MyCommand,
    #[command(title = "My command with arguments")]
    MyCommandWithArguments(u32),
}

// For this example, we don't need any state.
struct MyServerState;

// This macro provides default implementations for all required types.
#[sync_lsp::type_provider]
impl TypeProvider for MyServerState {
    type Command = MyCommand;
}

fn main() {
    let transport = Transport::stdio();
    let mut server = Server::new(MyServerState, transport);

    // One example for a way to send commands to the client is the code lens request.
    server.on_code_lens(|_, _| {
        vec![
            CodeLens {
                // For this example, we just return a code lens at the beginning of the document.
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { line: 0, character: 0 }
                },
                // This command will be executed when the user clicks on the code lens.
                command: Some(MyCommand::MyCommandWithArguments(1)),
                // Since we didn't override TypeProvider::CodeLensData, we have to use UnitType here.
                data: UnitType
            }
        ]
    });

    server.on_execute_command(|_, command| {
        // Instead of executing the command here, we just log it.
        info!("Received command: {:?}", command);
    });

    server.serve().unwrap();
}