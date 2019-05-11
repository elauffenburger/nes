pub use super::pattern_table::*;
pub use super::attr_table::*;
pub use super::tiles::*;

pub const NAME_TABLE_SIZE: usize = 0x03c0;

pub type NameTableData<'table> = &'table [u8; NAME_TABLE_SIZE];

pub struct NameTable<'table> {
    pub index: u8,
    pub data: NameTableData<'table>,
    pub attribute_table: AttributeTable<'table>,
}

impl<'table> NameTable<'table> {
    pub fn get_tile_at_loc(&self, row: u8, col: u8, pattern_table: PatternTable) -> Option<Tile> {
        let index = (row as u16 * 30u16) + col as u16;

        if index > NAME_TABLE_SIZE as u16 {
            return None;
        }

        let pattern_table_tile_index = self.data[index as usize];
        let pattern_table_tile = pattern_table
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

                0b0001_1111 & (palette_num << 2) & color_index
            })
            .collect();

        Some(Tile {
            pattern_table_tile: pattern_table_tile,
            colors: colors,
        })
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
