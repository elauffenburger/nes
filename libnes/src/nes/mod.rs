use crate::cpu::mem::CpuMemoryAccessEvent;
use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::Cpu;
use crate::ppu::Ppu;

pub trait Nes {
    fn start(&mut self) -> ();
    fn reset(&mut self) -> ();
    fn clock(&mut self) -> ();

    fn get_cpu(&mut self) -> Rc<RefCell<Cpu>>;
    fn get_ppu(&mut self) -> Rc<RefCell<Ppu>>;
}

pub struct DefaultNes {
    cpu: Rc<RefCell<Cpu>>,
    ppu: Rc<RefCell<Ppu>>,
}

impl Nes for DefaultNes {
    fn start(&mut self) {
        self.cpu.borrow_mut().start();
        self.ppu.borrow_mut().start();
    }

    fn reset(&mut self) {}

    fn clock(&mut self) {
        self.cpu.borrow_mut().clock();
        self.ppu.borrow_mut().clock();
    }

    fn get_cpu(&mut self) -> Rc<RefCell<Cpu>> {
        self.cpu.clone()
    }

    fn get_ppu(&mut self) -> Rc<RefCell<Ppu>> {
        self.ppu.clone()
    }
}

impl DefaultNes {
    pub fn new(cpu: Rc<RefCell<Cpu>>, ppu: Rc<RefCell<Ppu>>) -> Self {
        // Wire Cpu up to Ppu
        {
            let cpu = cpu.clone();
            let ppu = ppu.clone();

            cpu.borrow_mut()
                .subscribe_mem(Box::from(move |event: &CpuMemoryAccessEvent| {
                    ppu.borrow_mut().on_cpu_memory_access(event);
                }));
        }

        let nes = DefaultNes { cpu, ppu };

        nes
    }
}
