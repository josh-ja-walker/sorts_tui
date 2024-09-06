use std::fmt::{self, Display};

use strum_macros::EnumIter;
use tui::style::Color;

#[derive(Clone, Copy, EnumIter)]
pub enum Sort {
	Bogo,
	Bubble,
	Insertion,
	Merge,
	Quick
}

impl Sort {	
	pub fn color(&self) -> Color {
		match self {
			Sort::Bogo => Color::LightRed,
			Sort::Bubble => Color::Cyan,
			Sort::Insertion => todo!(),
			Sort::Merge => todo!(),
			Sort::Quick => todo!(),
		}
	}
}

impl Display for Sort {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} Sort", match self {
			Sort::Bogo => "Bogo",
			Sort::Bubble => "Bubble",
			Sort::Insertion => "Insertion",
			Sort::Merge => "Merge",
			Sort::Quick => "Quick",
		})
	}
}
