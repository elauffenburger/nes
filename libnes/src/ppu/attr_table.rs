pub const ATTRIBUTE_TABLE_SIZE: usize = 0x40;
pub const ATTRIBUTE_TABLE_ADDRESSES: [u16; 4] = [0x23c0, 0x27c0, 0x2bc0, 0x2fc0];

#[derive(Clone)]
pub struct AttributeTableEntry(u8);

impl AttributeTableEntry {
    pub fn get_palette_num_for_quadrant(&self, quadrant: AttributeTableEntryQuadrant) -> u8 {
        match quadrant {
            AttributeTableEntryQuadrant::BottomRight => (0b1100_0000 & self.0) >> 6,
            AttributeTableEntryQuadrant::BottomLeft => (0b0011_0000 & self.0) >> 4,
            AttributeTableEntryQuadrant::TopLeft => (0b0000_1100 & self.0) >> 2,
            AttributeTableEntryQuadrant::TopRight => 0b0000_0011 & self.0,
        }
    }
}

pub struct AttributeTable(Vec<AttributeTableEntry>);

impl AttributeTable {
    pub fn new(data: Vec<u8>) -> Self {
        AttributeTable(data.into_iter().map(|b| AttributeTableEntry(b)).collect())
    }

    pub fn get_palette_num_for_tile_loc(&self, row: u8, col: u8) -> Option<u8> {
        let entry = self
            .get_entry_for_tile_loc(row, col)
            .expect("Invalid index into attr table");

        let tile_quadrant = self.get_quadrant_for_tile_loc(row, col);

        Some(entry.get_palette_num_for_quadrant(tile_quadrant))
    }

    pub fn get_entry_for_tile_loc(&self, row: u8, col: u8) -> Option<AttributeTableEntry> {
        let attr_table_row = row / 4u8;
        let attr_table_col = col / 4u8;

        let attr_table_index = (attr_table_row * 8) + attr_table_col;
        if attr_table_index > ATTRIBUTE_TABLE_SIZE as u8 {
            return None;
        }

        Some(self.0[attr_table_index as usize].clone())
    }

    pub fn get_quadrant_for_tile_loc(&self, row: u8, col: u8) -> AttributeTableEntryQuadrant {
        match col % 2 {
            0 => match row % 2 {
                0 => AttributeTableEntryQuadrant::TopLeft,
                1 => AttributeTableEntryQuadrant::BottomLeft,
                modulo @ _ => panic!("Impossible modulo result: {}", modulo),
            },
            1 => match row % 2 {
                0 => AttributeTableEntryQuadrant::TopRight,
                1 => AttributeTableEntryQuadrant::BottomRight,
                modulo @ _ => panic!("Impossible modulo result: {}", modulo),
            },
            modulo @ _ => panic!("Impossible modulo result: {}", modulo),
        }
    }
}

pub enum AttributeTableEntryQuadrant {
    BottomRight,
    BottomLeft,
    TopLeft,
    TopRight,
}

pub fn get_attribute_table_addr_at_index(index: u8) -> u16 {
    match index {
        i @ 0...3 => ATTRIBUTE_TABLE_ADDRESSES[i as usize],
        _ => panic!("Impossible attribtue table index val: {}", index)
    }
}