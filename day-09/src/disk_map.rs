type Block = Option<u64>;

pub struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    pub fn compact(self) -> Self {
        let mut blocks = self.blocks;

        let next_empty = |b: &[Block]| {
            let mut empty_it = b.iter().enumerate();
            loop {
                if let Some((index, block)) = empty_it.next() {
                    if block.is_none() {
                        return Some(index);
                    }
                } else {
                    break;
                }
            }

            None
        };

        let next_file = |b: &[Block]| {
            let mut file_it = b.iter().enumerate().rev();
            loop {
                if let Some((index, block)) = file_it.next() {
                    if block.is_some() {
                        return Some(index);
                    }
                } else {
                    break;
                }
            }

            None
        };

        loop {
            let empty_block = next_empty(&blocks);
            let file_block = next_file(&blocks);

            if empty_block.is_none() || file_block.is_none() {
                break;
            }

            let empty_block = empty_block.unwrap();
            let file_block = file_block.unwrap();

            if empty_block > file_block {
                break;
            }

            blocks.swap(empty_block, file_block);
        }

        Self { blocks }
    }

    pub fn checksum(&self) -> u64 {
        self.blocks.iter()
            .enumerate()
            .filter(|(_, b)| b.is_some())
            .map(|(i, b)| {
                let b = b.unwrap();
                u64::try_from(i).unwrap() * b
            })
            .sum()
    }
}

impl From<&str> for Disk {
    fn from(value: &str) -> Self {
        let mut processing_file = true;
        let mut file_index = 0;

        let mut disk_map_itr = value.trim().chars();

        let mut blocks = vec![];

        loop {
            if let Some(digit) = disk_map_itr.next() {
                let count = digit.to_digit(10).unwrap();
                
                let block = if processing_file {
                    let block = Some(file_index);
                    file_index += 1;
                    block
                } else {
                    None
                };

                processing_file = !processing_file;

                for _ in 0..count {
                    blocks.push(block);
                }

            } else {
                break;
            }
        }

        Self { blocks }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

const BASIC_EXAMPLE: &'static str = "12345";
const SIMPLE_EXAMPLE: &'static str = "2333133121414131402";

    #[test]
    fn basic_example() {
        let disk = Disk::from(BASIC_EXAMPLE);

        let disk = disk.compact();

        let checksum = disk.checksum();

        assert_eq!(60, checksum);
    }

    #[test]
    fn simple_example() {
        let disk = Disk::from(SIMPLE_EXAMPLE);

        let disk = disk.compact();

        let checksum = disk.checksum();

        assert_eq!(1928, checksum);
    }
}