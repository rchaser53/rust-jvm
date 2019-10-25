use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

pub fn read_file<'a, P: AsRef<Path>>(
    input: &'a P,
    buffer: &'a mut Vec<u8>,
) -> Result<&'a mut [u8]> {
    let mut f = File::open(input)?;
    f.read_to_end(buffer)?;
    Ok(buffer.as_mut_slice())
}

pub fn extract_x_byte_as_vec(input: &mut [u8], index: usize, x: usize) -> (Vec<u8>, usize) {
    let mut result = Vec::with_capacity(x);
    for i in 0..x {
        result.push(input[index + i]);
    }
    (result, index + x)
}

pub fn extract_x_byte_as_usize(input: &mut [u8], index: usize, x: usize) -> (usize, usize) {
    let mut result: usize = 0;
    for i in 0..x {
        result += (input[index + i] as usize) << (x - i - 1) * 8;
    }
    (result, index + x)
}

#[macro_export]
macro_rules! add_flags {
    ($flags:expr, $num:expr, $flag:expr) => {
        if $num & $flag as usize != 0 {
            $flags.push($flag)
        }
    };
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
