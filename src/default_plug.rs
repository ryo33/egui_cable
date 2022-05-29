use egui::{Rect, Response, Sense, Ui, Widget};
use epaint::Stroke;

use crate::{
    plug_params::PlugParams,
    utils::{widget_visuals, SIZE},
};

#[derive(Debug)]
pub struct DefaultPlug;

impl Widget for DefaultPlug {
    fn ui(self, ui: &mut Ui) -> Response {
        let params = PlugParams::get(ui.data());
        let vector = params.vector;
        let active = params.active;
        let plugged = params.plugged;
        let locked = params.locked;

        let (rect, response) = ui.allocate_exact_size(
            SIZE,
            if (active || !plugged) && !locked {
                Sense::drag()
            } else {
                // minimum sense to make not interactive
                Sense::hover()
            },
        );

        // handle drag
        let pos = rect.left_top() + response.drag_delta();
        let size = rect.size();
        // this should not be response.rect.center() for painting it correctly while dragging
        let center_pos = pos + size / 2.0;

        if ui.is_rect_visible(Rect::from_center_size(center_pos, size)) {
            let visuals = if locked {
                ui.visuals().widgets.noninteractive
            } else {
                widget_visuals(ui, &response)
            };
            if response.dragged() {
                if let Some(vector) = vector {
                    ui.painter().arrow(
                        center_pos,
                        vector * size.x / 2.0 * 1.5,
                        Stroke::new(2.0, visuals.fg_stroke.color),
                    );
                }
            }
            ui.painter().add(epaint::CircleShape {
                center: center_pos,
                radius: size.x / 2.0 * if active { 0.7 } else { 0.2 },
                fill: visuals.fg_stroke.color,
                stroke: visuals.fg_stroke,
            });
        }

        response
    }
}
