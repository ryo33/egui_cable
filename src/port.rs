use std::fmt::Debug;
use std::hash::Hash;

use egui::{vec2, Id, Sense, Widget};

use crate::{
    state::State,
    utils::{widget_visuals, FAR},
};

pub type PortId = Id;

#[derive(Debug)]
pub struct Port {
    port_id: PortId,
}

impl Port {
    pub fn new<T: Hash + Eq + Debug + Send + Sync + 'static>(port_id: T) -> Self {
        Port {
            port_id: PortId::new(port_id),
        }
    }
}

impl Widget for Port {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // This widget is not need to use egui::Area

        let mut state = State::get_cloned(ui.data());
        let size = 12.0;
        let (rect, response) = ui.allocate_exact_size(vec2(size, size), Sense::hover());

        // advance generation if this port is rendered twice
        state.advance_generation_if_twice(self.port_id);
        // update port's position used for plug rendering
        state.update_port_pos(self.port_id, response.rect.left_top());

        // distance between the port and the dragged plug
        let distance_sq = response
            .rect
            .center()
            .distance_sq(state.dragged_plug().unwrap_or(FAR));

        // distance required because `response.hovered()` always returns false when plug is interacted
        let hovered = response.hovered() || distance_sq < size.powi(2);

        // update hovered port id used for cable connection
        if hovered {
            state.update_hovered_port_id(self.port_id);
        }

        // paint the port
        let visuals = if hovered {
            ui.visuals().widgets.hovered
        } else {
            widget_visuals(ui, &response)
        };
        ui.painter().add(epaint::CircleShape {
            center: rect.center(),
            radius: rect.height() / 2.0,
            fill: visuals.bg_fill,
            stroke: visuals.fg_stroke,
        });

        // finally update the state
        state.store(ui);

        response
    }
}
