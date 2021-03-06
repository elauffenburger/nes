use std::cell::RefCell;
use std::rc::Rc;

use crate::bits::get_bit_val;
use crate::cart::mappers::{get_mapper, Mapper, MapperOptions};
use crate::cart::CartLoader;
use crate::cpu::Cpu;
use crate::nes::Nes;
use crate::util::take_elems;

const HEADER_SIZE: usize = 16;
const TRAINER_SIZE: usize = 512;
const PRG_ROM_UNIT_SIZE: usize = 16384;
const CHR_ROM_UNIT_SIZE: usize = 8192;

pub struct iNESLoader {}

impl iNESLoader {
    pub fn new() -> Self {
        iNESLoader {}
    }
}

impl<T> CartLoader<T> for iNESLoader
where
    T: Nes + 'static,
{
    fn load(&self, nes_ref: Rc<RefCell<T>>, cart_data: &[u8]) -> Result<(), String> {
        let cpu = nes_ref.borrow_mut().get_cpu();

        let header = read_header(cart_data)?;

        if header.has_trainer {
            let trainer = take_elems(cart_data, 16, TRAINER_SIZE)?;

            cpu.borrow_mut().write_bytes_to(&0x7000u16.into(), trainer);
        }

        let rom_addr_offset = match header.has_trainer {
            true => TRAINER_SIZE,
            false => 0,
        };

        let prg_rom_start_addr = rom_addr_offset + HEADER_SIZE;
        let prg_rom_end_addr =
            prg_rom_start_addr + (header.num_prg_rom_banks as usize * PRG_ROM_UNIT_SIZE);

        let prg_rom = &cart_data[prg_rom_start_addr..prg_rom_end_addr];

        let chr_rom_start_addr = prg_rom_end_addr;
        let chr_rom_end_addr =
            chr_rom_start_addr + (header.num_chr_rom_banks as usize * CHR_ROM_UNIT_SIZE);

        let chr_rom = &cart_data[chr_rom_start_addr..chr_rom_end_addr];

        let mapper = get_mapper(header.mapper_id)?;
        mapper.map(
            nes_ref,
            MapperOptions {
                cart_data,
                prg_rom,
                chr_rom,
            },
        )?;

        Ok(())
    }
}

fn read_header(cart_data: &[u8]) -> Result<iNESHeader, String> {
    match take_elems(cart_data, 0, 4)? {
        &[0x4E, 0x45, 0x53, 0x1A] => {}
        id @ _ => {
            return Err(format!(
                "Expected header ID to be 'NES' but received '{:?}'",
                &id
            ));
        }
        _ => return Err(format!("Unexpected non-ASCII sequence in header ID")),
    };

    let num_prg_rom_banks = match take_elems(cart_data, 4, 1) {
        Ok(num_banks) => num_banks[0],
        _ => return Err(format!("")),
    };

    let num_chr_rom_banks = match take_elems(cart_data, 5, 1) {
        Ok(num_banks) => num_banks[0],
        _ => return Err(format!("")),
    };

    let control_byte = match take_elems(cart_data, 6, 1) {
        Ok(byte) => byte[0],
        _ => return Err(format!("")),
    };

    let control_byte_2 = match take_elems(cart_data, 7, 1) {
        Ok(byte) => byte[0],
        _ => return Err(format!("")),
    };

    let num_ram_banks = match take_elems(cart_data, 8, 1) {
        Ok(num_banks) => num_banks[0],
        _ => return Err(format!("")),
    };

    match take_elems(cart_data, 9, 7) {
        Ok(&[0, 0, 0, 0, 0, 0, 0]) => {}
        // TODO: how to handle this?
        data @ _ => {}
    };

    let mapper_id_lo = control_byte >> 4;
    let mapper_id_hi = control_byte_2 & 0b11110000;
    let mapper_id = mapper_id_hi | mapper_id_lo;

    let has_trainer = get_bit_val(control_byte, 2);

    Ok(iNESHeader {
        num_prg_rom_banks: num_prg_rom_banks,
        num_chr_rom_banks: num_chr_rom_banks,
        mapper_id: mapper_id,
        has_trainer: has_trainer,
    })
}

struct iNESHeader {
    pub num_prg_rom_banks: u8,
    pub num_chr_rom_banks: u8,
    pub mapper_id: u8,
    pub has_trainer: bool,
}

#[cfg(test)]
mod test {}
