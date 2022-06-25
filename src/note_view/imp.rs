use gtk::subclass::prelude::*;
use gtk::{glib};
use std::cell::RefCell;
use std::time;
use std::thread;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::note_view::NoteViewData;
use gtk::prelude::TextViewExt;
use gtk::prelude::TextBufferExt;


#[derive(Default)]
pub struct NoteViewObject {
    pub vals: Arc<Mutex<NoteViewData>>
}

#[glib::object_subclass]
impl ObjectSubclass for NoteViewObject {
    const NAME: &'static str = "NoteView";
    type Type = super::NoteViewObject;
    type ParentType = gtk::TextView;

}

impl ObjectImpl for NoteViewObject {
    fn constructed(&self, obj: &Self::Type) {
        let own_vals = Arc::clone(&self.vals);
        obj.buffer().connect_changed( move |_arg1| {
            let mut this = own_vals.lock().unwrap();
            let timer = (*this).timer;
            (*this).timer = 0;
            println!("Key pressed -- resetting timer");

        });


        let vv = Arc::clone(&self.vals);
        thread::spawn(move || {
            loop {
                let mut this = vv.lock().unwrap();
                (*this).timer += 1;
                if (*this).timer == 5 {
                    println!("5 seconds elapsed Saving...");
                }
                drop(this);
                thread::sleep(time::Duration::from_millis(1000));
                println!("incrementing timer");
            }
        });

        self.parent_constructed(obj);
    }
}
impl TextViewImpl for NoteViewObject {}
impl WidgetImpl for NoteViewObject {}
unsafe impl Send for NoteViewObject {}
unsafe impl Sync for NoteViewObject {}
