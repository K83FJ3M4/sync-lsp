use std::collections::HashMap;

use serde::Serialize;

use crate::text_document::{DocumentUri, TextEdit};

#[derive(Serialize, Debug, Default)]
pub struct WorkspaceEdit {
    pub changes: HashMap<DocumentUri, Vec<TextEdit>>
}