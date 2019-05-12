mod address;

use crate::ev::{Observable, Observer, Subject};

pub use address::*;

pub trait CpuMemoryMap {
    fn get(&self, addr: &Address) -> u8;
    fn set(&mut self, addr: &Address, val: u8) -> ();
    fn subscribe(&mut self, handler: Box<FnMut(&CpuMemoryAccessEvent)>);
}

pub enum CpuMemoryAccessEvent {
    Get(Address, u8),
    Set(Address, u8),
}

pub struct DefaultCpuMemoryMap {
    memory: [u8; 0xffff + 1],
    subject: Subject<CpuMemoryAccessEvent>,
}

impl DefaultCpuMemoryMap {
    pub fn new() -> DefaultCpuMemoryMap {
        DefaultCpuMemoryMap {
            memory: [0; 0xffff + 1],
            subject: Subject::new(),
        }
    }
}

impl CpuMemoryMap for DefaultCpuMemoryMap {
    fn get(&self, addr: &Address) -> u8 {
        let effective_addr = addr.get_addr();
        let byte = self.memory[effective_addr as usize];

        // Notify subscribers
        self.subject
            .next(CpuMemoryAccessEvent::Get(addr.clone(), byte));

        byte
    }

    fn set(&mut self, addr: &Address, val: u8) {
        let effective_addr = addr.get_addr();

        self.memory[effective_addr as usize] = val;

        // Notify subscribers
        self.subject
            .next(CpuMemoryAccessEvent::Set(addr.clone(), val));
    }

    fn subscribe(&mut self, handler: Box<FnMut(&CpuMemoryAccessEvent)>) {
        self.subject.subscribe(handler);
    }
}
