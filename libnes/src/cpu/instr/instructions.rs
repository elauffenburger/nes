use crate::cpu::instr::addressing::AddressingMode;
use crate::cpu::Cpu;

pub trait Instruction {
    fn run(self, cpu: &mut Cpu);
}

pub struct CpuInstruction<'a> {
    pub opcode: u8,
    pub addr_mode: AddressingMode,
    do_run: &'a Fn(&mut Cpu),
}

impl<'a> CpuInstruction<'a> {
    pub fn new(
        opcode: u8,
        addr_mode: AddressingMode,
        do_run: &'a Fn(&mut Cpu),
    ) -> CpuInstruction<'a> {
        CpuInstruction {
            opcode: opcode,
            addr_mode: addr_mode,
            do_run: do_run,
        }
    }
}

impl<'a> Instruction for CpuInstruction<'a> {
    fn run(self, cpu: &mut Cpu) {
        (self.do_run)(cpu)
    }
}

impl<'a> From<u8> for CpuInstruction<'a> {
    fn from(opcode: u8) -> CpuInstruction<'a> {
        match opcode {
            0xa9 => CpuInstruction::new(opcode, AddressingMode::Immediate, &lda),
            _ => panic!("opcode {:x} is not implemented!", opcode),
        }
    }
}

fn lda(cpu: &mut Cpu) {}
