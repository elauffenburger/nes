mod instrs;
mod helpers;
pub use instrs::*;

use crate::cpu::instr::addressing::AddressingMode;
use crate::cpu::Cpu;

pub struct CpuInstruction<'op, 'cpu> {
    pub opcode: u8,
    pub instr: &'op str,
    pub addr_mode: AddressingMode,
    cpu: &'cpu mut Cpu,
    do_run: &'op Fn(&mut Cpu, AddressingMode),
}

impl<'op, 'cpu> CpuInstruction<'op, 'cpu> {
    pub fn run(self) {
        (self.do_run)(self.cpu, self.addr_mode)
    }
}

impl<'op, 'cpu> std::fmt::Debug for CpuInstruction<'op, 'cpu> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} ({:x})", self.instr, self.opcode)
    }
}

impl<'op, 'cpu> CpuInstruction<'op, 'cpu> {
    pub fn new(
        opcode: u8,
        cpu: &'cpu mut Cpu,
        instr: &'static str,
        addr_mode: AddressingMode,
        do_run: &'op Fn(&mut Cpu, AddressingMode),
    ) -> Self {
        CpuInstruction {
            opcode: opcode,
            instr: instr,
            addr_mode: addr_mode,
            cpu: cpu,
            do_run: do_run,
        }
    }

    pub fn from(opcode: u8, cpu: &'cpu mut Cpu) -> Self {
        match opcode {
            0x69 => CpuInstruction::new(opcode, cpu, "adc", AddressingMode::Immediate, &adc),
            0x65 => CpuInstruction::new(opcode, cpu, "adc", AddressingMode::ZeroPage, &adc),
            0x75 => CpuInstruction::new(opcode, cpu, "adc", AddressingMode::ZeroPageX, &adc),
            0x6D => CpuInstruction::new(opcode, cpu, "adc", AddressingMode::Absolute, &adc),
            0x7D => CpuInstruction::new(opcode, cpu, "adc", AddressingMode::AbsoluteX, &adc),
            0x79 => CpuInstruction::new(opcode, cpu, "adc", AddressingMode::AbsoluteY, &adc),
            0x61 => CpuInstruction::new(opcode, cpu, "adc", AddressingMode::IndexedIndirect, &adc),
            0x71 => CpuInstruction::new(opcode, cpu, "adc", AddressingMode::IndirectIndexed, &adc),

            0x29 => CpuInstruction::new(opcode, cpu, "and", AddressingMode::Immediate, &and),
            0x25 => CpuInstruction::new(opcode, cpu, "and", AddressingMode::ZeroPage, &and),
            0x35 => CpuInstruction::new(opcode, cpu, "and", AddressingMode::ZeroPageX, &and),
            0x2D => CpuInstruction::new(opcode, cpu, "and", AddressingMode::Absolute, &and),
            0x3D => CpuInstruction::new(opcode, cpu, "and", AddressingMode::AbsoluteX, &and),
            0x39 => CpuInstruction::new(opcode, cpu, "and", AddressingMode::AbsoluteY, &and),
            0x21 => CpuInstruction::new(opcode, cpu, "and", AddressingMode::IndexedIndirect, &and),
            0x31 => CpuInstruction::new(opcode, cpu, "and", AddressingMode::IndirectIndexed, &and),

            0x0A => CpuInstruction::new(opcode, cpu, "asl", AddressingMode::Acc, &asl),
            0x06 => CpuInstruction::new(opcode, cpu, "asl", AddressingMode::ZeroPage, &asl),
            0x16 => CpuInstruction::new(opcode, cpu, "asl", AddressingMode::ZeroPageX, &asl),
            0x0E => CpuInstruction::new(opcode, cpu, "asl", AddressingMode::Absolute, &asl),
            0x1E => CpuInstruction::new(opcode, cpu, "asl", AddressingMode::AbsoluteX, &asl),

            0x90 => CpuInstruction::new(opcode, cpu, "bcc", AddressingMode::Relative, &bcc),

            0xB0 => CpuInstruction::new(opcode, cpu, "bcs", AddressingMode::Relative, &bcs),

            0xF0 => CpuInstruction::new(opcode, cpu, "beq", AddressingMode::Relative, &beq),

            0x24 => CpuInstruction::new(opcode, cpu, "bit", AddressingMode::ZeroPage, &bit),
            0x2C => CpuInstruction::new(opcode, cpu, "bit", AddressingMode::Absolute, &bit),

            0x30 => CpuInstruction::new(opcode, cpu, "bmi", AddressingMode::Relative, &bmi),

            0xD0 => CpuInstruction::new(opcode, cpu, "bne", AddressingMode::Relative, &bne),

            0x10 => CpuInstruction::new(opcode, cpu, "bpl", AddressingMode::Relative, &bpl),

            0x00 => CpuInstruction::new(opcode, cpu, "brk", AddressingMode::Implied, &brk),

            0x50 => CpuInstruction::new(opcode, cpu, "bvc", AddressingMode::Relative, &bvc),

            0x70 => CpuInstruction::new(opcode, cpu, "bvs", AddressingMode::Relative, &bvs),

            0x18 => CpuInstruction::new(opcode, cpu, "clc", AddressingMode::Implied, &clc),

            0xD8 => CpuInstruction::new(opcode, cpu, "cld", AddressingMode::Implied, &cld),

            0x58 => CpuInstruction::new(opcode, cpu, "cli", AddressingMode::Implied, &cli),

            0xB8 => CpuInstruction::new(opcode, cpu, "clv", AddressingMode::Implied, &clv),

            0xC9 => CpuInstruction::new(opcode, cpu, "cmp", AddressingMode::Immediate, &cmp),
            0xC5 => CpuInstruction::new(opcode, cpu, "cmp", AddressingMode::ZeroPage, &cmp),
            0xD5 => CpuInstruction::new(opcode, cpu, "cmp", AddressingMode::ZeroPageX, &cmp),
            0xCD => CpuInstruction::new(opcode, cpu, "cmp", AddressingMode::Absolute, &cmp),
            0xDD => CpuInstruction::new(opcode, cpu, "cmp", AddressingMode::AbsoluteX, &cmp),
            0xD9 => CpuInstruction::new(opcode, cpu, "cmp", AddressingMode::AbsoluteY, &cmp),
            0xC1 => CpuInstruction::new(opcode, cpu, "cmp", AddressingMode::IndexedIndirect, &cmp),
            0xD1 => CpuInstruction::new(opcode, cpu, "cmp", AddressingMode::IndirectIndexed, &cmp),

            0xE0 => CpuInstruction::new(opcode, cpu, "cpx", AddressingMode::Immediate, &cpx),
            0xE4 => CpuInstruction::new(opcode, cpu, "cpx", AddressingMode::ZeroPage, &cpx),
            0xEC => CpuInstruction::new(opcode, cpu, "cpx", AddressingMode::Absolute, &cpx),

            0xC0 => CpuInstruction::new(opcode, cpu, "cpy", AddressingMode::Immediate, &cpy),
            0xC4 => CpuInstruction::new(opcode, cpu, "cpy", AddressingMode::ZeroPage, &cpy),
            0xCC => CpuInstruction::new(opcode, cpu, "cpy", AddressingMode::Absolute, &cpy),

            0xC6 => CpuInstruction::new(opcode, cpu, "dec", AddressingMode::ZeroPage, &dec),
            0xD6 => CpuInstruction::new(opcode, cpu, "dec", AddressingMode::ZeroPageX, &dec),
            0xCE => CpuInstruction::new(opcode, cpu, "dec", AddressingMode::Absolute, &dec),
            0xDE => CpuInstruction::new(opcode, cpu, "dec", AddressingMode::AbsoluteX, &dec),

            0xCA => CpuInstruction::new(opcode, cpu, "dex", AddressingMode::Implied, &dex),

            0x88 => CpuInstruction::new(opcode, cpu, "dey", AddressingMode::Implied, &dey),

            0x49 => CpuInstruction::new(opcode, cpu, "eor", AddressingMode::Immediate, &eor),
            0x45 => CpuInstruction::new(opcode, cpu, "eor", AddressingMode::ZeroPage, &eor),
            0x55 => CpuInstruction::new(opcode, cpu, "eor", AddressingMode::ZeroPageX, &eor),
            0x4D => CpuInstruction::new(opcode, cpu, "eor", AddressingMode::Absolute, &eor),
            0x5D => CpuInstruction::new(opcode, cpu, "eor", AddressingMode::AbsoluteX, &eor),
            0x59 => CpuInstruction::new(opcode, cpu, "eor", AddressingMode::AbsoluteY, &eor),
            0x41 => CpuInstruction::new(opcode, cpu, "eor", AddressingMode::IndexedIndirect, &eor),
            0x51 => CpuInstruction::new(opcode, cpu, "eor", AddressingMode::IndirectIndexed, &eor),

            0xE6 => CpuInstruction::new(opcode, cpu, "inc", AddressingMode::ZeroPage, &inc),
            0xF6 => CpuInstruction::new(opcode, cpu, "inc", AddressingMode::ZeroPageX, &inc),
            0xEE => CpuInstruction::new(opcode, cpu, "inc", AddressingMode::Absolute, &inc),
            0xFE => CpuInstruction::new(opcode, cpu, "inc", AddressingMode::AbsoluteX, &inc),

            0xE8 => CpuInstruction::new(opcode, cpu, "inx", AddressingMode::Implied, &inx),

            0xC8 => CpuInstruction::new(opcode, cpu, "iny", AddressingMode::Implied, &iny),

            0x4C => CpuInstruction::new(opcode, cpu, "jmp", AddressingMode::Absolute, &jmp),
            0x6C => CpuInstruction::new(opcode, cpu, "jmp", AddressingMode::Indirect, &jmp),

            0x20 => CpuInstruction::new(opcode, cpu, "jsr", AddressingMode::Absolute, &jsr),

            0xA9 => CpuInstruction::new(opcode, cpu, "lda", AddressingMode::Immediate, &lda),
            0xA5 => CpuInstruction::new(opcode, cpu, "lda", AddressingMode::ZeroPage, &lda),
            0xB5 => CpuInstruction::new(opcode, cpu, "lda", AddressingMode::ZeroPageX, &lda),
            0xAD => CpuInstruction::new(opcode, cpu, "lda", AddressingMode::Absolute, &lda),
            0xBD => CpuInstruction::new(opcode, cpu, "lda", AddressingMode::AbsoluteX, &lda),
            0xB9 => CpuInstruction::new(opcode, cpu, "lda", AddressingMode::AbsoluteY, &lda),
            0xA1 => CpuInstruction::new(opcode, cpu, "lda", AddressingMode::IndexedIndirect, &lda),
            0xB1 => CpuInstruction::new(opcode, cpu, "lda", AddressingMode::IndirectIndexed, &lda),

            0xA2 => CpuInstruction::new(opcode, cpu, "ldx", AddressingMode::Immediate, &ldx),
            0xA6 => CpuInstruction::new(opcode, cpu, "ldx", AddressingMode::ZeroPage, &ldx),
            0xB6 => CpuInstruction::new(opcode, cpu, "ldx", AddressingMode::ZeroPageY, &ldx),
            0xAE => CpuInstruction::new(opcode, cpu, "ldx", AddressingMode::Absolute, &ldx),
            0xBE => CpuInstruction::new(opcode, cpu, "ldx", AddressingMode::AbsoluteY, &ldx),

            0xA0 => CpuInstruction::new(opcode, cpu, "ldy", AddressingMode::Immediate, &ldy),
            0xA4 => CpuInstruction::new(opcode, cpu, "ldy", AddressingMode::ZeroPage, &ldy),
            0xB4 => CpuInstruction::new(opcode, cpu, "ldy", AddressingMode::ZeroPageX, &ldy),
            0xAC => CpuInstruction::new(opcode, cpu, "ldy", AddressingMode::Absolute, &ldy),
            0xBC => CpuInstruction::new(opcode, cpu, "ldy", AddressingMode::AbsoluteX, &ldy),

            0x4A => CpuInstruction::new(opcode, cpu, "lsr", AddressingMode::Acc, &lsr),
            0x46 => CpuInstruction::new(opcode, cpu, "lsr", AddressingMode::ZeroPage, &lsr),
            0x56 => CpuInstruction::new(opcode, cpu, "lsr", AddressingMode::ZeroPageX, &lsr),
            0x4E => CpuInstruction::new(opcode, cpu, "lsr", AddressingMode::Absolute, &lsr),
            0x5E => CpuInstruction::new(opcode, cpu, "lsr", AddressingMode::AbsoluteX, &lsr),

            0xEA => CpuInstruction::new(opcode, cpu, "nop", AddressingMode::Implied, &nop),

            0x09 => CpuInstruction::new(opcode, cpu, "ora", AddressingMode::Immediate, &ora),
            0x05 => CpuInstruction::new(opcode, cpu, "ora", AddressingMode::ZeroPage, &ora),
            0x15 => CpuInstruction::new(opcode, cpu, "ora", AddressingMode::ZeroPageX, &ora),
            0x0D => CpuInstruction::new(opcode, cpu, "ora", AddressingMode::Absolute, &ora),
            0x1D => CpuInstruction::new(opcode, cpu, "ora", AddressingMode::AbsoluteX, &ora),
            0x19 => CpuInstruction::new(opcode, cpu, "ora", AddressingMode::AbsoluteY, &ora),
            0x01 => CpuInstruction::new(opcode, cpu, "ora", AddressingMode::IndexedIndirect, &ora),
            0x11 => CpuInstruction::new(opcode, cpu, "ora", AddressingMode::IndirectIndexed, &ora),

            0x48 => CpuInstruction::new(opcode, cpu, "pha", AddressingMode::Implied, &pha),

            0x08 => CpuInstruction::new(opcode, cpu, "php", AddressingMode::Implied, &php),

            0x68 => CpuInstruction::new(opcode, cpu, "pla", AddressingMode::Implied, &pla),

            0x28 => CpuInstruction::new(opcode, cpu, "plp", AddressingMode::Implied, &plp),

            0x2A => CpuInstruction::new(opcode, cpu, "rol", AddressingMode::Acc, &rol),
            0x26 => CpuInstruction::new(opcode, cpu, "rol", AddressingMode::ZeroPage, &rol),
            0x36 => CpuInstruction::new(opcode, cpu, "rol", AddressingMode::ZeroPageX, &rol),
            0x2E => CpuInstruction::new(opcode, cpu, "rol", AddressingMode::Absolute, &rol),
            0x3E => CpuInstruction::new(opcode, cpu, "rol", AddressingMode::AbsoluteX, &rol),

            0x6A => CpuInstruction::new(opcode, cpu, "ror", AddressingMode::Acc, &ror),
            0x66 => CpuInstruction::new(opcode, cpu, "ror", AddressingMode::ZeroPage, &ror),
            0x76 => CpuInstruction::new(opcode, cpu, "ror", AddressingMode::ZeroPageX, &ror),
            0x6E => CpuInstruction::new(opcode, cpu, "ror", AddressingMode::Absolute, &ror),
            0x7E => CpuInstruction::new(opcode, cpu, "ror", AddressingMode::AbsoluteX, &ror),

            0x40 => CpuInstruction::new(opcode, cpu, "rti", AddressingMode::Implied, &rti),

            0x60 => CpuInstruction::new(opcode, cpu, "rts", AddressingMode::Implied, &rts),

            0xE9 => CpuInstruction::new(opcode, cpu, "sbc", AddressingMode::Immediate, &sbc),
            0xE5 => CpuInstruction::new(opcode, cpu, "sbc", AddressingMode::ZeroPage, &sbc),
            0xF5 => CpuInstruction::new(opcode, cpu, "sbc", AddressingMode::ZeroPageX, &sbc),
            0xED => CpuInstruction::new(opcode, cpu, "sbc", AddressingMode::Absolute, &sbc),
            0xFD => CpuInstruction::new(opcode, cpu, "sbc", AddressingMode::AbsoluteX, &sbc),
            0xF9 => CpuInstruction::new(opcode, cpu, "sbc", AddressingMode::AbsoluteY, &sbc),
            0xE1 => CpuInstruction::new(opcode, cpu, "sbc", AddressingMode::IndexedIndirect, &sbc),
            0xF1 => CpuInstruction::new(opcode, cpu, "sbc", AddressingMode::IndirectIndexed, &sbc),

            0x38 => CpuInstruction::new(opcode, cpu, "sec", AddressingMode::Implied, &sec),

            0xF8 => CpuInstruction::new(opcode, cpu, "sed", AddressingMode::Implied, &sed),

            0x78 => CpuInstruction::new(opcode, cpu, "sei", AddressingMode::Implied, &sei),

            0x85 => CpuInstruction::new(opcode, cpu, "sta", AddressingMode::ZeroPage, &sta),
            0x95 => CpuInstruction::new(opcode, cpu, "sta", AddressingMode::ZeroPageX, &sta),
            0x8D => CpuInstruction::new(opcode, cpu, "sta", AddressingMode::Absolute, &sta),
            0x9D => CpuInstruction::new(opcode, cpu, "sta", AddressingMode::AbsoluteX, &sta),
            0x99 => CpuInstruction::new(opcode, cpu, "sta", AddressingMode::AbsoluteY, &sta),
            0x81 => CpuInstruction::new(opcode, cpu, "sta", AddressingMode::IndexedIndirect, &sta),
            0x91 => CpuInstruction::new(opcode, cpu, "sta", AddressingMode::IndirectIndexed, &sta),

            0xdb => CpuInstruction::new(opcode, cpu, "stp", AddressingMode::Implied, &stp),

            0x86 => CpuInstruction::new(opcode, cpu, "stx", AddressingMode::ZeroPage, &stx),
            0x96 => CpuInstruction::new(opcode, cpu, "stx", AddressingMode::ZeroPageY, &stx),
            0x8E => CpuInstruction::new(opcode, cpu, "stx", AddressingMode::Absolute, &stx),

            0x84 => CpuInstruction::new(opcode, cpu, "sty", AddressingMode::ZeroPage, &sty),
            0x94 => CpuInstruction::new(opcode, cpu, "sty", AddressingMode::ZeroPageX, &sty),
            0x8C => CpuInstruction::new(opcode, cpu, "sty", AddressingMode::Absolute, &sty),

            0xAA => CpuInstruction::new(opcode, cpu, "tax", AddressingMode::Implied, &tax),

            0xA8 => CpuInstruction::new(opcode, cpu, "tay", AddressingMode::Implied, &tay),

            0xBA => CpuInstruction::new(opcode, cpu, "tsx", AddressingMode::Implied, &tsx),

            0x8A => CpuInstruction::new(opcode, cpu, "txa", AddressingMode::Implied, &txa),

            0x9A => CpuInstruction::new(opcode, cpu, "txs", AddressingMode::Implied, &txs),

            0x98 => CpuInstruction::new(opcode, cpu, "tya", AddressingMode::Implied, &tya),

            _ => panic!("opcode {:x} is not implemented!", opcode),
        }
    }
}
