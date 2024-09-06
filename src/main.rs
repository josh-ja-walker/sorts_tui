use console::Term;
use dialoguer::{Input, Select};

use tui::{backend::CrosstermBackend, Terminal};
use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode}, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

use strum::IntoEnumIterator;

use std::{fmt::{self, Display}, io, time::Duration};

mod sort;
mod sort_instance;

use sort::Sort;
use sort_instance::{Count, SortInstance};

const MIN: usize = 2;
const DEFAULT: usize = 50;
const MAX: usize = 100;

#[derive(Debug)]
enum Error {
	Interrupted,
	Quit,
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
				Error::Quit => String::from("quit"),
				Error::Interrupted => String::from("interrupted"),
				Error::IOError(io_err) => io_err.to_string(),
				Error::DialoguerError(dialoguer_err) => dialoguer_err.to_string(),
			}
		)
	}
}

/* Format select options */
fn format_items<T>(options: Vec<T>) -> Vec<String> where T: Display {
    options.iter()
        .map(|option| format!("\u{2022} {option}"))
        .collect()
}

fn main() -> Result<(), Error> {    
	loop {
		match run_loop() {
			Ok(()) | Err(Error::Interrupted) => (),
			Err(Error::Quit) => { break; },
			err => err? 
		}
	}

	Ok(())
}

/* Proxy to handle quit and interrupted errors */
fn run_loop() -> Result<(), Error> {
	let chosen_sort: Sort = sort_choice()?;
	let num_items = num_items_input()?;

	let count = run(chosen_sort, num_items)?;

	Term::stdout().write_line("Sorted")?;
	Term::stdout().write_line(&format!("{} performed", count))?;
	
	Term::stdout().read_line()?;
	Term::stdout().clear_screen()?;

	Ok(())
}

/* Choose sort to run */
fn sort_choice() -> Result<Sort, Error> {
	let sorts: Vec<Sort> = Sort::iter().collect();
	let mut options: Vec<String> = sorts.iter()
		.map(Sort::to_string).collect();

	options.push(String::from("Quit"));

	let index = Select::new()
		.with_prompt("Sorting Algorithms")
		.items(&format_items(options))
		.default(0)
		.interact()?;

	sorts.get(index).ok_or(Error::Quit).copied()
}

/* Input number of items */
fn num_items_input() -> Result<usize, Error> {
	let prompt: String = format!("Sort how many items? [{} - {}] (default = {})", MIN, MAX, DEFAULT);
	
	let n = Input::<usize>::new()
		.with_prompt(prompt)
		.validate_with(|n: &usize| (MIN..MAX + 1).contains(n)
			.then_some(())
			.ok_or(format!("must be between {} and {}", MIN, MAX)))
		.default(DEFAULT)
		.show_default(false)
		.interact()?;

	Ok(n)
}

/* Render and run */
fn run(sort: Sort, num_items: usize) -> Result<Count, Error> {
    /* Setup terminal */
    crossterm::terminal::enable_raw_mode()?;

	let mut stdout = io::stdout();
	crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
	
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

	/* Run sorting algorithm */
	let count = SortInstance::new(sort, num_items, &mut terminal).run()?;

    /* Restore terminal */
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(count)
}

/* Returns other io error if interrupted */
fn terminal_sleep(duration: Duration) -> Result<(), Error> {
	if event::poll(duration)? {
		if let Event::Key(key) = event::read()? {
			if let KeyCode::Char('q') = key.code {
				return Err(Error::Interrupted);
			}
		}
	}
	
	Ok(())
}
