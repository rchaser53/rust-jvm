use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn read_file<'a>(input: &'a str, buffer: &'a mut Vec<u8>) -> Result<&'a [u8]> {
    let mut f = File::open(input)?;
    f.read_to_end(buffer)?;
    Ok(buffer.as_slice())
}

pub fn extract_x_byte_as_vec(input: &mut Vec<u8>, index: usize, x: usize) -> (Vec<u8>, usize) {
    let mut result = Vec::with_capacity(x);
    for i in 0..x {
        result.push(input[index + i]);
    }
    (result, index + x)
}

pub fn extract_x_byte_as_usize(input: &mut Vec<u8>, index: usize, x: usize) -> (usize, usize) {
    let mut result: usize = 0;
    for i in 0..x {
        result += (input[index + i] as usize) << (x - i - 1) * 8;
    }
    (result, index + x)
}

#[test]
pub fn test_extract_x_byte_as_vec() {
    let mut input = vec![1, 2, 3, 4];
    assert_eq!(extract_x_byte_as_vec(&mut input, 1, 2), (vec![2, 3], 3));

    let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(
        extract_x_byte_as_vec(&mut input, 3, 4),
        (vec![4, 5, 6, 7], 7)
    );
}

#[test]
pub fn test_extract_x_byte_as_usize() {
    let mut input = vec![1, 2, 3, 4];
    assert_eq!(
        extract_x_byte_as_usize(&mut input, 1, 2),
        ((2 << 8) as usize + 3, 3)
    );
    assert_eq!(input[3], 4);

    let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let fourth = 1 << (8 * 3);
    let third = 2 << (8 * 2);
    let second = 3 << (8 * 1);
    let first = 4 << (8 * 0);

    assert_eq!(
        extract_x_byte_as_usize(&mut input, 0, 4),
        (first + second + third + fourth, 4)
    );
    assert_eq!(input[4], 5);
}
