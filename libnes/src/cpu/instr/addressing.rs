use crate::cpu::Cpu;

pub enum AddressingMode {
    ZeroPage,
    IndexedZeroPageX,
    IndexedZeroPageY,
    Absolute,
    IndexedAbsoluteX,
    IndexedAbsoluteY,
    Indirect,
    Implied,
    Acc,
    Immediate,
    Relative,
    IndexedIndirect,
    IndirectIndexed,
}