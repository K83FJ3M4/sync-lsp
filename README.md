# Sync Lsp

[![MIT licensed][mit-badge]][mit-url]
![Static Badge](https://img.shields.io/badge/potato-wedges-yellow)

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/K83FJ3M4/sync-lsp/blob/main/LICENSE

Sync Lsp is a synchronous [lsp](https://microsoft.github.io/language-server-protocol/) implementation for language servers.
These are the main features of this library:

- **Automation**: Sync Lsp handles registration and unregistration aswell as capabilitie negotiations by itself. Ontop of that none of the lifecycle messages are exposed to users of this library.

- **Compatabilitie**: Capabilitie handling is also done internally. This means that the user of this library does not have to worry about checking if a client supports a certain feature or not.

- **Error Handling**: Almost all protocol related errors are proccessed internally. Therefore the api is very easy to use and does not require the user to implement their own error handling.

# Example

```rust
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
```

# Feature Flags

| Flag | Description |
|------|-------------|
| `mio` | The [mio](https://github.com/tokio-rs/mio) crate will be used to poll for messages and therefore enable request cancellation support. Without this flag the `Connection::cancelled` method is still available, but will always return false. |
| `dynamic-callbacks` | If this feature is disabled, there should be no calls to `Server::on_*` after `Server::server` is called, and the server's performance may improve. Note that this is mainly a performance feature and does not equate to the client's ability to register capabilities dynamically via the lsp. |