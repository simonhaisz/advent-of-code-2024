use pest::Parser;
use pest_derive::Parser;
type XY = (u32, u32);

pub struct Machine {
	a_button: XY,
	b_button: XY,
	prize: XY,
}

#[derive(Parser)]
#[grammar = "machine.pest"]
pub struct MachineParser;

pub fn parse_monkeys(input: &str) -> Vec<Machine> {
	let parsed_machines = MachineParser::parse(Rule::file, input)
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
							parsed_machine_property.into_inner()
						},
						Rule::b_button => {

						},
						Rule::prize => {

						},
						_ => panic!("")
					}
				}
			},
			_ => panic!("unexpected rule")
		}
	}

	machines
}