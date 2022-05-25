use egui::{vec2, Order, Painter, Pos2, Rect, Sense, Widget};

use crate::{cable::CableId, state::State, utils::widget_visuals};

pub(crate) struct CableControl {
    pub id: CableId,
    pub pos: Pos2,
}

impl Widget for CableControl {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut state = State::get_cloned(ui.data());
        let size = state
            .cable_control_size(&self.id)
            .unwrap_or(vec2(20.0, 20.0));
        egui::Area::new((self.id, "cable_control"))
            // must be top-left of the widget
            .current_pos(self.pos - size / 2.0)
            // should be displayed on foreground
            .order(Order::Foreground)
            .show(ui.ctx(), |ui| {
                let size = 20.0;
                let (rect, response) =
                    ui.allocate_exact_size(vec2(size, size), Sense::click_and_drag());

                // update cable control size for calculate the next position of this area
                state.update_cable_control_size(self.id, rect.size());
                state.store_to(ui.data());

                let painter = Painter::new(ui.ctx().clone(), ui.layer_id(), Rect::EVERYTHING);
                let visuals = widget_visuals(ui, &response);
                painter.rect(response.rect, 3.0, visuals.bg_fill, visuals.fg_stroke);
                response
            })
            .inner
    }
}
