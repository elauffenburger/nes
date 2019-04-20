use crate::cpu::Cpu;

mod nrom;

pub use nrom::*;

pub trait Mapper {
    fn map(&self, cpu: &mut Cpu, options: MapperOptions) -> Result<(), String>;
}

pub struct MapperOptions<'a> {
    pub cart_data: &'a [u8],
    pub prg_rom: &'a [u8],
}

pub fn get_mapper(id: u8) -> Result<impl Mapper, String> {
    match id {
        0 => Ok(NROMMapper::new()),
        _ => Err(format!("Unsupported Mapper '{}'", id)),
    }
}
