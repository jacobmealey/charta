use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, TextView};

fn main() {
    println!("Hello, world!");
    let app = Application::builder()
        .application_id("xyz.jacobmealey.Notes")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let builder = gtk::Builder::from_string(include_str!("window.ui"));
    
    let window: ApplicationWindow = builder
        .object("window")
        .expect("Could not get object 'window' from builder");

    let text: TextView = builder
        .object("text")
        .expect("Could not get object 'button' from builder");

    text.set_editable(true);
    
    window.set_application(Some(app));

    text.buffer().connect_changed(move |buff| {
        let begin_itter = buff.start_iter();
        let end_itter = buff.end_iter();
        println!("{}", buff.text(&begin_itter, &end_itter, true));
    });

    window.set_child(Some(&text));
    window.present();
}
