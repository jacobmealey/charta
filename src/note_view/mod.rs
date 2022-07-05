pub mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use std::sync::Arc;
use gtk::WrapMode;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use sqlite;

glib::wrapper! {
    pub struct NoteViewObject(ObjectSubclass<imp::NoteViewObject>)
    @extends gtk::TextView, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, 
    gtk::ConstraintTarget, gtk::Orientable;
}

impl NoteViewObject {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `NoteView`.")
    }

    pub fn setup(&self) {
        self.set_editable(true);
        self.set_wrap_mode(WrapMode::Word);
        self.set_left_margin(35);
        self.set_right_margin(35);
        self.set_top_margin(24);
        self.set_bottom_margin(24);
    }

    pub fn set_name(&self, name: String) {
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().name = name;
    }
 
    pub fn set_file(&self, filename: String) {
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().filename = filename;
    }
    pub fn set_id(&self, id: u32) {
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().note_id = id;
    }  



}

#[derive(Default)]
pub struct NoteViewData {
    pub name: String,
    pub timer: u32,
    pub buffer: String,
    pub filename: String,
    pub note_id: u32,
}
