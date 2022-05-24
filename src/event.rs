use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Connect {
        plug_type: PlugType,
        port_id: PortId,
    },
}
