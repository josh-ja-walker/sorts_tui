use std::time::Duration;

use ratatui::{
	DefaultTerminal,
	layout::Size, 
	style::Style, 
	crossterm::event::{self, Event, KeyCode, KeyEventKind}, 
	widgets::{Bar, BarChart, BarGroup, Block, Borders}, 
};

use crate::{sort::Sort, Error};

const BAR_GAP: u16 = 1;


/* Build group of bars from the data */
fn build_bars(bar_width: u16, data: &Vec<u64>) -> BarGroup {
	let val_str = |x: &u64| {
		let max = data.len() + 1;
		let txt_width = max.ilog10() + 1;

		if txt_width > bar_width as u32 {
			String::from("")
		} else {
			format!("{x:^width$}", width = bar_width as usize)
		}
	};
	
	BarGroup::default().bars(
		&data.iter()
			.map(|x| Bar::default().value(*x + 2).text_value(val_str(x)))
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
	fn calc_bar_sizes(&self, num_items: usize) -> Result<(u16, u16), Error> {
		let Size { width: term_width, .. } = self.term.size()?;
		
		let mut bar_gap: u16 = BAR_GAP;
		
		loop {
			let bar_width = (term_width - (bar_gap * (num_items - 1) as u16)) / num_items as u16;
			if bar_width < 1 {
				bar_gap = bar_gap.checked_sub(1).expect("Bar width could not be calculated");
			} else {
				return Ok((bar_width, bar_gap));
			}
		}
	}


    /* Render the bar chart */
	pub fn render(&mut self, sort: Sort, data: &Vec<u64>) -> Result<(), Error> {
		let (bar_width, bar_gap) = self.calc_bar_sizes(data.len())?;

        let block = Block::default()
            .title(sort.to_string())
            .borders(Borders::ALL);

		let bar_chart = BarChart::default()
			.bar_style(Style::default().fg(sort.color()))
			.value_style(Style::default())
            .block(block)
            .bar_width(bar_width)
            .bar_gap(bar_gap)
            .data(build_bars(bar_width, data));
			
		self.term.draw(|frame| {
			frame.render_widget(bar_chart, frame.area());
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
