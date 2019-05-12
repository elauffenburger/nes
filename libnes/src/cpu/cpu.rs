use crate::cpu::mem::CpuMemoryAccessEvent;
use std::fmt::Debug;

use byteorder::{ByteOrder, LittleEndian};

use crate::bits::lsb;
use crate::bits::msb;
use crate::cpu::instr::CpuInstruction;
use crate::ev::{Observable, Observer};

use super::mem::{Address, CpuMemoryMap, DefaultCpuMemoryMap};
use super::Registers;

pub const NMI_INTERRUPT_ADDR_START: u16 = 0xfffa;
pub const RESET_INTERRUPT_ADDR_START: u16 = 0xfffc;
pub const IRQ_INTERRUPT_ADDR_START: u16 = 0xfffe;
pub const BRK_INTERRUPT_ADDR_START: u16 = 0xffe6;

pub trait Cpu {
    fn start(&mut self);
    fn stop(&mut self);
    fn reset(&mut self);
    fn clock(&mut self);

    fn step(&mut self);
    fn run(&mut self);
    fn is_running(&self) -> bool;

    fn write_bytes_to(&mut self, start_addr: &Address, bytes: &[u8]);
    fn load_mem(&mut self, mem: Box<CpuMemoryMap>);
    fn subscribe_mem(&mut self, handler: Box<FnMut(&CpuMemoryAccessEvent)>);

    fn next_u8(&mut self) -> u8;
    fn next_u16(&mut self) -> u16;

    fn read_u8_at(&self, addr: &Address) -> u8;
    fn read_u16_at(&self, addr: &Address) -> u16;

    fn push(&mut self, val: u8);
    fn push_u16(&mut self, val: u16);
    fn push_bytes(&mut self, bytes: &[u8]);

    fn pop(&mut self) -> u8;
    fn pop_u16(&mut self) -> u16;

    fn get_registers(&self) -> &Registers;
    fn get_registers_mut(&mut self) -> &mut Registers;
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let registers = self.get_registers();

        write!(
            f,
            "A: {:#04x}, X: {:#04x}, Y: {:#04x}, SP: {:#04x}, PC: {:#06x}, P: {:?}",
            registers.acc, registers.x, registers.y, registers.sp, registers.pc, registers.p
        )
    }
}

pub struct DefaultCpu {
    pub memory: Box<CpuMemoryMap>,
    pub registers: Registers,
    is_stopped: bool,
    debug: bool,
    has_started_up: bool,
}

impl Debug for DefaultCpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        (self as &Cpu).fmt(f)
    }
}

impl Cpu for DefaultCpu {
    fn start(&mut self) {
        self.registers.sp = 0xfd;

        self.reset();
    }

    fn stop(&mut self) {
        self.is_stopped = true;
    }

    fn reset(&mut self) {
        let (lower, upper) = (
            self.read_u8_at(&RESET_INTERRUPT_ADDR_START.into()),
            self.read_u8_at(&(RESET_INTERRUPT_ADDR_START + 1).into()),
        );

        self.registers.pc = LittleEndian::read_u16(&[lower, upper]);
    }

    fn clock(&mut self) {
        self.step()
    }

    fn next_u8(&mut self) -> u8 {
        let pc = self.registers.pc;

        self.registers.pc += 1;

        self.memory.get(&pc.into())
    }

    fn next_u16(&mut self) -> u16 {
        let (lower, upper) = (self.next_u8(), self.next_u8());

        LittleEndian::read_u16(&[lower, upper])
    }

    fn write_bytes_to(&mut self, start_addr: &Address, bytes: &[u8]) {
        let raw_start_addr: u16 = start_addr.into();

        for (i, byte) in bytes.iter().enumerate() {
            let addr = (raw_start_addr + (i as u16)).into();

            self.memory.set(&addr, byte.clone());
        }
    }

    fn read_u8_at(&self, addr: &Address) -> u8 {
        self.memory.get(addr)
    }

    fn read_u16_at(&self, addr: &Address) -> u16 {
        let (first_byte_addr, second_byte_addr) = (addr, addr + (1 as u8));
        let (lower, upper) = (
            self.read_u8_at(&first_byte_addr),
            self.read_u8_at(&second_byte_addr),
        );

        LittleEndian::read_u16(&[lower, upper])
    }

    fn push(&mut self, val: u8) {
        self.registers.sp -= 1;
        self.write_bytes_to(&self.registers.sp.into(), &[val]);
    }

    fn push_u16(&mut self, val: u16) {
        self.push_bytes(&[msb(val), lsb(val)]);
    }

    fn push_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes.iter() {
            self.registers.sp -= 1;
            self.write_bytes_to(&self.registers.sp.into(), &[byte.clone()]);
        }
    }

    fn pop(&mut self) -> u8 {
        let value = self.read_u8_at(&self.registers.sp.into());
        self.registers.sp += 1;

        value
    }

    fn pop_u16(&mut self) -> u16 {
        let lo = self.pop();
        let hi = self.pop();

        LittleEndian::read_u16(&[lo, hi])
    }

    fn get_registers(&self) -> &Registers {
        &self.registers
    }

    fn get_registers_mut(&mut self) -> &mut Registers {
        &mut self.registers
    }

    fn run(&mut self) {
        if !self.has_started_up {
            self.start();
        }

        loop {
            self.step();

            if self.is_stopped {
                break;
            }
        }

        if self.debug {
            println!("");
        }
    }

    fn step(&mut self) {
        let debug = self.debug;

        if debug {
            println!("cpu pre: {:?}", self as &mut Cpu);
        }

        let instruction = self.next_instr();

        if debug {
            println!("instr: {:?}", instruction);
        }

        instruction.run();

        if debug {
            println!("cpu post: {:?}", self as &mut Cpu);
        }

        if debug {
            println!("");
        }
    }

    fn load_mem(&mut self, mem: Box<CpuMemoryMap>) {
        self.memory = mem;
    }

    fn subscribe_mem(&mut self, handler: Box<FnMut(&CpuMemoryAccessEvent)>) {
        self.memory.subscribe(handler);
    }

    fn is_running(&self) -> bool {
        !self.is_stopped
    }
}

impl DefaultCpu {
    pub fn new(debug: bool) -> DefaultCpu {
        DefaultCpu {
            memory: Box::new(DefaultCpuMemoryMap::new()),
            registers: Registers::new(),
            is_stopped: false,
            debug: debug,
            has_started_up: false,
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    fn next_instr(&mut self) -> CpuInstruction<Self> {
        let opcode = self.next_u8();

        CpuInstruction::from(opcode, self)
    }
}
