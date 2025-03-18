mod imp;

use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::{Object};
use gtk::glib::{self, types::StaticType};
// use adw::prelude::*;
// use adw::subclass::prelude::*;

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
        let my_string_vec = vec!["one", "two", "apples", "appstore", "append"];
        let imgs_strlist = gtk::StringList::new(my_string_vec.as_slice()); 
        let exp = gtk::PropertyExpression::new(
            gtk::StringObject::static_type(),
            None::<gtk::Expression>,
            "string",
        );
        drop_menu.set_expression(Some(exp));
        drop_menu.set_model(Some(&imgs_strlist));
        // let liste_client 
        // drop_menu.expression();
    }
}

impl Default for PageConsultClient{
    fn default() -> Self {
        Self::new()
    }
}