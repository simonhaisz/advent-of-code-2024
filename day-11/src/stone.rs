use std::collections::HashMap;

pub trait Stone {
    fn blink(self) -> BlinkResult;
}

impl Stone for u64 {
    fn blink(self) -> BlinkResult {
        if self == 0 {
            BlinkResult::Replace(1)
        } else if self.digit_count() % 2 == 0 {
            let (a, b) = self.split_digits();

            BlinkResult::Split(a, b)
        } else {
            BlinkResult::Replace(self * 2024)
        }
    }
}

pub enum BlinkResult {
    Replace(u64),
    Split(u64, u64),
}

trait StoneNumber {
    fn digit_count(&self) -> u32;
    fn split_digits(&self) -> (u64, u64);
}

impl StoneNumber for u64 {
    fn digit_count(&self) -> u32 {
        self.ilog10() + 1
    }

    fn split_digits(&self) -> (u64, u64) {
        let digit_count = self.digit_count();

        let factor = 10u64.pow(digit_count / 2);

        let a = self / factor;
        let b = self % factor;

        (a, b)
    }
}

pub struct StoneLine {
    stones: Vec<u64>,
}

impl StoneLine {
    pub fn blink(self) -> Self {
        let mut stones = vec![];

        for stone in self.stones.into_iter() {
            match stone.blink() {
                BlinkResult::Replace(s) => {
                    stones.push(s);
                },
                BlinkResult::Split(a, b) => {
                    stones.push(a);
                    stones.push(b);
                }
            }
        }

        Self { stones }
    }

    pub fn multi_blink_stone_count(mut stone_line: StoneLine, count: u32) -> usize {
        for _ in 0..count {
            stone_line = stone_line.blink();
        }

        stone_line.stones.len()
    }
}

impl From<&str> for StoneLine {
    fn from(line: &str) -> Self {
        let stones = line.trim().split(' ')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<_>>();

        Self { stones }
    }
}

pub struct StoneCollection {
    stones: HashMap<u64, u64>,
}

impl StoneCollection {
    pub fn from(stone_line: StoneLine) -> Self {
        let mut stones = HashMap::new();

        for s in stone_line.stones.into_iter() {
            let count = stones.entry(s).or_default();
            *count += 1;
        }

        Self { stones }
    }

    pub fn blink(self) -> Self {
        let mut stones = HashMap::new();

        for (number, count) in self.stones.into_iter() {
            match number.blink() {
                BlinkResult::Replace(n) => {
                    let total_count = stones.entry(n).or_default();
                    *total_count += count
                },
                BlinkResult::Split(a, b) => {
                    if a == b {
                        let total_count = stones.entry(a).or_default();
                        *total_count += count * 2;
                    } else {
                        let total_count_a = stones.entry(a).or_default();
                        *total_count_a += count;

                        let total_count_b = stones.entry(b).or_default();
                        *total_count_b += count;
                    }
                },
            }
        }

        Self { stones }
    }

    pub fn multi_blink_stone_count(mut stone_collection: StoneCollection, count: u32) -> u64 {
        for _ in 0..count {
            stone_collection = stone_collection.blink();
        }

        let mut total_count = 0;

        for (_, count) in stone_collection.stones.iter() {
            total_count += count;
        }

        total_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_EXAMPLE: &'static str = "0 1 10 99 999";

    const SIMPLE_EXAMPLE: &'static str = "125 17";

    #[test]
    fn basic_example() {
        let stone_line = StoneLine::from(BASIC_EXAMPLE);

        let stone_line = stone_line.blink();

        assert_eq!(vec![1, 2024, 1, 0, 9, 9, 2021976], stone_line.stones);
    }

    #[test]
    fn simple_example() {
        let mut stone_line = StoneLine::from(SIMPLE_EXAMPLE);

        for _ in 0..6 {
            stone_line = stone_line.blink();
        }

        assert_eq!(22, stone_line.stones.len());

        for _ in 0..19 {
            stone_line = stone_line.blink();
        }

        assert_eq!(55312, stone_line.stones.len());
    }
}