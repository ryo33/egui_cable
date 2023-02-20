use std::sync::Arc;

use egui::Id;

#[derive(Clone)]
pub struct PortParams {
    pub hovered: bool,
}

impl PortParams {
    pub fn get(ui: &mut egui::Ui) -> Arc<Self> {
        ui.data_mut(|data| data.get_persisted::<Arc<PortParams>>(Id::null()).unwrap())
    }

    pub(crate) fn set(self, ui: &mut egui::Ui) {
        ui.data_mut(|data| data.insert_persisted(Id::null(), Arc::new(self)));
    }
}
