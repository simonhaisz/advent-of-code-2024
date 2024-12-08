use utils::{part_selection, Part, Timer};
use word_search::WordSearch;

mod word_search;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-04/input.txt").unwrap();

    let part = part_selection();

    match part {
        Part::One => part_1(&text),
        Part::Two => part_2(&text),
    }
}

fn part_1(input: &str) {
    let word_search = WordSearch::new("XMAS", input);

    let count = word_search.search_all();

    println!("{count}");
}

fn part_2(input: &str) {
    let word_search = WordSearch::new("MAS", input);

    let count = word_search.search_x_all();

    println!("{count}");
}
