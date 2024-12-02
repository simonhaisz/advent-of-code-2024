use std::collections::HashMap;

pub struct LocationPair {
    a: Vec<i32>,
    b: Vec<i32>,
}

#[derive(Debug)]
pub enum Optimization {
    None,
    Indexed,
}

impl LocationPair {
    pub fn new(a: Vec<i32>, b: Vec<i32>) -> Self {
        assert_eq!(a.len(), b.len(), "List pairs must match in length");

        Self { a, b }
    }

    pub fn pair_distances(&self) -> Vec<i32> {
        let mut a = self.a.clone();
        a.sort();

        let mut b = self.b.clone();
        b.sort();

        let mut distances = vec![];

        for i in 0..a.len() {
            let distance = (b[i] - a[i]).abs();

            distances.push(distance);
        }

        distances
    }

    pub fn total_pair_distance(&self) -> i32 {
        let distances = self.pair_distances();

        distances
            .iter()
            .sum()
    }

    pub fn similarity_score(&self, optimization: Optimization) -> i32 {
        match optimization {
            Optimization::None => {
                let mut total_score = 0;

                for a in self.a.iter() {
                    let value = *a;
                    let match_count = self.b.iter().filter(|&b| *b == value).count() as i32;

                    total_score += value * match_count;
                }
                total_score
            },
            Optimization::Indexed => {
                let mut value_score_map = HashMap::<i32, i32>::new();

                let mut total_score = 0;
        
                let value_count_map = {
                    let mut value_count_map = HashMap::<i32, i32>::new();
        
                    for b in self.b.iter() {
                        let value = *b;
                        let count = value_count_map.entry(value).or_default();
                        *count += 1;
                    }
        
                    value_count_map
                };
        
                for a in self.a.iter() {
                    let value = *a;
                    let score = value_score_map.entry(value).or_insert_with(|| {
                        let count = value_count_map.get(&value);
        
                        if let Some(count) = count {
                            count * value
                        } else {
                            0
                        }
                    });
        
                    total_score += *score;
                }
        
                total_score
            }
        }
    }
}

impl From<String> for LocationPair {
    fn from(text: String) -> Self {
        let mut a = vec![];
        let mut b = vec![];

        for line in text.lines() {
            let pair = line
                .split(" ")
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            assert_eq!(pair.len(), 2, "Line should have been split into two");

            a.push(parse_int(pair[0]));
            b.push(parse_int(pair[1]));
        }

        Self::new(a, b)
    }
}

fn parse_int(text: &str) -> i32 {
    text.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_parse() {
        let text = r"
3   4
4   3
2   5
1   3
3   9
3   3
        ".trim().to_string();

        let pair = LocationPair::from(text);

        assert_eq!(&pair.a, &[3, 4, 2, 1, 3, 3]);
        assert_eq!(&pair.b, &[4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn example_total_distance() {
        let text = r"
3   4
4   3
2   5
1   3
3   9
3   3
        ".trim().to_string();
        
        let pair = LocationPair::from(text);

        let total_distance = pair.total_pair_distance();
        
        assert_eq!(total_distance, 11);
    }

    #[test]
    fn example_similarity_score_n_squared() {
        let text = r"
3   4
4   3
2   5
1   3
3   9
3   3
        ".trim().to_string();

        let pair = LocationPair::from(text);

        let similarity_score = pair.similarity_score(Optimization::None);

        assert_eq!(similarity_score, 31);
    }

    #[test]
    fn example_similarity_score_indexed() {
        let text = r"
3   4
4   3
2   5
1   3
3   9
3   3
        ".trim().to_string();

        let pair = LocationPair::from(text);

        let similarity_score = pair.similarity_score(Optimization::Indexed);

        assert_eq!(similarity_score, 31);
    }
}