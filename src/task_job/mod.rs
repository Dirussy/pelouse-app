mod imp;


use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrInt, AttrList};


use crate::task_object::TaskObject;

glib::wrapper! {
    pub struct JobTask(ObjectSubclass<imp::JobTask>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl JobTask {
    pub fn new() -> Self {
        // Create new window
        Object::builder().build()
    }

    pub fn bind(&self, task_object: &TaskObject) {
        // Get state
        let content_label = self.imp().label_name.get();
        // let days = self.imp().nb_days.borrow_mut();
        let mut bindings = self.imp().bindings.borrow_mut();

        // let day_binding = task_object
        //     .bind_property("number", &days, "number")
        //     .sync_create()
        //     .build();

        // Bind `task_object.content` to `task_row.content_label.label`
        let content_label_binding = task_object
            .bind_property("content", &content_label, "label")
            .sync_create()
            .build();
        // Save binding
        bindings.push(content_label_binding);
    }
    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}

impl Default for JobTask{
    fn default() -> Self {
        Self::new()
    }
}
