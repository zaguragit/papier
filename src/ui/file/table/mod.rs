
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

use crate::data::FileID;
use crate::data::table::TableContent;
use crate::data::table::column::Column;
use crate::data::table::row::Row;
use crate::db::DB;
use crate::db::Saveable;
use glib::BoxedAnyObject;
use glib::Object;
use glib::SignalHandlerId;
use glib::clone;
use gtk4::ColumnView;
use gtk4::ColumnViewColumn;
use gtk4::EventControllerKey;
use gtk4::Inhibit;
use gtk4::ListItem;
use gtk4::MultiSelection;
use gtk4::Orientation;
use gtk4::SelectionModel;
use gtk4::SignalListItemFactory;
use gtk4::Text;
use gtk4::gdk::Key;
use gtk4::gio::ListStore;
use gtk4::{prelude::*, Widget};

use super::components::create_header;

#[derive(Debug, Clone)]
struct TableEditingState {
    columns: Rc<RefCell<Vec<Column>>>,
    model: Rc<RefCell<SelectionModel>>,
}

impl TableEditingState {
    fn make_table_content(&self) -> TableContent {
        let mut cells = Vec::new();
        for item in self.model.borrow_mut().iter::<Object>() {
            let mut item = item.unwrap()
                .downcast::<BoxedAnyObject>()
                .unwrap()
                .borrow_mut::<Row>().clone();
            cells.append(&mut item.cells);
        }
        TableContent {
            columns: self.columns.borrow_mut().clone(),
            cells,
        }
    }
}

impl Saveable for TableEditingState {
    fn save(&self, root: String, id: FileID) {
        self.make_table_content().save(root, id)
    }
}

pub fn display_table<F: Fn(String) + 'static>(
    db: &Rc<Mutex<DB>>,
    id: FileID,
    title: &str,
    keywords: Vec<String>,
    mut content: TableContent,
    on_rename: F,
) -> (Widget, Box<dyn Saveable>) {
    if content.is_empty() {
        content.insert_row(0);
    }
    let view = gtk4::Box::builder()
        .css_classes(["file-editor", "text-editor"])
        .orientation(Orientation::Vertical)
        .hexpand(true)
        // .width_request(MAX_WIDTH)
        // .halign(Align::Center)
        .build();
    view.append(&create_header(db, id, title, keywords, on_rename));

    let grid = ColumnView::builder()
        .enable_rubberband(true)
        .build();

    let list_model = ListStore::new(BoxedAnyObject::static_type());
    for row in content.take_rows() {
        list_model.append(&BoxedAnyObject::new(row.clone()));
    }
    let model = MultiSelection::new(Some(list_model.clone()));

    grid.set_model(Some(&model));

    let state = TableEditingState {
        columns: Rc::new(RefCell::new(content.columns)),
        model: Rc::new(RefCell::new(model.upcast())),
    };

    let column_c = state.columns.borrow_mut().len();
    for (i, column) in state.columns.borrow_mut().iter().enumerate() {
        let factory = SignalListItemFactory::new();
        factory.connect_setup(clone!(@strong list_model => move |_, item| {
            let item = item
                .downcast_ref::<ListItem>().unwrap();
            let widget = Text::builder()
                .editable(true)
                .css_name("cell")
                .build();
            let controller = EventControllerKey::new();
            controller.connect_key_pressed(clone!(@strong list_model, @strong item => move |_, key, _, _| {
                match key {
                    Key::Return => {
                        list_model.insert(item.position() + 1, &BoxedAnyObject::new(Row::create_empty(column_c)));
                        Inhibit(true)
                    },
                    _ => Inhibit(false),
                }
            }));
            widget.add_controller(controller);
            item.set_child(Some(&widget));
        }));
        factory.connect_bind(clone!(@strong list_model, @strong state, @strong column => move |_, item| {
            let item = item
                .downcast_ref::<ListItem>().unwrap();
            let text = item.child()
                .and_downcast::<Text>().unwrap();
            let row = item.item()
                .and_downcast::<BoxedAnyObject>()
                .unwrap()
                .borrow_mut::<Row>().clone();
            let cell = row.get_cell(&state.columns.borrow_mut(), column.id).unwrap();
            text.set_text(match &cell.content {
                Some(c) => c,
                None => "",
            });
            let position = item.position();
            let handler_id = text.connect_text_notify(clone!(@strong list_model => move |text| {
                let row = list_model.item(position)
                    .and_downcast::<BoxedAnyObject>().unwrap();
                let mut row = row.borrow_mut::<Row>();
                row.cells[i].content = Some(text.text().to_string());
            }));
            unsafe {
                text.set_data("text-notify-signal", handler_id);
            }
        }));
        factory.connect_unbind(|_, item| {
            let item = item
                .downcast_ref::<ListItem>().unwrap();
            let text = item.child()
                .and_downcast::<Text>().unwrap();
            unsafe {
                let handler_id = text.steal_data::<SignalHandlerId>("text-notify-signal").unwrap();
                text.disconnect(handler_id);
            }
        });
        let column_view = ColumnViewColumn::builder()
            .title(column.name.as_str())
            .factory(&factory)
            .build();
        grid.append_column(&column_view);
    }

    view.append(&grid);

    (view.upcast(), Box::new(state))
}