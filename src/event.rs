use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Connected {
        plug_type: PlugType,
        port_id: PortId,
    },
    Disconnected {
        plug_type: PlugType,
        port_id: PortId,
    },
}
