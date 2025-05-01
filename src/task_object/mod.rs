mod imp;

use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk::glib::{self};

glib::wrapper! {
    pub struct TaskObject(ObjectSubclass<imp::TaskObject>);
}

impl TaskObject {
    pub fn new(nb_days: i64, irregular: bool, content: String) -> Self {
        Object::builder()
            .property("value", nb_days)
            .property("content", content)
            .property("complete", irregular)
            .build()
    }
    pub fn number_of_day(&self) -> i64{
        self.imp().data.borrow().nb_days
    }
    pub fn is_irregular(&self) -> bool{
        self.imp().data.borrow().irregular
    }
}

#[derive(Default)]
pub struct TaskData {
    pub nb_days: i64,
    pub irregular : bool,
    pub content: String,
}