use crate::cpu::Cpu;

pub mod ines;
pub mod mappers;

use ines::iNESLoader;

pub trait CartLoader {
    fn load(&self, cpu: &mut Cpu, cart_data: &[u8]) -> Result<(), String>;
}

pub fn get_cart_loader(format: RomFormat) -> Result<impl CartLoader, String> {
    match format {
        RomFormat::iNes => Ok(iNESLoader::new()),
    }
}

pub enum RomFormat {
    iNes,
}
