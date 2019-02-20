use super::{thread_rng, Rng};
use super::collision::Dist;
use std::ops::Range;
use num_iter::range;

pub fn rng_range(range : &Range<Dist>) -> Dist {
	if range.start >= range.end { range.start } else { thread_rng().gen_range(range.start, range.end) }
}
