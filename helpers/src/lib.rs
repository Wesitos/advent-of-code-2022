use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_chars<P>(filename: P) -> io::Result<impl Iterator<Item = io::Result<char>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).bytes().map(|r| match r {
        Ok(b) => Ok(char::from_u32(b as u32).unwrap()),
        Err(err) => Err(err),
    }))
}
