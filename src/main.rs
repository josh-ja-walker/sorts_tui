use clap::{value_parser, Parser};
use console::Term;
use sort::Sort;

use std::{fmt::{self, Display}, io::{self}};

mod sort_type;
mod sort;
mod terminal;

use sort_type::SortType;


const DEFAULT_QUANTITY: u64 = 50;
const MIN_QUANTITY: u64 = 2;
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
    // #[arg(short, long = "sort")]
    sort_type: SortType,
	
    /// Number of items to sort (2 - 150)
    #[arg(short = 'n', long, 
		default_value_t = DEFAULT_QUANTITY, 
		value_parser = value_parser!(u64).range(MIN_QUANTITY..MAX_QUANTITY))]
	quantity: u64,

	/// How often interface reloads (in milliseconds)
    #[arg(short, long, default_value_t = DEFAULT_TICK)]
    tick_rate: u64,
}

fn main() -> Result<(), Error> {    
	let args = Args::parse();

	let count = Sort::with_args(args).run()?;
	Term::stdout().write_line(&format!("Sorted: {} performed", count))?;
	
	Ok(())
}
