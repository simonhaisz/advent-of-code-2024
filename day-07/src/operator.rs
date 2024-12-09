#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Operator {
    Add,
    Multiply,
}

impl Operator {
    pub fn from(value: u8) -> Operator {
        match value {
            0 => Operator::Add,
            1 => Operator::Multiply,
            _ => panic!("Invalid value {value} - must be 0 or 1")
        }
    }

    pub fn all() -> &'static [Operator] {
        &[
            Operator::Add,
            Operator::Multiply,
        ]
    }

    pub fn generate(&self, count: usize) -> Vec<Operator> {
        let mut operators = vec!{};

        for _ in 0..count {
            operators.push(*self);
        }
        operators
    }

    pub fn toggle(&self) -> Operator {
        match *self {
            Operator::Add => Operator::Multiply,
            Operator::Multiply => Operator::Add,
        }
    }
}

pub struct OperatorSet {
    count: u32,
    limit: u64,
    current: u64,
}

impl OperatorSet {
    pub fn new(count: u32) -> Self {
        let limit = 2u64.pow(count);
        let current = 0;

        Self { count, limit, current }
    }
}

impl Iterator for OperatorSet {
    type Item = Vec<Operator>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.current == self.limit {
            None
        } else {
            let mut operators = vec![];

            for n in 0..self.count {
                let digit = 2u64.pow(n);
                let op_value = if (self.current & digit) == digit {
                    Operator::Multiply
                } else {
                    Operator::Add
                };

                operators.push(op_value);
            }

            self.current += 1;

            Some(operators)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_one() {
        let operator_set = OperatorSet::new(1);

        let operators = operator_set.collect::<Vec<_>>();

        assert_eq!(
            operators,
            vec![
                vec![Operator::Add],
                vec![Operator::Multiply]
            ]
        );
    }

    #[test]
    fn iterator_three() {
        let operator_set = OperatorSet::new(3);

        let operators = operator_set.collect::<Vec<_>>();

        assert_eq!(
            operators,
            vec![
                vec![Operator::Add, Operator::Add, Operator::Add],
                vec![Operator::Multiply, Operator::Add, Operator::Add],
                vec![Operator::Add, Operator::Multiply, Operator::Add],
                vec![Operator::Multiply, Operator::Multiply, Operator::Add],
                vec![Operator::Add, Operator::Add, Operator::Multiply],
                vec![Operator::Multiply, Operator::Add, Operator::Multiply],
                vec![Operator::Add, Operator::Multiply, Operator::Multiply],
                vec![Operator::Multiply, Operator::Multiply, Operator::Multiply],
            ]
        );
    }
}