use std::collections::HashSet;

type Block = Option<u64>;

pub struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    pub fn compact_blocks(self) -> Self {
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

    pub fn compact_files(self) -> Self {
        let mut blocks = self.blocks;

        let mut moved_files = HashSet::new();

        let mut next_file = |b: &[Block]| {
            let mut file_it = b.iter().enumerate().rev();

            let mut file = None;
            let mut file_end = None;
            let mut file_start = None;

            loop {
                if let Some((index, block)) = file_it.next() {
                    if file_end.is_none() {
                        if block.is_some() {
                            let block_file = block.unwrap();
                            if moved_files.contains(&block_file) {
                                continue;
                            }
                            file = Some(block_file);
                            file_end = Some(index);
                        }
                    } else if file_start.is_none() {
                        if let Some(block) = block {
                            if file.as_ref().unwrap() != block {
                                // hit a different file block
                                file_start = Some(index + 1);
                                break;
                            }
                        } else {
                            // hit empty blocks
                            file_start = Some(index + 1);
                            break;
                        }
                    } else {
                        panic!("still searching for file locations after start and end have been found")
                    }
                } else {
                    break;
                }
            }

            if file_start.is_some() && file_end.is_some() {
                moved_files.insert(file.unwrap());
                Some((file_start.unwrap(), file_end.unwrap()))
            } else {
                None
            }
        };

        let next_empty = |b: &[Block], required_space| {
            let mut empty_it = b.iter().enumerate();

            loop {
                let mut empty_start = None;
                let mut empty_end = None;

                loop {
                    if let Some((index, block)) = empty_it.next() {
                        if empty_start.is_none() {
                            if block.is_none() {
                                empty_start = Some(index);
                            }
                        }
                        else if empty_end.is_none() {
                            if block.is_some() {
                                empty_end = Some(index - 1);
                                break;
                            }
                        } else {

                        }
                    } else {
                        break;
                    }
                }

                if empty_end.is_some() && empty_end.is_some() {
                    let empty_start = empty_start.unwrap();
                    let empty_end = empty_end.unwrap();
    
                    let size = empty_end - empty_start + 1;
                    if size >= required_space {
                        break Some((empty_start, empty_end));
                    }
                } else {
                    break None;
                }
            }
        };

        loop {
            let file_block = next_file(&blocks);
            
            if file_block.is_none() {
                break;
            }

            let (file_start, file_end) = file_block.unwrap();

            let file_size = file_end - file_start + 1;

            loop {
                let empty_block = next_empty(&blocks, file_size);

                if empty_block.is_none() {
                    break;
                }

                let (empty_start, empty_end) = empty_block.unwrap();

                if empty_end > file_start {
                    break;
                }

                let empty_size = empty_end - empty_start + 1;

                if empty_size < file_size {
                    continue;
                }

                for offset in 0..file_size {
                    blocks.swap(empty_start + offset, file_start + offset);
                }

                break;
            }
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
    fn basic_example_blocks() {
        let disk = Disk::from(BASIC_EXAMPLE);

        let disk = disk.compact_blocks();

        let checksum = disk.checksum();

        assert_eq!(60, checksum);
    }

    #[test]
    fn simple_example_blocks() {
        let disk = Disk::from(SIMPLE_EXAMPLE);

        let disk = disk.compact_blocks();

        let checksum = disk.checksum();

        assert_eq!(1928, checksum);
    }

    #[test]
    fn basic_example_files() {
        let disk = Disk::from(BASIC_EXAMPLE);

        let disk = disk.compact_files();

        let checksum = disk.checksum();

        assert_eq!(132, checksum);
    }

    #[test]
    fn simple_example_files() {
        let disk = Disk::from(SIMPLE_EXAMPLE);

        let disk = disk.compact_files();

        let checksum = disk.checksum();

        assert_eq!(2858, checksum);
    }
}