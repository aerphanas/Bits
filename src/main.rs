mod window;

use gtk::{
    gio::resources_register_include,
    glib,
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::GtkWindowExt,
    Application,
};
use window::Window;

const APP_ID: &str = "io.github.aerphanas";

fn main() -> glib::ExitCode {
    resources_register_include!("src_gresource.resources").expect("Failed to register resources");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
