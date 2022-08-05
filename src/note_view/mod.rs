pub mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use std::sync::Arc;
use gtk::WrapMode;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use sqlite;

use std::sync::Mutex;

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

    pub fn set_name(&self, name: &String) {
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().name = name.to_string();
    }
 
    pub fn set_file(&self, filename: &String) {
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().filename = filename.to_string();
    }
    pub fn set_id(&self, id: u32) {
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().note_id = id;
    }  

    pub fn set_timer(&self, time: u32) {
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().timer = time;
    }  

    pub fn set_buffstring(&self, buffstring: &String) {
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().buffer = buffstring.to_string();
    }  

    pub fn get_file(&self) -> String {
        let vals = Arc::clone(&self.imp().vals);
        let filename = &vals.lock().unwrap().filename; 
        filename.to_string()
    }

    pub fn get_name(&self) -> String {
        let vals = Arc::clone(&self.imp().vals);
        let name = &vals.lock().unwrap().name; 
        name.to_string()
    }

    pub fn get_id(&self) -> u32{
        let vals = Arc::clone(&self.imp().vals);
        let id = vals.lock().unwrap().note_id; id
    }

    pub fn get_vals(&self) -> Arc<Mutex<NoteViewData>> {
        Arc::clone(&self.imp().vals)
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
