struct Submarine {
	depth: i32,
	horizontal_position: i32,
}

impl Submarine {
	fn new() -> Self { Submarine { depth: 0, horizontal_position: 0} }

	fn forward(&mut self, distance: i32) {
		self.horizontal_position += distance;
	}

	fn down(&mut self, distance: i32) {
		self.depth += distance;
	}

	fn up(&mut self, distance: i32) {
		self.depth -= distance;
	}
}

fn main() {
	let mut submarine = Submarine::new();

	let input = std::fs::read_to_string("input.txt").expect("Could not open file");

	for line in input.lines() {
		let mut iter = line.split_whitespace();

		let command = iter.next().unwrap();
		let value = iter.next().unwrap().parse::<i32>().unwrap();

		match command {
			"forward" => submarine.forward(value),
			"down" => submarine.down(value),
			"up" => submarine.up(value),
			_ => panic!("Unknown command")
		};
	}

	println!("Final depth: {}, final horizontal position: {}", submarine.depth, submarine.horizontal_position);

	println!("Their product: {}", submarine.depth * submarine.horizontal_position);
}
