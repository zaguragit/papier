use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

use crate::APP_ID;
use crate::data::{FileID, Category};
use crate::db::{DB, Saveable};
use adw::{TabView, TabBar, ApplicationWindow, Application};
use glib::{clone, VariantTy};

use gtk4::{Widget, ScrolledWindow, PolicyType, Button, Orientation, Inhibit};
use gtk4::gio::{ActionEntry, ThemedIcon};
use gtk4::{prelude::*, glib, HeaderBar};

use self::cmd::search::command_search_window;
use self::file::display_file;

mod cmd;
mod file;

const ICON_SPREADSHEET: &str = "x-office-spreadsheet-symbolic";
const ICON_TEXTDOC: &str = "x-office-document-symbolic";

pub fn build_ui(app: &Application) {
    let db = Rc::new(Mutex::new(DB::load(
        format!("{}/.var/app/{}/data", std::env::var("HOME").unwrap(), APP_ID))));

    let ui = UI::new(app, &db);

    let ui = Rc::new(RefCell::new(ui));
    let cmd = ActionEntry::builder("cmd")
        .activate(clone!(@strong db, @strong app => move |_, _, _| command_search_window(&db, &app, false)))
        .build();

    let new_text = ActionEntry::builder("new-text")
        .activate(clone!(@strong db, @strong ui => move |_, _, _| {
            let title = "New Text File";
            let id = db.lock().unwrap().new_file(title.to_string(), Category::Text);
            UI::open_file(&ui, &db, id)
        }))
        .build();

    let new_table = ActionEntry::builder("new-table")
        .activate(clone!(@strong db, @strong ui => move |_, _, _| {
            let title = "New Table File";
            let id = db.lock().unwrap().new_file(title.to_string(), Category::Table);
            UI::open_file(&ui, &db, id)
        }))
        .build();

    let open_file = ActionEntry::builder("open-file")
        .parameter_type(Some(VariantTy::STRING))
        .activate(clone!(@strong db, @strong ui => move |_, _, param| {
            let id = param.unwrap().str().unwrap().parse::<FileID>().unwrap();
            UI::open_file(&ui, &db, id)
        }))
        .build();

    ui.borrow_mut().window.connect_close_request(clone!(@strong ui, @strong db => move |_| {
        ui.borrow_mut().save_all(&db);
        Inhibit(false)
    }));

    let ui = ui.borrow_mut();
    
    ui.window.add_action_entries([cmd, new_text, new_table, open_file]);

    ui.window.present();

    command_search_window(&db, &app, true);
}

struct UI {
    window: ApplicationWindow,
    tab_view: TabView,
}

impl UI {
    fn new(app: &Application, db: &Rc<Mutex<DB>>) -> UI {
        let ui: gtk4::Box = gtk4::Box::builder()
            .orientation(Orientation::Vertical)
            .build();
    
        let tab_view = TabView::builder()
            .build();
    
        let tab_bar = TabBar::builder()
            .autohide(false)
            .expand_tabs(true)
            .view(&tab_view)
            .hexpand(true)
            .build();
    
        let header_bar = HeaderBar::builder()
            .build();
    
        header_bar.pack_start(&Button::builder()
            .icon_name("system-search-symbolic")
            .action_name("win.cmd")
            .build());
        header_bar.set_title_widget(Some(&tab_bar));
    
        ui.append(&header_bar);
        ui.append(&ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .hscrollbar_policy(PolicyType::Never)
            .width_request(240)
            .height_request(240)
            .child(&tab_view)
            .build());
    
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Papier")
            .content(&ui)
            .default_width(640)
            .default_height(480)
            .build();

        tab_view.connect_close_page(clone!(@strong db, @strong app => move |tab_view, page| {
            let id = unsafe {
                page.data::<FileID>("id").unwrap().as_ref()
            };
            let saveable = unsafe {
                page.data::<Box<dyn Saveable>>("saveable").unwrap().as_ref()
            };
            saveable.save(db.lock().unwrap().root.clone(), *id);
            // This is the last one, but isn't removed yet
            if tab_view.n_pages() == 1 {
                command_search_window(&db, &app, true);
            }
            false
        }));
    
        UI {
            window,
            tab_view,
        }
    }

    fn open_file(ui: &Rc<RefCell<Self>>, db: &Rc<Mutex<DB>>, id: FileID) {
        let uii = ui.borrow_mut();
        if !uii.try_switch_to_tab(id) {
            let (v, s) = display_file(db, id, clone!(@strong ui, @strong db => move |title| {
                ui.borrow_mut().rename_tab(id, title.as_str());
                db.lock().unwrap().rename_file(id, title);
            }));
            let d = db.lock().unwrap();
            let display = &d.get_file(id).unwrap();
            uii.open_tab(id, match display.category {
                Category::Text => ICON_TEXTDOC,
                Category::Table => ICON_SPREADSHEET,
            }, display.title.as_str(), &v, s);
        }
    }

    fn open_tab(&self, id: FileID, icon_name: &str, title: &str, content: &impl IsA<Widget>, saveable: Box<dyn Saveable>) {
        let page = self.tab_view.append(content);
        page.set_title(title);
        page.set_icon(Some(&ThemedIcon::from_names(&[icon_name])));
        unsafe {
            page.set_data("id", id);
            page.set_data("saveable", saveable);
        }
    }

    fn try_switch_to_tab(&self, id: FileID) -> bool {
        for i in 0..self.tab_view.n_pages() {
            let this_id = unsafe {
                self.tab_view.nth_page(i).data::<FileID>("id").unwrap().as_ref()
            };
            if this_id == &id {
                self.tab_view.pages().select_item(i as u32, true);
                return true;
            }
        }
        false
    }

    fn rename_tab(&self, id: FileID, new_name: &str) -> bool {
        for i in 0..self.tab_view.n_pages() {
            let this_id = unsafe {
                self.tab_view.nth_page(i).data::<FileID>("id").unwrap().as_ref()
            };
            if this_id == &id {
                self.tab_view.nth_page(i).set_title(new_name);
                return true;
            }
        }
        false
    }

    fn save_all(&self, db: &Rc<Mutex<DB>>) {
        for i in 0..self.tab_view.n_pages() {
            let page = self.tab_view.nth_page(i);
            let id = unsafe {
                page.data::<FileID>("id").unwrap().as_ref()
            };
            let saveable = unsafe {
                page.data::<Box<dyn Saveable>>("saveable").unwrap().as_ref()
            };
            saveable.save(db.lock().unwrap().root.clone(), *id);
        }
    }
}