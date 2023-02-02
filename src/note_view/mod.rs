pub mod imp;

use glib::Object;
use gtk::prelude::*;
use std::sync::Arc;
use gtk::WrapMode;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib;
use std::fs;
use std::vec::Vec;
use regex::Regex;


use std::sync::Mutex;

glib::wrapper! {
    pub struct NoteViewObject(ObjectSubclass<imp::NoteViewObject>)
    @extends gtk::TextView, gtk::Widget, gtk::gio::SimpleActionGroup,
    @implements gtk::Accessible, gtk::Buildable,  
    gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for NoteViewObject {
    fn default() -> Self {
        Self::new()
    }
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
        let mut open_tag: Vec<gtk::TextTag> = Vec::new();

        let mut ret = String::from("");
        while iter < end {
            let mut next = iter;
            next.forward_char();
            // All tags at the current position
            for tag in iter.toggled_tags(true) {
                ret.push_str(&format!("<{}>", tag.name().unwrap()));
                open_tag.push(tag);
            }

            if !open_tag.is_empty() && iter.ends_tag(Some(open_tag.last().unwrap())){
                let tag = open_tag.pop().expect("Nothing to pop...");
                ret.push_str(&format!("</{}>", tag.name().expect("No Tag Name")));
            }
            ret.push_str(&next.visible_text(&iter));

            iter.forward_char();
       }
        println!("Ret:\n {}", ret);
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().serialized = ret;

    }

    pub fn load(&mut self, markup: String) {
        println!("Begin Parsing: {}", markup);

        let re = Regex::new(r"<[a-z]*>").unwrap();
        let mat = match re.find(&markup) {
            Some(m) => m,
            None => {self.buffer().insert(&mut self.buffer().end_iter(), &markup); return}
        };

        // get the name of the tag and generate the end tag based on it
        let tag_name = &markup[mat.start() + 1..mat.end() - 1];
        let end_tag = format!("</{}>", tag_name);
        println!("Tag Name: {}", tag_name);

        // get all text before and after the start tag.
        let (pre, post) = markup.as_str().split_at(mat.start());
        let post = post.replacen(&format!("<{}>", tag_name), "", 1);
        // push the pre to the buffer 
        self.buffer().insert(&mut self.buffer().end_iter(), pre);
        let tag_start = self.buffer().end_iter().offset();

        // we must find the eneding tag in the post section 
        // then we will attempt to parse everything between the tags 
        // and everything after the end tag
        let end_re = Regex::new(&end_tag).unwrap();
        let end_mat = match end_re.find(&post) {
            Some(m) => m,
            None => return // probably shouldn't return in this case but.. /shrug/
        };

        // in this case inner is the text between the two tags which may also be parsable
        // so we pass it to load as well. post all the text after the end of the tag 
        // which again may be parseable so we pass it to parse.
        // Note that we must handle everything in the inner before handling the post
        let (inner, post) = post.split_at(end_mat.start());
        let post = post.replacen(&format!("</{}>", tag_name), "", 1);
        // push the pre to the buffer 
        self.load(inner.to_string());
        let tag_end = self.buffer().end_iter().offset();
        println!("Applying tag: {}, from {}-{}", tag_name, tag_start, tag_end);
        self.buffer().apply_tag_by_name(tag_name, 
                                        &self.buffer().iter_at_offset(tag_start), 
                                        &self.buffer().iter_at_offset(tag_end));        
        self.load(post.to_string());
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
        // self.set_left_margin(35);
        // self.set_right_margin(35);
        // self.set_top_margin(24);
        // self.set_bottom_margin(24);

        let bold_tag = gtk::TextTag::new(Some("b"));
        bold_tag.set_weight(600);
        self.buffer().tag_table().add(&bold_tag);

        let italic_tag = gtk::TextTag::new(Some("i"));
        italic_tag.set_font(Some("Sans italic 12"));
        self.buffer().tag_table().add(&italic_tag);

        let bullet_tag = gtk::TextTag::builder()
                .name("bullet")
                .indent_set(true)
                .indent(10)
                .build();

        self.buffer().tag_table().add(&bullet_tag);

        self.buffer().connect_changed(|note|  {
            let mut cursor = note.iter_at_offset(note.cursor_position());
            let line_start = note.iter_at_line(cursor.line()).expect("Unable to get line start");
            let parsing = note.slice(&line_start, &cursor, true);
            
            static mut SIZE: i32 = 0;
            let mut is_bullet = false;

            // PLEASE find a way to this in a safe way? 
            // The reason its unsafe is the use of a mutable static, we are tracking the size 
            // of the buffer, and if we are decreasing in size then don't attempt to insert 
            // another bullet
            unsafe{
                for tag in line_start.tags() {
                    if tag.name().expect("No tag name specified") == "bullet" && note.char_count() >  SIZE && line_start == cursor {
                        is_bullet = true;
                        break;
                    } else {
                        println!("texttag: {:?}", tag);
                    }
                }

                SIZE = note.char_count();
            }

            if is_bullet {
                note.insert_at_cursor("- ");
            } else if parsing == "-  " { // if it isn't the starting action bail out 
                println!("Starting bulleted list");
                note.apply_tag_by_name("bullet", &line_start, &note.iter_at_offset(note.cursor_position()));
                note.place_cursor(&note.iter_at_offset(note.cursor_position() - 1));
            }
        });


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
