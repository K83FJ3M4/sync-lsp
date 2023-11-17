use crate::Connection;
use crate::connection::{Callback, Endpoint};
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_repr::{Serialize_repr, Deserialize_repr};
use super::{TextDocumentIdentifer, TextDocumentPositionParams, Position, TextEdit};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CompletionOptions {
    resolve_provider: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    trigger_characters: Vec<String>
}

#[derive(Clone, Default)]
pub(crate) struct ResolveCompletionOptions;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompletionList<T> {
    pub is_incomplete: bool,
    pub items: Vec<CompletionItem<T>>,
}

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

#[repr(i32)]
#[derive(Serialize_repr, Deserialize_repr, Debug)]
pub enum InsertTextFormat {
    PlainText = 1,
    Snippet = 2
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItem<T = ()> {
    pub label: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CompletionItemKind>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_text: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_text: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text_format: Option<InsertTextFormat>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_edit: Option<TextEdit>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_text_edits: Vec<TextEdit>,
    //TODO set to Command
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    pub data: T
}

impl CompletionOptions {
    pub(crate) const METHOD: &'static str = "textDocument/completion";
    
    pub(super) fn endpoint<T>() -> Endpoint<T, CompletionOptions> {
        Endpoint::new(Callback::request(|_, _: TextDocumentPositionParams| CompletionList::<()>::default()))
    }
}

impl ResolveCompletionOptions {
    pub(crate) const METHOD: &'static str = "completionItem/resolve";

    pub(super) fn endpoint<T>() -> Endpoint<T, ResolveCompletionOptions> {
        Endpoint::new(Callback::request(|_, item: CompletionItem<Value>| item))
    }
}

impl<T> Connection<T> {
    pub fn on_completion<D: 'static + Serialize + DeserializeOwned>(&mut self,
        callback: fn(&mut Connection<T>, TextDocumentIdentifer, Position) -> CompletionList<D>,
        resolve: fn(&mut Connection<T>, CompletionItem<D>) -> CompletionItem<D>)
    {
        self.text_document.completion.set_callback(Callback::request(move |connection, params: TextDocumentPositionParams| {
            callback(connection, params.text_document, params.position)
        }));

        self.text_document.resolve_completion.set_callback(Callback::request(move |connection, item: CompletionItem<D>| {
            resolve(connection, item)
        }));
    }

    pub fn set_completion_trigger_character(&mut self, trigger_characters: Vec<String>) {
        self.text_document.completion.options_mut().trigger_characters = trigger_characters;
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