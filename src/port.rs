use egui::{vec2, Id, Pos2, Sense, Widget};

use crate::{
    cable::{Cable, CableTo},
    port_id::PortId,
    state::State,
};

#[derive(Debug)]
pub struct Port {
    port_id: PortId,
    cables: Vec<CableTo>,
}

impl Port {
    pub fn new(port_id: PortId) -> Self {
        Port {
            port_id,
            cables: Vec::new(),
        }
    }
    pub fn with_cable(mut self, to: PortId, cable: Cable) -> Self {
        self.cables.push(CableTo { cable, to });
        self
    }
}

impl Widget for Port {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut state = ui
            .data()
            .get_persisted::<State>(Id::null())
            .unwrap_or_default();
        let size = 12.0;
        let (rect, response) = ui.allocate_exact_size(vec2(size, size), Sense::drag());
        let mut visuals = ui.visuals().widgets.active;
        let pos = response.rect.center();
        state.update_port_pos(self.port_id.clone(), pos.clone());
        if response.hovered {
            visuals = ui.visuals().widgets.hovered;
        }
        ui.painter().add(epaint::CircleShape {
            center: rect.center(),
            radius: rect.height() / 2.0,
            fill: visuals.bg_fill,
            stroke: visuals.fg_stroke,
        });

        if response.drag_started() {
            state.drag_start_port = Some(self.port_id.clone());
        }
        if response.dragged() {
            let cursor_pos = ui.input().pointer.hover_pos().unwrap_or(Pos2::ZERO);
            ui.painter()
                .line_segment([pos.clone(), cursor_pos], visuals.fg_stroke);
        }
        println!("begin {:?}", self.port_id);
        for cable in &self.cables {
            if let Some(to) = state.port_pos(&cable.to) {
                ui.painter()
                    .line_segment([pos, to.clone()], visuals.fg_stroke);
            }
        }
        println!("end {:?}", self.port_id);
        ui.data().insert_persisted(Id::null(), state);
        response
    }
}
