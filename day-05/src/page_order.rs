pub struct PageOrderRule(pub u32, pub u32);

pub type Printing = Vec<u32>;

trait ValidateRules<Rule> {
    fn validate(&self, rules: &[Rule]) -> bool;
}

impl ValidateRules<PageOrderRule> for Printing {
    fn validate(&self, rules: &[PageOrderRule]) -> bool {
        for rule in rules {
            let left_index = self.iter().position(|&p| p == rule.0);
            let right_index = self.iter().position(|&p| p == rule.1);

            if left_index.is_some() & right_index.is_some() {
                let left_index = left_index.unwrap();
                let right_index = right_index.unwrap();
                if left_index >= right_index {
                    return false;
                }
            }
        }

        true
    }
}

#[derive(Default)]
pub struct PageOrdering {
    order_rules: Vec<PageOrderRule>,
    printings: Vec<Printing>,
}

impl PageOrdering {
    fn add_rule_line(&mut self, line: &str) {
        let split = line.trim().split('|').collect::<Vec<_>>();
        if split.len() != 2 {
            panic!("Rule lines should always be split into 2 - found {} from '{}'", split.len(), line);
        }

        let left = parse_page_number(split[0]);
        let right = parse_page_number(split[1]);

        self.order_rules.push(PageOrderRule(left, right));
    }

    fn add_printing(&mut self, line: &str) {
        let printing = line.trim()
            .split(',')
            .map(|n| parse_page_number(n))
            .collect::<Printing>();

        self.printings.push(printing);
    }

    pub fn check_order_printing(&self) -> (usize, u32) {
        let mut count = 0;
        let mut total = 0;

        for printing in self.printings.iter() {
            if printing.validate(&self.order_rules) {
                count += 1;

                let mid_point = printing.len() / 2;

                total += printing[mid_point];
            }
        }

        (count, total)
    }

}

impl From<&str> for PageOrdering {
    fn from(value: &str) -> Self {
        let mut page_ordering = PageOrdering::default();

        let mut completed_rules = false;
        for line in value.trim().lines() {
            if line.is_empty() {
                completed_rules = true;
            } else if completed_rules {
                page_ordering.add_printing(line);
            } else {
                page_ordering.add_rule_line(line);
            }
        }

        page_ordering
    }
}

fn parse_page_number(n: &str) -> u32 {
    n.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"
    47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
    ";

    #[test]
    fn valid() {
        let single = r"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
        ".trim();

        let page_ordering = PageOrdering::from(single);

        let (valid_count, _) = page_ordering.check_order_printing();

        assert_eq!(1, valid_count);
    }

    #[test]
    fn invalid() {
        let printing: Printing = vec![
            75,
            97,
            47,
            61,
            53,
        ];

        let rules = vec![
            PageOrderRule(97, 75)
        ];

        let valid = printing.validate(&rules);

        assert!(!valid);
    }

    #[test]
    fn example() {
        let page_ordering = PageOrdering::from(EXAMPLE);

        let (valid_count, total) = page_ordering.check_order_printing();

        assert_eq!(valid_count, 3);

        assert_eq!(total, 143)
    }
}