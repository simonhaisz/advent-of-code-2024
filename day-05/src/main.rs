use page_order::PageOrdering;
use utils::{part_selection, Part, Timer};

mod page_order;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-05/input.txt").unwrap();

    let part = part_selection();

    match part {
        Part::One => part_1(&text),
        Part::Two => part_2(&text),
    }
}

fn part_1(input: &str) {
    let page_ordering = PageOrdering::from(input);

    let (_, count) = page_ordering.check_order_printing();

    println!("{count}");
}

fn part_2(_input: &str) {
    unimplemented!()
}
