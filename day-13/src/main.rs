use lobby::Lobby;
use utils::{part_selection, Part, Timer};

mod machine;
mod lobby;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-13/input.txt").unwrap();

    let part = part_selection();

    match part {
        Part::One => part_1(&text),
        Part::Two => part_2(&text),
    }
}

fn part_1(text: &str) {
    let lobby = Lobby::from((text, None));

    let total_cost = lobby.total_cost();

    println!("{total_cost}");
}

fn part_2(text: &str) {
    let lobby = Lobby::from((text, Some(10000000000000)));

    let total_cost = lobby.total_cost();

    println!("{total_cost}");
}
