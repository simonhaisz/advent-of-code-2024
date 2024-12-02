#[derive(Debug, PartialEq, Eq)]
pub enum Safety {
    Safe,
    Unsafe
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    None,
    Increase,
    Decrease,
}

pub fn parse_reports(text: &str) -> Vec<Report> {
    text
        .lines()
        .map(|l| Report::from(l))
        .collect()
}

pub struct Report {
    levels: Vec<i32>,
}

impl Report {
    pub fn new(levels: Vec<i32>) -> Self {
        Self { levels }
    }

    pub fn safety_check(&self) -> Safety {
        levels_safety_check(&self.levels)
    }

    pub fn is_safe(&self) -> bool {
        let safety = self.safety_check();

        match safety {
            Safety::Safe => true,
            Safety::Unsafe => false,
        }
    }

    pub fn tolerance_safety_check(&self) -> Safety {
        let mut skip_index: Option<usize> = None;

        loop {
            let mut levels = self.levels.clone();
            if let Some(i) = skip_index {
                if i == levels.len() {
                    break;
                }
                levels.remove(i);
            }
            let safety = levels_safety_check(&levels);

            if safety == Safety::Safe {
                return Safety::Safe;
            }

            if let Some(i) = skip_index {
                skip_index.replace(i + 1);
            } else {
                skip_index = Some(0);
            }
        }

        Safety::Unsafe
    }

    pub fn is_safe_with_tolerance(&self) -> bool {
        let safety = self.tolerance_safety_check();

        match safety {
            Safety::Safe => true,
            Safety::Unsafe => false,
        }
    }
}

fn levels_safety_check(levels: &[i32]) -> Safety {
    let mut previous_direction = None;

    let mut previous = None;
    for level in levels.iter() {
        if let Some(previous) = previous {
            let delta: i32 = *level - previous;
            if delta.abs() > 3 || delta == 0 {
                return Safety::Unsafe;
            }

            let current_direction =  if delta > 0 {
                Direction::Increase
            } else if delta < 0 {
                Direction::Decrease
            } else {
                panic!("Delta should never be zero: {delta}")
            };

            if let Some(pd) = previous_direction {
                if current_direction == Direction::None {
                    previous_direction = Some(current_direction);
                } else if pd != current_direction {
                    return Safety::Unsafe;
                }
            } else {
                previous_direction = Some(current_direction);
            }
        }

        previous = Some(*level);
    }

    Safety::Safe
}

impl From<&str> for Report {
    fn from(line: &str) -> Self {
        let levels = line
            .split(' ')
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        Self::new(levels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_parse() {
        let text = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
        ".trim();
        let reports = parse_reports(text);

        assert_eq!(reports.len(), 6);
        assert_eq!(reports[0].levels, [7, 6, 4, 2, 1]);
        assert_eq!(reports[1].levels, [1, 2, 7, 8, 9]);
        assert_eq!(reports[2].levels, [9, 7, 6, 2, 1]);
        assert_eq!(reports[3].levels, [1, 3, 2, 4, 5]);
        assert_eq!(reports[4].levels, [8, 6, 4, 4, 1]);
        assert_eq!(reports[5].levels, [1, 3, 6, 7, 9]);
    }

    #[test]
    fn example_safety() {
        let text = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
        ".trim();
        let reports = parse_reports(text);

        let safe_count = reports.iter()
            .filter(|&r| r.is_safe())
            .count();

        assert_eq!(safe_count, 2);
    }

    #[test]
    fn example_safety_with_tolerance() {
        let text = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
        ".trim();
        let reports = parse_reports(text);

        let safe_count = reports.iter()
            .filter(|&r| r.is_safe_with_tolerance())
            .count();

        assert_eq!(safe_count, 4);
    }
}