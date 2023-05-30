use std::collections::HashMap;

use rand::random;

use crate::data::{FileID, FileDisplay, text::TextContent, Category, table::TableContent};

mod saveable;
mod store;

pub use saveable::*;

pub struct DB {
    pub root: String,
    files: HashMap<FileID, FileDisplay>,
}

impl DB {
    pub fn load(root: String) -> Self {
        Self {
            root: root.clone(),
            files: store::load_files(root)
        }
    }
    pub fn ids(&self) -> Vec<FileID> {
        self.files.keys().copied().collect()
    }
    pub fn get_file(&self, id: FileID) -> Option<&FileDisplay> {
        self.files.get(&id)
    }
    pub fn get_text_content(&self, id: FileID) -> TextContent {
        if let Some(content) = store::load_text_content(self.root.clone(), id) {
            content
        } else {
            let content = TextContent::default();
            store::store_note_content(self.root.clone(), id, &content);
            content
        }
    }
    pub fn get_table_content(&self, id: FileID) -> TableContent {
        if let Some(content) = store::load_table_content(self.root.clone(), id) {
            content
        } else {
            let content = TableContent::default();
            store::store_table_content(self.root.clone(), id, &content);
            content
        }
    }
    pub fn new_file(&mut self, title: String, category: Category) -> FileID {
        let id = self.gen_id();
        let d = FileDisplay { title, category, keywords: vec![] };
        store::store_file_display(self.root.clone(), id, &d);
        self.files.insert(id, d);
        id
    }
    pub fn rename_file(&mut self, id: FileID, title: String) {
        let d = self.files.get_mut(&id).unwrap();
        d.title = title;
        store::store_file_display(self.root.clone(), id, &d);
    }
    pub fn set_file_keywords(&mut self, id: FileID, keywords: Vec<String>) {
        let d = self.files.get_mut(&id).unwrap();
        d.keywords = keywords;
        store::store_file_display(self.root.clone(), id, &d);
    }
    fn gen_id(&self) -> FileID {
        let id = FileID(random::<u64>());
        if self.files.contains_key(&id) {
            self.gen_id()
        } else {
            id
        }
    }
}
