pub fn set_bit_val(num: u8, bit_num: usize, val: bool) -> u8 {
    let bit_val = match val {
        true => 1,
        false => 0,
    };

    num & (bit_val << bit_num)
}

pub fn lsb(num: u16) -> u8 {
    ((num << 8) >> 8) as u8
}

pub fn msb(num: u16) -> u8 {
    (num >> 8) as u8
}

pub fn bool_to_bit(val: bool) -> u8 {
    match val {
        true => 1,
        false => 0
    }
}