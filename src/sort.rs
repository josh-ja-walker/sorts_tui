use std::time::Duration;
use rand::{seq::SliceRandom, thread_rng};

use crate::{count::Count, sort_type::SortType, Args, Error, Renderer};

pub struct SortSnapshot {
    data: Vec<u64>,    
    is_sorted: bool,
    sort_type: SortType,
    count: Count,
}

impl SortSnapshot {
    pub fn get_data(&self) -> Vec<u64> {
        self.data.clone()
    }

    pub fn is_sorted(&self) -> bool {
        self.is_sorted
    } 

    pub fn get_count(&self) -> Count {
        self.count.clone()
    } 
    
    pub fn get_sort_type(&self) -> SortType {
        self.sort_type
    }
}


pub struct Sort<'a, R: Renderer> {
    renderer: &'a mut R,
	sort: SortType,
    count: Count,
    data: Vec<u64>,
    tick_rate: u64,
}


impl<'a, R: Renderer> Sort<'a, R> {
    pub fn new(renderer: &'a mut R, sort: SortType, quantity: usize, tick_rate: u64) -> Sort<R> {
        Sort {
            renderer,
            sort,
            count: Count::new(sort.count_type()),
            data: gen_data(quantity),
            tick_rate,
        }
    }

    pub fn from_args(renderer: &'a mut R, args: Args) -> Sort<R> {
        Sort::new(
            renderer,
            args.sort_type, 
            (args.quantity as u64).try_into().unwrap(), 
            args.tick_rate
        )
    }
    
    /* Generate snapshot to render */
    pub fn snapshot(&self) -> SortSnapshot {
        SortSnapshot {
            data: self.data.clone(),
            is_sorted: self.is_sorted(),
            sort_type: self.sort,
            count: self.count.clone(),
        }
    }

    /* Check if data is sorted */
	pub fn is_sorted(&self) -> bool {
        self.data.windows(2).all(|w| w[0] <= w[1])
	}

    /* Run the sorting algorithm, rendering to terminal */
    pub fn run(mut self) -> Result<Count, Error> {
        self.renderer.tick(self.snapshot(), Duration::from_millis(self.tick_rate))?;
        
        match self.sort {
            SortType::Bogo => self.bogosort(),
            SortType::Bubble => self.bubble_sort(),
            SortType::Insertion => todo!(),
            SortType::Merge => todo!(),
            SortType::Quick => todo!(),
        }?;

        self.renderer.tick(self.snapshot(), Duration::from_millis(5000))?;

        Ok(self.count)
	}

    /* Perform bogosort */
    fn bogosort(&mut self) -> Result<(), Error> {
        let mut rng = rand::thread_rng();

        loop {
            if self.is_sorted() { break; }
            
            self.data.shuffle(&mut rng);
            self.count.increment();
            
            self.renderer.tick(self.snapshot(), Duration::from_millis(self.tick_rate))?;
        }

        Ok(())
    }

    /* Perform bubble sort */
    fn bubble_sort(&mut self) -> Result<(), Error> {
        let mut swapped: bool;

        for i in 0 .. self.data.len() - 1 {
            swapped = false;

            for j in 0 .. self.data.len() - i - 1 {
                if self.data[j] > self.data[j + 1] {
                    self.data.swap(j, j + 1);
                    self.count.increment();

                    self.renderer.tick(self.snapshot(), Duration::from_millis(self.tick_rate))?;

                    swapped = true;
                }
            }
    
            if !swapped {
                break;
            }
        }

        Ok(())
    }

}


fn gen_data(quantity: usize) -> Vec<u64> {
    let mut data: Vec<u64> = (1..(quantity as u64 + 1)).collect();
    data.shuffle(&mut thread_rng());
    return data;
}
