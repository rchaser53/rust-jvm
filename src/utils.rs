use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn read_file<'a>(input: &'a str, buffer: &'a mut Vec<u8>) -> Result<&'a [u8]> {
    let mut f = File::open(input)?;
    f.read_to_end(buffer)?;
    Ok(buffer.as_slice())
}

pub fn extract_x_byte(input: &mut Vec<u8>, index: usize, x: usize) -> (Vec<u8>, usize) {
    let mut result = Vec::with_capacity(x);
    for i in 0..x {
        result.push(input[index + i]);
    }
    (result, index + x + 1)
}

#[test]
pub fn test_extract_x_byte() {
    let mut input = vec![1, 2, 3, 4];
    assert_eq!(extract_x_byte(&mut input, 1, 2), (vec![2, 3], 4));

    let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(extract_x_byte(&mut input, 3, 4), (vec![4, 5, 6, 7], 8));
}
