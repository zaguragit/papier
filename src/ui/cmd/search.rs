use std::rc::Rc;
use std::sync::Mutex;

use crate::db::DB;
use adw::{Window};
use glib::{BoxedAnyObject, clone, Object};
use gtk4::gdk::Key;
use gtk4::pango::EllipsizeMode;
use gtk4::{Label, ListItem, ScrolledWindow, Align, Image, SearchEntry, EventControllerKey, Inhibit};
use gtk4::gio::{ListStore};
use gtk4::{prelude::*, glib, Orientation, ListView, SignalListItemFactory, SingleSelection};
use rust_fuzzy_search::fuzzy_compare;

use super::Command;

impl Command {
    pub fn search(db: &DB, q: &str) -> ListStore {
        let ids = db.ids().into_iter();
        if q.is_empty() {
            let model = ListStore::new(BoxedAnyObject::static_type());
            for id in ids {
                let display = db.get_file(id).unwrap();
                model.append(&BoxedAnyObject::new(
                    Self::open_file(id, &display.title, display.category)));
            }
            return model;
        }
        let q = q.trim().to_lowercase();
    
        let mut commands = {
            Self::all().iter()
                .filter_map(|action| {
                    let m: f32 = action.keywords.iter().map(|k| fuzzy_compare(&q, &k)).sum();
                    if m > 0.45 {
                        Some((action.clone(), m))
                    } else {
                        None
                    }
                })
                .chain(ids.filter_map(|id| {
                    let display = db.get_file(id).unwrap();
                    let m = fuzzy_compare(&q, &display.title.trim().to_lowercase());
                    let mk: f32 = display.keywords.iter().map(|k| fuzzy_compare(&q, &k.trim().to_lowercase())).sum();
                    let m = m + mk;
                    if m > 0.4 {
                        Some((Self::open_file(id, &display.title, display.category), m))
                    } else {
                        None
                    }
                }))
                .collect::<Vec<_>>()
        };
    
        commands.sort_by(|(_, a), (_, b)| a.total_cmp(b));
        let commands = commands.into_iter().map(|(c, _)| c).collect::<Vec<_>>();
    
        let model = ListStore::new(BoxedAnyObject::static_type());
        for command in commands {
            model.append(&BoxedAnyObject::new(command));
        }
        model
    }
}

pub fn command_search_window(db: &Rc<Mutex<DB>>, app: &adw::Application, close_win_on_esc: bool) {
    let factory = SignalListItemFactory::new();
    factory.connect_setup(create_cmd_item_view);
    factory.connect_bind(clone!(@strong db => move |_, item| {
        let item = item
            .downcast_ref::<ListItem>().unwrap();
        let widget = item.child()
            .and_downcast::<gtk4::Box>().unwrap();
        let icon = widget.first_child().and_downcast::<Image>().unwrap();
        let label = widget.last_child().and_downcast::<Label>().unwrap();
        let command = item.item()
            .and_downcast::<BoxedAnyObject>()
            .unwrap()
            .borrow_mut::<Command>().clone();
        label.set_text(&command.label);
        icon.set_icon_name(Some(command.icon_name));
    }));

    let model = SingleSelection::builder()
        .model(&Command::search(&db.lock().unwrap(), ""))
        .can_unselect(true)
        .build();
    model.unselect_item(0);

    let list_view = ListView::builder()
        .factory(&factory)
        .model(&model)
        .css_classes(["command-list"])
        .hexpand(true)
        .build();

    let search_bar = SearchEntry::builder()
        .placeholder_text("Search files & actions…")
        .hexpand(true)
        .build();

    search_bar.connect_text_notify(clone!(@strong model, @strong db => move |sb| {
        model.set_model(Some(&Command::search(&db.lock().unwrap(), &sb.text())));
    }));

    let scroll = ScrolledWindow::builder()
        .child(&list_view)
        .height_request(256)
        .vexpand(true)
        .hexpand(true)
        .build();

    let widget = gtk4::Box::builder()
        .width_request(64)
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .css_classes(["command-searcher"])
        .build();
    widget.append(&search_bar);
    widget.append(&scroll);

    let w = app.active_window().unwrap();
    let window = Window::builder()
        .application(app)
        .destroy_with_parent(true)
        .modal(true)
        .transient_for(&w)
        .content(&widget)
        .default_width(320)
        .default_height(420)
        .build();

    let controller = EventControllerKey::new();
    controller.connect_key_pressed(clone!(@strong w, @strong window => move |_, k, _, _| {
        if k == Key::Escape {
            window.close();
            if close_win_on_esc {
                w.close();
            }
            Inhibit(true)
        } else {
            Inhibit(false)
        }
    }));
    window.add_controller(controller);
    search_bar.connect_stop_search(clone!(@strong w, @strong window => move |_| {
        window.close();
        if close_win_on_esc {
            w.close();
        }
    }));

    list_view.connect_activate(clone!(@strong app, @strong window => move |_, i| {
        let command = model.item(i)
            .and_downcast::<BoxedAnyObject>().unwrap()
            .borrow_mut::<Command>().clone();
        window.close();
        app.active_window().expect("No active window")
            .activate_action(command.action_name, command.param.as_ref()).unwrap();
    }));

    window.present();
}

fn create_cmd_item_view(_: &SignalListItemFactory, item: &Object) {
    let icon = Image::new();
    let label = Label::builder()
        .ellipsize(EllipsizeMode::End)
        .build();
    let widget = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .valign(Align::Center)
        .css_classes(["command-item"])
        .build();
    widget.append(&icon);
    widget.append(&label);
    item
        .downcast_ref::<ListItem>().unwrap()
        .set_child(Some(&widget));
}