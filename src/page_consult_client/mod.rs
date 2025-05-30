mod imp;

use chrono::Datelike;

use adw::{prelude::{ActionRowExt, ComboRowExt}, subclass::prelude::ObjectSubclassIsExt};
use glib::{Object, clone};
use gtk::{glib::{self, object::Cast, types::StaticType}, prelude::WidgetExt};
// use adw::prelude::*;
// use adw::subclass::prelude::*;

use crate::{sqlite_functions::Client};
use rusqlite::{self, Connection};

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
    pub fn setup_list_client(&self){
        let drop_menu = &*self.imp().drop_down_client;
        // let address_row = &*self.imp().address_row;

        let exp = gtk::PropertyExpression::new(
            gtk::StringObject::static_type(),
            None::<gtk::Expression>,
            "string",
        );
        drop_menu.set_expression(Some(exp));

        drop_menu.connect_map(clone!(
            #[weak(rename_to = window)]
            self, 
            move|_|{
                println!("Update List Client!!");
                let drop_down = &*window.imp().drop_down_client;
                let my_string_vec:Vec<&str> = vec![];
                let imgs_strlist = gtk::StringList::new(my_string_vec.as_slice()); 
                drop_down.set_model(Some(&imgs_strlist));
                
                let conn = Connection::open("PelouseData.db").expect("Cannot open database");

                let mut stmt = match conn.prepare("SELECT name_client FROM liste_clients")
                {
                    Ok(conn) => conn,
                    Err(_) => {
                        println!("No client in file!");
                        return;
                    },
                };
                
                let client_iter = stmt.query_map([], |row| {
                    Ok(ClientIter {
                        name: row.get(0)?,
                    })
                }).expect("Erreur loading client");
                for client in client_iter {
                    imgs_strlist.append(&client.unwrap().name);
                }

            }
        ));

        drop_menu.connect_selected_notify(clone!(
            #[weak(rename_to = window)]
            self, move|_|{
                let name_client = window.get_selected_client_name();
                if name_client != "None"
                {
                    let client_donner = Client::laod_from_name(&name_client);
                    window.load_page_consult_client(&client_donner);

                    window.load_client_job_pay_done(&client_donner);
                }

            }
        ));
    }
    pub fn setup_date(&self){
        let local_date = chrono::Local::now();
        
        let month_row_job = &*self.imp().month_row_jobs;
        let day_row_job = &*self.imp().day_spin_row_job;
        let years_row_job = &*self.imp().years_spin_row_jobs;

        let month_row_pay = &*self.imp().month_row_pay;
        let day_row_pay = &*self.imp().day_spin_row_pay;
        let years_row_pay = &*self.imp().years_spin_row_pay;

        month_row_job.set_selected(local_date.month() - 1);
        day_row_job.set_value(local_date.day() as f64);
        years_row_job.set_value(local_date.year() as f64);

        month_row_pay.set_selected(local_date.month() - 1);
        day_row_pay.set_value(local_date.day() as f64);
        years_row_pay.set_value(local_date.year() as f64);
    }
    pub fn get_selected_client_name(&self) -> String {
        let drop_down = &*self.imp().drop_down_client;
        match drop_down.selected_item()
        {
            Some(obj) => obj.downcast::<gtk::StringObject>().unwrap().string().into(),
            None => String::from("None")
        }
        
    }
    pub fn load_page_consult_client(&self, client : &Client)
    {
        let address_row = &*self.imp().address_row;
        address_row.set_subtitle(client.address());
        let price_row = &*self.imp().price_row;
        price_row.set_subtitle(format!("{}$", &client.cost()).as_str());
        let freq_row = &*self.imp().freq_row;
        freq_row.set_subtitle(format!("{} days", &client.freq()).as_str());
        let bag_row = &*self.imp().bag_row;
        bag_row.set_active(*client.is_bag_use());
        let note_row = &*self.imp().note_entry;
        note_row.set_subtitle(client.note());
        let custom_pay_row = &*self.imp().custom_payement_row;
        custom_pay_row.set_value(*client.cost());
    }
    pub fn load_client_job_pay_done(&self, client : &Client)
    {
        let conn = Connection::open("PelouseData.db").expect("Cannot open database");
        let client_id = client.get_client_id(&conn).expect("No client ID found (Impossible)");
        
        let job_done: i32 = conn.query_row(
            "SELECT COUNT(*) FROM liste_pelouse WHERE client_id = ?1",
            [client_id],
            |row| row.get(0),
        ).expect("Cannot get count!");
        println!("Job done {}", job_done);

        
        let pay_total: f64 = conn.query_row(
            "SELECT SUM(montant) FROM liste_payement WHERE client_id = ?1",
            [client_id],
            |row| row.get(0),
        ).unwrap_or(0.0);

        println!("Pay done {}", pay_total);
        let job_payed = (pay_total / client.cost()) as i32 ;
        let pay_missing = pay_total - (job_done as f64 * client.cost());

        self.imp().nb_job_done.set_subtitle(format!("Done: {}, missing: {}",job_done, job_payed - job_done).as_ref());
        self.imp().nb_pay_done.set_subtitle(format!("Payed: {}, missing: {}",pay_total, -1.0 * pay_missing).as_ref());
    }
}

impl Default for PageConsultClient{
    fn default() -> Self {
        Self::new()
    }
}


#[derive(Debug)]
struct ClientIter {
    name: String,
}