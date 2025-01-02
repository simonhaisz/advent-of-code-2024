use utils::{part_selection, Part, Timer};
use warehouse::Warehouse;

mod warehouse;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-15/input.txt").unwrap();

    let part = part_selection();

    match part {
        Part::One => part_1(&text),
        Part::Two => part_2(&text),
    }
}

fn part_1(text: &str) {
    let mut warehouse = Warehouse::from(text);

    warehouse.move_robot();

    let robot_gps_total = warehouse.robot_gps_total();

    println!("{robot_gps_total}");
}

fn part_2(text: &str) {
    let mut warehouse = Warehouse::from(text).embiggen();

    warehouse.move_robot();

    let robot_gps_total = warehouse.robot_gps_total();

    println!("{robot_gps_total}");
}
