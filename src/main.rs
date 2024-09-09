use clap::{CommandFactory, Parser};
use sort::{Sort, SortSnapshot};
use terminal::Terminal;

use std::{fmt, io, time::Duration};

mod sort;
mod sort_type;
mod count;
mod analytics;
mod terminal;

use sort_type::SortType;


const MIN_QUANTITY: usize = 2;
const DEFAULT_QUANTITY: usize = 50;
const MAX_QUANTITY: usize = 150;

const DEFAULT_TICK: u64 = 100;


enum Error {
	Interrupted,
	QuantityOutOfRange(usize),
	BarOverflow(usize),
	IOError(io::Error),
}

impl From<io::Error> for Error {
	fn from(error: io::Error) -> Self {
		Error::IOError(error)
	}
}

impl fmt::Debug for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let written = writeln!(f, "{}", match self {
			Error::Interrupted => String::from("Interrupted"),
			Error::BarOverflow(quantity) => format!("Terminal cannot render {} bars. Resize terminal or use smaller quantity", quantity),
			Error::QuantityOutOfRange(quantity) => format!("quantity {} is not in range [{} - {}]", quantity, MIN_QUANTITY, MAX_QUANTITY),
			Error::IOError(io_err) => io_err.to_string(),
		});

		if let Error::QuantityOutOfRange(_) = self {
			Args::command().print_help().unwrap();
		}

		written
	}
}


/// Sorts TUI: terminal interface for rendering and simulating sorting algorithms
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Sort algorithm to use
    sort_type: SortType,

	/// Number of items to sort (2 - 150)
    #[arg(short = 'n', long, default_value_t = DEFAULT_QUANTITY)]
	quantity: usize,

	/// How often interface reloads (in milliseconds)
    #[arg(short, long, default_value_t = DEFAULT_TICK)]
    tick_rate: u64,
}

impl Args {
	fn parse() -> Result<Args, Error> {
		let args: Args = <Args as Parser>::parse();
		
		/* If quantity is valid return */
		if (MIN_QUANTITY..=MAX_QUANTITY).contains(&args.quantity) {
			Ok(args)
		} else {
			Err(Error::QuantityOutOfRange(args.quantity))
		}
	}
}


fn main() -> Result<(), Error> {    
	let args = Args::parse()?;
	let mut terminal = Terminal::new()?;

	let _count = Sort::from_args(&mut terminal, args).run()?;

	terminal.restore()?;

	Ok(())
}


trait Renderer {
	fn tick(&mut self, snapshot: SortSnapshot, duration: Duration) -> Result<(), Error> {
		self.render(snapshot)?;
		self.sleep(duration)
	}
	
	fn render(&mut self, snapshot: SortSnapshot) -> Result<(), Error>;
	fn sleep(&self, duration: Duration) -> Result<(), Error>;
}