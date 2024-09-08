use console::Term;
use dialoguer::{Input, Select};

use strum::IntoEnumIterator;

use std::{fmt::{self, Display}, io::{self}};

mod sort_type;
mod sort;
mod terminal;

use sort_type::SortType;
use sort::Sort;

const MIN: usize = 2;
const DEFAULT: usize = 50;
const MAX: usize = 150;


#[derive(Debug)]
enum Error {
	Interrupted,
	IOError(io::Error),
	DialoguerError(dialoguer::Error)
}

impl From<io::Error> for Error {
	fn from(error: io::Error) -> Self {
		Error::IOError(error)
	}
}

impl From<dialoguer::Error> for Error {
	fn from(error: dialoguer::Error) -> Self {
		Error::DialoguerError(error)
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
				Error::Interrupted => String::from("interrupted"),
				Error::IOError(io_err) => io_err.to_string(),
				Error::DialoguerError(dialoguer_err) => dialoguer_err.to_string(),
			}
		)
	}
}


fn main() -> Result<(), Error> {    
	loop {
		let choice = Select::new()
			.with_prompt("Sorts TUI")
			.items(&format_items(vec!["Start", "Settings", "Quit"]))
			.default(0)
			.report(false)
			.interact()?;

		let action = match choice {
			0 => run_sort(),
			1 => edit_settings(),
			2 => { break; },
			_ => unreachable!()
		};

		match action {
			Ok(()) => (),
			Err(Error::Interrupted) => { Term::stdout().clear_screen()?; () },
			err => err? 
		}
	}

	Ok(())
}

/* Format select options */
fn format_items<T>(options: Vec<T>) -> Vec<String> where T: Display {
    options.iter()
        .map(|option| format!("\u{2022} {option}"))
        .collect()
}

/* Proxy to handle quit and interrupted errors */
fn run_sort() -> Result<(), Error> {
	let chosen_sort: SortType = choose_sort()?;
	let num_items = choose_num_items()?;

	let sort_instance: Sort = chosen_sort.perform_with(num_items);
	let count = sort_instance.run()?;

	Term::stdout().write_line("Sorted")?;
	Term::stdout().write_line(&format!("{} performed", count))?;
	
	Term::stdout().read_line()?;
	Term::stdout().clear_screen()?;

	Ok(())
}

/* Edit running settings */
fn edit_settings() -> Result<(), Error> {
	todo!()
}

/* Choose sort to run */
fn choose_sort() -> Result<SortType, Error> {
	let sorts: Vec<SortType> = SortType::iter().collect();

	let index = Select::new()
		.with_prompt("Which sorting algorithm")
		.items(&format_items(sorts.clone()))
		.default(0)
		.interact()?;

	Ok(sorts[index])
}

/* Input number of items */
fn choose_num_items() -> Result<usize, Error> {
	let num_items = Input::<usize>::new()
		.with_prompt("Number of items to sort")
		.validate_with(|n: &usize| (MIN..MAX + 1).contains(n)
			.then_some(())
			.ok_or(format!("must be between {} and {}", MIN, MAX)))
		.default(DEFAULT)
		.interact()?;

	Ok(num_items)
}
