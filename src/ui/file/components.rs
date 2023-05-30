use std::{rc::Rc, sync::Mutex};

use glib::clone;
use gtk4::{Orientation, FlowBox, traits::{BoxExt, WidgetExt, EditableExt, ButtonExt, EntryExt, PopoverExt}, Align, SelectionMode, Label, Text, Button, Popover, Entry};

use crate::{db::DB, data::FileID};

pub(super) fn create_header<F: Fn(String) + 'static>(
    db: &Rc<Mutex<DB>>,
    id: FileID,
    title: &str,
    keywords: Vec<String>,
    on_rename: F,
) -> gtk4::Box {
    let header = gtk4::Box::builder()
        .css_classes(["header"])
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .build();
    let title = Text::builder()
        .css_classes(["title-1"])
        .placeholder_text("Title")
        .hexpand(true)
        .editable(true)
        .text(title)
        .build();
    title.connect_text_notify(clone!(@strong title => move |title| {
        on_rename(title.text().to_string())
    }));
    header.append(&title);
    let keywords_box = FlowBox::builder()
        .css_classes(["keywords"])
        .orientation(Orientation::Horizontal)
        .halign(Align::Start)
        .min_children_per_line(2)
        .selection_mode(SelectionMode::None)
        .build();
    for k in keywords {
        let keyword_chip = create_keyword_chip(db, id, &keywords_box, k);
        keywords_box.append(&keyword_chip);
    }
    let add_button = Button::builder()
        .css_classes(["add-button"])
        .icon_name("list-add-symbolic")
        .build();

    let entry = Entry::builder()
        .placeholder_text("New Keyword")
        .build();
    let popover = Popover::builder()
        .autohide(true)
        .child(&entry)
        .build();
    entry.connect_activate(clone!(@strong db, @strong keywords_box, @strong add_button => move |entry| {
        let mut locked_db = db.lock().unwrap();
        let mut keywords = locked_db.get_file(id).unwrap().keywords.clone();
        let k = entry.text();
        if !keywords.iter().any(|x| x == &k) {
            keywords.push(k.to_string());
            locked_db.set_file_keywords(id, keywords);
            let keyword_chip = create_keyword_chip(&db, id, &keywords_box, k.to_string());
            keywords_box.insert(&keyword_chip, 0);
        }
    }));
    popover.set_parent(&add_button);
    
    add_button.connect_clicked(clone!(@strong db, @strong keywords_box => move |_| {
        popover.popup();
    }));
    keywords_box.append(&add_button);
    header.append(&keywords_box);
    header
}

fn create_keyword_chip(db: &Rc<Mutex<DB>>, id: FileID, keywords_box: &FlowBox, k: String) -> gtk4::Box {
    let keyword_chip = gtk4::Box::builder()
        .css_classes(["keyword-chip"])
        .orientation(Orientation::Horizontal)
        .build();
    keyword_chip.append(&Label::new(Some(&k)));
    let remove_button = Button::builder()
        .icon_name("window-close-symbolic")
        .build();
    remove_button.connect_clicked(clone!(@strong db, @strong keywords_box, @strong keyword_chip => move |_| {
        let mut db = db.lock().unwrap();
        let mut keywords = db.get_file(id).unwrap().keywords.clone();
        if let Some(i) = keywords.iter().position(|x| x == &k) {
            keywords.swap_remove(i);
            db.set_file_keywords(id, keywords);
        }
        keywords_box.remove(&keyword_chip);
    }));
    keyword_chip.append(&remove_button);
    keyword_chip
}