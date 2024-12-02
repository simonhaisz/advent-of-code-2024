mod location;

use location::{LocationPair, Optimization};
use utils::{part_selection, Part, Timer};

fn main() {
    let _timer = Timer::new();
    
    let text = std::fs::read_to_string("./day-01/input.txt").unwrap();

    let pair = LocationPair::from(text);

    let part = part_selection();

    match part {
        Part::One => part_1(pair),
        Part::Two => part_2(pair),
    }
}

fn part_1(pair: LocationPair) {
    let total_distance = pair.total_pair_distance();

    println!("{total_distance}")
}

fn part_2(pair: LocationPair) {
    let optimization = optimization_selection();

    println!("Optimization: {optimization:?}");

    let similarity_score = pair.similarity_score(optimization);

    println!("{similarity_score}")
}

fn optimization_selection() -> Optimization {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 2 {
        if args[2] == "--index" {
            Optimization::Indexed
        } else {
            Optimization::None
        }
    } else {
        Optimization::None
    }
}