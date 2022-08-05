use crate::note_view::NoteViewData;
use std::sync::{Arc, Mutex};
use gtk::prelude::TextViewExt;
use gtk::prelude::TextBufferExt;
use gtk::subclass::prelude::*;
use gtk::{glib};



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
        self.parent_constructed(obj);
    }

}

impl TextViewImpl for NoteViewObject {}
impl WidgetImpl for NoteViewObject {}
unsafe impl Send for NoteViewObject {}
unsafe impl Sync for NoteViewObject {}
