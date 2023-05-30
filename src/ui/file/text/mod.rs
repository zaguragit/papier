use std::{cell::RefCell, rc::Rc, sync::Mutex};

use glib::Cast;
use gtk4::{Widget, traits::{BoxExt, EditableExt, TextBufferExt, TextViewExt}, Orientation, Text, gio::Menu, TextView};

use crate::{data::{text::{TextContent, Paragraph}, FileID}, db::{Saveable, DB}};

use self::components::{create_p, create_heading};

use super::components::create_header;

mod components;

#[derive(Debug, Clone)]
struct TextEditingState {
    content: Rc<RefCell<Vec<TextEditingParagraph>>>,
}

#[derive(Debug, Clone)]
enum TextEditingParagraph {
    Text(TextView),
    H2(Text),
    H3(Text),
    H4(Text),
}

impl TextEditingState {
    fn make_text_content(&self) -> TextContent {
        TextContent {
            paragraphs: self.content.borrow_mut().iter().map(|p| match p {
                TextEditingParagraph::Text(p) => {
                    let b = p.buffer();
                    Paragraph::Text(b.text(&b.start_iter(), &b.end_iter(), true).to_string())
                },
                TextEditingParagraph::H2(p) => Paragraph::H2(p.text().to_string()),
                TextEditingParagraph::H3(p) => Paragraph::H3(p.text().to_string()),
                TextEditingParagraph::H4(p) => Paragraph::H4(p.text().to_string()),
            }).collect()
        }
    }
}

impl Saveable for TextEditingState {
    fn save(&self, root: String, id: FileID) {
        self.make_text_content().save(root, id)
    }
}

//const MAX_WIDTH: i32 = 720;

pub fn display_text<F: Fn(String) + 'static>(
    db: &Rc<Mutex<DB>>,
    id: FileID,
    title: &str,
    keywords: Vec<String>,
    mut content: TextContent,
    on_rename: F,
) -> (Widget, Box<dyn Saveable>) {
    if content.paragraphs.is_empty() {
        content.paragraphs.push(Paragraph::Text(String::new()));
    }
    
    let edit_content = Rc::new(RefCell::new(Vec::new()));

    let view = gtk4::Box::builder()
        .css_classes(["file-editor", "text-editor"])
        .orientation(Orientation::Vertical)
        .hexpand(true)
        // .width_request(MAX_WIDTH)
        // .halign(Align::Center)
        .build();
    view.append(&create_header(db, id, title, keywords, on_rename));

    let text_menu = Menu::new();
    text_menu.append(Some("Into Heading 2"), Some("text.into-heading2"));
    text_menu.append(Some("Into Heading 3"), Some("text.into-heading3"));
    text_menu.append(Some("Into Heading 4"), Some("text.into-heading4"));
    text_menu.append(Some("Into Text"), Some("text.into-text"));
    for p in content.paragraphs {
        match p {
            Paragraph::Text(text) => {
                let v = create_p(&edit_content, &view, text);
                view.append(&v);
                edit_content.borrow_mut().push(TextEditingParagraph::Text(v));
            },
            Paragraph::H2(text) => {
                let p = create_heading(&edit_content, &view, 2, text);
                view.append(&p);
                edit_content.borrow_mut().push(TextEditingParagraph::H2(p));
            },
            Paragraph::H3(text) => {
                let p = create_heading(&edit_content, &view, 3, text);
                view.append(&p);
                edit_content.borrow_mut().push(TextEditingParagraph::H3(p));
            },
            Paragraph::H4(text) => {
                let p = create_heading(&edit_content, &view, 4, text);
                view.append(&p);
                edit_content.borrow_mut().push(TextEditingParagraph::H4(p));
            },
        }
    }
    (view.upcast(), Box::new(TextEditingState { content: edit_content }))
}