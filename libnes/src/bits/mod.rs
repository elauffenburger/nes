pub fn get_bit_val(num: u8, bit_num: usize) -> bool {
    guard_bit_op(bit_num);

    let left = 7 - bit_num;
    (num << left) >> 7 == 1
}

pub fn set_bit_val(num: u8, bit_num: usize, val: bool) -> u8 {
    guard_bit_op(bit_num);

    match val {
        true => num | (1 << bit_num),
        false => num & !(1 << bit_num),
    }
}

fn guard_bit_op(bit_num: usize) {
    if bit_num < 0 || bit_num > 7 {
        panic!("bit_num {} must be between 0 and 7", bit_num);
    }
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
        false => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_get_bit() {
        assert_eq!(false, get_bit_val(0b00100100, 0));
        assert_eq!(false, get_bit_val(0b00100100, 7));
        assert_eq!(true, get_bit_val(0b00100100, 5));
        assert_eq!(true, get_bit_val(0b00100100, 2));
    }

    #[test]
    fn can_set_bit() {
        assert_eq!(0b00000001, set_bit_val(0b00000000, 0, true));
        assert_eq!(0b10000000, set_bit_val(0b00000000, 7, true));
        assert_eq!(0b10000000, set_bit_val(0b10010000, 4, false));
        assert_eq!(0b10010000, set_bit_val(0b10000000, 4, true));
    }
}