use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
use gtk::glib::GString;
use std::option::Option;
const APP_ID: &str = "org.gtk_rs.HelloWorld3";
fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}
fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        let my_string: Option<GString> = Some("Press me!".into());
        if button.label() == my_string {
            button.set_label("Hello World!");
        }
        else {
            button.set_label("Press me!")
        }
        // Set the label to "Hello World!" after the button has been clicked on
        
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}