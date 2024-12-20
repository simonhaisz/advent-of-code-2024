use report::{parse_reports, Report};
use utils::{part_selection, Part, Timer};

mod report;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-02/input.txt").unwrap();

    let reports = parse_reports(&text);

    let part = part_selection();

    match part {
        Part::One => part_1(reports),
        Part::Two => part_2(reports),
    }
}

fn part_1(reports: Vec<Report>) {
    let safe_count = reports.iter().filter(|&r| r.is_safe()).count();

    println!("{safe_count}");
}

fn part_2(reports: Vec<Report>) {
    let tolerance = tolerance_selection();

    let safe_count = match tolerance {
        Tolerance::BruteForce => {
            reports.iter()
            .filter(|&r| r.is_safe_with_brute_force_tolerance())
            .count()
        },
        Tolerance::Optimized => {
            reports.iter()
            .filter(|&r| r.is_safe_with_optimized_tolerance())
            .count()
        }
    };

    println!("{safe_count}");
}

enum Tolerance {
    BruteForce,
    Optimized,
}

fn tolerance_selection() -> Tolerance {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 2 {
        match args[2].as_str() {
            "--brute-force" => return Tolerance::BruteForce,
            "--optimized" => return Tolerance::Optimized,
            _ => {}
        }
    }

    panic!("Tolerance not specified - must include '--brute-force' or '--optimized'")
}