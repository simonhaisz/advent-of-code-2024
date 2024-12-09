use crate::operator::{Operator, OperatorSet};

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
                }
            } else {
                break;
            }
        }

        result
    }

    pub fn valid(&self) -> bool {
        let operator_set = OperatorSet::new((self.right_hand_values.len() - 1) as u32);

        for operators in operator_set {
            let value = self.test(&operators);

            if value == self.left_hand_value {
                return true;
            }
        }

        false
    }

    pub fn possible(&self) -> bool {
        let count = self.right_hand_values.len() - 1;

        let add = self.test(&Operator::Add.generate(count));
        let multiply = self.test(&Operator::Multiply.generate(count));

        let min = add.min(multiply);
        let max = add.max(multiply);

        let lhv = self.left_hand_value;

        lhv >= min && lhv <= max
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
    pub fn result(&self) -> u64 {
        self.equations.iter()
            .filter(|e| e.valid())
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
    fn example_valid() {
        let equation = Equation::from("190: 10 19");

        let valid = equation.valid();

        assert!(valid);

        let equation = Equation::from("3267: 81 40 27");

        let valid = equation.valid();

        assert!(valid);

        let equation = Equation::from("292: 11 6 16 20");

        let valid = equation.valid();

        assert!(valid);
    }

    #[test]
    fn example_invalid() {
        let equation = Equation::from("83: 17 5");

        let valid = equation.valid();

        assert!(!valid);

        let equation = Equation::from("156: 15 6");

        let valid = equation.valid();

        assert!(!valid);

        let equation = Equation::from("7290: 6 8 6 15");

        let valid = equation.valid();

        assert!(!valid);

        let equation = Equation::from("161011: 16 10 13");

        let valid = equation.valid();

        assert!(!valid);

        let equation = Equation::from("192: 17 8 14");

        let valid = equation.valid();

        assert!(!valid);

        let equation = Equation::from("21037: 9 7 18 13");

        let valid = equation.valid();

        assert!(!valid);
    }

    #[test]
    fn example() {
        let calibration = Calibration::from(EXAMPLE);

        let result = calibration.result();

        assert_eq!(result, 3749);
    }
}