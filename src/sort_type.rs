use std::fmt::{self, Display};

use clap::ValueEnum;
use ratatui::style::Color;
use strum_macros::EnumIter;

use crate::{analytics::{Analytics, Complexity, Notation, Rate}, count::CountType};

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

	pub fn count_type(&self) -> CountType {
		match self {
			SortType::Bogo => CountType::Shuffles,
			SortType::Bubble => CountType::Comparisons,
			SortType::Insertion => todo!(),
			SortType::Merge => todo!(),
			SortType::Quick => todo!(),
		}
	}

	/* Worst, average and best time complexities */
	fn time_complexity(&self) -> (Complexity, Complexity, Complexity) {
		match self {
			SortType::Bogo => (
				Complexity::big_o(Rate::Infinite), 
				Complexity::new(Notation::LowerOmega, Rate::NNFact), 
				Complexity::new(Notation::UpperOmega, Rate::Linear), 
			),
			SortType::Bubble | SortType::Insertion => (
				Complexity::big_o(Rate::Quadratic), 
				Complexity::big_o(Rate::Quadratic), 
				Complexity::big_o(Rate::Linear), 
			),
			SortType::Merge => (
				Complexity::big_o(Rate::NLogN), 
				Complexity::new(Notation::Theta, Rate::NLogN), 
				Complexity::new(Notation::UpperOmega, Rate::NLogN), 
			),
			SortType::Quick => (
				Complexity::big_o(Rate::Quadratic),
				Complexity::big_o(Rate::NLogN),
				Complexity::big_o(Rate::NLogN),
			),
		}
	}

	/* Worst case space complexity */
	fn space_complexity(&self) -> Complexity {
		Complexity::big_o(match self {
			SortType::Bogo => Rate::Linear,
			SortType::Bubble => Rate::Linear,
			SortType::Insertion => Rate::Linear,
			SortType::Merge => Rate::Linear,
			SortType::Quick => Rate::Linear,
		})
	}

	pub fn analytics(&self) -> Analytics {
		let (worst, average, best) = self.time_complexity();
		Analytics::new(worst, best, average, self.space_complexity())
	}
}

impl Display for SortType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} Sort", match self {
			SortType::Bogo => "Bogo",
			SortType::Bubble => "Bubble",
			SortType::Insertion => "Insertion",
			SortType::Merge => "Merge",
			SortType::Quick => "Quick",
		})
	}
}
