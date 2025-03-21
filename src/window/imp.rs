
// use std::cell::RefCell;

use glib::subclass::InitializingObject;
// use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use crate::page_new_client::PageNewClient;
use crate::page_consult_client::PageConsultClient;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/PelouseApp/ui/window.ui")]
pub struct Window {
    #[template_child]
    pub main_tool_bar_view: TemplateChild<adw::ToolbarView>,
    #[template_child]
    pub header_bar: TemplateChild<adw::HeaderBar>,
    #[template_child]
    pub stack: TemplateChild<adw::ViewStack>,
    #[template_child]
    pub switcher_bar: TemplateChild<adw::ViewSwitcherBar>,
    #[template_child]
    pub page_new_client: TemplateChild<PageNewClient>,
    #[template_child]
    pub page_consult_client: TemplateChild<PageConsultClient>,
    // // pub tasks: RefCell<Option<gio::ListStore>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "ProjectDatabaseRustWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        
        klass.bind_template();        
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        //Call construct on parents
        self.parent_constructed();

        //Setup
        // let obj = self.obj();
        // obj.setup_task();
        // obj.setup_callback
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}

// Trait shared by all adwaita application windows
impl AdwApplicationWindowImpl for Window {}