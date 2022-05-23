use std::fmt::Debug;
use std::hash::Hash;

use egui::{Id, Painter, Rect, Sense, Widget};

use crate::{
    plug::{PlugId, PlugType},
    prelude::*,
};

pub type CableId = Id;

#[derive(Debug)]
pub struct Cable {
    pub id: CableId,
    in_plug: Plug,
    out_plug: Plug,
}

impl Cable {
    pub fn new<T: Debug + Eq + Hash + Send + Sync + 'static>(
        id: T,
        in_plug: impl Into<Plug>,
        out_plug: impl Into<Plug>,
    ) -> Self {
        Cable {
            id: CableId::new(id),
            in_plug: in_plug.into(),
            out_plug: out_plug.into(),
        }
    }
}

impl From<Id> for Plug {
    fn from(port_id: Id) -> Self {
        Plug::plug_to(port_id)
    }
}

impl Widget for Cable {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let in_response = ui.add(self.in_plug.id(PlugId::new(self.id.clone(), PlugType::In)));
        let out_response = ui.add(
            self.out_plug
                .id(PlugId::new(self.id.clone(), PlugType::Out)),
        );

        let in_pos = in_response.rect.center();
        let out_pos = out_response.rect.center();
        let response = ui.allocate_rect(Rect::from_two_pos(in_pos, out_pos), Sense::drag());
        let painter = Painter::new(ui.ctx().clone(), response.layer_id, Rect::EVERYTHING);
        painter.line_segment([in_pos, out_pos], ui.visuals().widgets.active.fg_stroke);

        // TODO
        in_response.union(out_response)
    }
}
