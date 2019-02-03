use crate::world_gen::Task;
use crate::world_gen::gen_wanderer_bounds;
use crate::collision::move_to_target;
use crate::world_gen::World;

impl World {
	pub fn upd(&mut self) {
		for i in 0..self.objects.len() {
			let wanderer = &self.objects[i];
			if let Some(Task::MovingTo(target)) = wanderer.tasks.first() {
				let mut obstacles = self.objects.iter().map(|w| &w.bounds);
				if target.dist(&wanderer.bounds.coords) < 1.0 {
					gen_wanderer_bounds(self).map(|b| { self.objects[i].tasks = vec![Task::MovingTo(b.coords)]; });
				} else {
					self.objects[i].bounds.coords = wanderer.bounds.coords +
						move_to_target(&wanderer.bounds, target, &mut obstacles, wanderer.speed);
				}
			}
		}
	}
}
