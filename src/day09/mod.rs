use aocd::*;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Block {
    addr: usize,
    length: usize,
    value: usize,
}

impl Block {
    fn fragment(&self) -> Vec<Block> {
        (0..self.length)
            .map(|i| Block {
                addr: self.addr + i,
                length: 1,
                value: self.value,
            })
            .collect()
    }

    fn score(&self) -> usize {
        (0..self.length).map(|i| (self.addr + i) * self.value).sum()
    }

    fn end(&self) -> usize {
        self.addr + self.length
    }
}

#[derive(Debug)]
struct Diskmap {
    blocks: Vec<Block>,
}

impl From<&str> for Diskmap {
    fn from(value: &str) -> Self {
        let mut start = 0;
        let blocks: Vec<Block> = value
            .chars()
            .chunks(2)
            .into_iter()
            .enumerate()
            .map(|(i, mut chunk)| {
                let used = chunk.next().unwrap().to_digit(10).unwrap() as usize;
                let free = chunk.next().unwrap_or('0').to_digit(10).unwrap() as usize;
                start += used + free;
                Block {
                    addr: start - used - free,
                    length: used,
                    value: i,
                }
            })
            .collect();
        Diskmap { blocks }
    }
}

impl Diskmap {
    fn fragment(&mut self) {
        self.blocks = self
            .blocks
            .iter()
            .flat_map(|block| block.fragment())
            .collect();
    }

    fn free_space(&mut self) -> Vec<(usize, usize)> {
        self.blocks.sort_by_key(|block| block.addr);
        self.blocks
            .windows(2)
            .filter_map(|pair| {
                let space = pair[1].addr - pair[0].end();
                (space > 0).then_some((pair[0].end(), space))
            })
            .collect()
    }

    fn compress(&mut self) {
        let mut free_space = self.free_space();
        for block in self.blocks.iter_mut().rev() {
            'search: for space in free_space.iter_mut() {
                if space.0 < block.addr && block.length <= space.1 {
                    block.addr = space.0;
                    *space = (block.end(), space.1 - block.length);
                    break 'search;
                }
            }
        }
    }

    fn checksum(&self) -> usize {
        self.blocks.iter().map(|block| block.score()).sum()
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
