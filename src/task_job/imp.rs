
use std::cell::{Cell, RefCell};

use glib::Binding;
use glib::subclass::InitializingObject;
use adw::{prelude::*};
use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};



// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gnome/pelouse_app_rust/ui/jobs_task.ui")]
pub struct JobTask {
    #[template_child]
    pub label_name: TemplateChild<gtk::Label>,
    #[template_child]
    pub day_label: TemplateChild<gtk::Label>,
    // Vector holding the bindings to properties of `TaskObject`
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for JobTask {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "JobTask";
    type Type = super::JobTask;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {

        klass.bind_template();
        
        klass.install_action_async("win.task_job_button", None, |task,_,_| async move {
            println!("test!");
        });

    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for JobTask {}

// Trait shared by all widgets
impl WidgetImpl for JobTask {}

// Trait shared by all boxes
impl BoxImpl for JobTask {}