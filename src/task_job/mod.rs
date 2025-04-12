mod imp;

use glib::{Object};
use gtk::{glib};
use adw::{ prelude::*};
use adw::subclass::prelude::*;


use crate::sqlite_functions::Client;

glib::wrapper! {
    pub struct JobTask(ObjectSubclass<imp::JobTask>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl JobTask {
    pub fn new() -> Self {
        // Create new window
        Object::builder().build()
    }

}
impl Default for JobTask{
    fn default() -> Self {
        Self::new()
    }
}
