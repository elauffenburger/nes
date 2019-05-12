use std::rc::Rc;
use std::cell::RefCell;

use crate::nes::Nes;

mod nrom;

pub use nrom::*;

pub trait Mapper {
    fn map(&self, nes: Rc<RefCell<Nes>>, options: MapperOptions) -> Result<(), String>;
}

pub struct MapperOptions<'a> {
    pub cart_data: &'a [u8],
    pub prg_rom: &'a [u8],
    pub chr_rom: &'a [u8],
}

pub fn get_mapper(id: u8) -> Result<Box<impl Mapper>, String> 
{
    match id {
        0 => Ok(Box::from(NROMMapper::new())),
        _ => Err(format!("Unsupported Mapper '{}'", id)),
    }
}
