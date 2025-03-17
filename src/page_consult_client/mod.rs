mod imp;

use glib::{Object};
use gtk::{glib};
// use adw::prelude::*;
// use adw::subclass::prelude::*;

glib::wrapper! {
    pub struct PageConsultClient(ObjectSubclass<imp::PageConsultClient>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PageConsultClient {
    pub fn new() -> Self {
        // Create new window
        Object::builder().build()
    }
}

impl Default for PageConsultClient{
    fn default() -> Self {
        Self::new()
    }
}