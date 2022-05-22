use std::{any::Any, rc::Rc};

use crate::port_id::PortId;

#[derive(Debug)]
pub struct Cable {
    data: Box<dyn Any>,
}

#[derive(Debug)]
pub(crate) struct CableTo {
    pub cable: Cable,
    pub to: PortId,
}

impl Cable {
    pub fn new<T: 'static>(data: T) -> Self {
        Cable {
            data: Box::new(data),
        }
    }

    pub fn unit() -> Self {
        Cable { data: Box::new(()) }
    }
}

impl Default for Cable {
    fn default() -> Self {
        Self::unit()
    }
}

impl<T: 'static> AsRef<T> for Cable {
    fn as_ref(&self) -> &T {
        self.data
            .downcast_ref()
            .expect("Cannot downcast cable data. Check your type.")
    }
}
