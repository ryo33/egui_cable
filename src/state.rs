use std::fmt::Debug;
use std::hash::Hash;
use std::{any::Any, collections::HashMap, ops::DerefMut, sync::Arc};

use egui::{util::IdTypeMap, Id, Pos2, Ui};

use crate::cable::CableState;
use crate::{cable::CableId, plug::PlugId, prelude::*};

#[derive(Default, Clone, Debug)]
pub(crate) struct State {
    previous: GenerationState,
    current: GenerationState,
    pub ephemeral: EphemeralState,
}

#[derive(Default, Clone, Debug)]
pub(crate) struct GenerationState {
    kv: HashMap<&'static str, HashMap<Id, Arc<dyn Any + Send + Sync + 'static>>>,
    hovered_port_id: Option<PortId>,
}

#[derive(Default, Clone, Debug)]
pub(crate) struct EphemeralState {
    pub response_id_to_cable_id: HashMap<Id, CableId>,
    pub event: HashMap<CableId, Event>,
}

const PORT_POS: &str = "port_pos";
const PLUG_POS: &str = "plug_pos";
const CABLE_STATE: &str = "cable_state";

impl State {
    pub fn next_generation(&mut self) {
        std::mem::swap(&mut self.previous, &mut self.current);
        self.current = Default::default();
        self.ephemeral = Default::default();
    }

    fn update_kv<K, V>(&mut self, key: &'static str, id: K, data: V)
    where
        K: Hash + Eq + Debug + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        let kv = self.current.kv.entry(key).or_default();
        kv.insert(Id::new(id), Arc::new(data));
    }

    fn get_kv<K, V>(&self, key: &'static str, id: &K) -> Option<V>
    where
        K: Clone + Hash + Eq + Debug + Send + Sync + 'static,
        V: Clone + Send + Sync + 'static,
    {
        let get = |generation: &GenerationState| {
            generation
                .kv
                .get(key)
                .and_then(|kv| kv.get(&Id::new(id.clone())))
                .cloned()
        };
        get(&self.current)
            .or_else(|| get(&self.previous))
            .map(|data| data.downcast_ref::<V>().unwrap().clone())
    }

    pub fn update_port_pos(&mut self, port_id: PortId, pos: Pos2) {
        // if the port_id will be updated twice, advances the generation
        if self
            .current
            .kv
            .get(PORT_POS)
            .and_then(|kv| kv.get(&Id::new(port_id)))
            .is_some()
        {
            self.next_generation();
        }
        self.update_kv(PORT_POS, port_id, pos);
    }

    pub fn port_pos(&self, port_id: &PortId) -> Option<Pos2> {
        self.get_kv(PORT_POS, port_id)
    }

    pub fn update_hovered_port_id(&mut self, port_id: PortId) {
        self.current.hovered_port_id = Some(port_id);
    }

    pub fn hovered_port_id(&self) -> Option<PortId> {
        self.current
            .hovered_port_id
            .as_ref()
            .or(self.previous.hovered_port_id.as_ref())
            .cloned()
    }

    pub fn update_plug_pos(&mut self, id: PlugId, pos: Pos2) {
        self.update_kv(PLUG_POS, id, pos);
    }

    pub fn plug_pos(&self, id: &PlugId) -> Option<Pos2> {
        self.get_kv(PLUG_POS, id)
    }

    pub fn update_cable_state(&mut self, id: CableId, pos: CableState) {
        self.update_kv(CABLE_STATE, id, pos);
    }

    pub fn cable_state(&self, id: &CableId) -> Option<CableState> {
        self.get_kv(CABLE_STATE, id)
    }

    pub fn get_cloned(mut data: impl DerefMut<Target = IdTypeMap>) -> Self {
        Self::clone(
            &data
                .get_persisted::<Arc<State>>(Id::null())
                .unwrap_or_default(),
        )
    }

    pub fn get(mut data: impl DerefMut<Target = IdTypeMap>) -> Arc<Self> {
        data.get_persisted::<Arc<State>>(Id::null())
            .unwrap_or_default()
    }

    pub fn store(self, ui: &mut Ui) {
        ui.data().insert_persisted(Id::null(), Arc::new(self));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        // assert not advanced for third gen
        assert_eq!(state.port_pos(&PortId::new(3)), Some(Pos2::new(3.0, 2.0)));

        // third gen
        state.update_port_pos(PortId::new(1), Pos2::new(1.0, 4.0));

        assert_eq!(state.port_pos(&PortId::new(1)), Some(Pos2::new(1.0, 4.0)));
        assert_eq!(state.port_pos(&PortId::new(2)), Some(Pos2::new(2.0, 3.0)));
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
}
