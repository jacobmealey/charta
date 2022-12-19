mod note_view;

use note_view::NoteViewObject;
use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use gtk::{Application, 
          ApplicationWindow, 
          ScrolledWindow, 
          StackSidebar, 
          Grid, 
          Stack, 
          HeaderBar, 
          Button, 
          EditableLabel,
          Separator,
    };
use gtk::glib;
use gtk::gio::SimpleAction;
use gtk::gio::SimpleActionGroup;


fn main() {
    println!("Notes");
    println!("Author: Jacob Mealey");
    println!("Website: jacobmealey.xyz");
    let app = Application::builder()
        .application_id("xyz.jacobmealey.Notes")
        .build();

    app.connect_activate(build_ui);
    app.set_accels_for_action("win.quit", &["<Ctrl>Q"]);
    app.set_accels_for_action("note.b", &["<Ctrl>B"]);
    app.set_accels_for_action("note.i", &["<Ctrl>I"]);
    app.run();
}

fn build_ui(app: &Application) {
    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .build();


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

	let stack_clone = stack_rc.clone();
	note_title.connect_changed(move |arg1| {
		let new_name = &arg1.text().to_string();
		// if for any reason the name is empty, bail out because
		// the querry will fail 
		if new_name.is_empty() {
			return;
		}

		stack_clone.page(&stack_clone.visible_child().unwrap()).set_title(new_name);
	});

	// Update note_title to represent what we have clicked on :)
	stack_rc.connect_visible_child_notify(move |arg1| {
		let stackname = &arg1.visible_child_name().unwrap().to_string();
		println!("stackname: {}", stackname);
	});

    // ideally actions should be a global list somewhere (like in an XML file? fuck that.) 
    // so for now just try to keep the ducks in a row :). This code seems self explanatory
    // now. I will comment it tomorrow (?) -- Aug 26 2022 will I come back???
    let actions = vec!["b", "i"];
    let action_group = SimpleActionGroup::new();

    for action in actions {
        let stack_actions = stack_rc.clone();
        let act = SimpleAction::new(action, None);
        act.connect_activate(move |_, _| {
            let top_child = stack_actions
                .visible_child().unwrap()
                .downcast::<ScrolledWindow>().unwrap()
                .child().unwrap();
            let current_note = top_child.downcast::<NoteViewObject>().unwrap();
            let (bound_start, bound_end) = current_note.buffer().selection_bounds().unwrap();
            let mut is_action: bool = false;
            for tag in bound_start.tags() {
                if tag.name().unwrap() == action {
                    is_action = true;
                    break
                }
            }
            current_note.buffer().remove_all_tags(&bound_start, &bound_end);
            if is_action {
                return;
            }
            current_note.buffer().remove_all_tags(&bound_start, &bound_end);
            current_note.buffer().apply_tag_by_name(action, &bound_start, &bound_end);
            current_note.serialize();
            current_note.get_vals().lock().unwrap().timer = 0;

            
            println!("{} Action triggered", action);
        });
        action_group.add_action(&act);
    }

    window.insert_action_group("note", Some(&action_group));
    for act in action_group.list_actions() {
        println!("{}", act);
    }

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

        scroll.set_child(Some(&noteview));
        rc.add_titled(&scroll, Some(&format!("note{}", &noteview.get_id())), &name);
        noteview.buffer().connect_changed( move |arg1| {
            noteview.set_timer(0);
            noteview.set_buffstring(&arg1.slice(&arg1.start_iter(), 
                                                &arg1.end_iter(), 
                                                false).to_string());
            noteview.serialize();
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

