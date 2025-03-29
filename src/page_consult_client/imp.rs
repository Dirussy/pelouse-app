
// use std::cell::RefCell;

use glib::subclass::InitializingObject;
use chrono::Datelike;

use adw::{AlertDialog, prelude::*};
use adw::{prelude::{ComboRowExt, ExpanderRowExt}, subclass::prelude::*};
use gtk::{glib, CompositeTemplate, DropDown};

use rusqlite::{self, Connection};

use crate::sqlite_functions::{add_pelouse, Payement};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gnome/pelouse_app_rust/ui/page_consult_client.ui")]
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
    #[template_child]
    pub overlay: TemplateChild<adw::ToastOverlay>,
    //Jobs Groups
    #[template_child]
    pub custom_date_job: TemplateChild<adw::ExpanderRow>,
    #[template_child]
    pub month_row_jobs: TemplateChild<adw::ComboRow>,
    #[template_child]
    pub day_spin_row_job: TemplateChild<adw::SpinRow>,
    #[template_child]
    pub years_spin_row_jobs: TemplateChild<adw::SpinRow>,
    


    //PayementsGroups
    #[template_child]
    pub custom_date_pay: TemplateChild<adw::ExpanderRow>,
    #[template_child]
    pub custom_payement_row: TemplateChild<adw::SpinRow>,
    #[template_child]
    pub month_row_pay: TemplateChild<adw::ComboRow>,
    #[template_child]
    pub day_spin_row_pay: TemplateChild<adw::SpinRow>,
    #[template_child]
    pub years_spin_row_pay: TemplateChild<adw::SpinRow>,
    #[template_child]
    pub note_entry_row_pay: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub is_cash_row: TemplateChild<adw::SwitchRow>,
    
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

        klass.install_action_async("win.add_job", None, |win,_,_| async move {
            //get client id

            println!("add jobs!");
            
            let mut month = win.imp().month_row_jobs.selected()as u32 + 1;
            let mut day = win.imp().day_spin_row_job.value() as u32;
            let mut year = win.imp().years_spin_row_jobs.value() as i32;
            let client_name = win.get_selected_client_name();
            //is custom date enable 
            if win.imp().custom_date_job.enables_expansion() == false
            {
                let local_date = chrono::Local::now();
                month = local_date.month();
                day = local_date.day();
                year = local_date.year();
            }
            let dialog = AlertDialog::new(Some("Add New Jobs!"), None);
            dialog.set_body(&format!("Name : {}\n Day: {}\n Month: {}\n Years: {}",
                client_name,
                day,
                month,
                year));
            
            dialog.add_responses(&[("cancel", "Cancel") ,("add_job", "Add Job")]);
            dialog.set_close_response("cancel");
            dialog.set_response_appearance("cancel", adw::ResponseAppearance::Destructive);
            let reponse = dialog.choose_future(&win).await;

            if reponse  == "add_job"
            {
                let conn = Connection::open("PelouseData.db").expect("Cannot open database");
                let client_id:  u32 = conn.query_row_and_then(
                    "SELECT id FROM liste_clients WHERE name_client=?1",
                    [client_name],
                    |row| row.get(0),
                ).unwrap();
                if add_pelouse(&conn, day, month, year, client_id)
                {
                    let toast = adw::Toast::new("Job Added succefully!");
                    win.imp().overlay.add_toast(toast);
                }else {
                    let toast = adw::Toast::new("Error when adding Job!");
                    win.imp().overlay.add_toast(toast);
                }
            }
        });


        klass.install_action_async("win.add_payement", None, |win,_,_| async move {
            println!("add payement!");
            let mut month = win.imp().month_row_pay.selected()as u32 + 1;
            let mut day = win.imp().day_spin_row_pay.value() as u32;
            let mut year = win.imp().years_spin_row_pay.value() as i32;
            let client_name = win.get_selected_client_name();
            //is custom date enable 
            if win.imp().custom_date_pay.enables_expansion() == false
            {
                let local_date = chrono::Local::now();
                month = local_date.month();
                day = local_date.day();
                year = local_date.year();
            }

            let dialog = AlertDialog::new(Some("Add New Payement!"), None);
            dialog.set_body(&format!("Name : {}\n Day: {}\n Month: {}\n Years: {}",
                client_name,
                day,
                month,
                year));
            
            dialog.add_responses(&[("cancel", "Cancel") ,("add_pay", "Add pay")]);
            dialog.set_close_response("cancel");
            dialog.set_response_appearance("cancel", adw::ResponseAppearance::Destructive);
            let reponse = dialog.choose_future(&win).await;

            if reponse  == "add_pay"
            {
                let conn = Connection::open("PelouseData.db").expect("Cannot open database");
                let client_id:  u32 = conn.query_row_and_then(
                    "SELECT id FROM liste_clients WHERE name_client=?1",
                    [client_name],
                    |row| row.get(0),
                ).unwrap();
                let pay = Payement::new(
                    win.imp().custom_payement_row.value(), 
                    day, 
                    month, 
                    year, 
                    &win.imp().note_entry_row_pay.text().to_string(), 
                    win.imp().is_cash_row.is_active(), 
                    client_id
                );
                if pay.add_payement(&conn)
                {
                    let toast = adw::Toast::new("Pay Added succefully!");
                    win.imp().overlay.add_toast(toast);
                }else {
                    let toast = adw::Toast::new("Error when adding Pay!");
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
impl ObjectImpl for PageConsultClient {
    fn constructed(&self) {
        //Call construct on parents
        self.parent_constructed();

        //Setup
         let obj = self.obj();
         obj.setup_list_client();
         obj.setup_date();
        // obj.setup_callback
    }
}

// Trait shared by all widgets
impl WidgetImpl for PageConsultClient {}

// // Trait shared by all
 impl BinImpl for PageConsultClient {}
