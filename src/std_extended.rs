use super::{thread_rng, Rng};
use super::collision::Dist;
use std::ops::Range;
use num_iter::range;

pub fn try_n_times<T>(attempts : i32, fun : &mut FnMut() -> Option<T>) -> Option<T> {
	if attempts <= 0 {
		None
	} else {
		let result = fun();
		if result.is_some() { result } else { try_n_times(attempts - 1, fun) }
	}
}

pub fn rng_range(range : &Range<Dist>) -> Dist {
	if range.start >= range.end { range.start } else { thread_rng().gen_range(range.start, range.end) }
}

pub fn index_iter<T>(v : &Vec<T>) -> impl Iterator<Item=usize> {
	range(0, v.len()).into_iter()
}