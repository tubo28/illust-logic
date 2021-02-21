use crate::common::*;

#[test]
fn all_works() {
    assert_eq!(all(0), 0b0);
    assert_eq!(all(4), 0b1111);
}
pub fn all(width: usize) -> Mask {
    (1 << width) - 1
}

#[test]
fn subset_works() {
    assert!(subset_bit(0b0110, 0b0110));
    assert!(subset_bit(0b0110, 0b0100));
    assert!(!subset_bit(0b0101, 0b0010));
}
pub fn subset_bit(sup: Mask, sub: Mask) -> bool {
    sup & sub == sub
}

#[test]
fn minus_works() {
    assert_eq!(minus(0b1100, 0b1010), 0b0100);
}
pub fn minus(x: Mask, y: Mask) -> Mask {
    // 0 0 -> 0
    // 0 1 -> 0
    // 1 1 -> 0
    // 1 0 -> 1
    x & !y
}

pub fn distinct(x: Mask, y: Mask) -> bool {
    x & y == 0
}

pub fn set(x: Mask, i: usize) -> bool {
    x >> i & 1 == 1
}
