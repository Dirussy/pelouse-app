
// use std::cell::RefCell;

use glib::subclass::InitializingObject;
use adw::{prelude::*};
use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};



// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gnome/pelouse_app_rust/ui/jobs_task.ui")]
pub struct JobTask {
    #[template_child]
    pub action_row: TemplateChild<adw::ActionRow>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for JobTask {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "JobTask";
    type Type = super::JobTask;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {

        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for JobTask {
    fn constructed(&self) {
        //Call construct on parents
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for JobTask {}

// // Trait shared by all
 impl  BinImpl for JobTask {}

