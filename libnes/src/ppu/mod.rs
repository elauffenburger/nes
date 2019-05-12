pub mod attr_table;
pub mod mem;
pub mod name_table;
pub mod pattern_table;
pub mod registers;
pub mod tiles;

use crate::bits::u16_from_u8s;
use crate::bits::{get_bit_val, get_bit_val_u8};
use crate::cpu::mem::CpuMemoryAccessEvent;
use crate::cpu::Cpu;

use mem::{Address, DefaultPpuMemoryMap, PpuMemoryMap};
use name_table::*;
use pattern_table::*;
use registers::*;
use std::cell::RefCell;
use std::rc::Rc;

pub const PATTERN_TABLE_ONE_START_ADDR: u16 = 0x0000;
pub const PATTERN_TABLE_TWO_START_ADDR: u16 = 0x1000;
pub const NUM_TILES: u8 = 0xff;

pub trait Ppu {
    fn start(&mut self);
    fn clock(&mut self);

    fn get_pattern_tables(&self) -> [PatternTable; 2];
    fn get_name_table(&self, table_index: u8) -> Option<NameTable>;

    fn write_bytes_to(&mut self, start_addr: &Address, bytes: &[u8]);
    fn read_bytes(&mut self, start_addr: &Address, num_bytes: u16) -> Vec<u8>;

    fn wire_cpu(&mut self, cpu: Rc<RefCell<Cpu>>);
}

pub struct DefaultPpu {
    ppuaddr: u16,
    pending_ppuaddr_hi: Option<u8>,
    mem: Box<PpuMemoryMap>,
}

impl Ppu for DefaultPpu {
    fn start(&mut self) {
        // TODO: impl
    }

    fn clock(&mut self) {
        let ppuctrl = self.read_ppuctrl();

        let ppuaddr = self.read_ppuaddr();
        if let Some(addr_hi) = self.pending_ppuaddr_hi {
            self.ppuaddr = u16_from_u8s(ppuaddr, addr_hi);

            self.pending_ppuaddr_hi = None;
        } else {
            self.pending_ppuaddr_hi = Some(ppuaddr);
        }
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

    fn write_bytes_to(&mut self, start_addr: &Address, bytes: &[u8]) {
        let raw_start_addr: u16 = start_addr.into();

        for (i, byte) in bytes.iter().enumerate() {
            let addr = (raw_start_addr + (i as u16)).into();

            self.mem.set(&addr, byte.clone());
        }
    }

    fn read_bytes(&mut self, start_addr: &Address, num_bytes: u16) -> Vec<u8> {
        let raw_start_addr: u16 = start_addr.into();

        let mut bytes = vec![];
        for i in raw_start_addr..raw_start_addr + num_bytes {
            let addr: Address = i.into();

            bytes.push(self.mem.get(&addr))
        }

        bytes
    }

    fn wire_cpu(&mut self, cpu: Rc<RefCell<Cpu>>) {
        cpu.borrow_mut()
            .subscribe_mem(Box::from(|event: &CpuMemoryAccessEvent| {
                self.on_cpu_memory_access(event)
            }));
    }
}

impl DefaultPpu {
    pub fn new() -> Self {
        DefaultPpu {
            mem: Box::from(DefaultPpuMemoryMap::new()),
            pending_ppuaddr_hi: None,
            ppuaddr: 0x0000,
        }
    }

    fn on_cpu_memory_access(&mut self, event: &CpuMemoryAccessEvent) {}

    pub fn read_pattern_table_at(&self, start_addr: u16) -> PatternTable {
        let mut table = PatternTable::new();
        let mut tile_index = 0u8;

        while tile_index < NUM_TILES {
            let addr_offset = (tile_index as u16) * (TILE_PLANE_SIZE as u16) * 2;

            let plane_one_addr = start_addr + addr_offset as u16;
            let plane_two_addr = plane_one_addr + TILE_PLANE_SIZE as u16;

            let plane_one = self.read_tile_plane_from(plane_one_addr);
            let plane_two = self.read_tile_plane_from(plane_two_addr);

            let tile = PatternTableTile::new(plane_one, plane_two);

            table.set_tile_at_index(tile_index, tile).unwrap();

            tile_index += 1;
        }

        table
    }

    fn read_tile_plane_from(&self, start_addr: u16) -> PatternTableTilePlane {
        let mut plane = [0u8; TILE_PLANE_SIZE];

        for i in 0..TILE_PLANE_SIZE {
            let byte_addr = start_addr + i as u16;
            let byte = self.mem.get(&byte_addr.into());

            plane[i as usize] = byte;
        }

        PatternTableTilePlane::from(plane)
    }

    fn read_ppuaddr(&mut self) -> u8 {
        self.mem.get(&0x2006u16.into())
    }
}

impl DefaultPpu {
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
