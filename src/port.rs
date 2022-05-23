use std::{any::Any, sync::Arc};

use egui::{vec2, Id, Sense, Widget};

use crate::state::State;

pub type PortId = Id;

#[derive(Debug)]
pub struct Port {
    data: Arc<dyn Any>,
    port_id: PortId,
}

impl Port {
    pub fn new<T: 'static>(port_id: PortId, data: T) -> Self {
        Port {
            data: Arc::new(data),
            port_id,
        }
    }

    pub fn unit(port_id: PortId) -> Self {
        Self::new(port_id, ())
    }

    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.data.downcast_ref()
    }
}

impl Widget for Port {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut state: State = State::clone(
            &ui.data()
                .get_persisted::<Arc<State>>(Id::null())
                .unwrap_or_default(),
        );
        let size = 12.0;
        let (rect, response) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
        let mut visuals = ui.visuals().widgets.active;
        let pos = response.rect.center();
        state.update_port_pos(self.port_id.clone(), pos.clone());
        if response.hovered {
            visuals = ui.visuals().widgets.hovered;
        }
        if ui.rect_contains_pointer(response.rect) {
            state.update_hovered_port_id(self.port_id);
        }
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
