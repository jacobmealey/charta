mod note_view;

use std::fs;
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
          CssProvider,
          StyleContext,

    };
use gtk::glib;
use gtk::gio::SimpleAction;
use gtk::gio::SimpleActionGroup;
use gtk::gdk::Display;

fn main() {
    println!("Notes");
    println!("Author: Jacob Mealey");
    println!("Website: jacobmealey.xyz");


    let app = Application::builder()
        .application_id("xyz.jacobmealey.Notes")
        .build();


    app.connect_activate(build_ui);
    app.connect_startup(|_| load_css());
    app.set_accels_for_action("win.quit", &["<Ctrl>Q"]);
    app.set_accels_for_action("note.b", &["<Ctrl>B"]);
    app.set_accels_for_action("note.i", &["<Ctrl>I"]);
    app.run();
}

// taken from https://gtk-rs.org/gtk4-rs/stable/latest/book/css.html
fn load_css() {
    let provider_charta = CssProvider::new();
    let charta_css_path = "/usr/share/charta/style.css";
    provider_charta.load_from_path(std::path::Path::new(&charta_css_path));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider_charta,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
    
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
    let note_title = EditableLabel::new("");
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
    sidebar.set_stack(&stack_rc);

	let stack_clone = stack_rc.clone();
	note_title.connect_changed(move |arg1| {
		let new_name = &arg1.text().to_string();
		// if for any reason the name is empty, bail out because
		// the querry will fail 
		if new_name.is_empty() {
			return;
		}
        // update name in stackviewobject
        let top_child = stack_clone
                .visible_child().unwrap()
                .downcast::<ScrolledWindow>().unwrap()
                .child().unwrap();
        let current_note = top_child.downcast::<NoteViewObject>().unwrap();
        current_note.set_name(new_name);
        // update name in stack sidebar
		stack_clone.page(&stack_clone.visible_child().unwrap()).set_title(new_name);
	});

	// Update note_title to represent what we have clicked on :)
	stack_rc.connect_visible_child_notify(move |arg1| {
		let stackname = &arg1.visible_child_name().unwrap().to_string();
        let top_child = arg1 
                .visible_child().unwrap()
                .downcast::<ScrolledWindow>().unwrap()
                .child().unwrap();
        let current_note = top_child.downcast::<NoteViewObject>().unwrap();
        println!("Current Note: {}", current_note.get_name());
        note_title.set_text(&current_note.get_name());
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
            // if the user has highlighted some section of text, get the bounds directly, if the
            // user has simply pressed a keybinding with no highlighint insert zero-width spaces
            // and mark them as the start and end locations.
            let (bound_start, bound_end) = match current_note.buffer().selection_bounds() {
                Some(bounds) => bounds,
                None => {
                    current_note.buffer().insert_at_cursor("\u{FEFF}\u{FEFF}\u{FEFF}");
                    let iter_a = current_note.buffer().
                        iter_at_offset(current_note
                                       .buffer()
                                       .cursor_position() - 3);

                    let iter_b = current_note.buffer().
                        iter_at_offset(current_note
                                       .buffer()
                                       .cursor_position() - 1);

                    let mut iter_c = current_note.buffer()
                        .iter_at_offset(current_note
                                        .buffer()
                                        .cursor_position());

                    iter_c.backward_chars(2);
                    current_note.buffer().place_cursor(&iter_c);
                    (iter_a, iter_b)
                }
            };
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

            
            println!("{} Action triggered", action);
        });
        action_group.add_action(&act);
    }

    window.insert_action_group("note", Some(&action_group));
    for act in action_group.list_actions() {
        println!("{}", act);
    }

    let new_note = move |value: Option<(&str, json::JsonValue)>| -> NoteViewObject {
        // get references to existing state
        let mut update_count = note_count.borrow_mut();
        let rc = stack_rc.clone();

        println!("Creating new note {}", update_count);
        let name_raw = format!("New Note {}", update_count);
        let filename_raw = format!("/usr/share/charta/json/new_note{}.txt", update_count);

        let (filename, contents) = value.unwrap_or((&filename_raw, 
                                    json::object!(name: name_raw, contents: "")));
        let name = contents["name"].to_string();
        // create a new noteview instance and bing 
        let scroll: ScrolledWindow = ScrolledWindow::new();
        let mut noteview: NoteViewObject = NoteViewObject::new();
        noteview.setup();
        noteview.set_name(&name);
        noteview.set_file(&filename.to_string());
        noteview.set_id(*update_count);
        noteview.load(contents["contents"].to_string());
        noteview.set_halign(gtk::Align::Center);

        *update_count += 1;

        scroll.set_child(Some(&noteview));
        rc.add_titled(&scroll, Some(&format!("note{}", &noteview.get_id())), &name);
        noteview 
    };

    let notes: Rc<RefCell<Vec<NoteViewObject>>> = Rc::new(RefCell::new(Vec::new()));

    let notes_2 = Rc::clone(&notes);
    // load entries from the directory we are reading from
    for entry in fs::read_dir("/usr/share/charta/json/").unwrap() {
        let file = entry.unwrap();
        let filename = file.path().into_os_string().into_string().unwrap();
        println!("{:?}", file.path());

        let note = json::parse(&fs::read_to_string(&filename).unwrap()).unwrap();
        println!("Name: {}, contents: {}", note["name"], note["contents"]);
        let mut notes = notes_2.borrow_mut();
        notes.push(new_note(Some((&filename, note))));
    }

    let notes_3 = Rc::clone(&notes);
    // create a new note when user clicks the new_note_button
    new_note_button.set_label("New");
    new_note_button.connect_clicked(move |_| {
        let mut notes = notes_3.borrow_mut();
        notes.push(new_note(None));
    });


    // save files on closing -- not sure how we can make this happen for every type of 
    // closing so I am not just going to do it for the close button press. 
    let notes_4 = Rc::clone(&notes);
    window.connect_close_request(move |_| {
        println!("Closing..."); 
        let notes = notes_4.borrow_mut();
        for note in &*notes {
            note.save()
        }

        gtk::Inhibit(false)
    });

    // add button to header
    header.pack_start(&new_note_button);

    // Set parameters for window settings
    window.set_titlebar(Some(&header));
    window.set_default_width(650);
    window.set_default_height(420);
    window.set_title(Some("Charta"));
    window.set_application(Some(app));
    window.set_child(Some(&grid));
    window.show();
}

