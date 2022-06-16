use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Connected { port_id: PortId },
    Disconnected,
    Hovered { port_id: PortId },
}
