
// use std::cell::RefCell;

use glib::subclass::InitializingObject;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/PelouseApp/ui/page_newclient.ui")]
pub struct PageNewClient {
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for PageNewClient {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "PageNewCLient";
    type Type = super::PageNewClient;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        
        klass.bind_template();        
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for PageNewClient {
    fn constructed(&self) {
        //Call construct on parents
        self.parent_constructed();

        //Setup
        let obj = self.obj();
        // obj.setup_task();
        // obj.setup_callback
    }
}

// Trait shared by all widgets
impl WidgetImpl for PageNewClient {}

// // Trait shared by all 
 impl BinImpl for PageNewClient {}