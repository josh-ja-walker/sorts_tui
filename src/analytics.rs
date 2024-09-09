use std::fmt::{self, Display};

pub struct Analytics {
	worst: Complexity,
	average: Complexity,
	best: Complexity, 
	worst_space: Complexity,
}

impl Analytics {
    pub fn new(worst_time: Complexity, average_time: Complexity, best_time: Complexity, worst_space: Complexity) -> Analytics {
        Analytics {
            worst: worst_time,
            average: average_time,
            best: best_time,
            worst_space,
        }
    }
}

impl Display for Analytics {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Worst time: {}\nAverage time: {}\nBest time: {}\nWorst space: {}\n", 
			self.worst,
			self.average,
			self.best,
			self.worst_space
		)
	}
}

pub struct Complexity {
	notation: Notation,
	rate: Rate
}

impl Complexity {
	pub fn new(notation: Notation, rate: Rate) -> Complexity {
		Complexity {
			notation,
			rate,
		}
	}

	pub fn big_o(rate: Rate) -> Complexity {
		Complexity {
			notation: Notation::BigO,
			rate,
		}
	}
}

impl Display for Complexity {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}({})", self.notation, self.rate)
	}
}

pub enum Notation {
	#[allow(dead_code)] SmallO,
	BigO,
	Theta,
	UpperOmega,
	LowerOmega
}

impl Display for Notation {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Notation::SmallO => "o",
			Notation::BigO => "O",
			Notation::Theta => "\u{03B8}",
			Notation::UpperOmega => "\u{03A9}",
			Notation::LowerOmega => "\u{03C9}",
		})
	}
}

pub enum Rate {
	#[allow(dead_code)] Constant,
	Linear,
	Quadratic,
	#[allow(dead_code)] LogN,
	NLogN,
	NNFact,
	Infinite
}

impl Display for Rate {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Rate::Constant => "1",
			Rate::Linear => "n",
			Rate::Quadratic => "n\u{00B2}",
			Rate::LogN => "log n",
			Rate::NLogN => "n log n",
			Rate::NNFact => "n x n!",
			Rate::Infinite => "\u{221E}",
		})
	}
}
