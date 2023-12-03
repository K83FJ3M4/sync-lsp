# Sync Lsp Example

This crate contains an example of how to use the [sync-lsp](https://crates.io/crates/sync-lsp) crate.
For running this example in VSCode, you can use the [vscode example extension](../sync-lsp-example-extension).

## Build

Building this crate will output a binary file, which uses stdio to communicate with the LSP client.
It doesn't take any arguments and requires a extension if used with most editors.

```bash
cargo build
```