mod note_view;

use note_view::NoteViewObject;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ScrolledWindow, StackSidebar, Grid, Stack};

fn main() {
    println!("Hello, world!");
    let app = Application::builder()
        .application_id("xyz.jacobmealey.Notes")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window: ApplicationWindow = ApplicationWindow::builder()
    .application(app)
    .build();

    let grid: Grid = Grid::new();

    let stack: Stack = Stack::new();
    stack.set_hexpand(true);
    stack.set_vexpand(true);

    grid.attach(&stack, 1, 0, 1, 1);

    let sidebar: StackSidebar = StackSidebar::new();
    sidebar.set_stack(&stack);

    grid.attach(&sidebar, 0, 0, 1, 1);

    for i in 1..4 {
        let title = format!("Page {}", i);
        //let label: Label = Label::builder()
        //    .label(&title)
        //    .build();
        
        let scroll: ScrolledWindow = ScrolledWindow::new();
        let noteview: NoteViewObject = NoteViewObject::new();
        noteview.setup();
        scroll.set_child(Some(&noteview));

        let name = format!("label{}", i);
        stack.add_titled(&scroll, Some(&name), &title);
    }

    window.set_default_width(650);
    window.set_default_height(420);
    window.set_title(Some("Notes"));

    
    window.set_application(Some(app));

    //// this is going to do something with a time for autosaving
    //text.buffer().connect_changed(move |buff| {
        //let begin_itter = buff.start_iter(); // << why was that so hard ??
        //let end_itter = buff.end_iter();
        //println!("{}", buff.text(&begin_itter, &end_itter, true));
    //});

    // scroll.set_child(Some(&text));
    // stackpage.set_child(Some(&scroll))
    // stack.set_child(Some(&stackpage));
    window.set_child(Some(&grid));
    window.present();
}
