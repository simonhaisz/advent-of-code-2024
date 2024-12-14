#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn increment(&mut self) -> bool {
        match *self {
            Operator::Add => *self = Operator::Multiply,
            Operator::Multiply => *self = Operator::Concatenate,
            Operator::Concatenate => *self = Operator::Add,
        }

        *self == Operator::Add
    }
}

pub struct OperatorBinaryIterator {
    count: u32,
    limit: u64,
    current: u64,
}

impl OperatorBinaryIterator {
    pub fn new(count: u32) -> Self {
        let limit = 2u64.pow(count);
        let current = 0;

        Self { count, limit, current }
    }
}

impl Iterator for OperatorBinaryIterator {
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

pub struct OperatorTrinaryIterator {
    count: u32,
    limit: u64,
    current: u64,
    operators_buffer: Option<Vec<Operator>>,
}

impl OperatorTrinaryIterator {
    pub fn new(count: u32) -> Self {
        let limit = 3u64.pow(count);
        let current = 0;
        let operators_buffer = None;

        Self { count, limit, current, operators_buffer }
    }
}

impl Iterator for OperatorTrinaryIterator {
    type Item = Vec<Operator>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.limit {
            None
        } else {
            let operators = if self.operators_buffer.is_none() {
                let operators = vec![Operator::Add; self.count as usize];
                self.operators_buffer = Some(operators.clone());

                operators
            } else {
                let operators = self.operators_buffer.as_mut().unwrap();
                let mut increment_index = operators.len() - 1;

                loop {
                    let op = operators.get_mut(increment_index).unwrap();
                    if op.increment() {
                        if increment_index == 0 {
                            break;
                        } else {
                            increment_index -= 1;
                        }
                    } else {
                        break;
                    }
                }

                operators.clone()
            };

            self.current += 1;

            Some(operators)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_binary_one() {
        let operator_set = OperatorBinaryIterator::new(1);

        let operators = operator_set.collect::<Vec<_>>();

        assert_eq!(
            operators,
            vec![
                vec![Operator::Add],
                vec![Operator::Multiply],
            ]
        );
    }

    #[test]
    fn iterator_binary_two() {
        let operator_set = OperatorBinaryIterator::new(2);

        let operators = operator_set.collect::<Vec<_>>();

        assert_eq!(
            operators,
            vec![
                vec![Operator::Add, Operator::Add],
                vec![Operator::Multiply, Operator::Add],
                vec![Operator::Add, Operator::Multiply],
                vec![Operator::Multiply, Operator::Multiply],
            ]
        );
    }

    #[test]
    fn iterator_binary_three() {
        let operator_set = OperatorBinaryIterator::new(3);

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

    #[test]
    fn iterator_trinary_one() {
        let iterator = OperatorTrinaryIterator::new(1);

        let operators = iterator.collect::<Vec<_>>();

        assert_eq!(
            operators,
            vec![
                vec![Operator::Add],
                vec![Operator::Multiply],
                vec![Operator::Concatenate]
            ]
        );
    }

    #[test]
    fn iterator_trinary_two() {
        let iterator = OperatorTrinaryIterator::new(2);

        let operators = iterator.collect::<Vec<_>>();

        assert_eq!(
            operators,
            vec![
                vec![Operator::Add, Operator::Add],
                vec![Operator::Add, Operator::Multiply],
                vec![Operator::Add, Operator::Concatenate],
                vec![Operator::Multiply, Operator::Add],
                vec![Operator::Multiply, Operator::Multiply],
                vec![Operator::Multiply, Operator::Concatenate],
                vec![Operator::Concatenate, Operator::Add],
                vec![Operator::Concatenate, Operator::Multiply],
                vec![Operator::Concatenate, Operator::Concatenate],
            ]
        );
    }
}