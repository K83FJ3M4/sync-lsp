const { LanguageClient, TransportKind } = require('vscode-languageclient');
const path = require('path');
vscode = require('vscode');

let client;

module.exports = {
    activate() {
        let server_exe = path.resolve(__dirname, '..', 'target', 'debug', 'sync-lsp-example');
    
        const executable    = { command: server_exe, args: [], transport: TransportKind.stdio  };
        const serverOptions = { run: executable, debug: executable };
    
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