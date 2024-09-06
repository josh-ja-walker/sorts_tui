use std::{fmt::{self, Display}, io::{self, Stdout}, time::Duration};

use rand::{seq::SliceRandom, thread_rng};
use strum_macros::Display;
use tui::{
    Terminal,
    backend::CrosstermBackend, 
    widgets::{BarChart, Block, Borders}, 
    style::{Color, Style},
}; 

use crate::{Error, sort::Sort, terminal_sleep};

const TICK: u64 = 100;
const BAR_GAP: u16 = 1;
const BAR_WIDTH: u16 = 4;

pub struct Count {
    count: usize,
    #[allow(dead_code)] count_type: CountType,
}

impl Count {
    fn new(count_type: CountType) -> Count {
        Count {
            count: 0,
            count_type
        }
    } 

    fn increment(&mut self) {
        self.count += 1
    }
}

#[derive(Display)]
enum CountType {
    Shuffles,
    Iterations,
    Comparisons,
}

impl Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}",
            self.count,
            self.count.to_string().to_lowercase()
        )
    }
}


fn gen_data(num_items: usize) -> Vec<u64> {
    let mut data: Vec<u64> = (1..(num_items as u64 + 1)).collect();
    data.shuffle(&mut thread_rng());
    return data;
}


pub struct SortInstance<'a> {
	sort: Sort,
 	data: Vec<u64>,
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> SortInstance<'a> {
    pub fn new(sort: Sort, num_items: usize, terminal: &'a mut Terminal<CrosstermBackend<Stdout>>) -> SortInstance<'a> {
        SortInstance {
            sort,
            terminal,
            data: gen_data(num_items),
        }
    }
    	
    /* Run the sorting algorithm, rendering to terminal */
    pub fn run(mut self) -> Result<Count, Error> {
        self.render()?;

        match self.sort {
            Sort::Bogo => self.bogosort(),
            Sort::Bubble => self.bubble_sort(),
            Sort::Insertion => todo!(),
            Sort::Merge => todo!(),
            Sort::Quick => todo!(),
        }
	}

    /* Render bar chart to terminal */
    fn render(&mut self) -> io::Result<()> {
        let formatted_data: Vec<(&str, u64)> = self.data.iter()
            .map(|&x| ("", x)).collect();
        
        let bar_chart = BarChart::default()
            .block(Block::default()
                .title(self.sort.to_string())
                .borders(Borders::ALL))
            .data(&formatted_data)
            .bar_width(BAR_WIDTH)
            .bar_gap(BAR_GAP)
            .bar_style(Style::default().fg(self.sort.color()))
            .value_style(Style::default().fg(Color::White).bg(self.sort.color()));
            
        self.terminal.draw(|f| {
            let size = f.size();
            f.render_widget(bar_chart, size);
        })?;

        Ok(())
    }

    /* Check if data is sorted */
	fn is_sorted(&self) -> bool {
        self.data.windows(2).all(|w| w[0] <= w[1])
	}
    
    /* Perform bogosort */
    fn bogosort(&mut self) -> Result<Count, Error> {
        let mut rng = rand::thread_rng();
        let mut count = Count::new(CountType::Shuffles);

        loop {
            self.render()?;
            terminal_sleep(Duration::from_millis(TICK))?;
            
            if self.is_sorted() { break; }
            
            self.data.shuffle(&mut rng);
            count.increment();
        }

        Ok(count)
    }

    /* Perform bubble sort */
    fn bubble_sort(&mut self) -> Result<Count, Error> {
        let mut swapped: bool;
        let mut count = Count::new(CountType::Comparisons);

        for i in 0 .. self.data.len() - 1 {
            swapped = false;

            for j in 0 .. self.data.len() - i - 1 {
                if self.data[j] > self.data[j + 1] {
                    self.data.swap(j, j + 1);
                    count.increment();

                    self.render()?;
                    terminal_sleep(Duration::from_millis(TICK))?;
                    
                    swapped = true;
                }
            }
    
            if !swapped {
                break;
            }
        }

        Ok(count)
    }

}
