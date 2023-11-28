use sync_lsp::{Transport, TypeProvider, Server, text_document::completion::CompletionList};

// For this example, we don't need any state.
struct MyServerState;

// This macro provides default implementations for all required types.
#[sync_lsp::type_provider]
impl TypeProvider for MyServerState {}

fn main() {
    let transport = Transport::stdio();
    let mut server = Server::new(MyServerState, transport);
    
    server.on_completion(|server, _, _| {
        let result = Vec::new();

        while !server.connection.cancelled() {
            // Do expensive work here
        }

        CompletionList {
            is_incomplete: false,
            items: result
        }
    });
}