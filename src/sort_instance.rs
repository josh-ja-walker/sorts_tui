use std::{io::{self, ErrorKind, Stdout}, thread, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use rand::{seq::SliceRandom, thread_rng};
use tui::{backend::CrosstermBackend, style::{Color, Style}, widgets::{BarChart, Block, Borders}, Terminal};

use crate::sort::Sort;

const TICK: u64 = 10;
const BAR_WIDTH: u16 = 4;

pub struct SortInstance {
	sort: Sort,
	data: Vec<u64>
}

fn gen_data(num_items: usize) -> Vec<u64> {
    let mut data: Vec<u64> = (1..(num_items as u64 + 1)).collect();
    data.shuffle(&mut thread_rng());
    return data;
}

impl SortInstance {
    pub fn new(sort: Sort, num_items: usize) -> SortInstance {
        SortInstance {
            sort,
            data: gen_data(num_items),
        }
    }
    	
    /* Run the sorting algorithm, rendering to terminal */
    pub fn run(mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<u64> {
        let mut iterations: u64 = 0;

        loop {
            iterations += 1;

            self.sort_iteration();

            self.render(terminal);

            if self.is_sorted() {
                thread::sleep(Duration::from_millis(5000));
                return Ok(iterations);
            }

            match Self::terminal_sleep(Duration::from_millis(TICK)) {
                Ok(()) => (),
                Err(e) if e.kind() == ErrorKind::Other => { return Ok(iterations); },
                error => error?
            }
        }
	}

    /* Returns other io error if interrupted */
    fn terminal_sleep(duration: Duration) -> io::Result<()> {
        if event::poll(duration).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if let KeyCode::Char('q') = key.code {
                    return Err(io::Error::other("Interrupted"));
                }
            }
        }
        
        Ok(())
    }

    fn render(&self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        let formatted_data: Vec<(&str, u64)> = self.data
            .iter()
            .map(|&x| ("", x))
            .collect();
        
        let bar_chart = BarChart::default()
            .block(Block::default()
                .title(self.sort.to_string().clone())
                .borders(Borders::ALL)
            )
            .data(&formatted_data)
            .bar_width(BAR_WIDTH)
            .bar_style(Style::default().fg(self.sort.color()))
            .value_style(Style::default().fg(Color::White).bg(self.sort.color()));
            
        terminal.draw(|f| {
            let size = f.size();
            f.render_widget(bar_chart, size);
        }).unwrap();
    }

    fn sort_iteration(&mut self) {
        match self.sort {
            Sort::Bogo => {
                let mut rng = thread_rng();
                self.data.shuffle(&mut rng);
            },
            Sort::Bubble => todo!(),
            Sort::Insertion => todo!(),
            Sort::Merge => todo!(),
            Sort::Quick => todo!(),
        };
    }
	
	fn is_sorted(&self) -> bool {
		for i in 0 .. (self.data.len() - 1) {
			if self.data[i] > self.data[i + 1] {
				return false;
			}
		}
		
		return true;
	}

}

