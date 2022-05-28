use std::{ops::DerefMut, sync::Arc};

use egui::{util::IdTypeMap, Id, Vec2};

#[derive(Clone)]
pub struct PlugParams {
    pub vector: Option<Vec2>,
    pub active: bool,
}

impl PlugParams {
    pub fn get(mut data: impl DerefMut<Target = IdTypeMap>) -> Arc<Self> {
        data.get_persisted::<Arc<PlugParams>>(Id::null()).unwrap()
    }

    pub(crate) fn set(self, mut data: impl DerefMut<Target = IdTypeMap>) {
        data.insert_persisted(Id::null(), Arc::new(self));
    }
}
