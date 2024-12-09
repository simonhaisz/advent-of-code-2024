use crate::operator::Operator;

pub struct Equation {
    left_hand_value: i32,
    right_hand_values: Vec<i32>,
}

impl Equation {
    pub fn test(&self, operators: &[Operator]) -> i32 {
        let mut value = None;
        let operator_index = 0;

        for rhv in self.right_hand_values {
            if value.is_none() {
                value = Some(rhv);
            } else {
                let value = value.as_mut().unwrap();

                let operator = operators[operator_index];

                match operator {
                    
                }
            }
        }
    }
    pub fn valid(&self) -> bool {
        unimplemented!()
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

        let left = split.nth(0).unwrap().trim();
        let right = split.nth(1).unwrap().trim();

        let left_hand_value = left.parse::<i32>().unwrap();

        let right_hand_values = right.split(" ").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>();

        Self { left_hand_value, right_hand_values }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {

    }
}