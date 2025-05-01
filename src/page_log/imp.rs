
use std::cell::RefCell;

use glib::subclass::InitializingObject;
use adw::{prelude::*};
use adw::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate, ListBox};

// use crate::task_job::JobTask;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gnome/pelouse_app_rust/ui/page_log.ui")]
pub struct PageLog {
    #[template_child]
    pub tasks_list: TemplateChild<ListBox>,
    #[template_child]
    pub drop_down: TemplateChild<gtk::DropDown>,
    #[template_child]
    pub list_string: TemplateChild<gtk::StringList>,
    pub tasks: RefCell<Option<gio::ListStore>>

}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for PageLog {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "PageLog";
    type Type = super::PageLog;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {

        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for PageLog {
    fn constructed(&self) {
        //Call construct on parents
        self.parent_constructed();

        //Setup
         let obj = self.obj();
        obj.setup_tasks();
        obj.load_task_from_db("PelouseData.db");
    }
}

// Trait shared by all widgets
impl WidgetImpl for PageLog {}

// // Trait shared by all
 impl BinImpl for PageLog {}

