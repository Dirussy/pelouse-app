mod imp;

use glib::{Object, clone};
use gtk::{glib, Ordering};
use adw::{prelude::*, ActionRow};
use gtk::{gio, NoSelection, SignalListItemFactory};
use adw::subclass::prelude::*;
use gtk::{prelude::*, ListItem, CustomFilter, FilterListModel, SortListModel, CustomSorter};


// use crate::task_job::JobTask;
use crate::task_object::TaskObject;
use crate::sqlite_functions::{load_list_client_last_job};

glib::wrapper! {
    pub struct PageLog(ObjectSubclass<imp::PageLog>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PageLog {
    pub fn new() -> Self {
        // Create new window
        Object::builder().build()
    }
    fn tasks(&self) -> gio::ListStore {
        // Get state
        self.imp()
            .tasks
            .borrow()
            .clone()
            .expect("Could not get current tasks.")
    }

    fn filter(&self) -> Option<CustomFilter> {
        // Get filter state from settings
        // Get filter_state from settings
        let filter_pos = self.imp().drop_down.selected();
        let filter_state= self.imp().list_string.string(filter_pos).unwrap().to_string();
        // let filter_state: String = self.settings().get("filter");

        let filter_today = CustomFilter::new(|obj| {
            // Get `TaskObject` from `glib::Object`
            let task_object = obj
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            // Only allow done tasks
            if task_object.number_of_day() == 0
            {
                return true;
            }
            false
        });
        let filter_late = CustomFilter::new(|obj| {
            // Get `TaskObject` from `glib::Object`
            let task_object = obj
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            // Only allow done tasks
            if task_object.number_of_day() < 0 && !task_object.is_irregular()
            {
                return true;
            }
            false
        });
        let filter_irregular = CustomFilter::new(|obj| {
            // Get `TaskObject` from `glib::Object`
            let task_object = obj
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            task_object.is_irregular()
        });

        let filter_new = CustomFilter::new(|obj| {
            // Get `TaskObject` from `glib::Object`
            let task_object = obj
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            if task_object.number_of_day() == i64::MIN
            {
                return true;
            }
            false
        });


        // Return the correct filter
        match filter_state.as_str() {
            "All" => None,
            "Today" => Some(filter_today),
            "Late" => Some(filter_late),
            "Irregular" => Some(filter_irregular),
            "First/New" => Some(filter_new),
            _ => unreachable!(),
        }
    }



    fn setup_tasks(&self) {
        // Create new model
        let model = gio::ListStore::new::<TaskObject>();

        // Get state and set model
        self.imp().tasks.replace(Some(model));

        // Wrap model with filter and selection and pass it to the list box
        let filter_model = FilterListModel::new(Some(self.tasks()), self.filter());
        let sort_model = SortListModel::new(Some(filter_model.clone()), self.sorter());
        let selection_model = NoSelection::new(Some(sort_model.clone()));
        self.imp().tasks_list.bind_model(
            Some(&selection_model),
            clone!(
                #[weak(rename_to = window)]
                self,
                #[upgrade_or_panic]
                move |obj| {
                    let task_object = obj
                        .downcast_ref()
                        .expect("The object should be of type `TaskObject`.");
                    let row = window.create_task_row(task_object);
                    row.upcast()
                }
            ),
        );
        

        // Filter model whenever the value of the key "filter" changes
        //TODO: This code is deprecated and need to be change but it work now
        self.imp().drop_down.connect_selected_notify(clone!(
            @weak self as window,
            @weak filter_model => move |_| {
                filter_model.set_filter(window.filter().as_ref());
            }
        ));


        // Assure that the task list is only visible when it is supposed to
        self.set_task_list_visible(&self.tasks());
        self.tasks().connect_items_changed(clone!(
            #[weak(rename_to = window)]
            self,
            move |tasks, _, _, _| {
                window.set_task_list_visible(tasks);
            }
        ));
    }

    /// Assure that `tasks_list` is only visible
    /// if the number of tasks is greater than 0
    fn set_task_list_visible(&self, tasks: &gio::ListStore) {
        self.imp().tasks_list.set_visible(tasks.n_items() > 0);
    }

    fn load_task_from_db(&self, path: &str)
    {
        let list_task = load_list_client_last_job(path).expect("Cannot load list task");
        
        for task in list_task
        {
            self.new_task(task.name(), task.irregular().clone(), {
                match task.nb_days() {
                    Some(days) => days.clone(),
                    None => i64::MIN,
                }
            }, task.address());
        }
    }

    fn new_task(&self,content : &str, irregular: bool, nb_days: i64, address: &str) {
        // Get content from entry and clear it

        // Add new task to model
        let task = TaskObject::new(nb_days,irregular, content.to_string(), address.to_string());
        self.tasks().append(&task);
    }

    fn sorter(&self) -> Option<CustomSorter>{
        //create custom sorter
        Some(CustomSorter::new(|obj1, obj2|
        {
            // Get `TaskObject` from `glib::Object`
            let task_object1 = obj1
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            let task_object2 = obj2
            .downcast_ref::<TaskObject>()
            .expect("The object needs to be of type `TaskObject`.");

            if task_object1.number_of_day() == task_object2.number_of_day()
            {
                return Ordering::Equal
            }
            if task_object1.number_of_day() < task_object2.number_of_day()
            {
                return Ordering::Smaller
            }
            if task_object1.number_of_day() > task_object2.number_of_day()
            {
                return Ordering::Larger
            }
            Ordering::Equal
        }))
    }
    
    fn create_task_row(&self, task_object: &TaskObject) -> ActionRow {

        // Create row
        let row = ActionRow::builder()
            // .activatable_widget(&check_button)
            .build();

        row.set_title(&task_object.imp().data.borrow().content);
        row.set_subtitle(&task_object.imp().data.borrow().address);

        let day_label = gtk::Label::builder().label(
        {
            let days = task_object.number_of_day();

            let day_label = match days {
                0 => "Today".to_string(),
                i64::MIN => "First".to_string(),
                _ => format!("days: {}", days),
            };
            day_label
        }).build();
        day_label.add_css_class("card");
        day_label.set_width_request(100);
        day_label.set_height_request(50);
        day_label.set_valign(gtk::Align::Center);
        row.add_suffix(&day_label);

        row
    }
    

}

impl Default for PageLog{
    fn default() -> Self {
        Self::new()
    }
}

// fn sort_function(one: &TaskObject, two: &TaskObject) -> Ordering
// {

// }