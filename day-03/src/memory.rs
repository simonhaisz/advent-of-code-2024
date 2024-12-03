use lazy_regex::regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Multiply(i32, i32);

impl Multiply {
    pub fn multiply(&self) -> i32 {
        self.0 * self.1
    }
}

pub trait MultiplyResults {
    fn multiplication_results(&self) -> i32;
}

pub struct Memory {
    instructions: Vec<Multiply>,
}

impl MultiplyResults for Memory {
    fn multiplication_results(&self) -> i32 {
        self.instructions
            .iter()
            .map(|i| i.multiply())
            .sum()
    }
}

pub struct MemoryAlways {
    memory: Memory,
}

impl MultiplyResults for MemoryAlways {
    fn multiplication_results(&self) -> i32 {
        self.memory.multiplication_results()
    }
}

impl From<&str> for MemoryAlways {
    fn from(text: &str) -> Self {
        let multiply_regex = regex!(r"mul\((?<a>\d+),(?<b>\d+)\)");

        let mut instructions = vec![];

        for capture in multiply_regex.captures_iter(text) {

            let a = capture.name("a").unwrap().as_str().parse::<i32>().unwrap();
            let b = capture.name("b").unwrap().as_str().parse::<i32>().unwrap();

            instructions.push(Multiply(a, b));
        }

        let memory = Memory { instructions };

        Self { memory }
    }
}

pub struct MemoryDoDont {
    memory: Memory
}

impl MultiplyResults for MemoryDoDont {
    fn multiplication_results(&self) -> i32 {
        self.memory.multiplication_results()
    }
}

impl From<&str> for MemoryDoDont {
    fn from(text: &str) -> Self {
        let multiply_regex = regex!(r"(?<mul>mul\((?<a>\d+),(?<b>\d+)\))|(?<do>do\(\))|(?<dont>don't\(\))");

        let mut instructions = vec![];

        let mut process_mul = true;

        for capture in multiply_regex.captures_iter(text) {

            if capture.name("do").is_some() {
                process_mul = true;
            } else if capture.name("dont").is_some() {
                process_mul = false;
            } else if process_mul && capture.name("mul").is_some() {
                let a = capture.name("a").unwrap().as_str().parse::<i32>().unwrap();
                let b = capture.name("b").unwrap().as_str().parse::<i32>().unwrap();
    
                instructions.push(Multiply(a, b));
            }
        }

        let memory = Memory { instructions };

        Self { memory }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply() {
        let instructions = vec![
            Multiply(2, 4),
            Multiply(5, 5),
            Multiply(11, 8),
            Multiply(8, 5),
        ];
        let memory = Memory { instructions };

        let result = memory.multiplication_results();

        assert_eq!(result, 161);
    }

    #[test]
    fn memory_always_parse() {
        let text = r"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        ".trim();

        let memory = MemoryAlways::from(text).memory;

        assert_eq!(memory.instructions.len(), 4);
        assert_eq!(memory.instructions[0], Multiply(2, 4));
        assert_eq!(memory.instructions[1], Multiply(5, 5));
        assert_eq!(memory.instructions[2], Multiply(11, 8));
        assert_eq!(memory.instructions[3], Multiply(8, 5));
    }

    #[test]
    fn memory_dodont_parse() {
        let text = r"
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        ".trim();

        let memory = MemoryDoDont::from(text).memory;

        assert_eq!(memory.instructions.len(), 2);
        assert_eq!(memory.instructions[0], Multiply(2, 4));
        assert_eq!(memory.instructions[1], Multiply(8, 5));
    }
}