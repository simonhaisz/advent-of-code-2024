#[derive(Default)]
pub struct PageOrder {
    order_rules: Vec<u32>,
}

impl PageOrder {
    pub fn add_rule_line(&mut self, line: &str) {
        let split = line.trim().split('|').collect::<Vec<_>>();
        if split.len() != 2 {
            panic!("Rule lines should always be split into 2 - found {} from {}", split.len());
        }

        self.add_rule(parse_page_number(split[0]), parse_page_number(split[1]));
    }

    pub fn add_rule(&mut self, left: u32, right: u32) {
        let left_index = self.order_rules.iter().find(|&&p| p == left);
        let right_index = self.order_rules.iter().find(|&&p| p == right);

        if left_index.is_none() && right_index.is_none() {
            self.order_rules.push(left);
            self.order_rules.push(right);
        } else if right_index.is_none() {
            self.order_rules.push(right);
        }
    }
}

fn parse_page_number(n: &str) -> u32 {
    n.parse::<u32>().unwrap()
}