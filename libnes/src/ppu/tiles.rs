use super::pattern_table::*;

pub struct Tile {
    pub pattern_table_tile: PatternTableTile,
    pub colors: Vec<u8>,
    pub index: u16,
    pub pattern_table_tile_index: u8,
}