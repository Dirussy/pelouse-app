
// use std::cell::RefCell;

use glib::subclass::InitializingObject;
use adw::{prelude::*};
use adw::AlertDialog;
use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use crate::sqlite_functions::{connect_database,add_client};


// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gnome/pelouse_app_rust/ui/page_newclient.ui")]
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
    #[template_child]
    pub overlay: TemplateChild<adw::ToastOverlay>,
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

        klass.install_action_async("win.add_new_client", None, |win,_,_| async move {
        // confirm to the user if everything is in order
            let dialog = AlertDialog::new(Some("Add New CLient!"), None);
            let client = win.charger_info_client();
            dialog.set_body(&format!("Name : {}\n Address: {}\n Cost: {}$\n Freq: {} Days\n Bag: {}\n Note: {}",
                client.name(),
                client.address(),
                client.cost(),
                client.freq(),
                client.is_bag_use(),
                client.note()));
            
            dialog.add_responses(&[("cancel", "Cancel") ,("add_client", "Add Client")]);
            dialog.set_close_response("cancel");
            dialog.set_response_appearance("cancel", adw::ResponseAppearance::Destructive);
            let reponse = dialog.choose_future(&win).await;

            if reponse  == "add_client"
            {
                let new_client = win.charger_info_client();
                let database = connect_database("PelouseData.db")
                    .expect("Erreur when oppening database!");
                if add_client(&database, &new_client) == true
                {
                    let toast = adw::Toast::new("Client Added succefully!");
                    win.imp().overlay.add_toast(toast);
                }else {
                    let toast = adw::Toast::new("Error when adding client!");
                    win.imp().overlay.add_toast(toast);
                }
            }
    
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
    }
}

// Trait shared by all widgets
impl WidgetImpl for PageNewClient {}

// // Trait shared by all
 impl BinImpl for PageNewClient {}


 fn alert_function(win : &PageNewClient)
 {
    println!("test");
 }