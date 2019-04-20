use crate::cpu::Cpu;

pub mod ines;
pub mod mappers;

pub trait CartLoader {
    fn load(&self, cpu: &mut Cpu, cart_data: &[u8]) -> Result<(), String>;
}