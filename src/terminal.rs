use std::{io, time::Duration};

use tui::{backend::CrosstermBackend, style::Style, widgets::{BarChart, Block, Borders}};
use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode}, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

use crate::{sort::Sort, Error};

const BAR_GAP: u16 = 1;

pub struct Terminal {
	term: tui::Terminal<CrosstermBackend<io::Stdout>>
}


impl Terminal {
    
    /* Initialise terminal to use for rendering chart */
	pub fn new() -> Result<Terminal, Error> {
		crossterm::terminal::enable_raw_mode()?;

		let mut stdout = io::stdout();
		crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
		
		let backend = CrosstermBackend::new(stdout);
		
		Ok(Terminal {
			term: tui::Terminal::new(backend)?
		})
	}

    /* Destroy chart terminal and return to normal terminal */
	pub fn destroy(mut self) -> Result<(), Error> {
		crossterm::terminal::disable_raw_mode()?;
		crossterm::execute!(
			self.term.backend_mut(),
			LeaveAlternateScreen,
			DisableMouseCapture
		)?;
	
		self.term.show_cursor()?;
	
		Ok(())
	}

	fn bar_settings(num_items: usize) -> Result<(u16, u16), Error> {
		let (term_width, _) = crossterm::terminal::size()?;
		
		let mut bar_gap: u16 = BAR_GAP;
		let mut bar_width = (term_width - (bar_gap * (num_items - 1) as u16)) as f32 / num_items as f32;
		
		if bar_width < 1.0 {
			bar_gap = 0;
			bar_width = (term_width - (bar_gap * (num_items - 1) as u16)) as f32 / num_items as f32;
		}

		Ok((bar_width as u16, bar_gap))
	}

    /* Render the bar chart */
	pub fn render(&mut self, sort: Sort, data: Vec<(&str, u64)>) -> Result<(), Error> {
        let block = Block::default()
            .title(sort.uncolored_string())
            .borders(Borders::ALL);

		let (w, g) = Self::bar_settings(data.len())?;

		let bar_chart = BarChart::default()
			.bar_style(Style::default().fg(sort.tui_color()))
			.value_style(Style::default().bg(sort.tui_color()))
            .block(block)
            .bar_width(w)
            .bar_gap(g)
            .data(&data);
			
		self.term.draw(|f| {
			let size = f.size();
			f.render_widget(bar_chart, size);
		})?;

		Ok(())
	}

	/* Returns other io error if interrupted */
	pub fn sleep(duration: Duration) -> Result<(), Error> {
		if event::poll(duration)? {
			if let Event::Key(key) = event::read()? {
				if let KeyCode::Char('q') | KeyCode::Esc = key.code {
					return Err(Error::Interrupted);
				}
			}
		}
		
		Ok(())
	}

}
