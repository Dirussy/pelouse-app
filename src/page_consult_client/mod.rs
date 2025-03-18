mod imp;

use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::{Object};
use gtk::{gio::prelude::ListModelExt, glib::{self, types::StaticType}, prelude::WidgetExt};
// use adw::prelude::*;
// use adw::subclass::prelude::*;

use crate::sqlite_functions;
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
        let exp = gtk::PropertyExpression::new(
            gtk::StringObject::static_type(),
            None::<gtk::Expression>,
            "string",
        );
        drop_menu.set_expression(Some(exp));

        drop_menu.connect_map(move|drop_down|{
            let list = drop_down.model().expect("no list");
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
        // let liste_client 
        // drop_menu.expression();
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