use itertools::Itertools;

use helpers::read_lines;

const INPUT_PATH: &str = "./input.txt";

#[derive(Copy, Clone, Debug)]
struct Crate {
    label: char,
}

impl From<&str> for Crate {
    fn from(src: &str) -> Self {
        Crate {
            label: src
                .trim_end_matches([' ', '['])
                .chars()
                .take(1)
                .last()
                .unwrap(),
        }
    }
}

impl std::fmt::Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.label)
    }
}

#[derive(Clone)]
struct CrateStack {
    crates: Vec<Crate>,
}

impl CrateStack {
    fn new() -> Self {
        Self { crates: Vec::new() }
    }
    fn push(&mut self, crate_: Crate) {
        self.crates.push(crate_);
    }
    fn pop(&mut self) -> Option<Crate> {
        self.crates.pop()
    }
    fn drain(&mut self, num: usize) -> Option<Vec<Crate>> {
        if self.crates.len() < num {
            return None;
        }
        let vec = Vec::from_iter(self.crates.drain(self.len() - num..));

        Some(vec)
    }
    fn len(&self) -> usize {
        self.crates.len()
    }
    fn get_top(&self) -> Option<Crate> {
        self.crates.last().copied()
    }
}

impl IntoIterator for CrateStack {
    type Item = Crate;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.crates.into_iter()
    }
}

impl From<&mut [Crate]> for CrateStack {
    fn from(slice: &mut [Crate]) -> Self {
        CrateStack {
            crates: slice.to_vec(),
        }
    }
}

#[derive(Clone)]
struct Storage {
    stacks: Vec<CrateStack>,
}

impl Storage {
    fn new(num_stacks: usize) -> Self {
        Self {
            stacks: vec![CrateStack::new(); num_stacks],
        }
    }
    fn stack_len(&self, id: usize) -> Option<usize> {
        let stack = self.get_stack(id)?;
        Some(stack.len())
    }
    fn get_stack(&self, stack_id: usize) -> Option<&CrateStack> {
        self.stacks.get(stack_id)
    }
    fn get_mut_stack(&mut self, stack_id: usize) -> Option<&mut CrateStack> {
        self.stacks.get_mut(stack_id)
    }

    fn push_crate(&mut self, stack_id: usize, crate_: Crate) {
        self.get_mut_stack(stack_id).unwrap().push(crate_);
    }

    fn move_crate(&mut self, src_stack_id: usize, dst_stack_id: usize) -> Option<()> {
        if self.stack_len(src_stack_id)? == 0 {
            return None;
        }

        let src_stack = self.get_mut_stack(src_stack_id)?;
        let crate_ = src_stack.pop()?;

        let dst_stack = self.get_mut_stack(dst_stack_id)?;

        dst_stack.push(crate_);
        Some(())
    }

    fn move_crates(&mut self, src_stack_id: usize, dst_stack_id: usize, num: usize) -> Option<()> {
        if self.stack_len(src_stack_id)? < num {
            return None;
        }

        let src_stack = self.get_mut_stack(src_stack_id)?;

        for crate_ in src_stack.drain(num)? {
            let dst_stack = self.get_mut_stack(dst_stack_id)?;
            dst_stack.push(crate_);
        }

        Some(())
    }

    fn get_top_crates(&self) -> Vec<Option<Crate>> {
        self.stacks.iter().map(|stack| stack.get_top()).collect()
    }

    fn execute(&mut self, model: i32, order: Order) -> Option<()> {
        match model {
            9000 => {
                for _ in 0..order.qty {
                    self.move_crate(order.src_id, order.dst_id)?;
                }
                Some(())
            }
            9001 => self.move_crates(order.src_id, order.dst_id, order.qty),
            _ => None,
        }
    }
}

impl From<&[String]> for Storage {
    fn from(src: &[String]) -> Self {
        const STACK_WIDTH: usize = 3;

        let mut line_iter = src.iter().rev();

        let footer_line = line_iter.next().unwrap();

        let footer_chunks = &footer_line.chars().chunks(STACK_WIDTH + 1);

        let num_stacks: usize = footer_chunks
            .into_iter()
            .last()
            .unwrap()
            .join("")
            .trim()
            .parse()
            .unwrap();

        let mut storage = Storage::new(num_stacks);

        for line in line_iter {
            for (stack_id, chunk) in line.chars().chunks(STACK_WIDTH + 1).into_iter().enumerate() {
                let cell: Vec<_> = chunk.collect();
                if cell[0] == '[' {
                    storage.push_crate(stack_id, Crate { label: cell[1] });
                }
            }
        }

        storage
    }
}

#[derive(Clone, Copy)]
struct Order {
    qty: usize,
    src_id: usize,
    dst_id: usize,
}

impl From<&String> for Order {
    fn from(src: &String) -> Self {
        From::from(src.as_str())
    }
}

impl From<&str> for Order {
    fn from(src: &str) -> Self {
        let words: Vec<_> = src.split(' ').collect();

        Order {
            qty: words[1].parse().unwrap(),
            src_id: words[3].parse::<usize>().unwrap() - 1,
            dst_id: words[5].parse::<usize>().unwrap() - 1,
        }
    }
}

fn parse_input(filename: &str) -> (Storage, Vec::<Order>) {
    if let Ok(mut lines) = read_lines(filename) {
        let header: Result<Vec<_>, _> = lines
            .by_ref()
            .take_while(|line| !line.as_ref().unwrap().eq(""))
            .collect();

        let storage = Storage::from(header.unwrap().as_slice());

        let orders = lines.map(|res| Order::from(&res.unwrap())).collect();

        (storage, orders)
    } else {
        panic!("IO error");
    }
}

fn main() {
    let (initial_storage, orders) = parse_input(INPUT_PATH);

    for model in [9000, 9001] {
        let mut storage = initial_storage.clone();


        for order in orders.iter() {
            storage.execute(model, *order);
        }

        let top_crates: Vec<_> = storage.get_top_crates();
        let top_crate_labels = top_crates.iter().map(|opt| match opt {
            Some(crate_) => crate_.label,
            None => ' ',
        });

        println!("Top crate labels (CrateMover {}): {}", model, top_crate_labels.collect::<String>());
    }
}
