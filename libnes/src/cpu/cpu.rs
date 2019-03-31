use byteorder::{ByteOrder, LittleEndian};
use std::fmt::Debug;

use super::Registers;
use crate::cpu::instr::{CpuInstruction};
use crate::mem::{Address, CpuMemoryMap, MemoryMap};

const NMI_INTERRUPT_ADDR_START: u16 = 0xfffa;
const RESET_INTERRUPT_ADDR_START: u16 = 0xfffc;
const IRQ_INTERRUPT_ADDR_START: u16 = 0xfffe;
const BRK_INTERRUPT_ADDR_START: u16 = 0xffe6;

pub struct Cpu {
    pub memory: Box<MemoryMap>,
    pub registers: Registers,
    is_stopped: bool,
    debug: bool,
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, 
            "A: {:x}, X: {:x}, Y: {:x}, SP: {:x}, PC: {:x}, P: {:?}", 
            self.registers.acc, 
            self.registers.x, 
            self.registers.y, 
            self.registers.sp,
            self.registers.pc,
            self.registers.p)
    }
}

impl Cpu {
    pub fn new(debug: bool) -> Cpu {
        Cpu {
            memory: Box::new(CpuMemoryMap::new()),
            registers: Registers::new(),
            is_stopped: false,
            debug: debug,
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn run(&mut self) {
        self.startup();

        loop {
            let debug = self.debug;

            if debug {
                println!("cpu pre: {:?}", self);
            }
            
            let instruction = self.next_instr();

            if debug {
                println!("instr: {:?}", instruction);
            }

            instruction.run();

            if debug {
                println!("cpu post: {:?}", self);
            }

            if self.is_stopped {
                break;
            }

            if debug {
                println!("");
            }
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

    pub fn push(&mut self, val: u8) {
        self.registers.sp -= 1;
        self.write_bytes_to(&self.registers.sp.into(), &[val]);
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes.iter() {
            self.registers.sp -= 1;
            self.write_bytes_to(&self.registers.sp.into(), &[byte.clone()]);
        }
    }

    pub fn pop(&mut self) -> u8 {
        let value = self.read_u8_at(&self.registers.sp.into());
        self.registers.sp += 1;

        value
    }

    pub fn perform_instr<F>(&mut self, instr: F)
    where
        F: Fn(&mut Cpu),
    {
        instr(self);
    }

    pub fn stop(&mut self) {
        self.is_stopped = true;
    }
}

#[cfg(test)]
mod test {
    use crate::bits::{msb, lsb};
    use super::{Address, Cpu, BRK_INTERRUPT_ADDR_START, RESET_INTERRUPT_ADDR_START};

    #[test]
    fn basic_program() {
        let mut cpu = Cpu::new(true);

        load_program_str(&mut cpu, "a9 01 8d 00 02 a9 05 8d 01 02 a9 08 8d 02 02");

        cpu.run();

        assert_eq!(cpu.registers.acc, 0x08);
    }

    #[test]
    fn lda_tax_inx_adc() {
        let mut cpu = Cpu::new(true);

        load_program_str(&mut cpu, "a9 c0 aa e8 69 c4 00");

        cpu.run();

        assert_eq!(cpu.registers.acc as u8, 0x84);
        assert_eq!(cpu.registers.x as u8, 0xc1);
    }

    #[test]
    fn ldx_dex_stx_cpx_bnx() {
        let mut cpu = Cpu::new(true);

        load_program_str(&mut cpu, "a2 08 ca 8e 00 02 e0 03 d0 f8 8e 01 02 00");

        cpu.run();

        assert_eq!(cpu.registers.acc as u8, 0x00);
        assert_eq!(cpu.registers.x as u8, 0x03);
    }

    #[test]
    fn lda_cmp_bne_sta_brk() {
        let mut cpu = Cpu::new(true);

        load_program_str(&mut cpu, "a9 01 c9 02 d0 02 85 22 00");

        cpu.run();

        assert_eq!(cpu.registers.acc as u8, 0x01);
        assert_eq!(cpu.registers.p.into_u8(), 0b10110100);
    }

    #[test]
    fn lda_sta_lda_sta_jmp() {
        let mut cpu = Cpu::new(true);

        load_program_str(&mut cpu, "a9 01 85 f0 a9 cc 85 f1 6c f0 00");

        cpu.run();

        assert_eq!(cpu.registers.acc as u8, 0xcc);
        assert_eq!(cpu.registers.p.into_u8(), 0b10110100);
    }

    #[test]
    fn jsr_lda_rts() {
        let mut cpu = Cpu::new(true);

        load_program_str(&mut cpu, "a9 01 20 08 06 a9 03 00 a9 02 60");

        cpu.run();

        assert_eq!(cpu.registers.acc as u8, 0x03);
    }

    #[test]
    fn rol_acc() {
        let mut cpu = Cpu::new(true);

        load_program_str(&mut cpu, "a9 81 2a");

        cpu.run();

        assert_eq!(cpu.registers.acc as u8, 0x02);
    }

    #[test]
    fn ror_acc() {
        let mut cpu = Cpu::new(true);

        load_program_str(&mut cpu, "a9 81 6a");

        cpu.run();

        assert_eq!(cpu.registers.acc as u8, 0x40);
    }

    #[test]
    fn rol_mem() {
        let mut cpu = Cpu::new(true);

        load_program_str(&mut cpu, "a9 81 8d 00 80 2e 00 80");

        cpu.run();

        assert_eq!(cpu.registers.acc as u8, 0x81);
        assert_eq!(cpu.read_u8_at(&0x8000u16.into()), 0x02);
    }

    #[test]
    fn ror_mem() {
        let mut cpu = Cpu::new(true);

        load_program_str(&mut cpu, "a9 81 8d 00 80 6e 00 80");

        cpu.run();

        assert_eq!(cpu.registers.acc as u8, 0x81);
        assert_eq!(cpu.read_u8_at(&0x8000u16.into()), 0x40);
    }

    fn to_bytes<'a>(byte_str: &'a str) -> Vec<u8> {
        byte_str
            .split(" ")
            .map(|b| {
                let byte = u8::from_str_radix(b.clone(), 16).unwrap();

                byte
            })
            .collect::<Vec<u8>>()
    }

    fn load_program_str(cpu: &mut Cpu, prog: &'static str) {
        load_program_str_with_options(cpu, prog, 0x0600)
    }

    fn load_program_str_with_options(cpu: &mut Cpu, prog: &'static str, start_addr: u16 ) {
        // write interrupt routine addr
        cpu.write_bytes_to(&Address::from(RESET_INTERRUPT_ADDR_START), &[lsb(start_addr), msb(start_addr)]);
        cpu.write_bytes_to(&Address::from(start_addr), &to_bytes(prog));

        // write stp instr to brk irq vector address (so that'll be run at the first brk)
        cpu.write_bytes_to(&Address::from(BRK_INTERRUPT_ADDR_START), &[0xef, 0xbe]);
        cpu.write_bytes_to(&Address::from(0xbeef), &to_bytes("db"));
    }
}
