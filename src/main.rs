mod note_view;

use note_view::NoteViewObject;
use gtk::prelude::*;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::fs;
use gtk::{Application, 
          ApplicationWindow, 
          ScrolledWindow, 
          StackSidebar, 
          Grid, 
          Stack, 
          HeaderBar, 
          Button, 
          TextBuffer,
          EditableLabel,
          Separator
    };
use sqlite;
use sqlite::State;
use std::thread;
use std::time;

use crate::note_view::NoteViewData;


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

    let connection = Arc::new(sqlite::open("notes_db.sql").unwrap());

    let header = HeaderBar::new();
    let grid: Grid = Grid::new();
    let active_note_grid = Grid::new();
    let note_count = Rc::new(RefCell::new(1));
    let stack_rc = Rc::new(Stack::new());
    let sidebar: StackSidebar = StackSidebar::new();
    let new_note_button = Button::new();
    let note_title = EditableLabel::new("damn note");
    let note_title_sep = Separator::new(gtk::Orientation::Horizontal);

    // set up stack and stacksidebar for organizing the screen
    stack_rc.set_hexpand(true);
    stack_rc.set_vexpand(true);

    // this is a very cursed line tbh. we are dereferencing the rc 
    // and the borrowing the stack
    active_note_grid.attach(&(*stack_rc), 0, 2, 1, 1);
    active_note_grid.attach(&note_title_sep, 0, 1, 1, 1);
    active_note_grid.attach(&note_title, 0, 0, 1, 1);
    grid.attach(&sidebar, 0, 0, 1, 1);
    grid.attach(&active_note_grid, 1, 0, 1, 1);
    sidebar.set_stack(&(*stack_rc));

    // update titles in DB when changing name
    let note_conn = connection.clone();
    let stack_clone = stack_rc.clone();
    note_title.connect_changed(move |arg1| {
        let new_name = &arg1.text().to_string();
        if new_name.is_empty() {
            return;
        }

        let stackname = stack_clone.visible_child_name().unwrap().to_string();
        let vec: Vec<&str> = stackname.split("e").collect();
        let note_id = vec.get(1).unwrap();

        let querry = format!("UPDATE notes SET name=\"{}\" WHERE note_id={}", new_name, note_id);
        println!("querry: {}", querry);

        note_conn.execute(querry).unwrap();
        stack_clone.page(&stack_clone.visible_child().unwrap()).set_title(new_name);
    });

    // Update note_title to represent what we have clicked on :)
    let stack_conn = connection.clone();
    stack_rc.connect_visible_child_notify(move |arg1| {
        let stackname = &arg1.visible_child_name().unwrap().to_string();

        let vec: Vec<&str> = stackname.split("e").collect();
        let note_id = vec.get(1).unwrap();

        let querry = format!("SELECT name FROM notes WHERE note_id={}", note_id);
        let mut statement = stack_conn.prepare(querry).unwrap();
        if let State::Row = statement.next().unwrap() {
            note_title.set_text(&statement.read::<String>(0).unwrap());
        }
    });


    // load exisiting notes from sql
    connection
        .iterate("SELECT * FROM notes", |pairs| {
            let rc = stack_rc.clone();

            let (_, note_id) = pairs[0];
            let (_, filename) = pairs[2];
            let (_, name) = pairs[1];
            println!("{}, {}", filename.unwrap(), name.unwrap());

            let scroll: ScrolledWindow = ScrolledWindow::new();
            let noteview: NoteViewObject = NoteViewObject::new();

            noteview.setup();
            noteview.set_name(&name.unwrap().to_string());
            noteview.set_file(&filename.unwrap().to_string());
            noteview.set_id(note_id.unwrap().parse::<u32>().unwrap()); 
            let read_in = fs::read_to_string(noteview.get_file()).expect("Unable to read file");


            noteview.set_buffer(Some(&TextBuffer::builder()
                                     .text(&read_in)
                                     .build()));

            new_note_bindings(&noteview);
            scroll.set_child(Some(&noteview));
            noteview.buffer().connect_changed( move |arg1| {
                noteview.set_timer(0);
                noteview.set_buffstring(&arg1.slice(&arg1.start_iter(), 
                                                    &arg1.end_iter(), 
                                                    false).to_string());
                println!("Key pressed -- resetting timer");
            });

            rc.add_titled(&scroll, 
                          Some(&format!("note{}", &note_id.unwrap())[..]),
                          &name.unwrap()[..]);

            true
        }).unwrap();


    // create a new note when user clicks the new_note_button
    new_note_button.set_label("New");
    new_note_button.connect_clicked(move |_| {
        // get references to existing state
        let mut update_count = note_count.borrow_mut();
        let rc = stack_rc.clone();

        *update_count += 1;
        println!("Creating new note {}", update_count);
        let name= format!("New Note {}", update_count);
        let filename = format!("new_note{}.txt", update_count);

        // create a new noteview instance and bing 
        let scroll: ScrolledWindow = ScrolledWindow::new();
        let noteview: NoteViewObject = NoteViewObject::new();
        noteview.setup();
        noteview.set_name(&name);
        noteview.set_file(&filename);
        noteview.set_id(*update_count + 1);

        // push new note into database
        let querry = format!("INSERT INTO notes VALUES ({}, \"{}\", \"{}\")", 
                             noteview.get_id() + 1, 
                             noteview.get_name(), 
                             noteview.get_file());
        println!("{}", querry);
        connection.execute(querry).unwrap();

        scroll.set_child(Some(&noteview));
        new_note_bindings(&noteview);
        noteview.buffer().connect_changed( move |arg1| {
            noteview.set_timer(0);
            noteview.set_buffstring(&arg1.slice(&arg1.start_iter(), 
                                                &arg1.end_iter(), 
                                                false).to_string());
            println!("Key pressed -- resetting timer");
        });
        rc.add_titled(&scroll, Some(&name), &name);
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

fn save(notes: &NoteViewData, conn: &sqlite::Connection) {
    let qurrey = format!("SELECT file FROM notes WHERE note_id={}", notes.note_id);
    println!("{}", qurrey);
    let mut statement = conn
        .prepare(qurrey)
        .unwrap();

    while let State::Row  = statement.next().unwrap() {
        let filename = statement.read::<String>(0).unwrap();
        println!("saving to: {}", filename);
        fs::write(filename, &notes.buffer).expect("Unable to write file");
    }
}


fn new_note_bindings(noteview: &NoteViewObject) {
    let vals_clone_t = noteview.get_vals();
    thread::spawn(move || {
        loop {
            let mut vals = vals_clone_t.lock().unwrap();
            (*vals).timer += 1;
            let conn = sqlite::open("notes_db.sql").unwrap();
            if (*vals).timer == 5 {
                save(&(*vals), &conn);
            }
            drop(vals);
            thread::sleep(time::Duration::from_millis(500));
        }
    });
}

