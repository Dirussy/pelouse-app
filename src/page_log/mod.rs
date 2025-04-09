mod imp;

use glib::{Object};
use gtk::{glib};
use adw::{ prelude::*};
use adw::subclass::prelude::*;


use crate::sqlite_functions::Client;

glib::wrapper! {
    pub struct PageLog(ObjectSubclass<imp::PageLog>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PageLog {
    pub fn new() -> Self {
        // Create new window
        Object::builder().build()
    }
    pub fn setup_page(&self) {
        let nav_view = &*self.imp().log_page_navigation_view;
        nav_view.push_by_tag("selection_list_page");
    }
}

impl Default for PageLog{
    fn default() -> Self {
        Self::new()
    }
}
