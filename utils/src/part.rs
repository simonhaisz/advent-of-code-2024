pub enum Part {
    One,
    Two
}

pub fn part_selection() -> Part {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Missing part selection argument")
    }

    let selection = &args[1];

    match selection.as_str() {
        "--part-1" => Part::One,
        "--part-2" => Part::Two,
        _ => panic!("Unknown selection '{selection}'")
    }
}