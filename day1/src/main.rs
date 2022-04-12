fn main() {
	let filename = "data.txt";
	let file_contents = std::fs::read_to_string(filename).expect("could not read file");
	let split = file_contents.split("\n");
	let lines: Vec<&str> = split.collect::<Vec<&str>>();

	if lines.len() < 2 {
		println!("Given file has less than two lines");
		return;
	}

	let mut last: u16 = lines[0].trim().parse().unwrap();
	let mut current: u16;

	let mut num_increased = 0;

	for line in lines.into_iter().skip(1) {
		current = line.trim().parse().unwrap();

		if current > last {
			num_increased += 1;
		}

		last = current;
	}

	println!("Result: {}", num_increased);
}
