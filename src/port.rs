use std::fmt::Debug;
use std::hash::Hash;

use egui::{Vec2, Widget};

use crate::{
    custom_widget::CustomWidget, default_port::DefaultPort, id::Id, plug::DraggedPlug,
    port_params::PortParams, state::State,
};

pub type PortId = Id;

#[derive(Debug)]
pub struct Port {
    port_id: PortId,
    widget: Option<CustomWidget>,
}

impl Port {
    pub fn new<T: Hash + Eq + Debug + Send + Sync + 'static>(port_id: T) -> Self {
        Port {
            port_id: PortId::new(port_id),
            widget: None,
        }
    }

    pub fn widget(mut self, widget: impl Into<CustomWidget>) -> Self {
        self.widget = Some(widget.into());
        self
    }
}

impl Widget for Port {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // This widget is not need to use egui::Area

        let mut state = State::get_cloned(ui);

        // Render port with params
        PortParams {
            hovered: state.hovered_port_id() == Some(self.port_id.clone()),
        }
        .set(ui);
        let response = self.widget.unwrap_or_else(|| DefaultPort.into()).ui(ui);

        // advance generation if this port is rendered twice
        state.advance_generation_if_twice(self.port_id.clone());
        // update port's position used for plug rendering
        state.update_port_pos(self.port_id.clone(), response.rect.left_top());

        let dragged_plug = state.dragged_plug().unwrap_or(DraggedPlug {
            pos: egui::pos2(-100.0, -100.0), // far
            size: Vec2::ZERO,
        });

        // distance between the port and the dragged plug
        let distance_sq = response.rect.center().distance_sq(dragged_plug.pos);
        let min_length = |vec: Vec2| vec.x.min(vec.y);
        let close_distance =
            (min_length(response.rect.size()) + min_length(dragged_plug.size)) / 2.0;

        // distance required because `response.hovered()` always returns false when plug is interacted
        let hovered = response.hovered() || distance_sq < close_distance.powi(2);

        // update hovered port id used for cable connection
        if hovered {
            state.update_hovered_port_id(self.port_id);
        }

        // finally update the state
        state.store_to(ui);

        response
    }
}
