mod address;

pub use address::*;

pub trait CpuMemoryMap {
    fn get(&self, addr: &Address) -> u8;
    fn set(&mut self, addr: &Address, val: u8) -> ();
}

pub struct DefaultCpuMemoryMap {
    memory: [u8; 0xffff + 1],
}

impl DefaultCpuMemoryMap {
    pub fn new() -> DefaultCpuMemoryMap {
        DefaultCpuMemoryMap { memory: [0; 0xffff + 1]}
    }
}

impl CpuMemoryMap for DefaultCpuMemoryMap {
    fn get(&self, addr: &Address) -> u8 {
        let effective_addr = addr.get_addr();

        self.memory[effective_addr as usize]
    }

    fn set(&mut self, addr: &Address, val: u8) {
        let effective_addr = addr.get_addr();

        self.memory[effective_addr as usize] = val;
    }
}