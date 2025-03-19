
// use std::cell::RefCell;

use glib::subclass::InitializingObject;
// use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, DropDown}; 

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/PelouseApp/ui/page_consult_client.ui")]
pub struct PageConsultClient {
    #[template_child]
    pub drop_down_client: TemplateChild<DropDown>,
    // Info client Group
    #[template_child]
    pub address_row: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub price_row: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub freq_row: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub bag_row: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub note_entry: TemplateChild<adw::ActionRow>,
    //Jobs Groups

    //PayementsGroups
    #[template_child]
    pub custom_payement_row: TemplateChild<adw::SpinRow>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for PageConsultClient {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "PageConsultClient";
    type Type = super::PageConsultClient;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        
        klass.bind_template();   
        
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for PageConsultClient {
    fn constructed(&self) {
        //Call construct on parents
        self.parent_constructed();

        //Setup
         let obj = self.obj();
         obj.setup_list_client();
        // obj.setup_callback
    }
}

// Trait shared by all widgets
impl WidgetImpl for PageConsultClient {}

// // Trait shared by all 
 impl BinImpl for PageConsultClient {}