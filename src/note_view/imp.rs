
use gtk::subclass::prelude::*;
use gtk::{glib};

#[derive(Default)]
pub struct NoteView;

#[glib::object_subclass]
impl ObjectSubclass for NoteView {
    const NAME: &'static str = "NoteView";
    type Type = super::NoteView;
    type ParentType = gtk::TextView;

}

impl ObjectImpl for NoteView {}
impl TextViewImpl for NoteView {}
impl WidgetImpl for NoteView {}
