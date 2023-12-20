use sort::Sort;
use std::{io, io::Write, str::FromStr};
mod sort;

const MAX: u32 = 100;

fn main() {    
	loop {
		print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear console
		println!("\n-----------Sorting-Algorithms-----------");
		println!(" - (1) Bogo Sort");
		println!(" - (2) Bubble Sort");
		println!(" - (3) Insertion Sort");
		println!(" - (4) Merge Sort");
		println!(" - (5) Quick Sort");
		
		let sort_num: u8 = match parse_read::<u8>("Select a sorting algorithm: ") {
			Ok(val @ 1..=4) => val,
			_ => continue,
		};
		
		let num_req: String = format!("Enter the number of items to sort [2 - {MAX}]: ");
		let num_items: u32 = match parse_read::<u32>(&num_req) {
			Ok(val @ 2..=MAX) => val,
			_ => continue,
		};
		
		choose_sort(sort_num).perform(num_items);

		read("Press enter to continue").unwrap();
	}
}

fn choose_sort(sort_num: u8) -> Sort {
	match sort_num {
		1 => Sort::Bogo,
		2 => Sort::Bubble,
		3 => Sort::Insertion,
		4 => Sort::Merge,
		5 => Sort::Quick,
		_ => panic!("Should have continued"),
	}
}

fn parse_read<T : FromStr>(request: &str) -> io::Result<T> {
	match read(request)?.trim().parse::<T>() {
		Ok(parsed) => Ok(parsed),
		Err(_) => Err(io::Error::other("could not parse"))
	}
}

fn read(request: &str) -> io::Result<String> {
	// Output a request for input to the console
	print!("{}", request); 
	io::stdout().flush()?;

	// Read input from console and save as string
	let mut input = String::new();
	io::stdin().read_line(&mut input)?; 
	
	Ok(input) // Return result containing read string
}