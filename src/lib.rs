pub mod cable;
pub mod cable_control;
pub mod cable_params;
mod custom_widget;
pub mod default_cable;
pub mod default_plug;
pub mod default_port;
pub mod event;
pub mod id;
pub mod plug;
pub mod plug_params;
pub mod port;
pub mod port_params;
pub mod response;
mod state;
mod utils;

pub mod prelude {
    pub use crate::cable::Cable;
    pub use crate::cable_params::CableParams;
    pub use crate::default_cable::DefaultControl;
    pub use crate::default_plug::DefaultPlug;
    pub use crate::default_port::DefaultPort;
    pub use crate::event::Event;
    pub use crate::plug::Plug;
    pub use crate::plug_params::PlugParams;
    pub use crate::port::{Port, PortId};
    pub use crate::port_params::PortParams;
    pub use crate::response::ResponseExt as _;
}
