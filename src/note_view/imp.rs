

use gtk::subclass::prelude::*;
use gtk::{glib};
use std::rc::Rc;
use std::cell::RefCell;

use crate::note_view::NoteViewData;
use gtk::prelude::TextViewExt;
use gtk::prelude::TextBufferExt;


#[derive(Default)]
pub struct NoteViewObject {
    pub vals: Rc<RefCell<NoteViewData>>
}

#[glib::object_subclass]
impl ObjectSubclass for NoteViewObject {
    const NAME: &'static str = "NoteView";
    type Type = super::NoteViewObject;
    type ParentType = gtk::TextView;

}

impl ObjectImpl for NoteViewObject {
    fn constructed(&self, obj: &Self::Type) {
        obj.buffer().connect_changed( move |arg1| {
            println!("text: {}", arg1.slice(&arg1.start_iter(), &arg1.end_iter(), false));
        });
        self.parent_constructed(obj);
        let mut vals = self.vals.borrow_mut();
        vals.name = "oui".to_string();
        drop(vals);
        println!("{}", self.vals.borrow().name);
    }
}
impl TextViewImpl for NoteViewObject {}
impl WidgetImpl for NoteViewObject {}
