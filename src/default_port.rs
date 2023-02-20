use egui::{Response, Sense, Ui, Widget};

use crate::{
    port_params::PortParams,
    utils::{widget_visuals, SIZE},
};

#[derive(Debug)]
pub struct DefaultPort;

impl Widget for DefaultPort {
    fn ui(self, ui: &mut Ui) -> Response {
        let params = PortParams::get(ui);
        let hovered = params.hovered;

        let (rect, response) = ui.allocate_exact_size(SIZE, Sense::click());

        if ui.is_rect_visible(rect) {
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
        }

        response
    }
}
