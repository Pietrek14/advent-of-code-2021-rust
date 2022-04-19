fn binary_digit_value(chr: &char) -> u8 {
	return match &chr {
		'1' => 1,
		'0' => 0,
		_ => panic!("Invalid input"),
	};
}

fn not(bin_num: &Vec<bool>) -> Vec<bool> {
	let mut result = vec![];
	for &digit in bin_num {
		result.push(!digit);
	}
	return result;
}

fn bin_to_dec(bin_num: &Vec<bool>) -> u32 {
	let mut result = 0;
	let mut power = 0;
	for digit in bin_num.iter().rev() {
		result += if *digit { 2u32.pow(power) } else { 0 };
		power += 1;
	}
	return result;
}

fn main() {
	let input = std::fs::read_to_string("input.txt").expect("Could not open file");

	let mut lines = input.lines();

	let lines_cpy = lines.clone();

	let lines_num = lines_cpy.count();

	let num_length = lines.next().unwrap().len();

	let mut gamma_rate = vec![false; num_length];

	println!("num length: {}", num_length);

	for digit_index in 0..num_length {
		let mut digit_sum: u32 = 0;

		let lines_cpy = lines.clone();

		for line in lines_cpy {
			let digit = binary_digit_value(&line.chars().nth(digit_index).unwrap());
			digit_sum += u32::from(digit);
		}

		println!("{}", digit_sum);

		gamma_rate[digit_index] = digit_sum > lines_num as u32 / 2;
	}

	let gamma_rate_dec = bin_to_dec(&gamma_rate);

	println!("Gamma rate: {}", gamma_rate_dec);

	let epsilon_rate = not(&gamma_rate);
	let epsilon_rate_dec = bin_to_dec(&epsilon_rate);

	println!("Epsilon rate: {}", &epsilon_rate_dec);

	let power_consumption = gamma_rate_dec * epsilon_rate_dec;

	println!("Power consumption: {}", power_consumption);
}
