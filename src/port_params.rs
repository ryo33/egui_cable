use std::{ops::DerefMut, sync::Arc};

use egui::{util::IdTypeMap, Id};

#[derive(Clone)]
pub struct PortParams {
    pub hovered: bool,
}

impl PortParams {
    pub fn get(mut data: impl DerefMut<Target = IdTypeMap>) -> Arc<Self> {
        data.get_persisted::<Arc<PortParams>>(Id::null()).unwrap()
    }

    pub(crate) fn set(self, mut data: impl DerefMut<Target = IdTypeMap>) {
        data.insert_persisted(Id::null(), Arc::new(self));
    }
}
