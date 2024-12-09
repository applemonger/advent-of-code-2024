use aocd::*;

#[derive(Debug, Clone)]
struct Block {
    length: usize,
    value: Option<u64>,
    moved: bool
}

impl Block {
    fn fragment(&self) -> Vec<Block> {
        (0..self.length).map(|_| Block { length: 1, value: self.value, moved: self.moved }).collect()
    }

    fn score(&self, start: usize) -> u64 {
        let value = self.value.unwrap_or_default();
        (0..self.length).map(|i| (start + i) as u64 * value).sum()
    }

    fn space(&self) -> usize {
        self.value.is_none() as usize * self.length
    }
}

#[derive(Debug)]
struct Diskmap {
    blocks: Vec<Block>
}

impl From<&str> for Diskmap {
    fn from(value: &str) -> Self {
        let mut used = false;
        let mut id: u64 = 0;
        let blocks: Vec<Block> = value
            .chars()
            .flat_map(|c| {
                id += used as u64;
                used = !used;
                let length = c.to_digit(10).unwrap() as usize;
                if length > 0 {
                    Some(Block { length, value: used.then_some(id), moved: false })
                } else {
                    None
                }
            }).collect();
        Diskmap { blocks }
    }
}

impl Diskmap {
    fn fragment(&mut self) {
        self.blocks = self.blocks.iter().flat_map(|block| block.fragment()).collect();
    }

    fn find_space(&self, min_size: usize) -> Option<usize> {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.space() >= min_size {
                return Some(i)
            }
        }
        None
    }

    fn move_block(&mut self, to: usize, from: usize) {
        let diff: usize = self.blocks[to].length - self.blocks[from].length;
        if diff == 0 {
            self.blocks.swap(to, from);
            self.blocks[to].moved = true;
        } else if diff > 0 {
            self.blocks[to].value = self.blocks[from].value;
            self.blocks[to].length = self.blocks[from].length;
            self.blocks[to].moved = true;
            self.blocks[from].value = None;
            let empty = Block { length: diff, value: None, moved: false };
            self.blocks.insert(to+1, empty);
        }
    }

    fn smooth(&mut self) {
        for i in 1..self.blocks.len() {
            if self.blocks[i].value.is_none() && self.blocks[i-1].value.is_none() {
                self.blocks[i].length += self.blocks[i-1].length;
                self.blocks[i-1].length = 0;
            }
        }
        self.blocks.retain(|block| block.length > 0);
    }

    fn compress(&mut self) {
        let mut compressed = false;
        'compression: while !compressed {
            //self.print();
            compressed = true;
            for i in (0..self.blocks.len()).rev() {
                if !self.blocks[i].value.is_none() && !self.blocks[i].moved {
                    let size = self.blocks[i].length;
                    if let Some(idx) = self.find_space(size) {
                        if idx < i {
                            compressed = false;
                            self.move_block(idx, i);
                            self.smooth();
                            continue 'compression;
                        }
                    }
                }
                self.blocks[i].moved = true;
            }
        }
    }

    fn checksum(&self) -> u64 {
        let mut score = 0;
        self.blocks.iter()
            .fold(0_usize, |start, block| {
                score += block.score(start);
                start + block.length
            });
        score
    }
}

#[aocd(2024, 9)]
pub fn solution1() {
    let data = input!();
    let mut diskmap = Diskmap::from(data.as_str());
    diskmap.fragment();
    diskmap.compress();
    submit!(1, diskmap.checksum());
}

#[aocd(2024, 9)]
pub fn solution2() {
    let data = input!();
    let mut diskmap = Diskmap::from(data.as_str());
    diskmap.compress();
    submit!(2, diskmap.checksum());
}