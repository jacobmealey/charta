mod note_view;

use note_view::NoteViewObject;
use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
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
    let grid: Grid = Grid::new();
    let note_count = Rc::new(RefCell::new(1));
    let stack_rc = Rc::new(Stack::new());
    let sidebar: StackSidebar = StackSidebar::new();
    let new_note_button = Button::new();

    // set up stack and stacksidebar for organizing the screen
    stack_rc.set_hexpand(true);
    stack_rc.set_vexpand(true);

    // this is a very cursed line tbh. we are dereferencing the rc 
    // and the borrowing the stack
    grid.attach(&(*stack_rc), 1, 0, 1, 1);
    grid.attach(&sidebar, 0, 0, 1, 1);
    sidebar.set_stack(&(*stack_rc));

    // create a new note when user clicks the new_note_button
    new_note_button.set_label("New");
    new_note_button.connect_clicked(move |_| {
        // get references to existing state
        let mut update_count = note_count.borrow_mut();
        let rc = stack_rc.clone();

        println!("Creating new note {}", update_count);
        let title = format!("New Note {}", update_count);
        let name = format!("new_note{}", update_count);
        *update_count += 1;

        // create a new noteview instance and bing 
        let scroll: ScrolledWindow = ScrolledWindow::new();
        let noteview: NoteViewObject = NoteViewObject::new();
        noteview.setup();

        scroll.set_child(Some(&noteview));
        rc.add_titled(&scroll, Some(&name), &title);
    });

    // add button to header
    header.pack_start(&new_note_button);

    // Set parameters for window settings
    window.set_titlebar(Some(&header));
    window.set_default_width(650);
    window.set_default_height(420);
    window.set_title(Some("Notes"));
    window.set_application(Some(app));
    window.set_child(Some(&grid));
    window.present();
}
