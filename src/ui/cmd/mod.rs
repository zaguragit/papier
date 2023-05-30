use glib::{Variant, ToVariant};

use crate::data::{FileID, Category};

use super::{ICON_SPREADSHEET, ICON_TEXTDOC};

pub mod search;

#[derive(Debug, Clone)]
pub struct Command {
    pub label: String,
    pub keywords: Vec<String>,
    pub icon_name: &'static str,
    pub action_name: &'static str,
    pub param: Option<Variant>,
}

impl Command {
    pub fn all() -> [Self; 4] {[
        Self {
            label: "About Papier".to_string(),
            keywords: vec!["about".to_string(), "papier".to_string()],
            icon_name: "help-about-symbolic",
            action_name: "app.about",
            param: None,
        },
        Self {
            label: "New Text".to_string(),
            keywords: vec!["create".to_string(), "file".to_string(), "new".to_string(), "text".to_string()],
            icon_name: "document-new-symbolic",
            action_name: "win.new-text",
            param: None,
        },
        Self {
            label: "New Table".to_string(),
            keywords: vec!["create".to_string(), "file".to_string(), "new".to_string(), "table".to_string()],
            icon_name: "document-new-symbolic",
            action_name: "win.new-table",
            param: None,
        },
        Self {
            label: "Show Keyboard Shortcuts".to_string(),
            keywords: vec!["help".to_string(), "accelerator".to_string(), "shortcut".to_string(), "keyboard".to_string()],
            icon_name: "document-new-symbolic",
            action_name: "win.show-help-overlay",
            param: None,
        },
    ]}

    fn open_file(id: FileID, title: &str, category: Category) -> Self {
        Command {
            label: "Open “".to_string() + title + "”",
            keywords: vec![], 
            icon_name: match category {
                Category::Table => ICON_SPREADSHEET,
                Category::Text => ICON_TEXTDOC,
            },
            action_name: "win.open-file",
            param: Some(id.to_string().to_variant()),
        }
    }
}