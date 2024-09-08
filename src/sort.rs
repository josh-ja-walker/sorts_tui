use std::{fmt::{self, Display}, time::Duration};

use rand::{seq::SliceRandom, thread_rng};
use strum_macros::Display;

use crate::{sort_type::SortType, terminal::Terminal, Error};

const TICK: u64 = 100;


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
    Comparisons
}

impl Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}",
            self.count,
            self.count_type.to_string().to_lowercase()
        )
    }
}


fn gen_data(quantity: usize) -> Vec<u64> {
    let mut data: Vec<u64> = (1..(quantity as u64 + 1)).collect();
    data.shuffle(&mut thread_rng());
    return data;
}


pub struct Sort {
	sort: SortType,
 	data: Vec<u64>,
    terminal: Terminal,
}

impl Sort {
    pub fn new(sort: SortType, quantity: usize) -> Sort {
        Sort {
            sort,
            terminal: Terminal::new().unwrap(),
            data: gen_data(quantity),
        }
    }

    /* Render bar chart in terminal */
    fn render(&mut self) -> Result<(), Error> {
        let sort = self.sort.clone();
        self.terminal.render(sort, &self.data)
    }

    /* Run the sorting algorithm, rendering to terminal */
    pub fn run(mut self) -> Result<Count, Error> {
        self.render()?;
        
        let count = match self.sort {
            SortType::Bogo => self.bogosort(),
            SortType::Bubble => self.bubble_sort(),
            SortType::Insertion => todo!(),
            SortType::Merge => todo!(),
            SortType::Quick => todo!(),
        }?;

        Terminal::sleep(Duration::from_millis(5000))?;
        self.terminal.restore()?;

        Ok(count)
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
            if self.is_sorted() { break; }
            
            self.data.shuffle(&mut rng);
            count.increment();
            
            self.render()?;
            Terminal::sleep(Duration::from_millis(TICK))?;
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
                    Terminal::sleep(Duration::from_millis(TICK))?;
                    
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
