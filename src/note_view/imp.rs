use crate::note_view::NoteViewData;

use std::time;
use std::thread;
use std::sync::{Arc, Mutex};
use std::fs;

use gtk::prelude::TextViewExt;
use gtk::prelude::TextBufferExt;
use gtk::subclass::prelude::*;
use gtk::{glib};

use sqlite::State;


#[derive(Default)]
pub struct NoteViewObject {
    pub vals: Arc<Mutex<NoteViewData>>
}

fn save(notes: &NoteViewData, conn: &sqlite::Connection) {
    let qurrey = format!("SELECT file FROM notes WHERE note_id={}", notes.note_id);
    let mut statement = conn
        .prepare(qurrey)
        .unwrap();
   
    while let State::Row  = statement.next().unwrap() {
        let filename = statement.read::<String>(0).unwrap();
        println!("saving to: {}", filename);
        fs::write(filename, &notes.buffer).expect("Unable to write file");
    }


}

#[glib::object_subclass]
impl ObjectSubclass for NoteViewObject {
    const NAME: &'static str = "NoteView";
    type Type = super::NoteViewObject;
    type ParentType = gtk::TextView;

}

impl ObjectImpl for NoteViewObject {
    fn constructed(&self, obj: &Self::Type) {

        // Signal handler for the text buffer
        let vals_clone_buff = Arc::clone(&self.vals);
        obj.buffer().connect_changed( move |arg1| {
            let mut this = vals_clone_buff.lock().unwrap();
            (*this).timer = 0;
            (*this).buffer = arg1.slice(&arg1.start_iter(), &arg1.end_iter(), false).to_string();
            println!("Key pressed -- resetting timer");
        });


        // Thread for updating NoteViewData.timer and 
        // saving the contents of the buffer 
        let vals_clone_t = Arc::clone(&self.vals);
        thread::spawn(move || {
            loop {
                let mut this = vals_clone_t.lock().unwrap();
                (*this).timer += 1;

                if (*this).timer == 5 {
                    let connection = sqlite::open("notes_db.sql").unwrap();
                    println!("5 seconds elapsed Saving...");
                    save(&(*this), &connection);
                }
                // Drop lock before delay
                drop(this); 
                thread::sleep(time::Duration::from_millis(1000));
            }
        });

        self.parent_constructed(obj);
    }

}

impl TextViewImpl for NoteViewObject {}
impl WidgetImpl for NoteViewObject {}
unsafe impl Send for NoteViewObject {}
unsafe impl Sync for NoteViewObject {}
