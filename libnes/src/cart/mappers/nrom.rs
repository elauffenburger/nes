use crate::cart::mappers::{Mapper, MapperOptions};
use crate::cpu::Cpu;

pub struct NROMMapper {}

impl NROMMapper {
    pub fn new() -> Self {
        NROMMapper {}
    }
}

impl Mapper for NROMMapper {
    fn map(&self, cpu: &mut Cpu, options: MapperOptions) -> Result<(), String> {
        cpu.write_bytes_to(&0x8000.into(), &options.prg_rom);
        // TODO: actually impl mirroring
        cpu.write_bytes_to(&0xC000.into(), &options.prg_rom);

        Ok(())
    }
}
