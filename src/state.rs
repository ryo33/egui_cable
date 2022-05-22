use std::collections::HashMap;

use egui::Pos2;

use crate::port_id::PortId;

#[derive(Default, Clone, Debug)]
pub(crate) struct State {
    pub drag_start_port: Option<PortId>,
    previous_port_pos: HashMap<PortId, Pos2>,
    current_port_pos: HashMap<PortId, Pos2>,
}

impl State {
    pub fn update_port_pos(&mut self, port_id: PortId, pos: Pos2) {
        if self.current_port_pos.contains_key(&port_id) {
            self.previous_port_pos = self.current_port_pos.drain().collect();
        }
        self.current_port_pos.insert(port_id, pos);
    }

    pub fn port_pos(&self, port_id: &PortId) -> Option<&Pos2> {
        self.current_port_pos
            .get(port_id)
            .or_else(|| self.previous_port_pos.get(port_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_port_pos_inserts_current_port_pos() {
        let mut state = State::default();
        state.update_port_pos(PortId::new(1), Pos2::new(1.0, 2.0));
        state.update_port_pos(PortId::new(2), Pos2::new(2.0, 2.0));
        assert_eq!(
            state.current_port_pos,
            [
                (PortId::new(1), Pos2::new(1.0, 2.0)),
                (PortId::new(2), Pos2::new(2.0, 2.0))
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(state.previous_port_pos, [].into_iter().collect());
    }

    #[test]
    fn update_port_pos_use_previous_port_pos() {
        let mut state = State::default();
        state.update_port_pos(PortId::new(1), Pos2::new(1.0, 2.0));
        state.update_port_pos(PortId::new(2), Pos2::new(2.0, 2.0));
        state.update_port_pos(PortId::new(3), Pos2::new(3.0, 2.0));
        assert_eq!(state.previous_port_pos.len(), 0);
        state.update_port_pos(PortId::new(1), Pos2::new(1.0, 3.0));

        assert_eq!(
            state.current_port_pos,
            [(PortId::new(1), Pos2::new(1.0, 3.0)),]
                .into_iter()
                .collect()
        );
        assert_eq!(
            state.previous_port_pos,
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
}
