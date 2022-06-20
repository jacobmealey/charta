
use gtk::subclass::prelude::*;
use gtk::{glib};
use std::cell::Cell;

#[derive(Default)]
pub struct NoteView {
    name: Cell<u32>
}

#[glib::object_subclass]
impl ObjectSubclass for NoteView {
    const NAME: &'static str = "NoteView";
    type Type = super::NoteView;
    type ParentType = gtk::TextView;

}

impl ObjectImpl for NoteView {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        println!("{}", &self.name.get());
    }
}
impl TextViewImpl for NoteView {}
impl WidgetImpl for NoteView {}
