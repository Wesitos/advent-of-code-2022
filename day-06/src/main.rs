use std::collections::HashSet;

use helpers::read_chars;

const INPUT_PATH: &str = "./input.txt";

struct Ring<T: Copy> {
    buf: Vec<T>,
    idx: usize,
    size: usize,
}

impl<'a, T: Copy> Ring<T> {
    fn push(&mut self, value: T) {
        if self.buf.len() < self.size {
            self.buf.push(value);
        } else {
            self.buf[self.idx] = value;
        }
        self.idx = (self.idx + 1) % self.size;
    }

    fn new(value: T, size: usize) -> Self {
        Self {
            buf: vec![value; size],
            idx: 0,
            size,
        }
    }

    fn as_vec(&self) -> Vec<T> {
        let mut v = Vec::from(&self.buf[self.idx..]);
        v.extend_from_slice(&self.buf[..self.idx]);
        v
    }
}

fn find_start(chars: &mut impl Iterator<Item=char>, marker_size: usize) -> i32 {
    let enumerated_iter = chars.enumerate();

    let mut ring = Ring::new(' ', marker_size);
    let mut set = HashSet::new();

    for (idx, c) in enumerated_iter {
        ring.push(c);

        if idx < marker_size {
            continue;
        }

        let is_marker_start = (*ring.as_vec()).iter().all(|c| set.insert(*c));

        if is_marker_start {
            return idx as i32 + 1;
        }

        set.drain();
    }

    return -1;
}

fn main() {
    const BUFFER_SIZE: usize = 4;

    if let Ok(chars) = read_chars(INPUT_PATH) {
        let mut char_iter = chars.map(|res| res.unwrap());

        let packet_start = find_start(&mut char_iter, 4);

        println!("Start of packet detected at position: {}", packet_start);
    } else {
        panic!("Error reading file");
    }

    if let Ok(chars) = read_chars(INPUT_PATH) {
        let mut char_iter = chars.map(|res| res.unwrap());

        let message_start = find_start(&mut char_iter, 14);

        println!("Start of message detected at position: {}", message_start);
    } else {
        panic!("Error reading file");
    }
}
