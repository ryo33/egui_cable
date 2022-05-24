use std::fmt::Debug;
use std::hash::Hash;

use egui::{vec2, Id, Sense, Widget};

use crate::{state::State, utils::visual};

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
        let mut state = State::get_cloned(ui.data());
        let size = 12.0;
        let (rect, response) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
        let pos = response.rect.center();
        state.update_port_pos(self.port_id, pos);
        if response.hovered {
            state.update_hovered_port_id(self.port_id);
        }
        let visuals = visual(ui, &response);
        ui.painter().add(epaint::CircleShape {
            center: rect.center(),
            radius: rect.height() / 2.0,
            fill: visuals.bg_fill,
            stroke: visuals.fg_stroke,
        });
        state.store(ui);
        response
    }
}
