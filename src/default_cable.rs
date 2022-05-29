use egui::{Rect, Response, Sense, Ui, Widget};

use crate::{
    prelude::CableParams,
    utils::{widget_visuals, FAR, SIZE},
};

// This is not public because CableParams::get cannot be used twice.
#[derive(Debug)]
pub(crate) struct DefaultCable;

impl Widget for DefaultCable {
    fn ui(self, ui: &mut Ui) -> Response {
        let params = CableParams::get(ui.data());
        let active = params.active;
        let line_hovered = params.line_hovered;
        let plugs_interacted = params.plugs_interacted;
        let cable_control = params.cable_control;
        let mut bezier = params.bezier;
        let cable_control_pos = cable_control.pos;

        // We don't want to show cable control when cable is not hovered or a plug is interacted.
        let response = if line_hovered && !plugs_interacted {
            ui.add(cable_control)
        } else {
            // allocate empty space
            ui.allocate_rect(
                Rect::from_two_pos(FAR, FAR),
                Sense::focusable_noninteractive(),
            )
        };

        let in_pos = bezier.points[0];
        let out_pos = bezier.points[2];

        if ui.is_rect_visible(Rect::from_two_pos(in_pos, out_pos)) {
            // visual of bezier curve
            let cable_visual = if active {
                ui.visuals().widgets.active
            } else {
                widget_visuals(ui, &response)
            };
            bezier.stroke = cable_visual.fg_stroke;

            // paint bezier curve or circle
            if in_pos == out_pos {
                // If loop, draw circle.
                let center = Rect::from_two_pos(in_pos, cable_control_pos).center();
                ui.painter().circle_stroke(
                    center,
                    cable_control_pos.distance(in_pos) / 2.0,
                    cable_visual.fg_stroke,
                )
            } else {
                ui.painter().add(bezier);
            }
        }

        response
    }
}

#[derive(Debug)]
pub struct DefaultControl;

impl Widget for DefaultControl {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(SIZE, Sense::click_and_drag());
        if ui.is_rect_visible(rect) {
            let visuals = widget_visuals(ui, &response);
            ui.painter()
                .rect(rect, 3.0, visuals.bg_fill, visuals.fg_stroke);
        }
        response
    }
}
