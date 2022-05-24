use egui::{vec2, Pos2, Rect, Sense, Widget};

pub(crate) struct CableLabel {
    pub pos: Pos2,
}

impl Widget for CableLabel {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let response = ui.allocate_rect(
            Rect::from_center_size(self.pos, vec2(20.0, 10.0)),
            Sense::click(),
        );
		response
    }
}
