use disk_map::Disk;
use utils::{part_selection, Part, Timer};

mod disk_map;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-09/input.txt").unwrap();

    let part = part_selection();

    match part {
        Part::One => part_1(&text),
        Part::Two => part_2(&text),
    }
}

fn part_1(input: &str) {
    let disk = Disk::from(input);
    let disk = disk.compact_blocks();

    let checksum = disk.checksum();

    println!("{checksum}");
}

fn part_2(input: &str) {
    let disk = Disk::from(input);
    let disk = disk.compact_files();

    let checksum = disk.checksum();

    println!("{checksum}");
}
