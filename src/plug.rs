use std::fmt::Debug;
use std::hash::Hash;

use egui::{vec2, Order, Pos2, Vec2, Widget};

use crate::{
    cable::CableId, custom_widget::CustomWidget, default_plug::DefaultPlug, event::Event,
    plug_params::PlugParams, prelude::PortId, state::State,
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
    pub plug_to: Option<PortId>,
    pos: Option<Pos2>,
    // inserted by Cable widget
    id: Option<PlugId>,
    // inserted by Cable widget
    default_pos: Option<Pos2>,
    // inserted by Cable widget
    cable_active: bool,
    // inserted by Cable widget
    vec: Option<Vec2>,
    widget: Option<CustomWidget>,
}

#[derive(Debug, Clone)]
pub(crate) struct DraggedPlug {
    pub pos: Pos2,
    pub size: Vec2,
}

impl Plug {
    pub fn unplugged() -> Self {
        Plug::default()
    }

    pub fn to<T: Hash + Eq + Debug + Send + Sync + 'static>(port: T) -> Self {
        Plug {
            plug_to: Some(PortId::new(port)),
            ..Default::default()
        }
    }

    pub fn pos(mut self, pos: Pos2) -> Self {
        self.pos = Some(pos);
        self
    }

    pub fn widget(mut self, widget: impl Into<CustomWidget>) -> Self {
        self.widget = Some(widget.into());
        self
    }

    pub fn default_pos(mut self, pos: Pos2) -> Self {
        self.default_pos = Some(pos);
        self
    }

    // used by cable
    pub(crate) fn default_pos_no_overwrite(mut self, pos: Pos2) -> Self {
        if self.default_pos.is_none() {
            self.default_pos = Some(pos);
        }
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
        let get_pos = || {
            if let Some(pos) = self.pos {
                pos
            } else {
                default_pos + plug_state.pos_offset
            }
        };
        let mut pos = if plug_state.dragged {
            get_pos()
        } else {
            self.plug_to
                .as_ref()
                .and_then(|port_id| state.port_pos(port_id))
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
        egui::Area::new(id.clone())
            // must be top-left of the widget
            .current_pos(pos)
            // should be displayed on foreground
            .order(order)
            .show(ui.ctx(), |ui| {
                // render plug with params
                PlugParams {
                    vector: self.vec,
                    active: self.cable_active || self.plug_to.is_none(),
                }
                .set(ui.data());
                let response = self.widget.unwrap_or_else(|| DefaultPlug.into()).ui(ui);

                let size = response.rect.size();

                // handle drag
                pos += response.drag_delta();

                // this should not be response.rect.center_size for painting it correctly
                let center_pos = pos + size / 2.0;

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

                // finally store states
                plug_state.pos_offset = pos - default_pos;
                state.update_plug_state(id.clone(), plug_state);
                state.store_to(ui.data());

                response
            })
            .inner
    }
}
