use std::{fmt::{self, Display}, time::Duration};

use rand::{seq::SliceRandom, thread_rng};
use strum_macros::Display;

use crate::{sort_type::SortType, terminal::Terminal, Args, Error};



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
    tick_rate: u64,
 	data: Vec<u64>,
    terminal: Terminal,
}

impl Sort {
    pub fn new(sort: SortType, quantity: usize, tick_rate: u64) -> Sort {
        Sort {
            sort,
            tick_rate,
            terminal: Terminal::new().unwrap(),
            data: gen_data(quantity),
        }
    }

    pub fn with_args(args: Args) -> Sort {
        Sort::new(args.sort_type, (args.quantity as u64).try_into().unwrap(), args.tick_rate)
    }

    /* Render bar chart in terminal */
    fn render(&mut self) -> Result<(), Error> {
        let sort = self.sort.clone();
        self.terminal.render(sort, &self.data)
    }

    /* Wait another tick */
    fn sleep(&self) -> Result<(), Error> {
        Terminal::sleep(Duration::from_millis(self.tick_rate))
    }

    /* Re-render chart and wait tick */
    fn reload(&mut self) -> Result<(), Error> {
        self.render()?;
        self.sleep()
    }


    /* Run the sorting algorithm, rendering to terminal */
    pub fn run(mut self) -> Result<Count, Error> {
        self.reload()?;
        
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
            
            self.reload()?;
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

                    self.reload()?;
                    
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
