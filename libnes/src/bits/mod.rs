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

pub fn to_bytes<'a>(byte_str: &'a str) -> Vec<u8> {
    byte_str
        .split(" ")
        .map(|b| {
            let byte = u8::from_str_radix(b.clone(), 16).unwrap();

            byte
        })
        .collect::<Vec<u8>>()
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

pub enum RotateDirection {
    Left,
    Right,
}

pub fn rotate(value: u8, direction: RotateDirection) -> (u8, bool) {
    match direction {
        RotateDirection::Left => {
            let old_bit_7 = get_bit_val(value, 7);

            (set_bit_val(value << 1, 0, old_bit_7), old_bit_7)
        }
        RotateDirection::Right => {
            let old_bit_0 = get_bit_val(value, 0);

            (set_bit_val(value >> 1, 7, old_bit_0), old_bit_0)
        }
    }
}

fn guard_bit_op(bit_num: usize) {
    if bit_num < 0 || bit_num > 7 {
        panic!("bit_num {} must be between 0 and 7", bit_num);
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

    #[test]
    fn can_rotate_left() {
        assert_eq!(
            (0b00000000, false),
            rotate(0b00000000, RotateDirection::Left)
        );
        assert_eq!(
            (0b00000010, false),
            rotate(0b00000001, RotateDirection::Left)
        );
        assert_eq!(
            (0b00000001, true),
            rotate(0b10000000, RotateDirection::Left)
        );
        assert_eq!(
            (0b00100011, true),
            rotate(0b10010001, RotateDirection::Left)
        );
    }

    #[test]
    fn can_rotate_right() {
        assert_eq!(
            (0b00000000, false),
            rotate(0b00000000, RotateDirection::Right)
        );
        assert_eq!(
            (0b00000001, false),
            rotate(0b00000010, RotateDirection::Right)
        );
        assert_eq!(
            (0b10000000, true),
            rotate(0b00000001, RotateDirection::Right)
        );
        assert_eq!(
            (0b10010001, true),
            rotate(0b00100011, RotateDirection::Right)
        );
    }
}
