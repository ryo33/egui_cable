use egui::{vec2, Order, Painter, Pos2, Rect, Sense, Widget};

use crate::{cable::CableId, utils::visual};

pub(crate) struct CableControl {
    pub id: CableId,
    pub pos: Pos2,
}

impl Widget for CableControl {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let size = 20.0;
        egui::Area::new((self.id, "cable_control"))
            .current_pos(self.pos - vec2(size / 2.0, size / 2.0))
            .order(Order::Foreground)
            .show(ui.ctx(), |ui| {
                let response = ui.allocate_rect(
                    Rect::from_center_size(self.pos, vec2(size, size)),
                    Sense::drag(),
                );
                let painter = Painter::new(ui.ctx().clone(), ui.layer_id(), Rect::EVERYTHING);
                let visuals = visual(ui, &response);
                painter.rect(response.rect, 3.0, visuals.bg_fill, visuals.fg_stroke);
                response
            })
            .inner
    }
}
