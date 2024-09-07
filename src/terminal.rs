use std::time::Duration;

use ratatui::{
	DefaultTerminal,
	layout::Size, 
	style::Style, 
	widgets::{Bar, BarChart, BarGroup, Block, Borders}, 
	crossterm::event::{self, Event, KeyCode, KeyEventKind}, 
};

use crate::{sort::Sort, Error};

const BAR_GAP: u16 = 1;

pub struct Terminal {
	term: DefaultTerminal
}


impl Terminal {
    
    /* Initialise terminal to use for rendering chart */
	pub fn new() -> Result<Terminal, Error> {
		let terminal = ratatui::init();

		Ok(Terminal {
			term: terminal
		})
	}

    /* Destroy chart terminal and return to normal terminal */
	pub fn restore(self) -> Result<(), Error> {
		ratatui::restore();
		Ok(())
	}

	fn bar_settings(&self, num_items: usize) -> Result<(u16, u16), Error> {
		let Size { width, .. } = self.term.size()?;
		
		let mut bar_gap: u16 = BAR_GAP;
		let mut bar_width = (width - (bar_gap * (num_items - 1) as u16)) as f32 / num_items as f32;
		
		if bar_width < 1.0 {
			bar_gap = 0;
			bar_width = (width - (bar_gap * (num_items - 1) as u16)) as f32 / num_items as f32;
		}

		Ok((bar_width as u16, bar_gap))
	}

    /* Render the bar chart */
	pub fn render(&mut self, sort: Sort, data: &Vec<u64>) -> Result<(), Error> {
		let bar = |x: &u64| Bar::default()
			.value(*x + 2).text_value(format!("{x:02}"));

		let bar_group = BarGroup::default()
			.bars(&data.iter().map(bar).collect::<Vec<Bar>>());

        let block = Block::default()
            .title(sort.to_string())
            .borders(Borders::ALL);

		let (w, g) = self.bar_settings(data.len())?;

		let bar_chart = BarChart::default()
			.bar_style(Style::default().fg(sort.color()))
			.value_style(Style::default())
            .block(block)
            .bar_width(w)
            .bar_gap(g)
            .data(bar_group);
			
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
