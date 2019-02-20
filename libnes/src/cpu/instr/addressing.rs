use crate::cpu::Cpu;

#[derive(Debug)]
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