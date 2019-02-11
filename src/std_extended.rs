use super::{thread_rng, Rng};
use super::collision::Dist;
use std::ops::Range;
use num_iter::range;

pub fn rng_range(range : &Range<Dist>) -> Dist {
	if range.start >= range.end { range.start } else { thread_rng().gen_range(range.start, range.end) }
}

pub fn index_iter<T>(v : &Vec<T>) -> impl Iterator<Item=usize> {
	range(0, v.len()).into_iter()
}

//pub fn with_index_iter<T>(v : &Vec<T>) -> impl Iterator<Item=(T, usize)> {
//	range(0, v.len()).into_iter().map( |i| (v[i], i))
//}