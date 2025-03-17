mod imp;

use glib::{clone, Object};
use gtk::{gio, glib};
use adw::prelude::*;
use adw::subclass::prelude::*;

glib::wrapper! {
    pub struct PageNewClient(ObjectSubclass<imp::PageNewClient>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PageNewClient {
    pub fn new() -> Self {
        // Create new window
        Object::builder().build()
    }
}

impl Default for PageNewClient{
    fn default() -> Self {
        Self::new()
    }
}