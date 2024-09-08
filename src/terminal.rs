use std::time::Duration;

use ratatui::{
	crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout}, style::Style, text::Line, widgets::{Bar, BarChart, BarGroup, Block, Borders, Padding}, DefaultTerminal 
};

use crate::{sort_type::SortType, Error};

const BAR_GAP: u16 = 1;


/* Build group of bars from the data */
fn build_bars(bar_width: u16, data: &Vec<u64>) -> BarGroup {
	let max = data.len() + 1;
	let txt_width = max.ilog10() + 1;

	let format_val = |x: &u64| {
		if txt_width > bar_width as u32 {
			String::from("")
		} else {
			format!("{x:^width$}", width = bar_width as usize)
		}
	};
	
	BarGroup::default().bars(
		&data.iter()
			.map(|x| Bar::default()
				.value(*x)
				.text_value(String::from(""))
				.label(Line::from(format_val(x))))
			.collect::<Vec<Bar>>())
}


pub struct Terminal {
	term: DefaultTerminal
}

impl Terminal {
    
    /* Initialise terminal to use for rendering chart */
	pub fn new() -> Result<Terminal, Error> {
		Ok(Terminal {
			term: ratatui::init()
		})
	}

    /* Destroy chart terminal and return to normal terminal */
	pub fn restore(self) -> Result<(), Error> {
		ratatui::restore();
		Ok(())
	}

	/* Calculate width and gap of bars */
	fn calc_bar_sizes(term_width: u16, num_items: usize) -> Result<(u16, u16), Error> {
		let mut bar_gap: u16 = BAR_GAP;
		
		loop {
			let bar_width = (term_width - (bar_gap * (num_items) as u16)) / num_items as u16;
			
			if bar_width < 1 {
				bar_gap = bar_gap.checked_sub(1).expect("Bar width could not be calculated");
			} else {
				return Ok((bar_width, bar_gap));
			}
		}
	}

    /* Render the bar chart */
	pub fn render(&mut self, sort: SortType, data: &Vec<u64>) -> Result<(), Error> {
		self.term.draw(|frame| {
			let horiz_pad: u16 = 4;

			let (bar_width, bar_gap) = Self::calc_bar_sizes(
				frame.area().width - horiz_pad, 
				data.len()
			).unwrap();
			
			/* Chart Width = n * (width + gap) + trailing gap */
			let chart_width = ((bar_width + bar_gap) * data.len() as u16) + bar_gap;
			
			let [_, area, _] = Layout::horizontal([
					Constraint::Min(0), 
					Constraint::Max(chart_width + horiz_pad), 
					Constraint::Min(0)
				]).vertical_margin(5)
				.areas(frame.area());
			
			let block = Block::default()
				.title(sort.to_string())
				.padding(Padding::new(horiz_pad / 2, horiz_pad / 2, 2, 0))
				.borders(Borders::ALL);

			let bar_chart = BarChart::default()
				.block(block)
				.bar_style(Style::default().fg(sort.color()))
				.value_style(Style::default())
				.bar_width(bar_width)
				.bar_gap(bar_gap)
				.data(build_bars(bar_width, data));

			frame.render_widget(bar_chart, area);
		})?;

		Ok(())
	}

	/* Returns other io error if interrupted */
	pub fn sleep(duration: Duration) -> Result<(), Error> {
		if event::poll(duration)? {
			if let Event::Key(key) = event::read()? {
				if key.kind == KeyEventKind::Press {
					if let KeyCode::Char('q') | KeyCode::Esc = key.code {
						return Err(Error::Interrupted);
					}
				}
			}
		}

		Ok(())
	}

}
