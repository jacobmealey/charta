

use gtk::subclass::prelude::*;
use gtk::{glib};
use std::rc::Rc;

use crate::note_view::NoteViewData;

#[derive(Default)]
pub struct NoteViewObject {
    pub vals: Rc<NoteViewData> 
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
        //println!("{}", &self.name.get());
    }
}
impl TextViewImpl for NoteViewObject {}
impl WidgetImpl for NoteViewObject {}
