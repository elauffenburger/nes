use crate::bits::get_bit_val;
use crate::cart::mappers::{get_mapper, Mapper, MapperOptions};
use std::str;

use crate::cart::CartLoader;
use crate::cpu::Cpu;
use crate::util::get_range;

const TRAINER_SIZE: usize = 512;
const PRG_ROM_UNIT_SIZE: usize = 16000;
const CHR_ROM_UNIT_SIZE: usize = 8000;

pub struct iNESLoader {}

impl CartLoader for iNESLoader {
    fn load(&self, cpu: &mut Cpu, cart_data: &[u8]) -> Result<(), String> {
        let header = read_header(cart_data)?;

        if header.has_trainer {
            let trainer = get_range(cart_data, 16, TRAINER_SIZE)?;

            cpu.write_bytes_to(&0x7000.into(), trainer);
        }

        let rom_addr_offset = match header.has_trainer {
            true => TRAINER_SIZE,
            false => 0,
        };

        let prg_rom =
            &cart_data[rom_addr_offset..(header.num_prg_rom_banks as usize * PRG_ROM_UNIT_SIZE)];

        let mapper = get_mapper(header.mapper_id)?;
        mapper.map(
            cpu,
            MapperOptions {
                cart_data: cart_data,
                prg_rom: prg_rom,
            },
        )?;

        Ok(())
    }
}

fn read_header(cart_data: &[u8]) -> Result<iNESHeader, String> {
    match get_range(cart_data, 0, 3)? {
        &[0x4E, 0x45, 0x53, 0x1A] => {}
        id @ _ => {
            return Err(format!(
                "Expected header ID to be 'NES' but received '{:?}'",
                &id
            ));
        }
        _ => return Err(format!("Unexpected non-ASCII sequence in header ID")),
    };

    match get_range(cart_data, 3, 4)? {
        &[0x1A] => {}
        fmt @ _ => return Err(format!("Expected 0x1A at byte 3; received '{:x}'", fmt[0])),
    };

    let num_prg_rom_banks = match get_range(cart_data, 4, 1) {
        Ok(num_banks) => num_banks[0],
        _ => return Err(format!("")),
    };

    let num_chr_rom_banks = match get_range(cart_data, 5, 1) {
        Ok(num_banks) => num_banks[0],
        _ => return Err(format!("")),
    };

    let control_byte = match get_range(cart_data, 6, 1) {
        Ok(byte) => byte[0],
        _ => return Err(format!("")),
    };

    let control_byte_2 = match get_range(cart_data, 7, 1) {
        Ok(byte) => byte[0],
        _ => return Err(format!("")),
    };

    let num_ram_banks = match get_range(cart_data, 8, 1) {
        Ok(num_banks) => num_banks[0],
        _ => return Err(format!("")),
    };

    match get_range(cart_data, 9, 16) {
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
