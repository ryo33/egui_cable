use std::fmt::Debug;
use std::hash::Hash;
use std::{any::Any, collections::HashMap, ops::DerefMut, sync::Arc};

use egui::Vec2;
use egui::{util::IdTypeMap, Id, Pos2};

use crate::cable::CableState;
use crate::plug::{DraggedPlug, PlugState};
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
    PlugState,
    CableState,
    HoveredPort,
    DraggedPlug,
    CableControlSize,
}

macro_rules! kvs {
    ($key:ident, $get:ident, $update:ident, $id:ty, $value:ty) => {
        pub fn $get(&self, id: &$id) -> Option<$value> {
            self.get_kv(Key::$key, id)
        }

        pub fn $update(&mut self, id: $id, value: $value) {
            self.update_kv(Key::$key, id, value);
        }
    };
}

macro_rules! kv {
    ($key:ident, $get:ident, $update:ident, $value:ty) => {
        pub fn $get(&self) -> Option<$value> {
            self.get_data(Key::$key)
        }

        pub fn $update(&mut self, value: $value) {
            self.update_data(Key::$key, value);
        }
    };
}

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
            .get(&Key::PortPos)
            .and_then(|kv| kv.get(&Id::new(port_id)))
            .is_some()
        {
            self.next_generation();
        }
    }

    kvs!(PortPos, port_pos, update_port_pos, PortId, Pos2);
    kvs!(PlugState, plug_state, update_plug_state, PlugId, PlugState);
    kvs!(
        CableState,
        cable_state,
        update_cable_state,
        CableId,
        CableState
    );
    kvs!(
        CableControlSize,
        cable_control_size,
        update_cable_control_size,
        CableId,
        Vec2
    );

    kv!(HoveredPort, hovered_port_id, update_hovered_port_id, PortId);
    kv!(DraggedPlug, dragged_plug, update_dragged_plug, DraggedPlug);

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

    pub fn store_to(self, mut data: impl DerefMut<Target = IdTypeMap>) {
        data.insert_persisted(Id::null(), Arc::new(self));
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
