use std::time::Duration;

use ratatui::{
	crossterm::event::{self, Event, KeyCode, KeyEventKind}, 
	layout::{Constraint, Layout}, 
	style::Style, 
	text::Line, 
	DefaultTerminal,
	widgets::{Bar, BarChart, BarGroup, Block, Borders, Padding}, 
};

use crate::{sort_type::SortType, Error};

const MAX_BAR_GAP: u16 = 1;
const BAR_WIDTH_MIN: u16 = 1;
const BAR_WIDTH_MAX: u16 = 3;

const HORIZ_PAD: u16 = 4;
const CHART_PAD: u16 = 2;


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

/* Calculate width and gap of bars */
fn calc_bar_sizes(term_width: u16, quantity: usize) -> (u16, u16) {
	let usable_term_width = term_width - HORIZ_PAD - CHART_PAD;
	let mut bar_gap: u16 = MAX_BAR_GAP;
	
	loop {
		/* Width of all gaps */
		let total_gap_width: u16 = bar_gap * (quantity - 1) as u16;

		/* If terminal width - total gaps > 0 */
		if let Some(all_bars_width) = usable_term_width.checked_sub(total_gap_width) {
			/* Calculate individual bar width */
			let bar_width: f32 = all_bars_width as f32 / (quantity as f32);
			
			/* If bar width is valid, clamp and return */
			if bar_width >= 1.0 {
				let clamped_bar_width = (bar_width.floor() as u16).clamp(BAR_WIDTH_MIN, BAR_WIDTH_MAX);
				return (clamped_bar_width, bar_gap);
			}
		} 
		
		/* If gap is invalid, minus 1 and reattempt */
		bar_gap = bar_gap.checked_sub(1).expect("Bar width could not be calculated");
	}
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
	
    /* Render the bar chart */
	pub fn render(&mut self, sort: SortType, data: &Vec<u64>) -> Result<(), Error> {
		self.term.draw(|frame| {
			/* Calculate bar width and gaps */
			let (bar_width, bar_gap) = calc_bar_sizes(frame.area().width, data.len());
			
			/* Chart Width = n * (width + gap) - extra gap + padding */
			let chart_width = (data.len() as u16 * (bar_width + bar_gap)) - bar_gap + CHART_PAD;
			
			/* Set up layout of chart - set width and center */
			let [_, area, _] = Layout::horizontal([
					Constraint::Fill(1),
					Constraint::Length(chart_width + HORIZ_PAD), 
					Constraint::Fill(1),
				])
				.vertical_margin(5) 
				.areas(frame.area());
			
			/* Set up containing block */
			let block = Block::default()
				.title(Line::styled(sort.to_string(), sort.color()))
				.padding(Padding::new(HORIZ_PAD / 2, HORIZ_PAD / 2, 2, 0))
				.borders(Borders::ALL);

			/* Set up bar chart */
			let bar_chart = BarChart::default()
				.block(block)
				.bar_style(Style::default().fg(sort.color()))
				.bar_width(bar_width)
				.bar_gap(bar_gap)
				.data(build_bars(bar_width, data));

			/* Render bar chart with set area */
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
