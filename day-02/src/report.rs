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
        let result = levels_safety_check(&self.levels);
        match result {
            Ok(_) => Safety::Safe,
            Err(_) => Safety::Unsafe,
        }
    }

    pub fn is_safe(&self) -> bool {
        let safety = self.safety_check();

        match safety {
            Safety::Safe => true,
            Safety::Unsafe => false,
        }
    }

    pub fn brute_force_tolerance_safety_check(&self) -> Safety {
        let mut skip_index: Option<usize> = None;

        loop {
            let mut levels = self.levels.clone();
            if let Some(i) = skip_index {
                if i == levels.len() {
                    break;
                }
                levels.remove(i);
            }
            let result = levels_safety_check(&levels);

            if result.is_ok() {
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

    pub fn is_safe_with_brute_force_tolerance(&self) -> bool {
        let safety = self.brute_force_tolerance_safety_check();

        match safety {
            Safety::Safe => true,
            Safety::Unsafe => false,
        }
    }

    pub fn optimized_tolerance_safety_check(&self) -> Safety {
        let mut error_index: Option<usize> = None;
        let mut error_offset = 0;

        loop {
            let mut levels = self.levels.clone();
            if let Some(index) = error_index {
                if error_offset > 2 || error_offset > index {
                    break;
                }
                levels.remove(index - error_offset);
            }
            let result = levels_safety_check(&levels);

            if result.is_ok() {
                return Safety::Safe;
            } else if let Err(index) = result {
                if error_index.is_none() {
                    error_index = Some(index);
                } else {
                    error_offset += 1;
                }
            }
        }

        Safety::Unsafe
    }

    pub fn is_safe_with_optimized_tolerance(&self) -> bool {
        let safety = self.optimized_tolerance_safety_check();

        match safety {
            Safety::Safe => true,
            Safety::Unsafe => false,
        }
    }
}

fn levels_safety_check(levels: &[i32]) -> Result<(), usize> {
    let mut previous_direction = None;

    let mut previous = None;
    for (index, level) in levels.iter().enumerate() {
        if let Some(previous) = previous {
            let delta: i32 = *level - previous;
            if delta.abs() > 3 || delta == 0 {
                return Err(index);
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
                    return Err(index);
                }
            } else {
                previous_direction = Some(current_direction);
            }
        }

        previous = Some(*level);
    }

    Ok(())
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
            .filter(|&r| r.is_safe_with_brute_force_tolerance())
            .count();

        assert_eq!(safe_count, 4);
    }

    #[test]
    fn example_safety_with_optimized_tolerance() {
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
            .filter(|&r| r.is_safe_with_optimized_tolerance())
            .count();

        assert_eq!(safe_count, 4);
    }

    #[test]
    fn compare_real_tolerances() {
        let text = std::fs::read_to_string("./input.txt").unwrap();

        let reports = parse_reports(&text);

        let brute_force_count = reports.iter()
            .filter(|&r| r.is_safe_with_brute_force_tolerance())
            .count();

        assert_eq!(brute_force_count, 311);

        let optimized_force_count = reports.iter()
            .filter(|&r| r.is_safe_with_optimized_tolerance())
            .count();

        assert_eq!(optimized_force_count, 311);
    }
}