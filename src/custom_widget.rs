use std::any::Any;
use std::fmt::Debug;

use egui::Widget;

pub struct CustomWidget {
    widget: Box<dyn Any + Send + Sync + 'static>,
    debug: fn(&dyn Any) -> &dyn Debug,
    ui: fn(Box<dyn Any>, &mut egui::Ui) -> egui::Response,
}

fn debug<T: Debug + 'static>(t: &dyn Any) -> &dyn Debug {
    t.downcast_ref::<T>().unwrap()
}

fn ui<T: Widget + 'static>(widget: Box<dyn Any>, ui: &mut egui::Ui) -> egui::Response {
    widget.downcast::<T>().unwrap().ui(ui)
}

impl Debug for CustomWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let debug = self.debug;
        f.debug_struct("CustomWidget")
            .field("widget", debug(&self.widget))
            .finish()
    }
}

impl CustomWidget {
    pub(crate) fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let ui_fn = self.ui;
        ui_fn(self.widget, ui)
    }
}

impl<T: Widget + Debug + Send + Sync + 'static> From<T> for CustomWidget {
    fn from(widget: T) -> Self {
        CustomWidget {
            widget: Box::new(widget),
            debug: debug::<T>,
            ui: ui::<T>,
        }
    }
}
