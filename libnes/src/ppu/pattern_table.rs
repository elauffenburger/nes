use super::NUM_TILES;

use crate::bits::get_bit_val_u8;

pub const TILE_PLANE_SIZE: usize = 0x08;
pub const TILE_SIZE: usize = 0x40;

pub struct PatternTable(Box<[PatternTableTile; NUM_TILES as usize]>);

impl PatternTable {
    pub fn new() -> Self {
        let mut tiles = [PatternTableTile::default(); NUM_TILES as usize];

        PatternTable(Box::from(tiles))
    }

    pub fn set_tile_at_index(
        &mut self,
        index: u8,
        tile: PatternTableTile,
    ) -> Result<(), &'static str> {
        if index > NUM_TILES {
            return Err("Index out of range");
        }

        self.0[index as usize] = tile;
        Ok(())
    }

    pub fn get_tile_at_index(&self, index: u8) -> Result<PatternTableTile, &'static str> {
        if index > NUM_TILES {
            return Err("Index out of range");
        }

        Ok(self.0[index as usize])
    }
}

#[derive(Default, Clone, Copy)]
pub struct PatternTableTilePlane([u8; TILE_PLANE_SIZE]);

impl From<[u8; TILE_PLANE_SIZE]> for PatternTableTilePlane {
    fn from(arr: [u8; TILE_PLANE_SIZE]) -> Self {
        PatternTableTilePlane(arr)
    }
}

#[derive(Default, Clone, Copy)]
pub struct PatternTableTile {
    pub plane_one: PatternTableTilePlane,
    pub plane_two: PatternTableTilePlane,
}

impl PatternTableTile {
    pub fn new(
        plane_one: PatternTableTilePlane,
        plane_two: PatternTableTilePlane,
    ) -> PatternTableTile {
        PatternTableTile {
            plane_one,
            plane_two,
        }
    }

    pub fn get_color_indices(&self) -> [u8; TILE_SIZE] {
        let mut colors = [0; TILE_SIZE];

        let mut i = 0;
        for row_index in 0..8 {
            let row_plane_one = self.plane_one.0[row_index];
            let row_plane_two = self.plane_two.0[row_index];

            for col_index in 0..8 {
                let plane_one_col_val = get_bit_val_u8(row_plane_one, col_index);
                let plane_two_col_val = get_bit_val_u8(row_plane_two, col_index);

                let color_val = plane_one_col_val & (plane_two_col_val << 1);
                let color = match color_val {
                    0...3 => color_val,
                    val @ _ => panic!("Impossible color val: {}", val),
                };

                colors[i] = color;

                i += 1;
            }
        }

        colors
    }
}
