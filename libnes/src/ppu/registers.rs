use crate::bits::get_bit_val;
use crate::bits::get_bit_val_u8;

// TODO: actually write what the default state of the reg should be
#[derive(Default)]
pub struct PpuCtrlRegister {
    pub nametable_index: u8,
    pub vram_addr_incr: u16,
    pub sprite_pattern_table_addr: u16,
    pub bg_pattern_table_index: u8,
    pub sprite_size_type: bool,
    pub ppu_master_slave_select: bool,
    pub gen_nmi: bool,
}

impl From<u8> for PpuCtrlRegister {
    fn from(byte: u8) -> Self {
        PpuCtrlRegister {
            nametable_index: {
                let lo = get_bit_val_u8(byte, 0);
                let hi = get_bit_val_u8(byte, 1);

                lo & (hi << 1)
            },
            vram_addr_incr: match get_bit_val_u8(byte, 2) {
                0 => 1,
                1 => 32,
                _ => panic!(),
            },
            sprite_pattern_table_addr: match get_bit_val(byte, 3) {
                false => 0x0000,
                true => 0x1000,
            },
            bg_pattern_table_index: match get_bit_val(byte, 4) {
                false => 0,
                true => 1,
            },
            sprite_size_type: get_bit_val(byte, 5),
            ppu_master_slave_select: get_bit_val(byte, 6),
            gen_nmi: get_bit_val(byte, 7),
        }
    }
}
