use egui::{Id, Order, Pos2, Rect, Sense, Vec2, Widget};

use crate::{
    cable::CableId,
    event::Event,
    state::State,
    utils::{widget_visuals, SIZE},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    // inserted by Cable widget
    cable_active: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct DraggedPlug {
    pub pos: Pos2,
    pub size: Vec2,
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

    pub(crate) fn cable_active(mut self, active: bool) -> Self {
        self.cable_active = active;
        self
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PlugState {
    pos: Pos2,
    dragged: bool,
}

impl Widget for Plug {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let id = self.id.unwrap();
        let mut state = State::get_cloned(ui.data());
        let mut plug_state = state.plug_state(&id).unwrap_or(PlugState {
            pos: self.default_pos.unwrap(),
            dragged: false,
        });
        plug_state.pos = if plug_state.dragged {
            plug_state.pos
        } else {
            self.plug_to
                .and_then(|port_id| state.port_pos(&port_id))
                // If port is not displayed, use saved plug pos
                .unwrap_or(plug_state.pos)
        };
        let order = if self.cable_active {
            // Make active plug be interactive
            Order::Foreground
        } else {
            // Make port which is foreground be interactive
            Order::Middle
        };
        egui::Area::new(id.clone())
            // must be top-left of the widget
            .current_pos(plug_state.pos)
            // should be displayed on foreground
            .order(order)
            .show(ui.ctx(), |ui| {
                let response = if self.plug_to.is_some() && !self.cable_active {
                    // minimum sense because it is not interactive
                    let (_rect, response) = ui.allocate_exact_size(SIZE, Sense::hover());
                    response
                } else {
                    let response = ui.allocate_rect(
                        Rect::from_two_pos(plug_state.pos, plug_state.pos + SIZE),
                        Sense::drag(),
                    );
                    let size = response.rect.size();

                    plug_state.pos += response.drag_delta();

                    let center_pos = plug_state.pos + size / 2.0;

                    // Update plug pos used for determining a port is hovered by plug
                    plug_state.dragged = response.dragged();
                    if plug_state.dragged {
                        state.update_dragged_plug(DraggedPlug {
                            pos: center_pos,
                            size: response.rect.size(),
                        });
                    }

                    if response.drag_released() {
                        match (self.plug_to, state.hovered_port_id()) {
                            // Connect event
                            (_, Some(port_id)) => {
                                state
                                    .ephemeral
                                    .event_of_plug
                                    .insert(response.id, Event::Connected { port_id });
                            }
                            // Disconnect event
                            (Some(_), None) => {
                                state
                                    .ephemeral
                                    .event_of_plug
                                    .insert(response.id, Event::Disconnected);
                            }
                            _ => {}
                        }
                    }

                    // paint circles
                    let visuals = widget_visuals(ui, &response);
                    ui.painter().add(epaint::CircleShape {
                        center: center_pos,
                        radius: size.x / 2.0,
                        fill: visuals.bg_fill,
                        stroke: visuals.fg_stroke,
                    });

                    response
                };

                let visuals = widget_visuals(ui, &response);
                ui.painter().add(epaint::CircleShape {
                    center: response.rect.center(),
                    radius: response.rect.size().x / 2.0 * 0.3,
                    fill: visuals.fg_stroke.color,
                    stroke: visuals.fg_stroke,
                });

                state.update_plug_state(id.clone(), plug_state);
                state.store_to(ui.data());

                response
            })
            .inner
    }
}
