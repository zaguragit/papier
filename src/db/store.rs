use std::{fs::{create_dir_all, read_to_string, read_dir}, collections::HashMap};
use json::{object, JsonValue};
use crate::data::{FileID, FileDisplay, Category, text::{TextContent, Paragraph}, table::{TableContent, column::Column, TableCell}};

pub fn load_files(root: String) -> HashMap<FileID, FileDisplay> {
    match read_dir(root + "/files") {
        Ok(x) => {
            x.filter_map(|x| {
                let file = x.ok()?;
                let name = file.file_name().into_string().ok()?;
                let id = name.parse().ok()?;
                let json = json::parse(&read_to_string(
                    file.path().to_str()?.to_string() + "/cover.json"
                ).ok()?).ok()?;
                let title = json["title"].as_str()?.to_string();
                let category = match json["category"].as_str()? {
                    "text" => Category::Text,
                    "table" => Category::Table,
                    _ => return None,
                };
                let keywords = match &json["keywords"] {
                    JsonValue::Array(k) =>
                        k.into_iter().map(|x| x.to_string()).collect(),
                    _ => vec![],
                };
                Some((id, FileDisplay { title, category, keywords }))
            }).collect()
        },
        Err(_) => HashMap::new(),
    }
}

pub fn store_file_display(root: String, id: FileID, note: &FileDisplay) {
    let dir = root + "/files/" + id.to_string().as_str();
    let _ = create_dir_all(&dir);
    let json = object! {
        title: note.title.as_str(),
        category: match note.category {
            Category::Text => "text".to_string(),
            Category::Table => "table".to_string(),
        },
        keywords: note.keywords.clone(),
    };
    let _ = std::fs::write(dir + "/cover.json", json.to_string());
}

pub fn load_text_content(root: String, id: FileID) -> Option<TextContent> {
    let path = root + "/files/" + id.to_string().as_str() + "/content.json";
    let json = read_to_string(path).ok()?;
    let json = json::parse(json.as_str()).ok()?;
    let c = match json {
        JsonValue::Array(paragraphs) => TextContent {
            paragraphs: paragraphs.into_iter().filter_map(|json| match json["type"].as_str()? {
                "p" => Some(Paragraph::Text(json["text"].to_string())),
                "h2" => Some(Paragraph::H2(json["text"].to_string())),
                "h3" => Some(Paragraph::H3(json["text"].to_string())),
                "h4" => Some(Paragraph::H4(json["text"].to_string())),
                _ => None
            }).collect()
        },
        _ => TextContent::default(),
    };
    Some(c)
}

pub fn store_note_content(root: String, id: FileID, note: &TextContent) {
    let dir = root + "/files/" + id.to_string().as_str();
    let _ = create_dir_all(&dir);
    let json = JsonValue::Array(note.paragraphs.iter().map(|x| match x {
        Paragraph::Text(text) => object! { "type": "p", text: text.to_string() },
        Paragraph::H2(text) => object! { "type": "h2", text: text.to_string() },
        Paragraph::H3(text) => object! { "type": "h3", text: text.to_string() },
        Paragraph::H4(text) => object! { "type": "h4", text: text.to_string() },
    }).collect());
    let _ = std::fs::write(dir + "/content.json", json.to_string());
}

pub fn load_table_content(root: String, id: FileID) -> Option<TableContent> {
    let path = root + "/files/" + id.to_string().as_str() + "/content.json";
    let json = read_to_string(path).ok()?;
    let json = json::parse(json.as_str()).ok()?;
    let columns = match &json["columns"] {
        JsonValue::Array(columns) =>
            columns.into_iter().map(|json| Column {
                id: json["id"].to_string().parse().unwrap(),
                name: json["name"].as_str().unwrap_or_else(|| "").to_string(),
                unique: json["unique"].as_bool().unwrap_or(false),
            }).collect(),
        _ => vec![],
    };
    let cells = match &json["cells"] {
        JsonValue::Array(cells) =>
            cells.into_iter().map(|json| TableCell {
                content: json.as_str().map(|s| s.to_string()),
            }).collect(),
        _ => vec![],
    };
    Some(TableContent { columns, cells })
}

pub fn store_table_content(root: String, id: FileID, table: &TableContent) {
    let dir = root + "/files/" + id.to_string().as_str();
    let _ = create_dir_all(&dir);
    let json = object! {
        columns: table.columns().iter().map(|column| object! {
            id: column.id.to_string(),
            name: column.name.clone(),
            unique: column.unique,
        }).collect::<Vec<_>>(),
        cells: table.cells.iter().map(|cell| cell.content.clone()).collect::<Vec<_>>(),
    };
    let _ = std::fs::write(dir + "/content.json", json.to_string());
}