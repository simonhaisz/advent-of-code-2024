use maze::Maze;
use utils::{part_selection, Part, Timer};

mod maze;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-16/input.txt").unwrap();

    let part = part_selection();

    match part {
        Part::One => part_1(&text),
        Part::Two => unimplemented!(),
    }
}

fn part_1(text: &str) {
    let maze = Maze::from(text);

    let lowest_score = maze.find_lowest_score_route().unwrap();

    println!("{lowest_score}");
}
