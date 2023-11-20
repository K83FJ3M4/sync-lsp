use sync_lsp::window::MessageType;
use sync_lsp::window::show_message_request::MessageActionItem;
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
    type ShowMessageRequestData = u32;
    type ApplyEditData = u32;
}

fn main() {
    let transport = Transport::stdio();
    let mut connection = Connection::new(LanguageServer, transport);


    connection.on_open(|connection, params| {
        info!("Open: {:?}", params);
        connection.show_message_request(MessageType::Info, "Choose an item 1".to_string(), vec![
            MessageActionItem {
                title: "Item 1".to_string(),
                data: 1
            },
            MessageActionItem {
                title: "Item 2".to_string(),
                data: 2
            },
        ]);
        connection.show_message_request(MessageType::Info, "Choose an item 2".to_string(), vec![
            MessageActionItem {
                title: "Item 1".to_string(),
                data: 1
            },
            MessageActionItem {
                title: "Item 2".to_string(),
                data: 2
            },
        ]);
        connection.show_message_request(MessageType::Info, "Choose an item 3".to_string(), vec![
            MessageActionItem {
                title: "Item 1".to_string(),
                data: 1
            },
            MessageActionItem {
                title: "Item 2".to_string(),
                data: 2
            },
        ]);
    });

    connection.on_show_message_response(|_, response| {
        info!("Show message response: {:?}", response);
    });

    connection.serve().unwrap();
}