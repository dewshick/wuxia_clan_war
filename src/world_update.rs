use crate::collision::move_to_target;
use crate::world_gen::World;
use crate::collision::Point;
use crate::colors::ColorTone;
use crate::collision::CircleBounds;
use crate::collision::Amount;
use crate::collision::Dist;
use std::ops::Range;
use crate::std_extended::rng_range;
use crate::world_gen::Tile;
use crate::world_gen::gen_circle_bounds;
use ordered_float::OrderedFloat;
use self::TaskUpd::*;
use std::collections::HashSet;
use crate::std_extended::index_iter;

#[derive(Debug)]
pub struct GameObj {
	pub blueprint : &'static GameObjBlueprint,
	pub bounds : CircleBounds,
	pub durability : Amount,
	pub tasks : Vec<Task>
}

impl GameObj {
	pub fn from(blueprint : &'static GameObjBlueprint, bounds : CircleBounds) -> GameObj {
		GameObj { blueprint, durability : blueprint.durability, bounds, tasks : blueprint.tasks.to_vec() }
	}
}

#[derive(Debug, Clone)]
pub struct GameObjBlueprint {
	pub genus : Genus,
	pub min_dist : f32, // required for worldgen
	pub radius : Range<f32>,
	pub color : ColorTone,
	pub durability : f32,
	pub speed : f32,
	pub tasks : &'static [Task]
}

impl GameObjBlueprint {
	pub const TREE: GameObjBlueprint = GameObjBlueprint {
		genus : Genus::Plant(Size::Big),
		min_dist : 10.0,
		radius: (8.0..18.0),
		color : ColorTone::Brown,
		durability : 100.0,
		speed : 0.0,
		tasks : &[]
	};

	pub const WANDERER: GameObjBlueprint = GameObjBlueprint {
		genus : Genus::Animal(),
		min_dist : 2.0,
		radius: (4.0..4.0),
		color : ColorTone::Black,
		durability : 20.0,
		speed : 0.33,
		tasks : &[Task::Wander()]
	};

	pub const HARE : GameObjBlueprint = GameObjBlueprint {
		genus : Genus::Animal(),
		min_dist : 2.0,
		radius: (3.0..3.0),
		color : ColorTone::White,
		durability : 15.0,
		speed : 0.9,
		tasks : &[Task::Eat(Genus::Plant(Size::Small))]
	};

	pub const GRASS: GameObjBlueprint = GameObjBlueprint {
		genus : Genus::Plant(Size::Small),
		min_dist : 0.0,
		radius: (2.0..2.0),
		color : ColorTone::DarkGreen,
		durability : 20.0,
		speed : 0.33,
		tasks : &[]
	};
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Genus { Plant(Size), Animal() }
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Size { Small, Big }

#[derive(Debug, Clone)]
pub enum Task {
	Wander(),
	GetTo(Point),
	Eat(Genus),
}

pub enum Action {
	Scream { bounds : CircleBounds },
	Hit { bounds : CircleBounds, damage : f32 },
	Swallow(usize),
	Pick(usize),
	MoveTo(Point)
}

pub enum TaskUpd {
	TaskPop,
	TaskPush(Task),
	TaskWait,
	TaskAct(Action)
}

impl GameObj {
	fn plan(&self, w : &World) -> TaskUpd {
		if self.tasks.is_empty() {
			TaskWait
		} else {
			match &self.tasks[0] {
				Task::Wander() => gen_circle_bounds(&w.map[1].bounds, &w.objects, &self.blueprint).
					map( |b| TaskPush(Task::GetTo(b.coords))).unwrap_or(TaskWait),
				Task::GetTo(target) => if target.dist(&self.bounds.coords) < 1.0 { TaskPop } else {
					let mut obstacles = w.objects.iter().map(|w| &w.bounds);
					TaskAct(Action::MoveTo(self.bounds.coords + move_to_target(&self.bounds, &target, &mut obstacles, self.blueprint.speed)))
				},
				Task::Eat(genus) => {
					if let Some((i, food)) = w.objects.iter().enumerate().filter( |(_, obj)| obj.blueprint.genus == *genus).
						min_by_key( |(_, obj)| OrderedFloat(obj.bounds.coords.dist(&self.bounds.coords))) {
						if food.bounds.coords.dist(&self.bounds.coords) < 1.0 { TaskAct(Action::Swallow(i))} else {
							TaskPush(Task::GetTo(food.bounds.coords))
						}
					} else {
						TaskPop
					}
				},
			}
		}
	}
}

impl World {
	pub fn upd(&mut self) {
		let mut removed_objects : HashSet<usize> = HashSet::new();
		let plans : Vec<(usize, TaskUpd)> = index_iter(&self.objects).map( |i| (i, self.objects[i].plan(&self))).collect();
		plans.into_iter().for_each(|(i, upd)| match upd {
				TaskUpd::TaskPop => { self.objects[i.clone()].tasks.pop(); },
				TaskUpd::TaskPush(task) => self.objects[i.clone()].tasks.push(task),
				TaskUpd::TaskWait => {},
				TaskUpd::TaskAct(action) => match action {
					Action::Scream { .. } => {},
					Action::Hit { .. } => {},
					Action::Swallow(i) => if removed_objects.contains(&i) { /*TODO*/ } else { removed_objects.insert(i.clone()); },
					Action::Pick(i) => if removed_objects.contains(&i) { /*TODO*/ } else { /*TODO add inventory*/ removed_objects.insert(i.clone()); },
					Action::MoveTo(point) => self.objects[i].bounds.coords = point,
				},
			});
//		for i in 0..self.objects.len() {
//			let wanderer = &self.objects[i];
//			if let Some(Task::GetTo(target)) = wanderer.tasks.first() {
//				let mut obstacles = self.objects.iter().map(|w| &w.bounds);
//				if target.dist(&wanderer.bounds.coords) < 1.0 {
//					gen_circle_bounds(self).map(|b| { self.objects[i].tasks = vec![Task::GetTo(b.coords)]; });
//				} else {
//					self.objects[i].bounds.coords = wanderer.bounds.coords +
//						move_to_target(&wanderer.bounds, &target, &mut obstacles, wanderer.speed);
//				}
//			}
//		}
	}
}