use egui::{pos2, vec2, Id, Order, Pos2, Rect, Sense, Widget};

use crate::{cable::CableId, event::Event, state::State, utils::widget_visuals};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlugType {
    In,
    Out,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PlugId {
    cable_id: CableId,
    plug_type: PlugType,
}

impl PlugId {
    pub fn new(cable_id: CableId, plug_type: PlugType) -> Self {
        PlugId {
            cable_id,
            plug_type,
        }
    }
}

#[derive(Debug, Default)]
pub struct Plug {
    pub plug_to: Option<Id>,
    pos: Option<Pos2>,
    // inserted by Cable widget
    id: Option<PlugId>,
    // inserted by Cable widget
    default_pos: Option<Pos2>,
}

impl Plug {
    pub fn new() -> Self {
        Plug::default()
    }

    pub fn plug_to(port: Id) -> Self {
        Plug {
            plug_to: Some(port),
            ..Default::default()
        }
    }

    pub(crate) fn default_pos(mut self, pos: Pos2) -> Self {
        self.default_pos = Some(pos);
        self
    }

    pub fn pos(mut self, pos: Pos2) -> Self {
        self.pos = Some(pos);
        self
    }

    pub(crate) fn id(mut self, id: PlugId) -> Self {
        self.id = Some(id);
        self
    }
}

impl Widget for Plug {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let id = self.id.unwrap();
        let mut state = State::get_cloned(ui.data());
        let size = 12.0;
        let mut pos = if let Some(port_id) = &self.plug_to {
            state.port_pos(port_id).unwrap_or(pos2(0.0, 0.0))
        } else {
            state
                .plug_pos(&id)
                .unwrap_or_else(|| self.default_pos.unwrap())
        };
        egui::Area::new(id.clone())
            .current_pos(pos - vec2(size / 2.0, size / 2.0))
            .order(Order::Foreground)
            .show(ui.ctx(), |ui| {
                let response = if self.plug_to.is_some() {
                    ui.allocate_rect(
                        Rect::from_center_size(pos, vec2(size, size)),
                        Sense::click(),
                    )
                } else {
                    let response = ui.allocate_rect(
                        Rect::from_center_size(pos, vec2(size, size)),
                        Sense::drag(),
                    );
                    pos += response.drag_delta();
                    state.update_plug_pos(id.clone(), pos);
                    if response.drag_released() {
                        if let Some(port_id) = state.hovered_port_id() {
                            state.ephemeral.event.insert(
                                id.cable_id,
                                Event::Connected {
                                    plug_type: id.plug_type,
                                    port_id,
                                },
                            );
                        }
                    }
                    let visuals = widget_visuals(ui, &response);
                    ui.painter().add(epaint::CircleShape {
                        center: pos,
                        radius: size / 2.0,
                        fill: visuals.bg_fill,
                        stroke: visuals.fg_stroke,
                    });
                    ui.painter().add(epaint::CircleShape {
                        center: pos,
                        radius: size / 4.0,
                        fill: visuals.bg_fill,
                        stroke: visuals.fg_stroke,
                    });
                    response
                };
                state.store(ui);
                response
            })
            .inner
    }
}
