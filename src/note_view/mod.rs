mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use gtk::WrapMode;
glib::wrapper! {
    pub struct NoteView(ObjectSubclass<imp::NoteView>)
    @extends gtk::TextView, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, 
    gtk::ConstraintTarget, gtk::Orientable;
}

impl NoteView{
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `NoteView`.")
    }

    pub fn setup(&self) {
        self.set_editable(true);
        self.set_wrap_mode(WrapMode::Word);
        self.set_left_margin(60);
        self.set_right_margin(60);
        self.set_top_margin(24);
        self.set_bottom_margin(24);
    }
}
