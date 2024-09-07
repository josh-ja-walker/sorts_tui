use std::fmt::{self, Display};

use strum_macros::EnumIter;

use crossterm::style::{Color as crossterm_color, Stylize};
use tui::style::Color as tui_color;

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
	
	pub fn crossterm_color(&self) -> crossterm_color {
		let (r, g, b) = self.rgb();
		crossterm_color::Rgb { r, g, b }
	}
	
	pub fn tui_color(&self) -> tui_color {
		let (r, g, b) = self.rgb();
		tui_color::Rgb(r, g, b)
	}
}

impl Display for Sort {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let text = format!("{} Sort", match self {
			Sort::Bogo => "Bogo",
			Sort::Bubble => "Bubble",
			Sort::Insertion => "Insertion",
			Sort::Merge => "Merge",
			Sort::Quick => "Quick",
		}).with(self.crossterm_color());

		write!(f, "{text}")
	}
}
