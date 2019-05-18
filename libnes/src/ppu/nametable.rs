pub use super::attr_table::*;
pub use super::pattern_table::*;
pub use super::tiles::*;
use std::cell::RefCell;
use std::rc::Rc;

pub const NAMETABLE_SIZE: usize = 0x03c0;
pub const NAMETABLE_ADDRESSES: [u16; 4] = [0x2000, 0x2400, 0x2800, 0x2c00];

// rows, cols
pub const NAMETABLE_DIMS: [u8; 2] = [30, 32];

pub type NameTableData = Vec<u8>;

pub struct NameTable {
    pub index: u8,
    pub data: NameTableData,
    pub attribute_table: AttributeTable,
}

impl NameTable {
    pub fn get_tile_at_loc(
        &self,
        row: u8,
        col: u8,
        pattern_table: &Rc<RefCell<PatternTable>>,
    ) -> Tile {
        let index = (row as u16 * NAMETABLE_DIMS[1] as u16) + col as u16;

        if index > NAMETABLE_SIZE as u16 {
            panic!("invalid tile index: {}", index);
        }

        let pattern_table_tile_index = self.data[index as usize];
        let pattern_table_tile = pattern_table
            .borrow()
            .get_tile_at_index(pattern_table_tile_index)
            .expect("couldn't find tile at index");

        let colors = pattern_table_tile
            .get_color_indices()
            .into_iter()
            .map(|color_index| {
                let palette_num: u8 = self
                    .attribute_table
                    .get_palette_num_for_tile_loc(row, col)
                    .expect("Failed to get palette_num for tile loc")
                    .into();

                // see https://wiki.nesdev.com/w/index.php/PPU_palettes
                // bg select is always 1
                let color = 0b0001_1111 & (palette_num << 2) & color_index;

                color
            })
            .collect();

        Tile {
            pattern_table_tile,
            colors,
            index,
            pattern_table_tile_index,
        }
    }
}

impl std::fmt::Debug for NameTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        for (i, tile_num) in self.data.iter().enumerate() {
            let col = i % NAMETABLE_DIMS[1] as usize;

            if col == 0 && i != 0 {
                write!(f, "\n")?;
            }

            write!(f, "{:02x} ", tile_num)?;
        }

        Ok(())
    }
}

pub struct Color {
    pub hue: u8,
    pub value: u8,
}

impl Into<u8> for &Color {
    fn into(self) -> u8 {
        (self.value << 4) | self.hue
    }
}

impl From<u8> for Color {
    fn from(val: u8) -> Self {
        Color {
            hue: 0x0f & val,
            value: ((0b00110000) & val) >> 4,
        }
    }
}

pub fn get_nametable_addr_at_index(index: u8) -> u16 {
    match index {
        i @ 0...3 => NAMETABLE_ADDRESSES[i as usize],
        _ => panic!("Impossible nametable index val: {}", index),
    }
}
