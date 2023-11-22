use sync_lsp::window::MessageType;
use sync_lsp::window::show_message_request::MessageActionItem;
use sync_lsp::{Transport, TypeProvider, Server};
use sync_lsp::workspace::execute_command::{Command as CommandDescriptor};
use log::info;

struct LanguageServer;

#[derive(Clone, Debug, CommandDescriptor)]
struct Command<T>(T);

#[sync_lsp::type_provider]
impl TypeProvider for LanguageServer {
    type ShowMessageRequestData = u32;
}

fn main() {
    let transport = Transport::stdio();
    let mut server = Server::new(LanguageServer, transport);

    server.on_open(|server, params| {
        info!("Open: {:?}", params);
        let token = server.connection.show_message_request(MessageType::Info, "Choose an item 1".to_string(), vec![
            MessageActionItem {
                title: "Item 1".to_string(),
                data: 1
            },
            MessageActionItem {
                title: "Item 1".to_string(),
                data: 2
            },
        ]).unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));
        server.connection.cancel(token);
    });

    server.on_show_message_response(|_, response| {
        info!("Show message response: {:?}", response);
    });

    server.serve().unwrap();
}