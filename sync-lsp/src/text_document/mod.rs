use crate::TypeProvider;
use crate::connection::Endpoint;
use crate::{connection::Callback, Server};
use self::code_action::CodeActionOptions;
use self::code_lens::{CodeLensOptions, CodeLensResolveOptions};
use self::completion::{CompletionOptions, ResolveCompletionOptions};
use self::definition::DefinitionOptions;
use self::document_highlight::DocumentHighlightOptions;
use self::document_link::{DocumentLinkOptions, DocumentLinkResolveOptions};
use self::document_symbol::DocumentSymbolOptions;
use self::formatting::DocumentFormattingOptions;
use self::hover::HoverOptions;
use self::on_type_formatting::DocumentOnTypeFormattingOptions;
use self::publish_diagnostics::PublishDiagnostics;
use self::range_formatting::RangeFormattingOptions;
use self::references::ReferenceOptions;
use self::rename::RenameOptions;
use self::signature_help::SignatureHelpOptions;
use self::{did_open::DidOpenOptions, did_change::DidChangeOptions, will_save::WillSaveOptions, will_save_wait_until::WillSaveWaitUntilOptions, did_save::DidSaveOptions, did_close::DidCloseOptions};
use serde::{Serialize, Deserialize};
use serde_repr::Serialize_repr;

pub mod did_open;
pub mod did_change;
pub mod will_save;
mod will_save_wait_until;
mod did_save;
mod did_close;
pub mod publish_diagnostics;
pub mod completion;
pub mod hover;
pub mod signature_help;
pub mod references;
pub mod document_highlight;
mod document_symbol;
pub mod formatting;
mod range_formatting;
pub mod on_type_formatting;
mod definition;
pub mod code_action;
pub mod code_lens;
pub mod document_link;
mod rename;

pub type DocumentUri = String;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TextDocumentPositionParams {
    pub text_document: TextDocumentIdentifer,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

#[derive(Serialize, Debug)]
pub struct Location {
    pub uri: DocumentUri,
    pub range: Range,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentIdentifer {
    pub uri: DocumentUri,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Position {
    pub line: i32,
    pub character: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Deserialize, Debug)]
pub struct VersionedTextDocumentIdentifier {
    pub uri: DocumentUri,
    pub version: i32,
}

pub(super) struct TextDocumentService<T: TypeProvider> {
    pub(super) sync_kind: TextDocumentSyncKind,
    pub(super) did_open: Endpoint<T, DidOpenOptions>,
    pub(super) did_change: Endpoint<T, DidChangeOptions>,
    pub(super) will_save: Endpoint<T, WillSaveOptions>,
    pub(super) will_save_wait_until: Endpoint<T, WillSaveWaitUntilOptions>,
    pub(super) did_save: Endpoint<T, DidSaveOptions>,
    pub(super) did_close: Endpoint<T, DidCloseOptions>,
    #[allow(unused)]
    publish_diagnostics: PublishDiagnostics,
    pub(super) completion: Endpoint<T, CompletionOptions>,
    pub(super) resolve_completion: Endpoint<T, ResolveCompletionOptions>,
    pub(super) hover: Endpoint<T, HoverOptions>,
    pub(super) signature_help: Endpoint<T, SignatureHelpOptions>,
    pub(super) references: Endpoint<T, ReferenceOptions>,
    pub(super) document_highlight: Endpoint<T, DocumentHighlightOptions>,
    pub(super) document_symbol: Endpoint<T, DocumentSymbolOptions>,
    pub(super) formatting: Endpoint<T, DocumentFormattingOptions>,
    pub(super) range_formatting: Endpoint<T, RangeFormattingOptions>,
    pub(super) on_type_formatting: Endpoint<T, DocumentOnTypeFormattingOptions>,
    pub(super) definition: Endpoint<T, DefinitionOptions>,
    pub(super) code_action: Endpoint<T, CodeActionOptions>,
    pub(super) code_lens: Endpoint<T, CodeLensOptions>,
    pub(super) resolve_code_lens: Endpoint<T, CodeLensResolveOptions>,
    pub(super) document_link: Endpoint<T, DocumentLinkOptions>,
    pub(super) resolve_document_link: Endpoint<T, DocumentLinkResolveOptions>,
    pub(super) rename: Endpoint<T, RenameOptions>
}

#[repr(i32)]
#[derive(Serialize_repr, Default, Clone, Copy)]
pub enum TextDocumentSyncKind {
    None = 0,
    Full = 1,
    #[default]
    Incremental = 2
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TextDocumentSyncOptions {
    pub open_close: bool,
    pub change: TextDocumentSyncKind,
    pub will_save: bool,
    pub will_save_wait_until: bool,
    pub save: DidSaveOptions
}

impl<T: TypeProvider> TextDocumentService<T> {
    pub(super) fn resolve(&self, method: &str) -> Option<Callback<Server<T>>> {
        match method {
            DidOpenOptions::METHOD => Some(self.did_open.callback()),
            DidChangeOptions::METHOD => Some(self.did_change.callback()),
            WillSaveOptions::METHOD => Some(self.will_save.callback()),
            WillSaveWaitUntilOptions::METHOD => Some(self.will_save_wait_until.callback()),
            DidSaveOptions::METHOD => Some(self.did_save.callback()),
            DidCloseOptions::METHOD => Some(self.did_close.callback()),
            CompletionOptions::METHOD => Some(self.completion.callback()),
            ResolveCompletionOptions::METHOD => Some(self.resolve_completion.callback()),
            HoverOptions::METHOD => Some(self.hover.callback()),
            SignatureHelpOptions::METHOD => Some(self.signature_help.callback()),
            ReferenceOptions::METHOD => Some(self.references.callback()),
            DocumentHighlightOptions::METHOD => Some(self.document_highlight.callback()),
            DocumentSymbolOptions::METHOD => Some(self.document_symbol.callback()),
            DocumentFormattingOptions::METHOD => Some(self.formatting.callback()),
            RangeFormattingOptions::METHOD => Some(self.range_formatting.callback()),
            DocumentOnTypeFormattingOptions::METHOD => Some(self.on_type_formatting.callback()),
            DefinitionOptions::METHOD => Some(self.definition.callback()),
            CodeActionOptions::METHOD => Some(self.code_action.callback()),
            CodeLensOptions::METHOD => Some(self.code_lens.callback()),
            CodeLensResolveOptions::METHOD => Some(self.resolve_code_lens.callback()),
            DocumentLinkOptions::METHOD => Some(self.document_link.callback()),
            DocumentLinkResolveOptions::METHOD => Some(self.resolve_document_link.callback()),
            RenameOptions::METHOD => Some(self.rename.callback()),
            _ => None
        }
    }
}

impl<T: TypeProvider> Default for TextDocumentService<T> {
    fn default() -> Self {
        TextDocumentService {
            sync_kind: Default::default(),
            did_open: DidOpenOptions::endpoint(),
            did_change: DidChangeOptions::endpoint(),
            will_save: WillSaveOptions::endpoint(),
            will_save_wait_until: WillSaveWaitUntilOptions::endpoint(),
            did_save: DidSaveOptions::endpoint(),
            did_close: DidCloseOptions::endpoint(),
            publish_diagnostics: Default::default(),
            completion: CompletionOptions::endpoint(),
            resolve_completion: ResolveCompletionOptions::endpoint(),
            hover: HoverOptions::endpoint(),
            signature_help: SignatureHelpOptions::endpoint(),
            references: ReferenceOptions::endpoint(),
            document_highlight: DocumentHighlightOptions::endpoint(),
            document_symbol: DocumentSymbolOptions::endpoint(),
            formatting: DocumentFormattingOptions::endpoint(),
            range_formatting: RangeFormattingOptions::endpoint(),
            on_type_formatting: DocumentOnTypeFormattingOptions::endpoint(),
            definition: DefinitionOptions::endpoint(),
            code_action: CodeActionOptions::endpoint(),
            code_lens: CodeLensOptions::endpoint(),
            resolve_code_lens: CodeLensResolveOptions::endpoint(),
            document_link: DocumentLinkOptions::endpoint(),
            resolve_document_link: DocumentLinkResolveOptions::endpoint(),
            rename: RenameOptions::endpoint()
        }
    }
}

impl<T: TypeProvider> Server<T> {
    pub fn set_document_sync(&mut self, sync_kind: TextDocumentSyncKind) {
        self.text_document.sync_kind = sync_kind;
    }
}