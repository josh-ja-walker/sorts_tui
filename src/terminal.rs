use std::{io, time::Duration};

use tui::{backend::CrosstermBackend, layout::Alignment, style::{Color, Style}, widgets::{BarChart, Block, Borders}};
use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode}, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

use crate::{sort::Sort, Error};

const BAR_GAP: u16 = 1;
const BAR_WIDTH: u16 = 2;


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

    /* Render the bar chart */
	pub fn render(&mut self, sort: Sort, data: Vec<(&str, u64)>) -> Result<(), Error> {
        let block = Block::default()
            .title(sort.to_string())
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL);

		let bar_chart = BarChart::default()
            .bar_style(Style::default().fg(sort.tui_color()))
            .value_style(Style::default().fg(Color::White).bg(sort.tui_color()))
            .block(block)
            .bar_width(BAR_WIDTH)
            .bar_gap(BAR_GAP)
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
