mod address;

pub use address::*;

use std::error::Error;

pub trait MemoryMap {
    fn get(&self, addr: &Address) -> u8;
    fn set(&mut self, addr: &Address, val: u8) -> ();
}

pub struct CpuMemoryMap {
    memory: [u8; 0xffff + 1],
}

impl CpuMemoryMap {
    pub fn new() -> CpuMemoryMap {
        CpuMemoryMap { memory: [0; 0xffff + 1]}
    }
}

impl MemoryMap for CpuMemoryMap {
    fn get(&self, addr: &Address) -> u8 {
        let raw_addr: u16 = addr.into();

        self.memory[raw_addr as usize]
    }

    fn set(&mut self, addr: &Address, val: u8) {
        let raw_addr: u16 = addr.into();

        // todo: handle memory mirroring

        self.memory[raw_addr as usize] = val;
    }
}