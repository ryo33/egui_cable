use egui::{Order, Pos2, Widget};

use crate::{cable::CableId, custom_widget::CustomWidget, state::State, utils::FAR};

#[derive(Debug)]
pub struct CableControl {
    pub(crate) id: CableId,
    pub(crate) pos: Pos2,
    pub(crate) widget: CustomWidget,
}

impl Widget for CableControl {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut state = State::get_cloned(ui.data());
        let size = state.cable_control_size(&self.id);
        egui::Area::new((self.id, "cable_control"))
            // must be top-left of the widget
            .current_pos(if let Some(size) = size {
                self.pos - size / 2.0
            } else {
                // Don't render the widget if the size is not available
                FAR
            })
            // should be displayed on cable bezier
            .order(Order::Debug)
            .show(ui.ctx(), |ui| {
                // cable control has click sense for make cable active, and drag sense for bezier deforming.
                let response = self.widget.ui(ui);

                // update cable control size for calculate the next position of this area
                state.update_cable_control_size(self.id, response.rect.size());
                state.store_to(ui.data());

                response
            })
            .inner
    }
}
