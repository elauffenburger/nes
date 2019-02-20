use byteorder::{ByteOrder, LittleEndian};

use super::Registers;
use crate::cpu::instr::addressing::AddressingMode;
use crate::cpu::instr::{CpuInstruction, Instruction};
use crate::mem::{Address, CpuMemoryMap, MemoryMap};

const NMI_INTERRUPT_ADDR_START: u16 = 0xfffa;
const RESET_INTERRUPT_ADDR_START: u16 = 0xfffc;
const IRQ_INTERRUPT_ADDR_START: u16 = 0xfffe;

pub struct Cpu {
    pub memory: Box<MemoryMap>,
    pub registers: Registers,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: Box::new(CpuMemoryMap::new()),
            registers: Registers::new(),
        }
    }

    pub fn run(mut self) {
        self.startup();

        loop {
            let instruction = self.next_instr();
            instruction.run();
        }
    }

    fn next_instr(&mut self) -> CpuInstruction {
        let opcode = self.next_u8();

        CpuInstruction::from(opcode, self)
    }

    fn startup(&mut self) {
        self.reset();
    }

    fn reset(&mut self) {
        let (lower, upper) = (
            self.read_u8_at(&RESET_INTERRUPT_ADDR_START.into()),
            self.read_u8_at(&(RESET_INTERRUPT_ADDR_START + 1).into()),
        );

        self.registers.pc = LittleEndian::read_u16(&[lower, upper]);
    }

    pub fn load_mem(&mut self, mem: Box<MemoryMap>) {
        self.memory = mem;
    }

    pub fn next_u8(&mut self) -> u8 {
        let pc = self.registers.pc;

        self.registers.pc += 1;

        self.memory.get(&pc.into())
    }

    pub fn next_u16(&mut self) -> u16 {
        let (lower, upper) = (self.next_u8(), self.next_u8());

        LittleEndian::read_u16(&[lower, upper])
    }

    pub fn write_bytes_to(&mut self, addr: &Address, bytes: &[u8]) {
        for (i, byte) in bytes.iter().enumerate() {
            self.memory.set(&(addr + (i as u16)), byte.clone());
        }
    }

    pub fn read_u8_at(&self, addr: &Address) -> u8 {
        self.memory.get(addr)
    }

    pub fn read_u16_at(&self, addr: &Address) -> u16 {
        let (first_byte_addr, second_byte_addr) = (addr, addr + (1 as u16));
        let (lower, upper) = (
            self.read_u8_at(&first_byte_addr),
            self.read_u8_at(&second_byte_addr),
        );

        LittleEndian::read_u16(&[lower, upper])
    }

    pub fn perform_instr<F>(&mut self, instr: F)
    where
        F: Fn(&mut Cpu),
    {
        let acc_pre = self.registers.acc;

        instr(self);

        let acc_post = self.registers.acc;

        // todo: set status flags based on pre and post states
    }
}

#[cfg(test)]
mod test {
    use super::{Address, Cpu, RESET_INTERRUPT_ADDR_START};
    use byteorder::LittleEndian;

    #[test]
    fn basic_program() {
        let mut cpu = Cpu::new();

        // write interrupt routine addr
        cpu.write_bytes_to(&Address::from(RESET_INTERRUPT_ADDR_START), &[0x2d, 0xd2]);
        cpu.write_bytes_to(
            &Address::from(0xd22d),
            &to_bytes("a9 01 8d 00 02 a9 05 8d 01 02 a9 08 8d 02 02"),
        );

        cpu.run();
    }

    fn to_bytes<'a>(byte_str: &'a str) -> Vec<u8> {
        byte_str
            .split(" ")
            .map(|b| u8::from_str_radix(b.clone(), 16).unwrap())
            .collect::<Vec<u8>>()
    }
}
