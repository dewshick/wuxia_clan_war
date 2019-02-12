use crate::collision::move_to_target;
use crate::world_gen::World;
use crate::collision::Point;
use crate::colors::ColorTone;
use crate::collision::CircleBounds;
use crate::collision::Amount;
use std::ops::Range;
use crate::world_gen::gen_circle_bounds;
use ordered_float::OrderedFloat;
use self::TaskUpd::*;
use std::collections::HashSet;
use crate::std_extended::index_iter;
use itertools::Itertools;
//use crate::std_extended::with_index_iter;

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
	pub name : &'static str,
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
		name : "Tree",
		genus : Genus::Plant(Size::Big),
		min_dist : 10.0,
		radius: (8.0..18.0),
		color : ColorTone::Brown,
		durability : 100.0,
		speed : 0.0,
		tasks : &[]
	};

	pub const WANDERER: GameObjBlueprint = GameObjBlueprint {
		name : "Wanderer",
		genus : Genus::Animal(Size::Average, FoodPreference::Herbivore),
		min_dist : 2.0,
		radius: (4.0..4.0),
		color : ColorTone::Black,
		durability : 20.0,
		speed : 0.33,
		tasks : &[Task::Wander()]
	};

	pub const HARE : GameObjBlueprint = GameObjBlueprint {
		name : "Hare",
		genus : Genus::Animal(Size::Small, FoodPreference::Herbivore),
		min_dist : 2.0,
		radius: (3.0..3.0),
		color : ColorTone::White,
		durability : 15.0,
		speed : 0.9,
		tasks : &[Task::Eat(Genus::Plant(Size::Small))]
	};

	pub const GRASS: GameObjBlueprint = GameObjBlueprint {
		name : "Grass",
		genus : Genus::Plant(Size::Small),
		min_dist : 0.0,
		radius: (2.0..2.0),
		color : ColorTone::DarkGreen,
		durability : 20.0,
		speed : 0.0,
		tasks : &[]
	};

	pub const WOLF: GameObjBlueprint = GameObjBlueprint {
		name : "Wolf",
		genus : Genus::Animal(Size::Average, FoodPreference::Carnivore),
		min_dist : 10.0,
		radius: (6.0..6.0),
		color : ColorTone::DimGrey,
		durability : 100.0,
		speed : 0.6,
		tasks : &[Task::Eat(Genus::Animal(Size::Small, FoodPreference::Herbivore))]
	};
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Genus { Plant(Size), Animal(Size, FoodPreference) }
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Size { Small, Average, Big }
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FoodPreference { Herbivore, Carnivore }

#[derive(Debug, Clone)]
pub enum Task {
	Wander(),
	GetTo(CircleBounds),
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
			match &self.tasks.last().unwrap() {
				Task::Wander() => gen_circle_bounds(&w.map[1].bounds, &w.objects, &self.blueprint).
					map( |b| TaskPush(Task::GetTo(b))).unwrap_or(TaskWait),
				Task::GetTo(target) => if target.collides_with(&self.bounds) { TaskPop } else {
					let mut obstacles = w.objects.iter().filter(|o| o.blueprint.genus != Genus::Plant(Size::Small)).map(|o| &o.bounds);
					TaskAct(Action::MoveTo(self.bounds.coords + move_to_target(&self.bounds, &target.coords, &mut obstacles, self.blueprint.speed)))
				},
				Task::Eat(genus) => {
					if let Some((i, food)) = w.objects.iter().enumerate().filter( |(_, obj)| obj.blueprint.genus == *genus).
						min_by_key( |(_, obj)| OrderedFloat(obj.bounds.coords.dist(&self.bounds.coords))) {
						if food.bounds.collides_with(&self.bounds) {
							TaskAct(Action::Swallow(i))
						} else {
							TaskPush(Task::GetTo(food.bounds.clone()))
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
		removed_objects.iter().sorted_by_key(|i| -(**i as i32)).for_each( |i| { self.objects.remove(*i); });
	}
}