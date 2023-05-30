use crate::data::{FileID, text::TextContent, table::TableContent};

use super::store;

pub trait Saveable {
    fn save(&self, root: String, id: FileID);
}

impl Saveable for TextContent {
    fn save(&self, root: String, id: FileID) {
        store::store_note_content(root, id, self);
    }
}

impl Saveable for TableContent {
    fn save(&self, root: String, id: FileID) {
        store::store_table_content(root, id, self);
    }
}