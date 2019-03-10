#[derive(Debug)]
pub enum AddressingMode {
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    Implied,
    Acc,
    Immediate,
    Relative,
    IndexedIndirect,
    IndirectIndexed,
}