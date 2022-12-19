pub mod imp;

use glib::Object;
use gtk::prelude::*;
use std::sync::Arc;
use gtk::WrapMode;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib;
use std::fs;

use json;

use std::sync::Mutex;

glib::wrapper! {
    pub struct NoteViewObject(ObjectSubclass<imp::NoteViewObject>)
    @extends gtk::TextView, gtk::Widget, gtk::gio::SimpleActionGroup,
    @implements gtk::Accessible, gtk::Buildable,  
    gtk::ConstraintTarget, gtk::Orientable;
}

impl NoteViewObject {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `NoteView`.")
    }

    // this function is going to be paired w/ a deserialize function
    // The goal is to insert tags from the tag table inline in a mark
    // up style format - so for bold text it will be:
    //      <bold> some text </bold>
    //  and Italics will be :
    //      <italic> some text </italic> 
    //
    // Right now this is just scaffoling - but I think it could use an 
    // "accumulator" string which characters are pushed to and if that 
    // iterator is also a tag start or end push <bold> or <italic>
    // 
    // ideally we can also use this for formatting bulleted and numbered
    // lists. 
    pub fn serialize(&self) {
        println!("serializes");
        let (start, end) = self.buffer().bounds();
        let mut iter = start;
        let mut open_tag = gtk::TextTag::new(Some("filler"));

        let mut ret = String::from("");
        while iter != end {
            let mut next = iter;
            next.forward_char();
            for tag in iter.toggled_tags(true) {
                let inter = tag.name().unwrap();
                let tag_name: Vec<&str>  = inter.split("=").collect();
                if tag_name.len() == 1 {
                    ret.push_str(&format!("<{}>", tag.name().unwrap()));
                } else {
                    ret.push_str(&format!("<span {}=\"{}\">", tag_name[0], tag_name[1]));
                }
                open_tag = tag;
            }

            if iter.ends_tag(Some(&open_tag)) {
                let inter = open_tag.name().unwrap();
                let tag_name: Vec<&str>  = inter.split("=").collect();
                //ret.push_str(&format!("</span>"));
                if tag_name.len() == 1 {
                    ret.push_str(&format!("</{}>", tag_name[0]));
                } else {
                    ret.push_str(&format!("</span>"));
                }
            }
            ret.push_str(&next.visible_text(&iter).to_string());

            iter.forward_char();
        }
        println!("Ret: {}", ret);
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().serialized = ret;

    }

    pub fn save(&self) {
        self.serialize();
        let binding = Arc::clone(&self.imp().vals);
        let vals = binding.lock().unwrap();
        let write_val = fs::write(&vals.filename, json::stringify(
                json::object!{name: &*vals.name, 
                              contents: &*vals.serialized
                }
        ));

        match write_val {
            Ok(_) => {},
            Err(e) => {println!("Error writing file: {e:?}");}
        }
    }

    pub fn setup(&self) {
        self.set_editable(true);
        self.set_wrap_mode(WrapMode::Word);
        self.set_left_margin(35);
        self.set_right_margin(35);
        self.set_top_margin(24);
        self.set_bottom_margin(24);

        let bold_tag = gtk::TextTag::new(Some("b"));
        bold_tag.set_weight(600);
        self.buffer().tag_table().add(&bold_tag);

        let italic_tag = gtk::TextTag::new(Some("i"));
        italic_tag.set_font(Some("Sans italic 12"));
        self.buffer().tag_table().add(&italic_tag);

        //self.add_action(&action_bold);
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
        self.serialize()
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
    pub serialized: String,
    pub filename: String,
    pub note_id: u32,
}
