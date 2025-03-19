mod imp;

use std::{num::NonZero, thread::sleep};

use adw::{prelude::{ActionRowExt, BinExt}, subclass::prelude::ObjectSubclassIsExt};
use glib::{Object, clone};
use gtk::{glib::{self, object::Cast, types::StaticType}, prelude::WidgetExt};
// use adw::prelude::*;
// use adw::subclass::prelude::*;

use crate::sqlite_functions::{self, Client};
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
        let address_row = &*self.imp().address_row;

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

                let mut stmt = conn.prepare("SELECT name_client FROM liste_clients").expect("Error from select");
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
                let nom_client = window.get_selected_client_name();
                let address_row = &*window.imp().address_row;
                println!("Client selectionner {}", &nom_client);
                if nom_client != "None"
                {
                    let conn = Connection::open("PelouseData.db").expect("Cannot open database");
                    let address: String = conn.query_row_and_then(
                        "SELECT address FROM liste_clients WHERE name_client=?1",
                        [nom_client],
                        |row| row.get(0),
                    ).unwrap();
                    println!("{address}");
                    address_row.set_subtitle(&address);
                }

            }
        ));
    }
    pub fn get_selected_client_name(&self) -> String {
        let drop_down = &*self.imp().drop_down_client;
        let client_name = match drop_down.selected_item()
        {
            Some(obj) => obj.downcast::<gtk::StringObject>().unwrap().string().into(),
            None => String::from("None")
        };
        client_name
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