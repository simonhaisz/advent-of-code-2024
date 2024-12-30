use crate::machine::{Machine, XY};
use pest::{iterators::{Pair, Pairs}, Parser};
use pest_derive::Parser;

#[derive(Debug, PartialEq, Eq)]
pub struct Lobby {
	machines: Vec<Machine>,
}

impl Lobby {
	pub fn total_cost(&self) -> u64 {
		self.machines.iter()
			.map(|m| m.solve_prize())
			.filter(|m| m.is_some())
			.map(|s| s.unwrap().cost())
			.sum()
	}
}

#[derive(Parser)]
#[grammar = "lobby.pest"]
struct LobbyParser;

impl From<(&str, Option<u64>)> for Lobby {
	fn from((text, claw_offset): (&str, Option<u64>)) -> Self {
		let parsed_machines = LobbyParser::parse(Rule::lobby, text)
			.expect("failed to parse machines")
			.next().unwrap();

		let mut machines = vec![];

		for parsed_machine in parsed_machines.into_inner() {

			let mut a_button: Option<XY> = None;
			let mut b_button: Option<XY> = None;
			let mut prize: Option<XY> = None;

			match parsed_machine.as_rule() {
				Rule::machine => {
					for parsed_machine_property in parsed_machine.into_inner() {
						match parsed_machine_property.as_rule() {
							Rule::a_button => {
								a_button.replace(extract_xy(parsed_machine_property.into_inner()));
							},
							Rule::b_button => {
								b_button.replace(extract_xy(parsed_machine_property.into_inner()));
							},
							Rule::prize => {
								prize.replace(extract_xy(parsed_machine_property.into_inner()));
							},
							_ => panic!("Unexpected rule: {}", parsed_machine_property.as_str())
						}
					}

					let a_button = a_button.unwrap();
					let b_button = b_button.unwrap();
					let mut prize = prize.unwrap();

					if let Some(claw_offset) = claw_offset {
						prize.0 += claw_offset;
						prize.1 += claw_offset;
					}

					machines.push(Machine{ a_button, b_button, prize });
				},
				Rule::EOI => {},
				_ => panic!("unexpected rule: {}", parsed_machine.as_str())
			}
		}

		Self { machines }
	}
}

fn extract_xy(mut prop_pair: Pairs<'_, Rule>) -> XY {
	let x = prop_pair.next().unwrap();
	let x = parse_value(&x);

	let y = prop_pair.next().unwrap();
	let y = parse_value(&y);

	XY(x, y)
}

fn parse_value(value: &Pair<'_, Rule>) -> u64 {
	value.as_str().parse::<_>().unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &'static str = r"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
	";

	#[test]
	fn parse_example() {
		let lobby = Lobby::from((EXAMPLE.trim(), None));

		assert_eq!(
			Lobby { machines: vec![
				Machine { a_button: XY(94, 34), b_button: XY(22, 67), prize: XY(8400, 5400) },
				Machine { a_button: XY(26, 66), b_button: XY(67, 21), prize: XY(12748, 12176) },
				Machine { a_button: XY(17, 86), b_button: XY(84, 37), prize: XY(7870, 6450) },
				Machine { a_button: XY(69, 23), b_button: XY(27, 71), prize: XY(18641, 10279) },
			]},
			lobby
		);
	}

	#[test]
	fn cost_example() {
		let lobby = Lobby::from((EXAMPLE.trim(), None));

		let total_cost = lobby.total_cost();

		assert_eq!(480, total_cost);
	}

	#[test]
	fn cost_claw_offset_example() {
		let lobby = Lobby::from((EXAMPLE.trim(), Some(10000000000000)));

		let total_cost = lobby.total_cost();

		assert_eq!(875318608908, total_cost);
	}
}