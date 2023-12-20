use std::io::{stdout, Write};
use std::{thread, time, str::FromStr};
use rand::{thread_rng, seq::SliceRandom};

const MARK_NUM: u32 = 10;
const SLEEP_MILLIS: u64 = 10;

macro_rules! sleep {
	($millis:tt) => { thread::sleep(time::Duration::from_millis($millis)); }
}

pub enum Sort {
	Bogo,
	Bubble,
	Insertion,
	Merge,
	Quick
}

impl Sort {
	pub fn perform(&self, n: u32) {
	let mut nums: Vec<u32> = gen_nums(n);
		match self {
			Sort::Bogo => loop {
				draw_graph(&nums);
				if check_sorted(&nums) {
					return;
				}

				sleep!(600);
				nums.shuffle(&mut thread_rng());

				print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear console
			},
			
			Sort::Bubble => bubble_sort(&mut nums),
			Sort::Insertion => todo!(),
			Sort::Merge => todo!(),
			Sort::Quick => todo!(),
		};
	}   
}

fn gen_nums(n: u32) -> Vec<u32> {
	let mut nums: Vec<u32> = (1..(n + 1)).collect();
	nums.shuffle(&mut thread_rng());
	return nums;
}

fn check_sorted(nums: &Vec<u32>) -> bool {
	for i in 0..nums.len() - 1{
		if nums[i] > nums[i + 1] {
			return false;
		}
	}
	
	return true;
}

fn bubble_sort(nums: &mut Vec<u32>) {
	loop {
		for i in 0 .. nums.len() - 1 {
			if nums[i] > nums[i + 1] {
				swap(i, i + 1, nums);
				reload(&nums);
				if check_sorted(nums) {
					return;
				}
			}
		}
	}
}

fn swap(i: usize, j: usize, nums: &mut Vec<u32>) {
	let tmp = nums[i];
	nums[i] = nums[j];
	nums[j] = tmp;
}

fn draw_graph(nums: &Vec<u32>) {
	let n: u32 = nums.len().try_into().unwrap();
	print!("\u{250C}");
	for _ in 1..=(n + 8) {
		print!("\u{2500}");
	}
	print!("\u{2510}\n");
	
	for i in (1..=(n / 2) + 1).rev() {
		print!("\u{2502}");
		let max = i * 2;
		if max % 10 == 0 {
			print!(" {: >3} \u{2500} ", max);
		} else {
			print!("     \u{2500} ");
		}
		
		for x in nums {
			let mut out = " ";
			if *x == max - 1 { 
				out = "\u{2584}";
			} else if *x >= max {
				out = "\u{2588}";
			}
			
			print!("{out}");
		}
		
		print!(" \u{2502}\n");
	}
	
	print!("\u{2514}");
	for _ in 1..=(n + 8) {
		print!("\u{2500}");
	}
	print!("\u{2518}");
	let _ = stdout().flush();
}

fn reload(nums: &Vec<u32>) {
	sleep!(SLEEP_MILLIS);
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear console
	draw_graph(nums);
}