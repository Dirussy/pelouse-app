
// use std::cell::RefCell;

use glib::subclass::InitializingObject;
// use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/PelouseApp/ui/page_newclient.ui")]
pub struct PageNewClient {
    #[template_child]
    pub entry_row_name: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub entry_row_address: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub spin_row_price: TemplateChild<adw::SpinRow>,
    #[template_child]
    pub spin_row_frequency: TemplateChild<adw::SpinRow>,
    #[template_child]
    pub toggle_bag_row: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub entry_row_note: TemplateChild<adw::EntryRow>,
    
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

        klass.install_action("win.add_new_client", None, move|win,_,_|{
            win.new_client_button_pressed();
        });
             
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
    }
}

// Trait shared by all widgets
impl WidgetImpl for PageNewClient {}

// // Trait shared by all 
 impl BinImpl for PageNewClient {}