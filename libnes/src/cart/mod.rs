use crate::cpu::Cpu;

pub mod ines;
pub mod mappers;

use ines::iNESLoader;

pub trait CartLoader<T>
where
    T: Cpu,
{
    fn load(&self, cpu: &mut T, cart_data: &[u8]) -> Result<(), String>;
}

pub fn get_cart_loader<T>(format: RomFormat) -> Result<impl CartLoader<T>, String>
where
    T: Cpu,
{
    match format {
        RomFormat::iNes => Ok(iNESLoader::new()),
    }
}

pub enum RomFormat {
    iNes,
}
