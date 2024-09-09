use clap::{value_parser, Parser};
use sort::{Sort, SortSnapshot};
use terminal::Terminal;

use std::{fmt::{self, Display}, io::{self}, time::Duration};

mod sort;
mod sort_type;
mod count;
mod analytics;
mod terminal;

use sort_type::SortType;


const MIN_QUANTITY: u64 = 2;
const DEFAULT_QUANTITY: u64 = 50;
const MAX_QUANTITY: u64 = 150;

const DEFAULT_TICK: u64 = 100;


#[derive(Debug)]
enum Error {
	Interrupted,
	IOError(io::Error),
}

impl From<io::Error> for Error {
	fn from(error: io::Error) -> Self {
		Error::IOError(error)
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
				Error::Interrupted => String::from("interrupted"),
				Error::IOError(io_err) => io_err.to_string(),
			}
		)
	}
}


/// Sort terminal visualizer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Sort algorithm to use
    sort_type: SortType,
	
    /// Number of items to sort (2 - 150)
    #[arg(short = 'n', long, 
		default_value_t = DEFAULT_QUANTITY, 
		value_parser = value_parser!(u64).range(MIN_QUANTITY..=MAX_QUANTITY))]
	quantity: u64,

	/// How often interface reloads (in milliseconds)
    #[arg(short, long, default_value_t = DEFAULT_TICK)]
    tick_rate: u64,
}

fn main() -> Result<(), Error> {    
	let args = Args::parse();
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