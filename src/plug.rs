use egui::{vec2, Id, Order, Pos2, Rect, Sense, Vec2, Widget};
use epaint::Stroke;

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
    // inserted by Cable widget
    vec: Option<Vec2>,
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

    pub(crate) fn vec(mut self, vec: Option<Vec2>) -> Self {
        self.vec = vec;
        self
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PlugState {
    pos_offset: Vec2,
    dragged: bool,
}

impl Widget for Plug {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // safe unwrap
        let id = self.id.unwrap();
        let default_pos = self.default_pos.unwrap();

        let mut state = State::get_cloned(ui.data());
        let mut plug_state = state.plug_state(&id).unwrap_or(PlugState {
            pos_offset: vec2(0.0, 0.0),
            dragged: false,
        });
        let get_pos = || default_pos + plug_state.pos_offset;
        let mut pos = if plug_state.dragged {
            get_pos()
        } else {
            self.plug_to
                .and_then(|port_id| state.port_pos(&port_id))
                // If port is not displayed, use saved plug pos
                .unwrap_or_else(get_pos)
        };
        let order = if self.cable_active {
            // Make active plug be interactive
            Order::Debug
        } else {
            // Make port which is foreground be interactive
            Order::Foreground
        };
        let mut center_pos = None;
        egui::Area::new(id.clone())
            // must be top-left of the widget
            .current_pos(pos)
            // should be displayed on foreground
            .order(order)
            .show(ui.ctx(), |ui| {
                let response = if self.plug_to.is_some() && !self.cable_active {
                    // minimum sense because it is not interactive
                    let (_rect, response) = ui.allocate_exact_size(SIZE, Sense::hover());
                    response
                } else {
                    let response =
                        ui.allocate_rect(Rect::from_two_pos(pos, pos + SIZE), Sense::drag());
                    let size = response.rect.size();

                    // handle drag
                    pos += response.drag_delta();

                    // this should not be response.rect.center_size for painting it correctly
                    center_pos = Some(pos + size / 2.0);

                    // Update plug pos used for determining a port is hovered by plug
                    plug_state.dragged = response.dragged();
                    if plug_state.dragged {
                        state.update_dragged_plug(DraggedPlug {
                            pos: center_pos.unwrap(),
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

                    response
                };

                let center_pos = center_pos.unwrap_or_else(|| response.rect.center());
                let size = response.rect.size();
                let visuals = widget_visuals(ui, &response);
                if let Some(vec) = self.vec {
                    if response.dragged() {
                        ui.painter().arrow(
                            center_pos,
                            vec * size.x / 2.0 * 1.5,
                            Stroke::new(2.0, visuals.fg_stroke.color),
                        );
                    }
                }
                ui.painter().add(epaint::CircleShape {
                    center: center_pos,
                    radius: size.x / 2.0,
                    fill: visuals.bg_fill,
                    stroke: visuals.fg_stroke,
                });
                ui.painter().add(epaint::CircleShape {
                    center: center_pos,
                    radius: response.rect.size().x / 2.0 * 0.3,
                    fill: visuals.fg_stroke.color,
                    stroke: visuals.fg_stroke,
                });

                // finally store states
                plug_state.pos_offset = pos - default_pos;
                state.update_plug_state(id.clone(), plug_state);
                state.store_to(ui.data());

                response
            })
            .inner
    }
}
