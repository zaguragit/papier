use std::{cell::RefCell, rc::Rc};

use glib::clone;
use gtk4::{traits::{BoxExt, WidgetExt, TextBufferExt, TextViewExt, EditableExt}, TextBuffer, TextView, EventControllerKey, gdk::Key, Inhibit, Text};

use super::TextEditingParagraph;

pub(super) fn create_heading(edit_content: &Rc<RefCell<Vec<TextEditingParagraph>>>, view: &gtk4::Box, level: usize, text: String) -> Text {
    Text::builder()
        .css_classes([format!("title-{level}").as_str()])
        .placeholder_text(format!("Heading {level}"))
        .hexpand(true)
        .editable(true)
        .text(&text)
        //.extra_menu(text_menu)
        .build()
}

pub(super) fn create_p(edit_content: &Rc<RefCell<Vec<TextEditingParagraph>>>, view: &gtk4::Box, text: String) -> TextView {
    let b = TextBuffer::builder().text(text).build();
    let v = TextView::builder()
        .buffer(&b)
        .hexpand(true)
        //.extra_menu(text_menu)
        .build();
    v.connect_paste_clipboard(|v| {
        
    });
    v.add_controller(create_text_controller(view, &v, edit_content));
    v
}

pub(super) fn create_text_controller(view: &gtk4::Box, me: &TextView, edit_content: &Rc<RefCell<Vec<TextEditingParagraph>>>) -> EventControllerKey {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(clone!(@strong view, @strong me, @strong edit_content => move |_, k, _, _| {
        let buffer = me.buffer();
        let c = buffer.cursor_position();
        match k {
            Key::Up if buffer.iter_at_offset(c).line() == 0 => {
                let edit_content = edit_content.borrow_mut();
                let i = edit_content.iter().position(|p| match p {
                    TextEditingParagraph::Text(p) => p == &me,
                    _ => false,
                }).unwrap();
                if i > 0 {
                    match &edit_content[i - 1] {
                        TextEditingParagraph::Text(b) => {
                            b.grab_focus();
                            let b = b.buffer();
                            b.place_cursor(&b.iter_at_line(b.line_count() - 1).unwrap());
                            return Inhibit(true);
                        },
                        TextEditingParagraph::H2(t) => {
                            t.grab_focus();
                            t.set_position(0);
                            return Inhibit(true);
                        }
                        TextEditingParagraph::H3(t) => {
                            t.grab_focus();
                            t.set_position(0);
                            return Inhibit(true);
                        }
                        TextEditingParagraph::H4(t) => {
                            t.grab_focus();
                            t.set_position(0);
                            return Inhibit(true);
                        }
                    }
                }
                Inhibit(false)
            },
            Key::Down if buffer.iter_at_offset(c).line() == buffer.line_count() - 1 => {
                let edit_content = edit_content.borrow_mut();
                let i = edit_content.iter().position(|p| match p {
                    TextEditingParagraph::Text(p) => p == &me,
                    _ => false,
                }).unwrap();
                if i + 1 < edit_content.len() {
                    match &edit_content[i + 1] {
                        TextEditingParagraph::Text(b) => {
                            b.grab_focus();
                            let b = b.buffer();
                            b.place_cursor(&b.iter_at_line(0).unwrap());
                            return Inhibit(true);
                        },
                        TextEditingParagraph::H2(t) => {
                            t.grab_focus();
                            t.set_position(0);
                            return Inhibit(true);
                        }
                        TextEditingParagraph::H3(t) => {
                            t.grab_focus();
                            t.set_position(0);
                            return Inhibit(true);
                        }
                        TextEditingParagraph::H4(t) => {
                            t.grab_focus();
                            t.set_position(0);
                            return Inhibit(true);
                        }
                    }
                }
                Inhibit(false)
            },
            Key::Return => {
                let new = buffer.text(&buffer.iter_at_offset(c), &buffer.end_iter(), true);
                buffer.delete(&mut buffer.iter_at_offset(c), &mut buffer.end_iter());
                let v = create_p(&edit_content, &view, new.to_string());

                let mut edit_content = edit_content.borrow_mut();
                let i = edit_content.iter().position(|p| match p {
                    TextEditingParagraph::Text(p) => p == &me,
                    _ => false,
                }).unwrap();

                view.insert_child_after(&v, Some(&me));
                v.grab_focus();
                v.buffer().place_cursor(&v.buffer().start_iter());
                edit_content.insert(i + 1, TextEditingParagraph::Text(v));
                
                Inhibit(true)
            },
            Key::BackSpace if c == 0 => {
                let mut edit_content = edit_content.borrow_mut();
                let i = edit_content.iter().position(|p| match p {
                    TextEditingParagraph::Text(p) => p == &me,
                    _ => false,
                }).unwrap();

                let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true);

                match &edit_content[i - 1] {
                    TextEditingParagraph::Text(b) => {
                        b.grab_focus();
                        let b = b.buffer();
                        b.place_cursor(&b.end_iter());
                        b.insert(&mut b.end_iter(), &text);
                        view.remove(&me);
                        edit_content.remove(i);
                        return Inhibit(true);
                    },
                    _ => (),
                }
                
                Inhibit(true)
            },
            _ => Inhibit(false),
        }
    }));
    controller
}