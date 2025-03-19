mod imp;

use std::{num::NonZero, thread::sleep};

use adw::{prelude::ActionRowExt, subclass::prelude::ObjectSubclassIsExt};
use glib::{Object, clone};
use gtk::{gio::prelude::{ListModelExt, ListModelExtManual}, glib::{self, object::{Cast, ObjectExt}, types::StaticType}, prelude::WidgetExt};
// use adw::prelude::*;
// use adw::subclass::prelude::*;

use crate::sqlite_functions::{self, Client};
use rusqlite::{self, Connection, params};

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
        let page = (drop_menu,address_row);

        let exp = gtk::PropertyExpression::new(
            gtk::StringObject::static_type(),
            None::<gtk::Expression>,
            "string",
        );
        drop_menu.set_expression(Some(exp));

        drop_menu.connect_map(move|drop_down|{
            println!("Update List Client!!");
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

        });

        drop_menu.connect_selected_notify(clone!(
            #[weak(rename_to = window)]
            self, move|_|{
                let drop_down = &*window.imp().drop_down_client;
                let address_row = &*window.imp().address_row;
                let selectionner = match drop_down.selected_item()
                {
                    Some(obj) => obj.downcast::<gtk::StringObject>().unwrap().string().into(),
                    None => String::from("None")
                };
                println!("Client selectionner {}", &selectionner);
                if selectionner != "None"
                {
                    let conn = Connection::open("PelouseData.db").expect("Cannot open database");
                    let address: String = conn.query_row_and_then(
                        "SELECT address FROM liste_clients WHERE name_client=?1",
                        [selectionner],
                        |row| row.get(0),
                    ).unwrap();
                    println!("{address}");
                    address_row.set_subtitle(&address);
                }

            }
        ));
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