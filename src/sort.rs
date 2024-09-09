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
    pub fn get_data(&self) -> &Vec<u64> {
        &self.data
    }
    
    pub fn is_sorted(&self) -> bool {
        self.is_sorted
    } 
    
    pub fn get_sort_type(&self) -> SortType {
        self.sort_type
    }
    
    pub fn get_count(&self) -> &Count {
        &self.count
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
            data: gen_data(quantity),
            tick_rate,
            count: Count::new(sort.count_type()),
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
            SortType::Insertion => self.insertion_sort(),
            SortType::Merge => self.merge_sort(),
            SortType::Quick => self.quick_sort(),
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

    /* Perform insertion sort */
    fn insertion_sort(&mut self) -> Result<(), Error> {
        for i in 1 .. self.data.len() {
            let key = self.data[i];
            let mut j = i;
    
            /* Move elements forward if greater than key */
            while j > 0 && self.data[j - 1] > key {
                self.data[j] = self.data[j - 1];
                j -= 1;
                
                self.count.increment();
                self.renderer.tick(self.snapshot(), Duration::from_millis(self.tick_rate))?;
            }

            self.data[j] = key;
        }
        
        Ok(())
    }


    /* Perform merge sort */
    fn merge_sort(&mut self) -> Result<(), Error> {
        self.merge_sort_helper(0, self.data.len())
    }

    /* Merge sort recursive indexed function */
    fn merge_sort_helper(&mut self, left: usize, right: usize) -> Result<(), Error> {
        if left < right - 1 {
            let mid: usize = left + (right - left) / 2;

            self.merge_sort_helper(left, mid)?;
            self.merge_sort_helper(mid, right)?;

            self.merge(left, mid, right)?;
        }

        Ok(())
    }

    /* Merge together data[left..mid) and data[mid..right) */
    fn merge(&mut self, left: usize, mid: usize, right: usize) -> Result<(), Error> {
        /* Temp arrays to save values */
        let left_data: Vec<u64> = self.data[left..mid].to_vec().clone();
        let right_data: Vec<u64> = self.data[mid..right].to_vec().clone();

        let mut i: usize = 0;
        let mut j: usize = 0;

        let mut k: usize = left;

        /* Merge temp arrays back into data */
        while i < left_data.len() && j < right_data.len() {
            if left_data[i] <= right_data[j] {
                self.data[k] = left_data[i];
                i += 1;
            } else {
                self.data[k] = right_data[j];
                j += 1;
            }
            
            k += 1;
            
            self.count.increment();
            self.renderer.tick(self.snapshot(), Duration::from_millis(self.tick_rate))?;
        }

        /* Copy remaining left array elements into data */
        for i in i..left_data.len() {
            self.data[k] = left_data[i];
            k += 1;

            self.renderer.tick(self.snapshot(), Duration::from_millis(self.tick_rate))?;
        }

        /* Copy remaining right array elements into data */
        for j in j..right_data.len() {
            self.data[k] = right_data[j];
            k += 1;

            self.renderer.tick(self.snapshot(), Duration::from_millis(self.tick_rate))?;
        }

        Ok(())
    }


    /* Perform quick sort */
    fn quick_sort(&mut self) -> Result<(), Error> {
        self.quick_sort_helper(0, self.data.len())
    }

    /* Quick sort recursive function */
    fn quick_sort_helper(&mut self, start: usize, end: usize) -> Result<(), Error> {
        if start < end {
            let partition_index = self.partition(start, end)?;
    
            self.quick_sort_helper(start, partition_index)?;
            self.renderer.tick(self.snapshot(), Duration::from_millis(self.tick_rate))?;
            
            self.quick_sort_helper(partition_index + 1, end)?;
            self.renderer.tick(self.snapshot(), Duration::from_millis(self.tick_rate))?;
        }

        Ok(())
    }

    /* Partition slice start to end, using data[end - 1] as pivot */
    fn partition(&mut self, start: usize, end: usize) -> Result<usize, Error> {
        let pivot = self.data[end - 1];
        let mut i = start;
        
        for j in start .. end - 1 {
            if self.data[j] <= pivot {
                self.count.increment();
                self.data.swap(i, j);
                self.renderer.tick(self.snapshot(), Duration::from_millis(self.tick_rate))?;
                
                i += 1;
            }
        }
        
        self.data.swap(i, end - 1);
        self.renderer.tick(self.snapshot(), Duration::from_millis(self.tick_rate))?;

        Ok(i)
    }

}


fn gen_data(quantity: usize) -> Vec<u64> {
    let mut data: Vec<u64> = (1..(quantity as u64 + 1)).collect();
    data.shuffle(&mut thread_rng());
    return data;
}
