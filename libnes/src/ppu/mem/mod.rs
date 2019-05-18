use std::ops::Add;

const PPU_MEMORY_MAP_SIZE: u32 = 0x10000u32;

#[derive(Debug, PartialEq)]
pub enum Address {
    Address(u16),
    Mirror {
        mirror_lo: u16,
        mirror_hi: u16,
        addr: u16,
    },
}

impl From<u16> for Address {
    fn from(addr: u16) -> Self {
        match addr {
            0x0000...0x2fff => Address::Address(addr),
            0x3000...0x3eff => Address::Mirror {
                mirror_lo: 0x2000,
                mirror_hi: 0x2eff,
                addr
            },
            0x3f00...0x3f1f => Address::Address(addr),
            0x3f20...0x3fff => Address::Mirror {
                mirror_lo: 0x3f00,
                mirror_hi: 0x3f1f,
                addr
            },
            0x4000...0x9fff => Address::Mirror {
                mirror_lo: 0x0000,
                mirror_hi: 0x3fff,
                addr
            },
            _ => panic!(format!(
                "Bad address '{:#06x}' while converting from u16 to Address",
                addr
            )),
        }
    }
}

impl Into<u16> for &Address {
    fn into(self) -> u16 {
        self.get_addr()
    }
}

impl Into<u16> for Address {
    fn into(self) -> u16 {
        (&self).into()
    }
}

impl From<u8> for Address {
    fn from(val: u8) -> Self {
        (val as u16).into()
    }
}

impl Add<i8> for &Address {
    type Output = Address;

    fn add(self, offset: i8) -> Address {
        Address::add_signed_offset(self, offset)
    }
}

impl Add<u8> for &Address {
    type Output = Address;

    fn add(self, offset: u8) -> Address {
        Address::add_unsigned_offset(self, offset)
    }
}

impl Clone for Address {
    fn clone(&self) -> Self {
        let as_u16: u16 = self.into();

        as_u16.into()
    }
}

impl Address {
    pub fn get_addr(&self) -> u16 {
        match self {
            Address::Address(addr) => addr.clone(),
            Address::Mirror {
                mirror_lo,
                mirror_hi,
                addr,
            } => {
                let mirror_size = mirror_hi - mirror_lo;
                let effective_addr = mirror_lo + (addr % mirror_size);

                effective_addr
            }
        }
    }

    fn add_unsigned_offset(&self, offset: u8) -> Address {
        let addr: u16 = self.into();

        addr.wrapping_add(offset as u16).into()
    }

    fn add_signed_offset(&self, offset: i8) -> Address {
        let addr: u16 = self.into();

        match offset >= 0 {
            true => addr.wrapping_add(offset as u16).into(),
            false => addr.wrapping_sub((offset * -1) as u16).into(),
        }
    }
}

pub trait PpuMemoryMap {
    fn get(&self, addr: &Address) -> u8;
    fn set(&mut self, addr: &Address, val: u8) -> ();
}

pub struct DefaultPpuMemoryMap {
    memory: Box<[u8; PPU_MEMORY_MAP_SIZE as usize]>,
}

impl PpuMemoryMap for DefaultPpuMemoryMap {
    fn get(&self, addr: &Address) -> u8 {
        let effective_addr = addr.get_addr();

        self.memory[effective_addr as usize]
    }

    fn set(&mut self, addr: &Address, val: u8) -> () {
        let effective_addr = addr.get_addr();

        self.memory[effective_addr as usize] = val;
    }
}

impl DefaultPpuMemoryMap {
    pub fn new() -> Self {
        DefaultPpuMemoryMap {
            memory: Box::from([0u8; PPU_MEMORY_MAP_SIZE as usize])
        }
    }
}