//! implementation of the `textDocument/completion` request
//! 
//! # Usage
//! [`Server::on_completion`] is invoked when completion items are requested for
//! a cursor position. As the computation of completion items can be expensive
//! [`Server::on_resolve_completion`] may be used to resolve additional
//! information for a completion item. Additionally, [`Server::set_completion_trigger_character`]
//! can be used to set the characters that trigger a completion. If the client
//! supports snippets, [`Server::snippet_support`] will return true. Otherwise,
//! [`InsertTextFormat::Snippet`] should not be used.

use crate::workspace::execute_command::{serialize_opt_command, deserialize_opt_command};
use crate::{Server, TypeProvider};
use crate::connection::{Callback, Endpoint};
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use super::{TextDocumentIdentifer, TextDocumentPositionParams, Position, TextEdit};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CompletionOptions {
    resolve_provider: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    trigger_characters: Vec<String>
}

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub(super) struct CompletionCapabilities {
    dynamic_registration: bool,
    completion_item: ItemCapabilities,
}

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct ItemCapabilities {
    snippet_support: bool
}


#[derive(Clone, Default)]
pub(crate) struct ResolveCompletionOptions;

/// A list of completion items.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "")]
pub struct CompletionList<T: TypeProvider> {
    /// This may cause recomputations if set to `true`.
    pub is_incomplete: bool,
    /// The completion items.
    pub items: Vec<CompletionItem<T>>,
}

/// Defines different kind of completion items.
/// Mainly used in the ui to resolve icons.
#[repr(i32)]
#[derive(Serialize_repr, Deserialize_repr, Debug)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18
}

/// Defines whether the insert text contains snippets.
#[repr(i32)]
#[derive(Serialize_repr, Deserialize_repr, Debug)]
pub enum InsertTextFormat {
    /// This completion item is plain text.
    PlainText = 1,
    /// **Warning:** This variant should only be used if [`Server::snippet_support`] returns true;
    /// This completion item is a [snippet](https://github.com/Microsoft/vscode/blob/master/src/vs/editor/contrib/snippet/common/snippet.md),
    /// allowing it to define placeholders
    /// using `$1`, `$2` and `${3:foo}`. `$0` may optionally be used to define
    /// the final tab stop, if omitted it will default to the end of the snippet.
    /// Equal identifiers are linked.
    Snippet = 2
}

/// The actual completion item.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItem<T: TypeProvider> {
    /// This is shown in the ui and also applied as the edit, if
    /// `insert_text` is not set.
    pub label: String,
    /// This is used to resolve the icon in the ui.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CompletionItemKind>,
    /// This is used to provide additional information.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// A doc-comment.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    /// A sort text may be used to alter the sorting of this in relation to others.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_text: Option<String>,
    /// This is used to filter the completion items.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_text: Option<String>,
    /// The text to insert.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text: Option<String>,
    /// The format of the insert text.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text_format: Option<InsertTextFormat>,
    /// A optional edit to be applied to the document.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_edit: Option<TextEdit>,
    /// An array of additional non overlapping edits.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_text_edits: Vec<TextEdit>,
    /// This command will be executed after inserting the completion.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_opt_command")]
    #[serde(deserialize_with = "deserialize_opt_command")]
    pub command: Option<T::Command>,
    /// Additional data to identify this completion item.
    pub data: T::CompletionData
}

impl CompletionOptions {
    pub(crate) const METHOD: &'static str = "textDocument/completion";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, CompletionOptions> {
        Endpoint::new(Callback::request(|_, _: TextDocumentPositionParams| CompletionList::<T>::default()))
    }
}

impl ResolveCompletionOptions {
    pub(crate) const METHOD: &'static str = "completionItem/resolve";

    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, ResolveCompletionOptions> {
        Endpoint::new(Callback::request(|_, item: CompletionItem<T>| item))
    }
}

impl<T: TypeProvider> Server<T> {

    /// Sets the callback that will be called to [compute completion items](self).
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon as completion items are requested:
    ///     * The server instance receiving the response.
    ///     * The [`TextDocumentIdentifer`] of the document for which completions are requested.
    ///     * `return` - A list of completions to display.


    pub fn on_completion(&mut self, callback: fn(&mut Server<T>, TextDocumentIdentifer, Position) -> CompletionList<T>) {
        self.text_document.completion.set_callback(Callback::request(move |server, params: TextDocumentPositionParams| {
            callback(server, params.text_document, params.position)
        }));
    }

    /// Sets the callback that will be called to compute missing information on [completion items](self), which were previously returned by [`Server::on_completion`].
    /// 
    /// # Argument
    /// * `callback` - A callback which is called with the following parameters as soon a completion can be resolved:
    ///     * The server instance receiving the response.
    ///     * The [`CompletionItem`] to resolve.
    ///     * `return` - The resolved completion.

    pub fn on_resolve_completion(&mut self, callback: fn(&mut Server<T>, CompletionItem<T>) -> CompletionItem<T>) {
        self.text_document.resolve_completion.set_callback(Callback::request(move |server, item| {
            callback(server, item)
        }));
    }

    /// The client will request [completions](self) if one of the characters in `trigger_characters` is typed.
    /// 
    /// # Argument
    /// * `trigger_characters` - A list of characters that trigger completion. 

    pub fn set_completion_trigger_character(&mut self, trigger_characters: Vec<String>) {
        self.text_document.completion.options_mut().trigger_characters = trigger_characters;
    }

    /// Returns whether the client supports snippets for [completion items](self).
    /// 
    /// # Return
    /// * `true` if the client supports snippets, `false` otherwise.

    pub fn snippet_support(&self) -> bool {
        self.capabilities.text_document.completion.completion_item.snippet_support
    }
}

impl Default for CompletionOptions {
    fn default() -> Self {
        CompletionOptions {
            resolve_provider: true,
            trigger_characters: Vec::new()
        }
    }
}


impl<T: TypeProvider> Default for CompletionList<T> {
    fn default() -> Self {
        CompletionList {
            is_incomplete: false,
            items: Vec::new()
        }
    }
}