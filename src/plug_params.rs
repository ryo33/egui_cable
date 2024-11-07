use std::sync::Arc;

use egui::{Id, Vec2};

#[derive(Clone)]
pub struct PlugParams {
    pub vector: Option<Vec2>,
    pub active: bool,
    pub plugged: bool,
    pub locked: bool,
}

impl PlugParams {
    pub fn get(ui: &mut egui::Ui) -> Arc<Self> {
        ui.data_mut(|data| data.get_persisted::<Arc<PlugParams>>(Id::NULL).unwrap())
    }

    pub(crate) fn set(self, ui: &mut egui::Ui) {
        ui.data_mut(|data| data.insert_persisted(Id::NULL, Arc::new(self)));
    }
}
