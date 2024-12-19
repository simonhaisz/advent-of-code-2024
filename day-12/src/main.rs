use garden::Garden;
use utils::{part_selection, Part, Timer};

mod garden;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-12/input.txt").unwrap();

    let part = part_selection();

    match part {
        Part::One => part_1(&text),
        Part::Two => unimplemented!(),
    }
}

fn part_1(input: &str)     {
    let garden = Garden::from(input);

    let fencing_price = garden.fencing_price();

    println!("{fencing_price}");
}
