use std::{collections::HashMap, sync::Arc};

use egui::{Id, Pos2, Ui};

use crate::{plug::PlugId, prelude::*};

#[derive(Default, Clone, Debug)]
pub(crate) struct State {
    previous: GenerationState,
    current: GenerationState,
}

#[derive(Default, Clone, Debug)]
pub(crate) struct GenerationState {
    port_pos: HashMap<PortId, Pos2>,
    hovered_port_id: Option<PortId>,
    plug_pos: HashMap<PlugId, Pos2>,
}

impl State {
    pub fn next_generation(&mut self) {
        std::mem::swap(&mut self.previous, &mut self.current);
        self.current = Default::default();
    }

    pub fn update_port_pos(&mut self, port_id: PortId, pos: Pos2) {
        if self.current.port_pos.contains_key(&port_id) {
            self.next_generation();
        }
        self.current.port_pos.insert(port_id, pos);
    }

    pub fn port_pos(&self, port_id: &PortId) -> Option<&Pos2> {
        self.current
            .port_pos
            .get(port_id)
            .or_else(|| self.previous.port_pos.get(port_id))
    }

    pub fn update_hovered_port_id(&mut self, port_id: PortId) {
        self.current.hovered_port_id = Some(port_id);
    }

    pub fn hovered_port_id(&self) -> Option<PortId> {
        self.current
            .hovered_port_id
            .as_ref()
            .or_else(|| self.previous.hovered_port_id.as_ref())
            .cloned()
    }

    pub fn update_plug_pos(&mut self, id: PlugId, pos: Pos2) {
        self.current.plug_pos.insert(id, pos);
    }

    pub fn plug_pos(&self, id: &PlugId) -> Option<Pos2> {
        self.current
            .plug_pos
            .get(id)
            .or_else(|| self.previous.plug_pos.get(id))
            .cloned()
    }

    pub fn store(self, ui: &mut Ui) {
        ui.data().insert_persisted(Id::null(), Arc::new(self));
    }
}

#[cfg(test)]
mod tests {
    use crate::{cable::CableId, plug::PlugType};

    use super::*;

    #[test]
    fn update_port_pos_inserts_current_port_pos() {
        let mut state = State::default();
        state.update_port_pos(PortId::new(1), Pos2::new(1.0, 2.0));
        state.update_port_pos(PortId::new(2), Pos2::new(2.0, 2.0));
        assert_eq!(
            state.current.port_pos,
            [
                (PortId::new(1), Pos2::new(1.0, 2.0)),
                (PortId::new(2), Pos2::new(2.0, 2.0))
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(state.previous.port_pos, [].into_iter().collect());
    }

    #[test]
    fn update_port_pos_use_previous_port_pos() {
        let mut state = State::default();
        state.update_port_pos(PortId::new(1), Pos2::new(1.0, 2.0));
        state.update_port_pos(PortId::new(2), Pos2::new(2.0, 2.0));
        state.update_port_pos(PortId::new(3), Pos2::new(3.0, 2.0));
        assert_eq!(state.previous.port_pos.len(), 0);
        state.update_port_pos(PortId::new(1), Pos2::new(1.0, 3.0));

        assert_eq!(
            state.current.port_pos,
            [(PortId::new(1), Pos2::new(1.0, 3.0)),]
                .into_iter()
                .collect()
        );
        assert_eq!(
            state.previous.port_pos,
            [
                (PortId::new(1), Pos2::new(1.0, 2.0)),
                (PortId::new(2), Pos2::new(2.0, 2.0)),
                (PortId::new(3), Pos2::new(3.0, 2.0)),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn port_pos() {
        let mut state = State::default();
        // first gen
        state.update_port_pos(PortId::new(1), Pos2::new(1.0, 2.0));
        state.update_port_pos(PortId::new(2), Pos2::new(2.0, 2.0));
        state.update_port_pos(PortId::new(3), Pos2::new(3.0, 2.0));
        // second gen
        state.update_port_pos(PortId::new(1), Pos2::new(1.0, 3.0));
        state.update_port_pos(PortId::new(2), Pos2::new(2.0, 3.0));
        // third gen
        state.update_port_pos(PortId::new(1), Pos2::new(1.0, 4.0));

        assert_eq!(state.port_pos(&PortId::new(1)), Some(&Pos2::new(1.0, 4.0)));
        assert_eq!(state.port_pos(&PortId::new(2)), Some(&Pos2::new(2.0, 3.0)));
        assert_eq!(state.port_pos(&PortId::new(3)), None);
    }

    #[test]
    fn update_hovered_port_id() {
        let mut state = State::default();
        state.update_hovered_port_id(PortId::new(1));
        assert_eq!(state.current.hovered_port_id, Some(PortId::new(1)));
    }

    #[test]
    fn test_hovered_port_id() {
        assert_eq!(State::default().hovered_port_id(), None);

        let mut state = State::default();
        state.current.hovered_port_id = Some(PortId::new(0));
        assert_eq!(state.hovered_port_id(), Some(PortId::new(0)));

        let mut state = State::default();
        state.previous.hovered_port_id = Some(PortId::new(0));
        assert_eq!(state.hovered_port_id(), Some(PortId::new(0)));

        let mut state = State::default();
        state.previous.hovered_port_id = Some(PortId::new(0));
        state.current.hovered_port_id = Some(PortId::new(1));
        assert_eq!(state.hovered_port_id(), Some(PortId::new(1)));
    }

    #[test]
    fn test_update_plug_pos() {
        let mut state = State::default();
        let id1 = PlugId::new(CableId::new(0), PlugType::In);
        let id2 = PlugId::new(CableId::new(0), PlugType::Out);
        state.update_plug_pos(id1.clone(), Pos2::new(1.0, 2.0));
        state.update_plug_pos(id2.clone(), Pos2::new(1.0, 3.0));
        assert_eq!(
            state.current.plug_pos,
            [(id1, Pos2::new(1.0, 2.0)), (id2, Pos2::new(1.0, 3.0))]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn test_plug_pos() {
        assert_eq!(
            State::default().plug_pos(&PlugId::new(CableId::new(0), PlugType::In)),
            None
        );

        let mut state = State::default();
        state.current.plug_pos.insert(
            PlugId::new(CableId::new(0), PlugType::In),
            Pos2::new(1.0, 2.0),
        );
        assert_eq!(
            state.plug_pos(&PlugId::new(CableId::new(0), PlugType::In)),
            Some(Pos2::new(1.0, 2.0))
        );

        let mut state = State::default();
        state.previous.plug_pos.insert(
            PlugId::new(CableId::new(0), PlugType::In),
            Pos2::new(1.0, 2.0),
        );
        assert_eq!(
            state.plug_pos(&PlugId::new(CableId::new(0), PlugType::In)),
            Some(Pos2::new(1.0, 2.0))
        );

        let mut state = State::default();
        state.previous.plug_pos.insert(
            PlugId::new(CableId::new(0), PlugType::In),
            Pos2::new(1.0, 2.0),
        );
        state.current.plug_pos.insert(
            PlugId::new(CableId::new(0), PlugType::In),
            Pos2::new(1.0, 3.0),
        );
        assert_eq!(
            state.plug_pos(&PlugId::new(CableId::new(0), PlugType::In)),
            Some(Pos2::new(1.0, 3.0))
        );
    }
}
