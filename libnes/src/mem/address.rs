use std::convert::{From, Into};
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub enum Address {
    ZeroPage(u16),
    Stack(u16),
    Ram(u16),
    RamMirror(u16),
    IoRegisters1(u16),
    IoRegistersMirror(u16),
    IoRegisters2(u16),
    ExpansionRom(u16),
    Sram(u16),
    PrgRomLowerBank(u16),
    PrgRomUpperBank(u16),
}

impl From<u16> for Address {
    fn from(val: u16) -> Self {
        match val {
            0x0000...0x00ff => Address::ZeroPage(val),
            0x0100...0x01ff => Address::Stack(val),
            0x0200...0x07ff => Address::Ram(val),
            0x0800...0x1fff => Address::RamMirror(val),
            0x2000...0x2007 => Address::IoRegisters1(val),
            0x2008...0x3fff => Address::IoRegistersMirror(val),
            0x4000...0x401f => Address::IoRegisters2(val),
            0x4020...0x5fff => Address::ExpansionRom(val),
            0x6000...0x7fff => Address::Sram(val),
            0x8000...0xbfff => Address::PrgRomLowerBank(val),
            0xc000...0xffff => Address::PrgRomUpperBank(val),
            _ => panic!(format!(
                "Bad address '{:x}' while converting from u16 to Address",
                val
            )),
        }
    }
}

impl From<u8> for Address {
    fn from(val: u8) -> Self {
        (val as u16).into()
    }
}

impl From<i8> for Address {
    fn from(val: i8) -> Self {
        (val as u16).into()
    }
}

impl From<i32> for Address {
    fn from(val: i32) -> Self {
        (val as u16).into()
    }
}

impl Into<u16> for &Address {
    fn into(self) -> u16 {
        match self {
            &Address::ZeroPage(val) => val,
            &Address::Stack(val) => val,
            &Address::Ram(val) => val,
            &Address::RamMirror(val) => val,
            &Address::IoRegisters1(val) => val,
            &Address::IoRegistersMirror(val) => val,
            &Address::IoRegisters2(val) => val,
            &Address::ExpansionRom(val) => val,
            &Address::Sram(val) => val,
            &Address::PrgRomLowerBank(val) => val,
            &Address::PrgRomUpperBank(val) => val,
        }
    }
}

impl Into<u16> for Address {
    fn into(self) -> u16 {
        (&self).into()
    }
}

impl Add<u16> for &Address {
    type Output = Address;

    fn add(self, raw_addr: u16) -> Address {
        Address::add(self, raw_addr as i16)
    }
}

impl Add<i8> for &Address {
    type Output = Address;

    fn add(self, raw_addr: i8) -> Address {
        Address::add(self, raw_addr as i16)
    }
}

impl Address {
    fn add(&self, raw_addr: i16) -> Address {
        let addr: u16 = self.into();

        // todo: implement 6502 bugs & wrapping, etc.
        ((addr as i16 + raw_addr) as u16).into()
    }
}

#[cfg(test)]
mod test {
    use super::Address;

    #[test]
    fn from_zero_page_addr() {
        for addr in vec![0x0000, 0x00fa, 0x00ff] {
            assert_eq!(Address::from(addr), Address::ZeroPage(addr));
        }

        assert_ne!(Address::from(0x0100), Address::ZeroPage(0x0100));
    }

    #[test]
    fn from_stack_addr() {
        for addr in vec![0x0100, 0x01ab, 0x01ff] {
            assert_eq!(Address::from(addr), Address::Stack(addr));
        }

        assert_ne!(Address::from(0x0200), Address::Stack(0x0200));
    }

    #[test]
    fn from_ram_addr() {
        for addr in vec![0x0200, 0x07ff] {
            assert_eq!(Address::from(addr), Address::Ram(addr));
        }

        assert_ne!(Address::from(0x0800), Address::Ram(0x0800));
    }

    #[test]
    fn from_ram_mirror_addr() {
        for addr in vec![0x0800, 0x1fff] {
            assert_eq!(Address::from(addr), Address::RamMirror(addr));
        }

        assert_ne!(Address::from(0x2000), Address::RamMirror(0x2000));
    }

    #[test]
    fn from_io_registers1_addr() {
        for addr in vec![0x2000, 0x2007] {
            assert_eq!(Address::from(addr), Address::IoRegisters1(addr));
        }

        assert_ne!(Address::from(0x2008), Address::IoRegisters1(0x2008));
    }

    #[test]
    fn from_io_registers_mirror_addr() {
        for addr in vec![0x2008, 0x3fff] {
            assert_eq!(Address::from(addr), Address::IoRegistersMirror(addr));
        }

        assert_ne!(Address::from(0x4000), Address::IoRegistersMirror(0x4000));
    }

    #[test]
    fn from_io_registers2_addr() {
        for addr in vec![0x4000, 0x401f] {
            assert_eq!(Address::from(addr), Address::IoRegisters2(addr));
        }

        assert_ne!(Address::from(0x4020), Address::IoRegisters2(0x4020));
    }

    #[test]
    fn from_expansion_rom_addr() {
        for addr in vec![0x4020, 0x5fff] {
            assert_eq!(Address::from(addr), Address::ExpansionRom(addr));
        }

        assert_ne!(Address::from(0x6000), Address::ExpansionRom(0x6000));
    }

    #[test]
    fn from_sram_addr() {
        for addr in vec![0x6000, 0x7fff] {
            assert_eq!(Address::from(addr), Address::Sram(addr));
        }

        assert_ne!(Address::from(0x8000), Address::Sram(0x8000));
    }

    #[test]
    fn from_prg_rom_lower_addr() {
        for addr in vec![0x8000, 0xbfff] {
            assert_eq!(Address::from(addr), Address::PrgRomLowerBank(addr));
        }

        assert_ne!(Address::from(0xc000), Address::Sram(0xc000));
    }

    #[test]
    fn from_prg_rom_upper_addr() {
        for addr in vec![0xc000, 0xffff] {
            assert_eq!(Address::from(addr), Address::PrgRomUpperBank(addr));
        }
    }
}
