use std::ops::Deref;

use egui::Response;

use crate::{prelude::*, state::State};

pub trait ResponseExt {
    /// Returns a in-plug response
    fn in_plug(&self) -> PlugResponse;
    /// Returns a out-plug response
    fn out_plug(&self) -> PlugResponse;
}

pub struct PlugResponse(pub(crate) Response);

impl ResponseExt for Response {
    fn in_plug(&self) -> PlugResponse {
        let response = State::get(self.ctx.data())
            .ephemeral
            .plug_responces_of_cable
            .get(&self.id)
            .unwrap()
            .0
            .clone();
        PlugResponse(response)
    }

    fn out_plug(&self) -> PlugResponse {
        let response = State::get(self.ctx.data())
            .ephemeral
            .plug_responces_of_cable
            .get(&self.id)
            .unwrap()
            .1
            .clone();
        PlugResponse(response)
    }
}

impl PlugResponse {
    pub fn connected_to(&self) -> Option<PortId> {
        let state = State::get(self.0.ctx.data());
        if let Some(Event::Connected { port_id }) = state.ephemeral.event_of_plug.get(&self.0.id) {
            Some(port_id.clone())
        } else {
            None
        }
    }

    pub fn disconnected(&self) -> bool {
        let state = State::get(self.0.ctx.data());
        matches!(
            state.ephemeral.event_of_plug.get(&self.0.id),
            Some(Event::Disconnected { .. })
        )
    }
}

impl Deref for PlugResponse {
    type Target = Response;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
