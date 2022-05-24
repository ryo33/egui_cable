use egui::Response;

use crate::{prelude::*, state::State};

pub trait ResponseExt {
    fn cable_event(&self) -> Option<Event>;
}

impl ResponseExt for Response {
    fn cable_event(&self) -> Option<Event> {
        let state = State::get(self.ctx.data());
        state
            .ephemeral
            .response_id_to_cable_id
            .get(&self.id)
            .and_then(|cable_id| state.ephemeral.event.get(cable_id))
            .cloned()
    }
}
