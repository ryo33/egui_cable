use std::sync::Arc;

use egui::Id;
use epaint::QuadraticBezierShape;

use crate::cable_control::CableControl;

#[derive(Debug)]
pub struct CableParams {
    pub active: bool,
    pub line_hovered: bool,
    pub plugs_interacted: bool,
    pub cable_control: CableControl,
    pub bezier: QuadraticBezierShape,
}

impl CableParams {
    pub fn get(ui: &mut egui::Ui) -> Self {
        ui.data_mut(|data| {
            let params = data.get_persisted::<Arc<CableParams>>(Id::NULL).unwrap();
            data.remove::<Arc<CableParams>>(Id::NULL);
            Arc::try_unwrap(params).unwrap()
        })
    }

    pub(crate) fn set(self, ui: &mut egui::Ui) {
        ui.data_mut(|data| data.insert_persisted(Id::NULL, Arc::new(self)));
    }
}
