use std::{ops::DerefMut, sync::Arc};

use egui::{util::IdTypeMap, Id};
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
    pub fn get(mut data: impl DerefMut<Target = IdTypeMap>) -> Self {
        let params = data.get_persisted::<Arc<CableParams>>(Id::null()).unwrap();
        data.remove::<Arc<CableParams>>(Id::null());
        Arc::try_unwrap(params).unwrap()
    }

    pub(crate) fn set(self, mut data: impl DerefMut<Target = IdTypeMap>) {
        data.insert_persisted(Id::null(), Arc::new(self));
    }
}
