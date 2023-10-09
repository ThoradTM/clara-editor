use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, TextView, TextBuffer, PopoverMenuBar, Box};

const APP_ID: &str = "org.gtk_rs.clara_editor";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    // Run the application

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|button| {
        button.set_label("Hello World!");
    });

    let textbuffer = TextBuffer::builder()
    .build();

    let textwindow = TextView::builder()
    .buffer(&textbuffer)
    .build();

    let menubar = PopoverMenuBar::builder()
    .build();

    let gtk_box = Box::builder()
        .build()
        .child(button)
        .child(&textwindow)
        .child(&menubar);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Clara Editor")
        .child(&gtk_box)
        .default_width(1280)
        .default_height(720)
        .build();
    
    window.present();
}