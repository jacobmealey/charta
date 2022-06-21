mod note_view;

use note_view::NoteViewObject;
use gtk::prelude::*;
use std::rc::Rc;
use gtk::{Application, ApplicationWindow, ScrolledWindow, 
          StackSidebar, Grid, Stack, HeaderBar, Button};

fn main() {
    println!("Notes");
    println!("Author: Jacob Mealey");
    println!("Website: jacobmealey.xyz");
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

    let header = HeaderBar::new();
    window.set_titlebar(Some(&header));
    let grid: Grid = Grid::new();


    let stack_rc = Rc::new(Stack::new());
    stack_rc.set_hexpand(true);
    stack_rc.set_vexpand(true);

    // this is a very cursed line tbh. we are dereferencing the rc 
    // and the borrowing the stack
    grid.attach(&*stack_rc, 1, 0, 1, 1);

    let sidebar: StackSidebar = StackSidebar::new();
    sidebar.set_stack(&*stack_rc);

    grid.attach(&sidebar, 0, 0, 1, 1);

    let button = Button::new();
    button.set_label("New");

    button.connect_clicked(move |_| {
        let rc = stack_rc.clone();
        println!("Creating new note");
        let title = format!("New Note");
        
        let scroll: ScrolledWindow = ScrolledWindow::new();
        let noteview: NoteViewObject = NoteViewObject::new();
        noteview.setup();
        scroll.set_child(Some(&noteview));

        let name = format!("new_note");
        rc.add_titled(&scroll, Some(&name), &title);
    });

    header.pack_start(&button);

    window.set_default_width(650);
    window.set_default_height(420);
    window.set_title(Some("Notes"));
    
    window.set_application(Some(app));

    // scroll.set_child(Some(&text));
    // stackpage.set_child(Some(&scroll))
    // stack.set_child(Some(&stackpage));
    window.set_child(Some(&grid));
    window.present();
}
