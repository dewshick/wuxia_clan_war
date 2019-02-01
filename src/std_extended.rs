use super::{thread_rng, Rng};
use super::collision::Dist;

pub fn try_n_times<T>(attempts : i32, fun : &Fn() -> Option<T>) -> Option<T> {
	if attempts <= 0 {
		None
	} else {
		let result = fun();
		if result.is_some() { result } else { try_n_times(attempts - 1, fun) }
	}
}

pub fn rng_range(lb : Dist, ub : Dist) -> Dist {
	thread_rng().gen_range(lb, ub)
}