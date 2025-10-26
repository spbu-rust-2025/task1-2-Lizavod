use std::io;

fn main() {
	
	let mut sum_n: i32 = 0;
	loop{
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Couldn't read the line");
		let number = input.trim();	
		if number == "-1"{
			break;
		}
		match number.parse::<i32>() {
			Ok(num) => {
				if num >= 0{
					sum_n += num;
				}
				else{
					println!("NaN");
					return;
				}
			}
			Err(_) => {
				println!("NaN");
				return;
			}
		}
	}
	println!("{}", sum_n);
}