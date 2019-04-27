use std::convert::{From, Into};
use std::ops::Add;

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
            0x0000...0x07ff => Address::Address(addr),
            0x0800...0x1fff => Address::Mirror {
                mirror_lo: 0x0000,
                mirror_hi: 0x7fff,
                addr: addr,
            },
            0x2000...0x2007 => Address::Address(addr),
            0x2008...0x3fff => Address::Mirror {
                mirror_lo: 0x2000,
                mirror_hi: 0x2007,
                addr: addr,
            },
            0x4000...0xffff => Address::Address(addr),
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

#[cfg(test)]
mod test {
}
