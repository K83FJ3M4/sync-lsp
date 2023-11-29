use sync_lsp::{
    Transport,
    TypeProvider,
    Server
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
    server.on_open(|server, document| {
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
                    data: document.uri
                }
            ]
        );
    });

    server.on_show_message_response(|server, item| {
        server.connection.show_message(
            MessageType::Info,
            format!("Performing {} on {}", item.title, item.data)
        );
    });

    // Block the current thread and listen for messages
    server.serve().unwrap();
}