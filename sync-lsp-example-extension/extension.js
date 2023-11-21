const { LanguageClient, TransportKind } = require('vscode-languageclient');
const path = require('path');
vscode = require('vscode');
net = require('net');

let client;

module.exports = {
    activate() {
        let server_exe = path.resolve(__dirname, '..', 'target', 'debug', 'sync-lsp-example');
    
        const executable    = { command: server_exe, args: [], transport: /*TransportKind.socket*/ TransportKind.stdio };
        const serverOptions = { run: executable, debug: executable };
        
        /*let serverOptions = () => {
            let socket = net.connect({ port: 4000, host: "127.0.0.1" });
            let result = {
                writer: socket,
                reader: socket
            };
            return Promise.resolve(result);
        };*/
    
        const clientOptions = {
            documentSelector: [{
                scheme:   'file',
                language: 'plaintext',
            }],
        }
        
        client = new LanguageClient(
            'languageServerExample', 
            'Language Server Example',
            serverOptions,
            clientOptions
        )

        client.start()
    },
    
    deactivate() {
        if(client) {
            client.stop()
        }
    }
}