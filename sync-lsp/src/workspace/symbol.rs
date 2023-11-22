use crate::TypeProvider;
use crate::text_document::Location;
use crate::{Server, connection::Endpoint};
use crate::connection::Callback;
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

#[derive(Default, Clone)]
pub(crate) struct SymbolOptions;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkspaceSymbolParams  {
    query: String
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInformation {
    pub name: String,
    pub kind: SymbolKind,
    pub location: Location,
    pub container_name: Option<String>
}

#[repr(i32)]
#[derive(Serialize_repr, Debug)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
}

impl SymbolOptions {

    pub(crate) const METHOD: &'static str = "workspace/symbol";
    
    pub(super) fn endpoint<T: TypeProvider>() -> Endpoint<T, SymbolOptions> {
        Endpoint::new(Callback::request(|_, _: WorkspaceSymbolParams| Vec::<SymbolInformation>::new()))
    }
}

impl<T: TypeProvider> Server<T> {
    pub fn on_symbol(&mut self, callback: fn(&mut Server<T>, String) -> Vec<SymbolInformation>) {
        self.workspace.symbol.set_callback(Callback::request(move |server, params: WorkspaceSymbolParams| {
            callback(server, params.query)
        }))
    }
}