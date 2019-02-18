#[derive(Default)]
pub struct Registers {
    pub pc: u16,
    pub sp: u8,
    pub acc: i8,
    pub x: i8,
    pub y: i8,
    pub p: ProcStatusFlags,
}

impl Registers {
    pub fn new() -> Registers {
        Registers::default()
    }
}

#[derive(Default)]
pub struct ProcStatusFlags {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal_mode: bool,
    pub break_command: bool,
    pub overflow: bool,
    pub negative: bool
}