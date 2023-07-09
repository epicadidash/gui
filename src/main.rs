use gtk::{prelude::*, Orientation};
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
    let button = Button::builder()
        .label("Submit")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let input = gtk::Entry::builder()
        .placeholder_text("input")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&input);
    gtk_box.append(&button);

    button.connect_clicked( | _button| {
        println!("{}", input.text().as_str()); 
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("gtk-app")
        .child(&gtk_box)
        .default_height(720)
        .default_width(360)
        .build();

    window.present();
}