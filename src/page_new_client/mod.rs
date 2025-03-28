mod imp;

use glib::{Object};
use gtk::{glib};
use adw::{ prelude::*};
use adw::subclass::prelude::*;


use crate::sqlite_functions::Client;

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

    pub fn charger_info_client(&self) -> Client
    {
        let imp = self.imp();
        let entry_name = &*imp.entry_row_name;
        let entry_address = &*imp.entry_row_address;
        let price_entry = &*imp.spin_row_price;
        let frequency_entry = &*imp.spin_row_frequency;
        let toggle_bag = &*imp.toggle_bag_row;
        let entry_optional_info = &*imp.entry_row_note;
        let new_client = Client::new(
                &entry_name.text().as_str(),
                &entry_address.text().as_str(),
                price_entry.value(),
                frequency_entry.value() as i32,
                toggle_bag.is_active(),
                &entry_optional_info.text().as_str()
            );
        new_client
    }
}

impl Default for PageNewClient{
    fn default() -> Self {
        Self::new()
    }
}
