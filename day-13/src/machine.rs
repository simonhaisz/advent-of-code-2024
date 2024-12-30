#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machine {
	pub a_button: XY,
	pub b_button: XY,
	pub prize: XY,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XY(pub u64, pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AB(pub u64, pub u64);

impl AB {
	pub fn cost(&self) -> u64 {
		self.0 * 3 + self.1
	}
}

impl Machine {
	pub fn solve_prize(&self) -> Option<AB> {
		solve_prize(&self.a_button, &self.b_button, &self.prize)
	}
}

fn solve_prize(a_button: &XY, b_button: &XY, prize: &XY) -> Option<AB> {
	let a_x = a_button.0 as i64;
	let a_y = a_button.1 as i64;
	let b_x = b_button.0 as i64;
	let b_y = b_button.1 as i64;
	let prize_x = prize.0 as i64;
	let prize_y = prize.1 as i64;
	
	let b_numerator = (a_x * prize_y - a_y * prize_x).abs();
	let b_denominator = (a_x * b_y - a_y * b_x).abs();

	if b_denominator == 0 {
		return None;
	}

	let b = b_numerator / b_denominator;

	let a_numerator = (prize_y - b_y * b).abs();
	let a_denoninator = a_y;

	if a_denoninator == 0 {
		return None;
	}

	let a = a_numerator / a_denoninator;

	let computed_x = a_x * a + b_x * b;
	let computed_y = a_y * a + b_y * b;

	if computed_x == prize_x && computed_y == prize_y {
		Some(AB(a as u64, b as u64))
	} else {
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn first_example_valid() {
		let machine = Machine{ a_button: XY(94, 34), b_button: XY(22, 67), prize: XY(8400, 5400) };

		let solution = machine.solve_prize().unwrap();

		assert_eq!(
			AB(80, 40),
			solution
		);

		assert_eq!(280, solution.cost());
	}

	#[test]
	fn second_example_invalid() {
		let machine = Machine{ a_button: XY(26, 66), b_button: XY(67, 21), prize: XY(12748, 12176) };

		let solution = machine.solve_prize();

		assert!(solution.is_none());
	}

	#[test]
	fn third_example_valid() {
		let machine = Machine{ a_button: XY(17, 86), b_button: XY(84, 37), prize: XY(7870, 6450) };

		let solution = machine.solve_prize().unwrap();

		assert_eq!(
			AB(38, 86),
			solution
		);

		assert_eq!(200, solution.cost())
	}

	#[test]
	fn fourth_example_invalid() {
		let machine = Machine{ a_button: XY(69, 23), b_button: XY(27, 71), prize: XY(18641, 10279) };

		let solution = machine.solve_prize();

		assert!(solution.is_none());
	}
}