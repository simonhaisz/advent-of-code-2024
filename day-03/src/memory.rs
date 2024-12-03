use lazy_regex::regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Multiply(i32, i32);

impl Multiply {
    pub fn multiply(&self) -> i32 {
        self.0 * self.1
    }
}

pub struct Memory {
    instructions: Vec<Multiply>,
}

impl Memory {
    pub fn multiplication_results(&self) -> i32 {
        self.instructions
            .iter()
            .map(|i| i.multiply())
            .sum()
    }
}

impl From<&str> for Memory {
    fn from(text: &str) -> Self {
        let multiply_regex = regex!(r"mul\((?<a>\d+),(?<b>\d+)\)");

        let mut instructions = vec![];

        for mul in multiply_regex.captures_iter(text) {
            let a = mul.name("a").unwrap().as_str().parse::<i32>().unwrap();
            let b = mul.name("b").unwrap().as_str().parse::<i32>().unwrap();

            instructions.push(Multiply(a, b));
        }

        Memory { instructions }
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
    fn memory_parse() {
        let text = r"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        ".trim();

        let memory = Memory::from(text);

        assert_eq!(memory.instructions.len(), 4);
        assert_eq!(memory.instructions[0], Multiply(2, 4));
        assert_eq!(memory.instructions[1], Multiply(5, 5));
        assert_eq!(memory.instructions[2], Multiply(11, 8));
        assert_eq!(memory.instructions[3], Multiply(8, 5));
    }
}