fn get_num(str: &str) -> u32 {
	return str.trim().parse::<u32>().unwrap();
}

fn sum(x: &str, y: &str, z: &str) -> u32 {
	return get_num(x) + get_num(y) + get_num(z);
}

fn main() {
	let filename = "data.txt";
	let file_contents = std::fs::read_to_string(filename).expect("could not read file");
	let split = file_contents.split("\n");
	let lines: Vec<&str> = split.collect::<Vec<&str>>();

	if lines.len() < 3 {
		println!("Given file has less than three lines");
		return;
	}

	let mut last = sum(
		lines[0],
		lines[1],
		lines[2]
	);
	let mut current;

	let mut num_increased = 0;

	for i in 3..lines.len() {
		current = sum(
			lines[i - 2],
			lines[i - 1],
			lines[i]
		);

		if current > last {
			num_increased += 1;
		}

		last = current;
	}

	println!("Result: {}", num_increased);
}
