use console::Term;
use dialoguer::{Input, Select};

use tui::{backend::CrosstermBackend, Terminal};
use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

use strum::IntoEnumIterator;

use std::{fmt::Display, io};

mod sort;
mod sort_instance;

use sort::Sort;
use sort_instance::SortInstance;

const MIN: usize = 2;
const MAX: usize = 100;


/* Format select options */
fn format_items<T>(options: Vec<T>) -> Vec<String> where T: Display {
    options.iter()
        .map(|option| format!("\u{2022} {option}"))
        .collect()
}

fn main() -> io::Result<()> {    
	let sorts: Vec<Sort> = Sort::iter().collect();

	loop {
		let sort_choice = Select::new()
			.with_prompt("Sorting Algorithms")
			.items(&format_items(
				sorts.iter()
					.map(|s| format!("{s} Sort"))
					.collect::<Vec<String>>()))
			.report(false)
			.default(0)
			.interact()
			.unwrap();

		let num_items = Input::<usize>::new()
			.with_prompt(format!("Enter number of items [{MIN} - {MAX}]"))
			.validate_with(|n: &usize| (MIN..MAX).contains(n)
				.then_some(())
				.ok_or(format!("must be between {MIN} and {MAX}")))
			.interact()
			.unwrap();

		let chosen_sort: Sort = sorts[sort_choice];
		println!("Took {} iterations", run(chosen_sort.perform_with(num_items))?);

		Term::stdout().read_line()?;
		Term::stdout().clear_screen()?;
	}
}

/* Render and run  */
fn run(sort_instance: SortInstance) -> io::Result<u64> {
    /* Setup terminal */
    crossterm::terminal::enable_raw_mode()?;

	let mut stdout = io::stdout();
	crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
	
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

	/* Run sorting algorithm */
	let iterations = sort_instance.run(&mut terminal)?;

    /* Restore terminal */
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(iterations)
}
