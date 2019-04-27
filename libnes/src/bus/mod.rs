use crate::mem::{Address, MemoryMap};

pub trait Bus {
}

pub struct DefaultBus {
    pub memory: Box<MemoryMap>,
}

impl Bus for DefaultBus {
}