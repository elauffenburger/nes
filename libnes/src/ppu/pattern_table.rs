use crate::util::rc_ref;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Debug;

use super::NUM_TILES;
use crate::bits::get_bit_val_u8;

pub const TILE_PLANE_SIZE: usize = 0x08;
pub const TILE_SIZE: usize = 0x40;

pub struct PatternTable(Rc<RefCell<[PatternTableTile; NUM_TILES as usize]>>);

impl PatternTable {
    pub fn new() -> Self {
        let tiles = [PatternTableTile::default(); NUM_TILES as usize];

        PatternTable(rc_ref(tiles))
    }

    pub fn set_tile_at_index(
        &mut self,
        index: u8,
        tile: PatternTableTile,
    ) -> Result<(), &'static str> {
        if index > NUM_TILES {
            return Err("Index out of range");
        }

        self.0.borrow_mut()[index as usize] = tile;
        Ok(())
    }

    pub fn get_tile_at_index(&self, index: u8) -> Result<PatternTableTile, &'static str> {
        if index > NUM_TILES {
            return Err("Index out of range");
        }

        Ok(self.0.borrow_mut()[index as usize])
    }
}

impl Debug for PatternTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        for (i, tile) in self.0.borrow().iter().enumerate() {
            write!(f, "tile {:02x}:\n{:?}\n", i, tile)?;
        }

        Ok(())
    }
}

#[derive(Default, Clone, Copy)]
pub struct PatternTableTilePlane([u8; TILE_PLANE_SIZE]);

impl From<[u8; TILE_PLANE_SIZE]> for PatternTableTilePlane {
    fn from(arr: [u8; TILE_PLANE_SIZE]) -> Self {
        PatternTableTilePlane(arr)
    }
}

impl Debug for PatternTableTilePlane {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        for byte in self.0.iter() {
            write!(f, "{:08b}\n", byte)?;
        }

        Ok(())
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
                let plane_one_col_val = get_bit_val_u8(row_plane_one, 7 - col_index);
                let plane_two_col_val = get_bit_val_u8(row_plane_two, 7 - col_index);

                let color_val = plane_one_col_val | plane_two_col_val << 1;
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

impl Debug for PatternTableTile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        // Planes
        write!(f, "planes:\n")?;
        write!(f, "{:?}\n", &self.plane_one)?;
        write!(f, "{:?}\n", &self.plane_two)?;

        // Composed
        write!(f, "composite:\n")?;
        for (i, byte) in self.get_color_indices().iter().enumerate() {
            if i != 0 && i % TILE_PLANE_SIZE == 0 {
                write!(f, "\n")?;
            }

            write!(f, "{:02x} ", byte)?;
        }

        write!(f, "\n")
    }
}
