use egui::Response;

use crate::{prelude::*, state::State};

pub trait ResponseExt {
    /// Returns an event if happens.
    fn cable_event(&self) -> Option<Event>;
    /// Returns a port Id if in-plug is connected.
    fn in_connected_to(&self) -> Option<PortId>;
    /// Returns a port Id if out-plug is connected.
    fn out_connected_to(&self) -> Option<PortId>;
    /// Returns a port Id if in-plug or out-plug is connected.
    fn connected_to(&self) -> Option<PortId>;
    /// Returns true if in-plug is disconnected.
    fn in_disconnected(&self) -> bool;
    /// Returns true if out-plug is disconnected.
    fn out_disconnected(&self) -> bool;
    /// Returns true if in-plug or out-plug is disconnected.
    fn disconnected(&self) -> bool;
}

impl ResponseExt for Response {
    fn cable_event(&self) -> Option<Event> {
        let state = State::get(self.ctx.data());
        state
            .ephemeral
            .response_id_to_cable_id
            .get(&self.id)
            .and_then(|cable_id| state.ephemeral.event.get(cable_id))
            .cloned()
    }

    fn in_connected_to(&self) -> Option<PortId> {
        if let Some(Event::Connected {
            plug_type: PlugType::In,
            port_id,
        }) = self.cable_event()
        {
            Some(port_id)
        } else {
            None
        }
    }

    fn out_connected_to(&self) -> Option<PortId> {
        if let Some(Event::Connected {
            plug_type: PlugType::Out,
            port_id,
        }) = self.cable_event()
        {
            Some(port_id)
        } else {
            None
        }
    }

    fn connected_to(&self) -> Option<PortId> {
        if let Some(Event::Connected {
            plug_type: _,
            port_id,
        }) = self.cable_event()
        {
            Some(port_id)
        } else {
            None
        }
    }

    fn in_disconnected(&self) -> bool {
        matches!(
            self.cable_event(),
            Some(Event::Disconnected {
                plug_type: PlugType::In,
                ..
            })
        )
    }

    fn out_disconnected(&self) -> bool {
        matches!(
            self.cable_event(),
            Some(Event::Disconnected {
                plug_type: PlugType::Out,
                ..
            })
        )
    }

    fn disconnected(&self) -> bool {
        matches!(self.cable_event(), Some(Event::Disconnected { .. }))
    }
}
