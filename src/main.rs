use adw::{Application, AboutWindow};
use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR, VERSION};
use gettextrs::{bindtextdomain, bind_textdomain_codeset, textdomain};
use gtk4::{gio::{ActionEntry, Resource, resources_register}, prelude::{ApplicationExt, ApplicationExtManual, ActionMapExtManual}, traits::{GtkWindowExt, GtkApplicationExt}, CssProvider, gdk::Display, STYLE_PROVIDER_PRIORITY_APPLICATION, style_context_add_provider_for_display};

mod data;
mod db;
mod ui;
mod config;

const APP_ID: &str = "one.zagura.Papier";

fn main() -> glib::ExitCode {
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    if let Ok(resources) = Resource::load(PKGDATADIR.to_owned() + "/papier.gresource") {
        resources_register(&resources);
    } else {
        eprintln!("Could not load resources");
    }

    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_startup(|_| load_css());
    app.connect_activate(ui::build_ui);
    actions(&app);
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));
    style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn actions(app: &Application) {
    let about = ActionEntry::builder("about")
        .activate(move |app: &Application, _, _| show_about(app))
        .build();
    app.add_action_entries([about]);
    app.set_accels_for_action("window.close", &["<primary>q"]);
    app.set_accels_for_action("win.cmd", &["<primary>k", "<primary>slash"]);
}

fn show_about(app: &Application) {
    let window = app.active_window().unwrap();
    AboutWindow::builder()
        .transient_for(&window)
        .application_name("Papier")
        .application_icon(APP_ID)
        .developer_name("Zagura")
        .version(VERSION)
        .developers(vec!["Zagura"])
        .build()
        .present();
}