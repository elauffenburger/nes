use crate::bits::get_bit_val;
use std::str;

use crate::cart::CartLoader;
use crate::cpu::Cpu;
use crate::util::get_range;

const TRAINER_SIZE: usize = 512;
const CHR_ROM_BANK_SIZE: usize = 8000;

pub struct iNESLoader {}

impl CartLoader for iNESLoader {
    fn load(cpu: &mut Cpu, cart_data: &[u8]) -> Result<(), String> {
        match str::from_utf8(get_range(cart_data, 0, 3)?) {
            Ok("NES") => {}
            Ok(id) => {
                return Err(format!(
                    "Expected header ID to be 'NES' but received '{}'",
                    id
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

        let has_trainer = get_bit_val(control_byte, 2);

        if has_trainer {
            let trainer = get_range(cart_data, 16, TRAINER_SIZE)?;

            cpu.write_bytes_to(&0x7000.into(), trainer);
        }

        let rom_addr_offset = match has_trainer {
            true => TRAINER_SIZE,
            false => 0,
        };

        // TODO: create appropriate memory mapper from memory mapper id
        // TODO: create memory mapper for UNROM
        // TODO: have UNROM load PRG-ROM banks

        Ok(())
    }
}

#[cfg(test)]
mod test {}
