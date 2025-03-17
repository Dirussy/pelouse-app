mod imp;

use glib::{Object};
use gtk::{glib};
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
    fn new_client_button_pressed(&self){
        let imp = self.imp();
        let entry_name = &*imp.entry_row_name;
        let entry_address = &*imp.entry_row_address;
        let price_entry = &*imp.spin_row_price;
        let frequency_entry = &*imp.spin_row_frequency;
        let toggle_bag = &*imp.toggle_bag_row;
        let entry_optional_info = &*imp.entry_row_note;
        println!("New Client {}, {}, {}, {}, {}, {}", 
            entry_name.text(),
            entry_address.text(),
            price_entry.value(),
            frequency_entry.value(),
            toggle_bag.is_active(),
            entry_optional_info.text()
            );
    }
}

impl Default for PageNewClient{
    fn default() -> Self {
        Self::new()
    }
}