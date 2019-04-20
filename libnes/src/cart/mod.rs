use crate::cpu::Cpu;

pub mod ines;

pub trait CartLoader {
    fn load(cpu: &mut Cpu, cart_data: &[u8]) -> Result<(), String>;
}