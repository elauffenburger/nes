use crate::bits::{bool_to_bit, set_bit_val};

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
        Registers {
            pc: 0x00,
            sp: 0xff,
            acc: 0x00,
            x: 0x00,
            y: 0x00,
            p: ProcStatusFlags::default(),
        }
    }
}

pub struct ProcStatusFlags {
    pub negative: bool,
    pub overflow: bool,
    pub break_command: bool,
    pub decimal_mode: bool,
    pub interrupt_disable: bool,
    pub zero: bool,
    pub carry: bool,
}

impl Default for ProcStatusFlags {
    fn default() -> Self {
        ProcStatusFlags {
            negative: false,
            overflow: false,
            break_command: true,
            decimal_mode: false,
            interrupt_disable: false,
            zero: false,
            carry: false,
        }
    }
}

impl std::fmt::Debug for ProcStatusFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "NV-BDIZC ({}{}1{}{}{}{}{})",
            bool_to_bit(self.negative),
            bool_to_bit(self.overflow),
            bool_to_bit(self.break_command),
            bool_to_bit(self.decimal_mode),
            bool_to_bit(self.interrupt_disable),
            bool_to_bit(self.zero),
            bool_to_bit(self.carry),
        )
    }
}

impl Into<u8> for ProcStatusFlags {
    fn into(self) -> u8 {
        self.into_u8()
    }
}

impl Clone for ProcStatusFlags {
    fn clone(&self) -> Self {
        ProcStatusFlags {
            carry: self.carry,
            zero: self.zero,
            interrupt_disable: self.interrupt_disable,
            decimal_mode: self.decimal_mode,
            break_command: self.break_command,
            overflow: self.overflow,
            negative: self.negative,
        }
    }
}

impl ProcStatusFlags {
    pub fn into_u8(&self) -> u8 {
        let mut result: u8 = 0;

        result = set_bit_val(result, 0, self.carry);
        result = set_bit_val(result, 1, self.zero);
        result = set_bit_val(result, 2, self.interrupt_disable);
        result = set_bit_val(result, 3, self.decimal_mode);
        result = set_bit_val(result, 4, self.break_command);
        result = set_bit_val(result, 5, true);
        result = set_bit_val(result, 6, self.overflow);
        result = set_bit_val(result, 7, self.negative);

        result
    }
}
