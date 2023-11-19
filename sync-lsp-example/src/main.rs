use sync_lsp::text_document::{Range, Position};
use sync_lsp::text_document::code_lens::CodeLens;
use sync_lsp::{Transport, Connection, TypeProvider};
use sync_lsp::workspace::execute_command::Command as CommandDescriptor;
use log::info;

struct LanguageServer;

#[derive(Clone, Debug, CommandDescriptor)]
struct Command<T>(T);

impl TypeProvider for LanguageServer {
    type Command = Command<u32>;
    type CodeLensData = ();
    type CompletionData = ();
    type Configuration = ();
    type InitializeOptions = ();
}

fn main() {
    let transport = Transport::stdio();
    let mut connection = Connection::new(LanguageServer, transport);

    connection.on_execute_command(|_, command| {
        info!("Command executed: {:?}", command);
    });

    connection.on_code_lens(|_, document| {
        info!("Code lens requested for {:?}", document);
        vec![CodeLens {
            range: Range {
                start: Position {
                    line: 0,
                    character: 10
                },
                end: Position {
                    line: 0,
                    character: 20
                }
            },
            command: Some(Command(12)),
            data: ()
        }]
    });

    connection.serve().unwrap();
}
