extern crate gtk;
use gtk::prelude::*;
use gtk::MessageDialog;
use gtk::Application;
use gtk::ButtonsType;
use gtk::MessageType;

fn main() {
    let app = Application::builder()
        .build();
    app.connect_activate(|application| {
        let dialog = MessageDialog::builder()
            .application(application)
            .message_type(MessageType::Info)
            .buttons(ButtonsType::Ok)
            .text("Hello world!")
            .secondary_text("This is an example dialog.")
            .build();
        dialog.connect_response(|dialog, _| dialog.destroy());
        dialog.present();
    });
    app.run();
}