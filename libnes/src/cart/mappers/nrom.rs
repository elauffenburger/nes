use std::cell::RefCell;
use std::rc::Rc;

use crate::cart::mappers::{Mapper, MapperOptions};
use crate::nes::Nes;

pub struct NROMMapper {}

impl NROMMapper {
    pub fn new() -> Self {
        NROMMapper {}
    }
}

impl Mapper for NROMMapper {
    fn map(&self, nes: Rc<RefCell<Nes>>, options: MapperOptions) -> Result<(), String> {
        let cpu = nes.borrow_mut().get_cpu();

        cpu.borrow_mut()
            .write_bytes_to(&0x8000u16.into(), &options.prg_rom);

        // TODO: actually impl mirroring
        // For now, just check if loading the prg rom would cause an overflow
        // to determine if we load lower prg rom or "fake mirror":
        if options.prg_rom.len() + 0xC000 <= 0x10000 {
            cpu.borrow_mut()
                .write_bytes_to(&0xC000u16.into(), &options.prg_rom);
        }

        let ppu = nes.borrow_mut().get_ppu();
        ppu.borrow_mut()
            .write_bytes_to(&0x0000u16.into(), &options.chr_rom);

        Ok(())
    }
}
