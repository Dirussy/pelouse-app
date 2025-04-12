
// use std::cell::RefCell;

use glib::subclass::InitializingObject;
use adw::{prelude::*};
use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use crate::task_job::JobTask;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gnome/pelouse_app_rust/ui/page_log.ui")]
pub struct PageLog {
    #[template_child]
    pub log_page_navigation_view: TemplateChild<adw::NavigationView>,
    #[template_child]
    pub today_job_group: TemplateChild<adw::PreferencesGroup>,

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

        klass.install_action_async("win.today_row_clicked",  None, |win,_,_| async move {
            println!("Clicked Action Row Today!");
            let nav_view = &*win.imp().log_page_navigation_view;
            let task_group = &*win.imp().today_job_group; 
            let job = JobTask::new();
            let action_row = &*job.imp().action_row;
            action_row.set_title("test");
            task_group.add(&job);

            nav_view.push_by_tag("today_job");

        });
        klass.install_action_async("win.late_row",  None, |win,_,_| async move {
            let nav_view = &*win.imp().log_page_navigation_view;
            nav_view.push_by_tag("late_page");
        });
        klass.install_action_async("win.next_row",  None, |win,_,_| async move {
            let nav_view = &*win.imp().log_page_navigation_view;
            nav_view.push_by_tag("next_page");
        });
        klass.install_action_async("win.irregular_row",  None, |win,_,_| async move {
            let nav_view = &*win.imp().log_page_navigation_view;
            nav_view.push_by_tag("irregular_page");
        });
        klass.install_action_async("win.new_row",  None, |win,_,_| async move {
            let nav_view = &*win.imp().log_page_navigation_view;
            nav_view.push_by_tag("new_page");
        });
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
         obj.setup_page();
    }
}

// Trait shared by all widgets
impl WidgetImpl for PageLog {}

// // Trait shared by all
 impl BinImpl for PageLog {}

