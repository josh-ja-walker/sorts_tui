use std::{fmt::{self, Display}, time::Duration};

use rand::{seq::SliceRandom, thread_rng};
use strum_macros::Display;

use crate::{sort::Sort, terminal::Terminal, Error};

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
    Comparisons,
    Iterations,
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


pub struct SortInstance {
	sort: Sort,
 	data: Vec<u64>,
    terminal: Terminal,
}

impl SortInstance {
    pub fn new(sort: Sort, num_items: usize) -> SortInstance {
        SortInstance {
            sort,
            terminal: Terminal::new().unwrap(),
            data: gen_data(num_items),
        }
    }

    /* Render bar chart in terminal */
    fn render(&mut self) -> Result<(), Error> {
        let sort = self.sort.clone();
        let data = self.data.iter()
            .map(|x| ("", *x))
            .collect();
        
        self.terminal.render(sort, data)
    }

    /* Run the sorting algorithm, rendering to terminal */
    pub fn run(mut self) -> Result<Count, Error> {
        self.render()?;
        
        let count = match self.sort {
            Sort::Bogo => self.bogosort(),
            Sort::Bubble => self.bubble_sort(),
            Sort::Insertion => todo!(),
            Sort::Merge => todo!(),
            Sort::Quick => todo!(),
        }?;

        self.terminal.destroy()?;

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
            self.render()?;
            Terminal::sleep(Duration::from_millis(TICK))?;
            
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
