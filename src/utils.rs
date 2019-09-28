use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn read_file<'a>(input: &'a str, buffer: &'a mut Vec<u8>) -> Result<&'a [u8]> {
    let mut f = File::open(input)?;
    f.read_to_end(buffer)?;
    Ok(buffer.as_slice())
}
