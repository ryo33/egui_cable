use std::fmt::Debug;
use std::hash::Hash;

use egui::{pos2, vec2, Id, Order, Pos2, Vec2, Widget};
use epaint::{Color32, QuadraticBezierShape};

use crate::{
    cable_control::CableControl,
    custom_widget::CustomWidget,
    default_cable::{DefaultCable, DefaultControl},
    plug::{PlugId, PlugType},
    prelude::*,
    state::State,
    utils::FAR,
};

pub type CableId = Id;

#[derive(Debug)]
pub struct Cable {
    pub id: CableId,
    in_plug: Plug,
    out_plug: Plug,
    widget: Option<CustomWidget>,
    control_widget: Option<CustomWidget>,
}

impl Cable {
    pub fn new<T: Debug + Eq + Hash + Send + Sync + 'static>(
        id: T,
        in_plug: Plug,
        out_plug: Plug,
    ) -> Self {
        Cable {
            id: CableId::new(id),
            in_plug,
            out_plug,
            widget: None,
            control_widget: None,
        }
    }

    pub fn widget(mut self, widget: impl Into<CustomWidget>) -> Self {
        self.widget = Some(widget.into());
        self
    }

    pub fn control_widget(mut self, widget: impl Into<CustomWidget>) -> Self {
        self.control_widget = Some(widget.into());
        self
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CableState {
    pub bezier_control_point_offset: Vec2,
    pub dragged: bool,
    pub drag_offset: Vec2,
    pub active: bool,
    pub in_vec: Option<Vec2>,
    pub out_vec: Option<Vec2>,
}

impl Default for CableState {
    fn default() -> Self {
        Self {
            // Default is not zero to make sophisticated cable view.
            bezier_control_point_offset: vec2(20.0, 25.0),
            active: false,
            dragged: false,
            drag_offset: vec2(0.0, 0.0),
            in_vec: None,
            out_vec: None,
        }
    }
}

impl Widget for Cable {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let available_rect = ui.available_rect_before_wrap();
        egui::Area::new(self.id)
            .order(Order::Foreground)
            // This should be the top-left of the window
            .current_pos(pos2(0.0, 0.0))
            // This is important to make other widgets intractive even when behind a cable
            .interactable(false)
            .show(ui.ctx(), |ui| {
                let mut cable_state = State::get(ui.data())
                    .cable_state(&self.id)
                    .unwrap_or_default();

                // fixme? This could be more smart.
                let default_in_pos = available_rect.left_top() + vec2(10.0, 0.0);
                let default_out_pos = available_rect.left_top() + vec2(50.0, 0.0);

                let in_response = ui.add(
                    self.in_plug
                        .id(PlugId::new(self.id, PlugType::In))
                        .default_pos(default_in_pos)
                        .cable_active(cable_state.active)
                        .vec(cable_state.in_vec),
                );
                let out_response = ui.add(
                    self.out_plug
                        .id(PlugId::new(self.id, PlugType::Out))
                        .default_pos(default_out_pos)
                        .cable_active(cable_state.active)
                        .vec(cable_state.out_vec),
                );

                // Given positions
                let in_pos = in_response.rect.center();
                let out_pos = out_response.rect.center();
                let midpoint = (in_pos.to_vec2() + out_pos.to_vec2()) / 2.0;

                let bezier_control_pos =
                    (midpoint + cable_state.bezier_control_point_offset).to_pos2();

                let bezier = QuadraticBezierShape::from_points_stroke(
                    [in_pos, bezier_control_pos, out_pos],
                    false,
                    Color32::TRANSPARENT,
                    // a dummy value overwriten later
                    (1.0, Color32::BLACK),
                );

                let pointer_pos = ui.input().pointer.interact_pos();
                let is_close = bezier_close(&bezier, pointer_pos.unwrap_or(FAR), 300.0);

                let line_hovered = is_close || cable_state.dragged;

                let plugs_interacted = in_response.hovered()
                    || in_response.dragged()
                    || out_response.hovered()
                    || out_response.dragged();

                let cable_control_pos = bezier.sample(0.5);

                CableParams {
                    active: cable_state.active,
                    line_hovered,
                    plugs_interacted,
                    cable_control: CableControl {
                        id: self.id,
                        pos: cable_control_pos,
                        widget: self.control_widget.unwrap_or_else(|| DefaultControl.into()),
                    },
                    bezier,
                }
                .set(ui.data());
                let response = self.widget.unwrap_or_else(|| DefaultCable.into()).ui(ui);

                if response.drag_started() {
                    cable_state.dragged = true;
                    // drag_diff is used to prevent cable from jumping when cable is dragged.
                    if let Some(origin) = ui.input().pointer.press_origin() {
                        cable_state.drag_offset = cable_control_pos - origin;
                    } else {
                        // rare case
                        cable_state.drag_offset = vec2(0.0, 0.0);
                    }
                }
                if response.drag_released() {
                    cable_state.dragged = false;
                }
                if response.dragged() {
                    if let Some(pointer_pos) = ui.input().pointer.interact_pos() {
                        // use drag_diff for prevent cable from jumping on click.
                        cable_state.bezier_control_point_offset +=
                            pointer_pos + cable_state.drag_offset - cable_control_pos;
                    }
                }

                // active or not
                if response.clicked() {
                    cable_state.active = true;
                }
                if response.clicked_elsewhere() {
                    cable_state.active = false;
                }

                // update plug vec state for rendering the plug
                if in_response.dragged() {
                    cable_state.in_vec =
                        Some((bezier.sample(0.0) - bezier.sample(0.05)).normalized());
                }
                if out_response.dragged() {
                    cable_state.out_vec =
                        Some((bezier.sample(1.0) - bezier.sample(0.95)).normalized());
                }

                // This must be after ui.add(plug) because state might be modified.
                let mut state = State::get_cloned(ui.data());

                // this id is used in ResponseExt
                state
                    .ephemeral
                    .plug_responces_of_cable
                    .insert(response.id, (in_response, out_response));

                // finally update the states
                state.update_cable_state(self.id, cable_state);
                state.store_to(ui.data());

                response
            })
            .inner
    }
}

fn bezier_close(bezier: &QuadraticBezierShape, pointer_pos: Pos2, distance_sq: f32) -> bool {
    let distance1 = bezier.points[0].distance(bezier.points[1]);
    let distance2 = bezier.points[1].distance(bezier.points[2]);
    let count = distance1 + distance2;
    (0..count as usize)
        .into_iter()
        .map(|t| bezier.sample(t as f32 / count))
        .any(|point| (point - pointer_pos).length_sq() < distance_sq)
}

#[cfg(test)]
mod tests {
    use epaint::Color32;

    use super::*;

    #[test]
    fn test_bezier_close() {
        let bezier = QuadraticBezierShape::from_points_stroke(
            [pos2(0.0, 0.0), pos2(0.0, 20.0), pos2(20.0, 20.0)],
            false,
            Color32::WHITE,
            (1.0, Color32::BLACK),
        );
        assert!(!bezier_close(&bezier, pos2(10.0, 10.0), 5.0));
        assert!(bezier_close(&bezier, pos2(10.0, 18.0), 5.0));
    }
}
