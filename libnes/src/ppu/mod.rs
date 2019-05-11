pub mod attr_table;
pub mod mem;
pub mod name_table;
pub mod pattern_table;
pub mod registers;
pub mod tiles;

use crate::bits::{get_bit_val, get_bit_val_u8};
use mem::PpuMemoryMap;
use name_table::*;
use pattern_table::*;
use registers::*;

pub const PATTERN_TABLE_ONE_START_ADDR: u16 = 0x0000;
pub const PATTERN_TABLE_TWO_START_ADDR: u16 = 0x1000;
pub const NUM_TILES: u8 = 0xff;

pub trait Ppu {
    fn clock(&mut self);
    fn get_pattern_tables(&self) -> [PatternTable; 2];
    fn get_name_table(&self, table_index: u8) -> Option<NameTable>;
}

pub struct DefaultPpu<TMemoryMap>
where
    TMemoryMap: PpuMemoryMap,
{
    mem: TMemoryMap,
}

impl<TMemoryMap> Ppu for DefaultPpu<TMemoryMap>
where
    TMemoryMap: PpuMemoryMap,
{
    fn clock(&mut self) {
        let ppuctrl = self.read_ppuctrl();
    }

    fn get_pattern_tables(&self) -> [PatternTable; 2] {
        [
            self.read_pattern_table_at(PATTERN_TABLE_ONE_START_ADDR),
            self.read_pattern_table_at(PATTERN_TABLE_TWO_START_ADDR),
        ]
    }

    fn get_name_table(&self, table_index: u8) -> Option<NameTable> {
        // TODO: impl
        None
    }
}

impl<TMemoryMap> DefaultPpu<TMemoryMap>
where
    TMemoryMap: PpuMemoryMap,
{
    pub fn read_pattern_table_at(&self, start_addr: u16) -> PatternTable {
        let mut table = PatternTable::new();

        let mut addr_offset = 0u16;
        let mut tile_index = 0u8;

        loop {
            if tile_index > NUM_TILES {
                break;
            }

            let tile_base_addr = start_addr + addr_offset;

            let plane_one = self.read_tile_plane_from(tile_base_addr);
            let plane_two = self.read_tile_plane_from(tile_base_addr + TILE_PLANE_SIZE as u16);
            let tile = PatternTableTile::new(plane_one, plane_two);

            table.set_tile_at_index(tile_index, tile);

            addr_offset += 2;
            tile_index += 1;
        }

        table
    }

    fn read_tile_plane_from(&self, start_addr: u16) -> PatternTableTilePlane {
        let mut plane = [0u8; TILE_PLANE_SIZE];

        for i in 0..TILE_PLANE_SIZE {
            let byte_addr = start_addr + i as u16;
            plane[i as usize] = self.mem.get(&byte_addr.into());
        }

        PatternTableTilePlane::from(plane)
    }
}

impl<TMemoryMap> DefaultPpu<TMemoryMap>
where
    TMemoryMap: PpuMemoryMap,
{
    fn read_ppuctrl(&self) -> PpuCtrlRegister {
        let byte = self.mem.get(&0x2000u16.into());

        PpuCtrlRegister {
            nametable_addr: {
                let lo = get_bit_val_u8(byte, 0);
                let hi = get_bit_val_u8(byte, 1);

                match lo & (hi << 1) {
                    0 => 0x2000,
                    1 => 0x2400,
                    2 => 0x2800,
                    3 => 0x2c00,
                    val @ _ => panic!("Impossible for nametable_addr val to be {}", val),
                }
            },
            vram_addr_incr_type: get_bit_val(byte, 2),
            sprite_pattern_table_addr: match get_bit_val(byte, 3) {
                false => 0x0000,
                true => 0x1000,
            },
            bg_pattern_table_addr: match get_bit_val(byte, 4) {
                false => 0x0000,
                true => 0x1000,
            },
            sprite_size_type: get_bit_val(byte, 5),
            ppu_master_slave_select: get_bit_val(byte, 6),
            gen_nmi: get_bit_val(byte, 7),
        }
    }
}
