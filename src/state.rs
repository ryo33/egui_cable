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
    kvs: HashMap<Key, HashMap<Id, Arc<dyn Any + Send + Sync + 'static>>>,
    kv: HashMap<Key, Arc<dyn Any + Send + Sync + 'static>>,
}

#[derive(Default, Clone, Debug)]
pub(crate) struct EphemeralState {
    pub response_id_to_cable_id: HashMap<Id, CableId>,
    pub event: HashMap<CableId, Event>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Key {
    PortPos,
    PlugPos,
    CableState,
    HoveredPort,
    DraggedPlug,
}

use Key::*;

impl State {
    pub fn next_generation(&mut self) {
        std::mem::swap(&mut self.previous, &mut self.current);
        self.current = Default::default();
        self.ephemeral = Default::default();
    }

    fn update_kv<K, V>(&mut self, key: Key, id: K, data: V)
    where
        K: Hash + Eq + Debug + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        let kv = self.current.kvs.entry(key).or_default();
        kv.insert(Id::new(id), Arc::new(data));
    }

    fn get_kv<K, V>(&self, key: Key, id: &K) -> Option<V>
    where
        K: Clone + Hash + Eq + Debug + Send + Sync + 'static,
        V: Clone + Send + Sync + 'static,
    {
        let get = |generation: &GenerationState| {
            generation
                .kvs
                .get(&key)
                .and_then(|kv| kv.get(&Id::new(id.clone())))
                .cloned()
        };
        get(&self.current)
            .or_else(|| get(&self.previous))
            .map(|data| data.downcast_ref::<V>().unwrap().clone())
    }

    fn update_data<V: Send + Sync + 'static>(&mut self, key: Key, data: V) {
        self.current.kv.insert(key, Arc::new(data));
    }

    fn get_data<V: Clone + Send + Sync + 'static>(&self, key: Key) -> Option<V> {
        let get = |generation: &GenerationState| generation.kv.get(&key).cloned();
        get(&self.current)
            .or_else(|| get(&self.previous))
            .map(|data| data.downcast_ref::<V>().unwrap().clone())
    }

    pub fn advance_generation_if_twice(&mut self, port_id: PortId) {
        if self
            .current
            .kvs
            .get(&PortPos)
            .and_then(|kv| kv.get(&Id::new(port_id)))
            .is_some()
        {
            self.next_generation();
        }
    }

    pub fn update_port_pos(&mut self, port_id: PortId, pos: Pos2) {
        self.update_kv(PortPos, port_id, pos);
    }

    pub fn port_pos(&self, port_id: &PortId) -> Option<Pos2> {
        self.get_kv(PortPos, port_id)
    }

    pub fn update_plug_pos(&mut self, id: PlugId, pos: Pos2) {
        self.update_kv(PlugPos, id, pos);
    }

    pub fn plug_pos(&self, id: &PlugId) -> Option<Pos2> {
        self.get_kv(PlugPos, id)
    }

    pub fn update_cable_state(&mut self, id: CableId, pos: CableState) {
        self.update_kv(CableState, id, pos);
    }

    pub fn cable_state(&self, id: &CableId) -> Option<CableState> {
        self.get_kv(CableState, id)
    }

    pub fn update_hovered_port_id(&mut self, port_id: PortId) {
        self.update_data(HoveredPort, port_id)
    }

    pub fn hovered_port_id(&self) -> Option<PortId> {
        self.get_data(HoveredPort)
    }

    pub fn update_dragged_plug(&mut self, pos: Pos2) {
        self.update_data(DraggedPlug, pos)
    }

    pub fn dragged_plug(&self) -> Option<Pos2> {
        self.get_data(DraggedPlug)
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
        state.advance_generation_if_twice(PortId::new(1));
        state.update_port_pos(PortId::new(1), Pos2::ZERO);
        state.advance_generation_if_twice(PortId::new(2));
        state.update_port_pos(PortId::new(2), Pos2::ZERO);
        state.advance_generation_if_twice(PortId::new(3));
        state.update_port_pos(PortId::new(3), Pos2::ZERO);
        // second gen
        state.advance_generation_if_twice(PortId::new(1));
        state.update_port_pos(PortId::new(1), Pos2::ZERO);
        state.advance_generation_if_twice(PortId::new(2));
        state.update_port_pos(PortId::new(2), Pos2::ZERO);

        // assert not advanced for third gen
        assert_eq!(state.port_pos(&PortId::new(3)), Some(Pos2::ZERO));

        // third gen
        state.advance_generation_if_twice(PortId::new(1));
        state.update_port_pos(PortId::new(1), Pos2::ZERO);

        assert_eq!(state.port_pos(&PortId::new(1)), Some(Pos2::ZERO));
        assert_eq!(state.port_pos(&PortId::new(2)), Some(Pos2::ZERO));
        assert_eq!(state.port_pos(&PortId::new(3)), None);
    }

    #[test]
    fn update_hovered_port_id() {
        let mut state = State::default();
        state.update_hovered_port_id(PortId::new(1));
        assert_eq!(state.hovered_port_id(), Some(PortId::new(1)));

        state.next_generation();
        assert_eq!(state.hovered_port_id(), Some(PortId::new(1)));

        state.next_generation();
        assert_eq!(state.hovered_port_id(), None);
    }
}
