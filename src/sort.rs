use std::fmt::{self, Display};

use ratatui::{style::Color, text::Span};
use strum_macros::EnumIter;

use crate::sort_instance::SortInstance;

#[derive(Clone, Copy, EnumIter)]
pub enum Sort {
	Bogo,
	Bubble,
	Insertion,
	Merge,
	Quick
}

impl Sort {	
	pub fn perform_with(self, num_items: usize) -> SortInstance {
		SortInstance::new(self, num_items)
	}
	
	fn rgb(&self) -> (u8, u8, u8) {
		match self {
			Sort::Bogo => (219, 77, 59),
			Sort::Bubble => (59, 126, 219),
			Sort::Insertion => (219, 124, 59),
			Sort::Merge => (50, 150, 52),
			Sort::Quick => (240, 128, 128),
		}		
	}

	pub fn color(&self) -> Color {
		let (r, g, b) = self.rgb();
		Color::Rgb(r, g, b)
	}

	fn uncolored_string(&self) -> String {
		format!("{} Sort", match self {
			Sort::Bogo => "Bogo",
			Sort::Bubble => "Bubble",
			Sort::Insertion => "Insertion",
			Sort::Merge => "Merge",
			Sort::Quick => "Quick",
		})
	}
}

impl Display for Sort {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", Span::styled(self.uncolored_string(), self.color()))
	}
}
