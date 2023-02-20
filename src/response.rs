use std::ops::Deref;

use egui::{Pos2, Response};

use crate::{prelude::*, state::State};

pub trait ResponseExt {
    /// Returns a in-plug response
    fn in_plug(&mut self) -> PlugResponse;
    /// Returns a out-plug response
    fn out_plug(&mut self) -> PlugResponse;
}

pub struct PlugResponse(pub(crate) Response);

impl ResponseExt for Response {
    fn in_plug(&mut self) -> PlugResponse {
        let response = State::get_with_ctx(&mut self.ctx)
            .ephemeral
            .plug_responses_of_cable
            .get(&self.id)
            .unwrap()
            .0
            .clone();
        PlugResponse(response)
    }

    fn out_plug(&mut self) -> PlugResponse {
        let response = State::get_with_ctx(&mut self.ctx)
            .ephemeral
            .plug_responses_of_cable
            .get(&self.id)
            .unwrap()
            .1
            .clone();
        PlugResponse(response)
    }
}

impl PlugResponse {
    pub fn connected_to(&mut self) -> Option<PortId> {
        let state = State::get_with_ctx(&mut self.0.ctx);
        if let Some(Event::Connected { port_id }) = state.ephemeral.event_of_plug.get(&self.0.id) {
            Some(port_id.clone())
        } else {
            None
        }
    }

    pub fn disconnected(&mut self) -> bool {
        let state = State::get_with_ctx(&mut self.0.ctx);
        matches!(
            state.ephemeral.event_of_plug.get(&self.0.id),
            Some(Event::Disconnected { .. })
        )
    }

    pub fn hovered_on(&mut self) -> Option<PortId> {
        let state = State::get_with_ctx(&mut self.0.ctx);
        if let Some(Event::Hovered { port_id }) = state.ephemeral.event_of_plug.get(&self.0.id) {
            Some(port_id.clone())
        } else {
            None
        }
    }

    pub fn next_position(&self) -> Pos2 {
        self.0.rect.left_top() + self.0.drag_delta()
    }
}

impl Deref for PlugResponse {
    type Target = Response;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
