use std::rc::Rc;
use std::sync::Mutex;

use crate::data::Category;
use crate::data::FileID;
use crate::db::DB;
use crate::db::Saveable;
use gtk4::Widget;

use self::table::display_table;
use self::text::display_text;

mod components;
mod table;
mod text;

pub fn display_file<F>(db: &Rc<Mutex<DB>>, id: FileID, on_rename: F) -> (Widget, Box<dyn Saveable>)
where F: Fn(String) + 'static {
    let borrowed_db = db.lock().unwrap();
    let file = borrowed_db.get_file(id).unwrap();
    match file.category {
        Category::Text => display_text(db, id, &file.title, file.keywords.clone(), borrowed_db.get_text_content(id), on_rename),
        Category::Table => display_table(db, id, &file.title, file.keywords.clone(), borrowed_db.get_table_content(id), on_rename),
    }
}