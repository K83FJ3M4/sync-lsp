use sync_lsp::{
    Transport,
    TypeProvider,
    Server,
    text_document::did_open::TextDocumentItem
};

use sync_lsp::window::{
    MessageType,
    show_message_request::MessageActionItem
};

// The state of the server, in this case it's empty,
// but it could be used to store information like
// syntax trees, diagnostics, etc.
struct MyServerState;

// Configuring the server to use strings as the 
// data attached to show message requests and
// using the default implementation for the rest
// by using a macro
#[sync_lsp::type_provider]
impl TypeProvider for MyServerState {
    type ShowMessageRequestData = String;
}

fn main() {
    // Creating a transport that uses stdin and stdout
    let transport = Transport::stdio();
    let mut server = Server::new(MyServerState, transport);

    // Listeners for events can be set via server.on_* methods
    server.on_open(MyServerState::on_open);
    server.on_show_message_response(MyServerState::on_show_message_response);
    // Block the current thread and listen for messages
    server.serve().unwrap();
}

impl MyServerState {
    fn on_open(server: &mut Server<Self>, document: TextDocumentItem) {
        server.connection.show_message_request(
            MessageType::Warning,
            format!("Example query: {}", document.uri),
            vec![
                MessageActionItem {
                    title: "Action 1".to_string(),
                    data: document.uri.clone()
                },
                MessageActionItem {
                    title: "Action 2".to_string(),
                    data: document.uri.clone()
                }
            ]
        );
    }

    fn on_show_message_response(server: &mut Server<Self>, item: MessageActionItem<String>) {
        server.connection.show_message(
            MessageType::Info,
            format!("Performing {} on {}", item.title, item.data)
        );
    }
}