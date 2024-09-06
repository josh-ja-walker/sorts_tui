use strum_macros::{Display, EnumIter};
use tui::style::Color;

use crate::sort_instance::SortInstance;

#[derive(Clone, Copy, Display, EnumIter)]
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

	pub fn color(&self) -> Color {
		match self {
			Sort::Bogo => Color::LightRed,
			Sort::Bubble => todo!(),
			Sort::Insertion => todo!(),
			Sort::Merge => todo!(),
			Sort::Quick => todo!(),
		}
	}
}
