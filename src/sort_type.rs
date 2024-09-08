use std::fmt::{self, Display};

use clap::ValueEnum;
use ratatui::{style::Color, text::Span};
use strum_macros::EnumIter;

#[derive(ValueEnum, EnumIter, Debug, Clone, Copy)]
pub enum SortType {
	Bogo,
	Bubble,
	Insertion,
	Merge,
	Quick
}

impl SortType {	
	fn rgb(&self) -> (u8, u8, u8) {
		match self {
			SortType::Bogo => (219, 77, 59),
			SortType::Bubble => (59, 126, 219),
			SortType::Insertion => (219, 124, 59),
			SortType::Merge => (50, 150, 52),
			SortType::Quick => (240, 128, 128),
		}		
	}

	pub fn color(&self) -> Color {
		let (r, g, b) = self.rgb();
		Color::Rgb(r, g, b)
	}

	fn uncolored_string(&self) -> String {
		format!("{} Sort", match self {
			SortType::Bogo => "Bogo",
			SortType::Bubble => "Bubble",
			SortType::Insertion => "Insertion",
			SortType::Merge => "Merge",
			SortType::Quick => "Quick",
		})
	}
}

impl Display for SortType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", Span::styled(self.uncolored_string(), self.color()))
	}
}
