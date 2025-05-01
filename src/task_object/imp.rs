use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::TaskData;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::TaskObject)]
pub struct TaskObject {
    #[property(name = "value", get, set, type = i64, member = nb_days)]
    #[property(name = "content", get, set, type = String, member = content)]
    #[property(name = "address", get, set, type = String, member = address)]
    #[property(name = "complete", get, set, type = bool, member = irregular)]
    pub data: RefCell<TaskData>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TaskObject {
    const NAME: &'static str = "TodoTaskObject";
    type Type = super::TaskObject;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for TaskObject {}