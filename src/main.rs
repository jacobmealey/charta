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
          Separator,
    };
use sqlite;
use sqlite::State;
use std::thread;
use std::time;
use gtk::glib;
use gtk::gio::SimpleAction;
use gtk::gio::SimpleActionGroup;

use crate::note_view::NoteViewData;


fn main() {
    println!("Notes");
    println!("Author: Jacob Mealey");
    println!("Website: jacobmealey.xyz");
    let app = Application::builder()
        .application_id("xyz.jacobmealey.Notes")
        .build();

    app.connect_activate(build_ui);
    app.set_accels_for_action("win.quit", &["<Ctrl>Q"]);
    app.set_accels_for_action("note.bold", &["<Ctrl>B"]);
    app.set_accels_for_action("note.italics", &["<Ctrl>I"]);
    app.run();
}

fn build_ui(app: &Application) {
    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .build();

    let connection = Arc::new(sqlite::open("/usr/share/goats/notes_db.sql").unwrap());

    let header = HeaderBar::new();
    let grid: Grid = Grid::new();
    let active_note_grid = Grid::new();
    let note_count = Rc::new(RefCell::new(1));
    let stack_rc = Rc::new(Stack::new());
    let sidebar: StackSidebar = StackSidebar::new();
    let new_note_button = Button::new();
    let note_title = EditableLabel::new("damn note");
    let note_title_sep = Separator::new(gtk::Orientation::Horizontal);

    let action_close = SimpleAction::new("quit", None);
    action_close.connect_activate(glib::clone!(@weak window => move |_, _| {
        println!("Action triggered");
        window.close();
    }));
    window.add_action(&action_close);

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

    let action_bold = SimpleAction::new("bold", None);
    let stack_bold = stack_rc.clone();
    action_bold.connect_activate(move |_, _| {
        let top_child = stack_bold.visible_child().unwrap().downcast::<ScrolledWindow>().unwrap().child().unwrap();
        let current_note = top_child.downcast::<NoteViewObject>().unwrap();
        let (bound_start, bound_end) = current_note.buffer().selection_bounds().unwrap();
        current_note.buffer().apply_tag_by_name("bold", &bound_start, &bound_end);
        println!("Bold Action triggered");
    });

    let action_italics = SimpleAction::new("italics", None);
    let stack_bold = stack_rc.clone();
    action_italics.connect_activate(move |_, _| {
        let top_child = stack_bold.visible_child().unwrap().downcast::<ScrolledWindow>().unwrap().child().unwrap();
        let current_note = top_child.downcast::<NoteViewObject>().unwrap();
        let (bound_start, bound_end) = current_note.buffer().selection_bounds().unwrap();
        current_note.buffer().apply_tag_by_name("italics", &bound_start, &bound_end);
        //println!("{} {}", bound_start, bound_end);
        println!("Italic Action triggered");
    });

    let actions = SimpleActionGroup::new();
    actions.add_action(&action_bold);
    actions.add_action(&action_italics);
    window.insert_action_group("note", Some(&actions));
    for act in actions.list_actions() {
        println!("{}", act);
    }
    // update titles in DB when changing name
    let note_conn = connection.clone();
    let stack_clone = stack_rc.clone();
    note_title.connect_changed(move |arg1| {
        let new_name = &arg1.text().to_string();
        // if for any reason the name is empty, bail out because
        // the querry will fail 
        if new_name.is_empty() {
            return;
        }

        let stackname = stack_clone.visible_child_name().unwrap().to_string();
        let vec: Vec<&str> = stackname.split("e").collect();
        let note_id = vec.get(1).unwrap();

        let querry = format!("UPDATE notes SET name=\"{}\" WHERE note_id={}", new_name, note_id);

        note_conn.execute(querry).unwrap();
        stack_clone.page(&stack_clone.visible_child().unwrap()).set_title(new_name);
    });

    // Update note_title to represent what we have clicked on :)
    let stack_conn = connection.clone();
    stack_rc.connect_visible_child_notify(move |arg1| {
        let stackname = &arg1.visible_child_name().unwrap().to_string();
        println!("stackname: {}", stackname);

        let vec: Vec<&str> = stackname.split("e").collect();
        let note_id = vec.get(1).unwrap();

        let querry = format!("SELECT name FROM notes WHERE note_id={}", note_id);
        println!("{}", querry);
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

            noteview.set_name(&name.unwrap().to_string());
            noteview.set_file(&filename.unwrap().to_string());
            noteview.set_id(note_id.unwrap().parse::<u32>().unwrap()); 
            let read_in = fs::read_to_string("/usr/share/goats/".to_owned() + &noteview.get_file()).expect("Unable to read file");


            noteview.set_buffer(Some(&TextBuffer::builder()
                                     .text(&read_in)
                                     .build()));

            // we call setup /after/ getting everything in place
            noteview.setup();

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
            let mut update_count = note_count.borrow_mut();
            *update_count += 1;

            true
        }).unwrap();


    let new_note = move || {
        // get references to existing state
        let mut update_count = note_count.borrow_mut();
        let rc = stack_rc.clone();

        println!("Creating new note {}", update_count);
        let name= format!("New Note {}", update_count);
        let filename = format!("new_note{}.txt", update_count);

        // create a new noteview instance and bing 
        let scroll: ScrolledWindow = ScrolledWindow::new();
        let noteview: NoteViewObject = NoteViewObject::new();
        noteview.setup();
        noteview.set_name(&name);
        noteview.set_file(&filename);
        noteview.set_id(*update_count);

        *update_count += 1;
        // push new note into database
        let querry = format!("INSERT INTO notes VALUES ({}, \"{}\", \"{}\")", 
                             noteview.get_id(), 
                             noteview.get_name(), 
                             noteview.get_file());
        println!("{}", querry);
        connection.execute(querry).unwrap();

        scroll.set_child(Some(&noteview));
        new_note_bindings(&noteview);
        rc.add_titled(&scroll, Some(&format!("note{}", &noteview.get_id())), &name);
        noteview.buffer().connect_changed( move |arg1| {
            noteview.set_timer(0);
            noteview.set_buffstring(&arg1.slice(&arg1.start_iter(), 
                                                &arg1.end_iter(), 
                                                false).to_string());
            println!("Key pressed -- resetting timer");
        });
    };

    // create a new note when user clicks the new_note_button
    new_note_button.set_label("New");
    new_note_button.connect_clicked(move |_| {new_note()});



    // add button to header
    header.pack_start(&new_note_button);

    // Shortcuts :)
    // new note short cut
    //let new_note_keybind = ShortcutTrigger::parse_string("<Control>n");
    //let new_note_action  = ShortcutAction::parse_string("activate");
    //let new_note_shortcut = Shortcut::new(Some(&new_note_keybind), Some(&new_note_action));

    //let shortcut_controller = ShortcutController::new();
    //shortcut_controller.add_shortcut(&new_note_shortcut);

    // Set parameters for window settings
    window.set_titlebar(Some(&header));
    //window.add_controller(&shortcut_controller);
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
        let filename = "/usr/share/goats/".to_owned() + &statement.read::<String>(0).unwrap();
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
            let conn = sqlite::open("/usr/share/goats/notes_db.sql").unwrap();
            if (*vals).timer == 5 {
                save(&(*vals), &conn);
            }
            drop(vals);
            thread::sleep(time::Duration::from_millis(100));
        }
    });
}

