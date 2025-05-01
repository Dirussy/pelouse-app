/* window.rs
 *
 * Copyright 2025 Dirusy
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::page_new_client::PageNewClient;
use crate::page_consult_client::PageConsultClient;
use crate::page_log::PageLog;


mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/pelouse_app_rust/ui/window.ui")]
    pub struct PelouseAppRustWindow {
        // Template widgets
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
    #[template_child]
    pub page_log: TemplateChild<PageLog>,
    
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PelouseAppRustWindow {
        const NAME: &'static str = "PelouseAppRustWindow";
        type Type = super::PelouseAppRustWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PelouseAppRustWindow {}
    impl WidgetImpl for PelouseAppRustWindow {}
    impl WindowImpl for PelouseAppRustWindow {}
    impl ApplicationWindowImpl for PelouseAppRustWindow {}
    impl AdwApplicationWindowImpl for PelouseAppRustWindow {}
}

glib::wrapper! {
    pub struct PelouseAppRustWindow(ObjectSubclass<imp::PelouseAppRustWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl PelouseAppRustWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
