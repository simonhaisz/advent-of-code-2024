use crate::operator::{Operator, OperatorBinaryIterator, OperatorTrinaryIterator};

pub struct Equation {
    left_hand_value: u64,
    right_hand_values: Vec<u64>,
}

impl Equation {
    pub fn test(&self, operators: &[Operator]) -> u64 {
        let mut values_it = self.right_hand_values.iter();
        let mut operators_it = operators.iter();

        let mut result = *values_it.next().unwrap();

        loop {
            if let Some(value) = values_it.next() {
                let operator = operators_it.next().unwrap();

                match *operator {
                    Operator::Add => result += *value,
                    Operator::Multiply => result *= *value,
                    Operator::Concatenate => {
                        let rhs = *value as u32;
                        
                        let rhs_digit_count = rhs.ilog10();
                        let lhs_multiplier = 10_u32.pow(rhs_digit_count + 1);

                        result = result * (lhs_multiplier as u64) + (rhs as u64);
                    },
                }
            } else {
                break;
            }
        }

        result
    }

    pub fn valid_binary(&self) -> bool {
        let operator_it = OperatorBinaryIterator::new((self.right_hand_values.len() - 1) as u32);

        self.valid(operator_it)
    }

    pub fn valid_trinary(&self) -> bool {
        let operator_it = OperatorTrinaryIterator::new((self.right_hand_values.len() - 1) as u32);

        self.valid(operator_it)
    }

    fn valid(&self, operator_it: impl Iterator<Item=Vec<Operator>>) -> bool {
        for operators in operator_it {
            let value = self.test(&operators);

            if value == self.left_hand_value {
                return true;
            }
        }

        false
    }
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let mut split = value.split(":");

        let left = split.next().unwrap().trim();
        let right = split.next().unwrap().trim();

        let left_hand_value = left.parse::<u64>().unwrap();

        let right_hand_values = right.split(" ").map(|v| v.parse::<u64>().unwrap()).collect::<Vec<_>>();

        Self { left_hand_value, right_hand_values }
    }
}

pub struct Calibration {
    equations: Vec<Equation>,
}

impl Calibration {
    pub fn result_binary(&self) -> u64 {
        self.equations.iter()
            .filter(|e| e.valid_binary())
            .map(|e| e.left_hand_value)
            .sum()
    }

    pub fn result_trinary(&self) -> u64 {
        self.equations.iter()
            .filter(|e| e.valid_trinary())
            .map(|e| e.left_hand_value)
            .sum()
    }
}

impl From<&str> for Calibration {
    fn from(text: &str) -> Self {
        let mut equations = vec![];

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let equation = Equation::from(line);
            equations.push(equation);
        }
    
        Self { equations }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
    ";

    #[test]
    fn example_binary_valid() {
        let equation = Equation::from("190: 10 19");

        let valid = equation.valid_binary();

        assert!(valid);

        let equation = Equation::from("3267: 81 40 27");

        let valid = equation.valid_binary();

        assert!(valid);

        let equation = Equation::from("292: 11 6 16 20");

        let valid = equation.valid_binary();

        assert!(valid);
    }

    #[test]
    fn example_binary_invalid() {
        let equation = Equation::from("83: 17 5");

        let valid = equation.valid_binary();

        assert!(!valid);

        let equation = Equation::from("156: 15 6");

        let valid = equation.valid_binary();

        assert!(!valid);

        let equation = Equation::from("7290: 6 8 6 15");

        let valid = equation.valid_binary();

        assert!(!valid);

        let equation = Equation::from("161011: 16 10 13");

        let valid = equation.valid_binary();

        assert!(!valid);

        let equation = Equation::from("192: 17 8 14");

        let valid = equation.valid_binary();

        assert!(!valid);

        let equation = Equation::from("21037: 9 7 18 13");

        let valid = equation.valid_binary();

        assert!(!valid);
    }

    #[test]
    fn example_binary() {
        let calibration = Calibration::from(EXAMPLE);

        let result = calibration.result_binary();

        assert_eq!(result, 3749);
    }

    #[test]
    fn examle_trinary() {
        let calibration = Calibration::from(EXAMPLE);

        let result = calibration.result_trinary();

        assert_eq!(result, 11387);
    }
}