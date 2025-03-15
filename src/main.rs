mod window;

use gtk::prelude::*;
use gtk::{gio, glib};
use window::Window;

const APP_ID: &str = "org.gtk_rs.pelouseApp";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("content.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}
fn build_ui(app: &adw::Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}