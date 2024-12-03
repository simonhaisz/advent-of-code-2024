
use memory::Memory;
use utils::{part_selection, Part, Timer};

mod memory;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-03/input.txt").unwrap();

    let part = part_selection();

    match part {
        Part::One => part_1(&text),
        Part::Two => part_2(),
    }
}

fn part_1(memory: &str) {
    let memory = Memory::from(memory);

    let multiply_result = memory.multiplication_results();

    println!("{multiply_result}");
}

fn part_2() {
    unimplemented!()
}