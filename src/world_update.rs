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
use itertools::Itertools;
use crate::collision::RectBounds;
use crate::world_gen::Bounds;
use crate::std_extended::rng_range;
use crate::collision::Dist;
//use crate::std_extended::with_index_iter;

pub type FrameCount = u64;
const FRAME_MOMENT : FrameCount = 60;
const FRAME_HOUR : FrameCount = 60 * FRAME_MOMENT;
const FRAME_DAY : FrameCount = FRAME_MOMENT * 24;
const FRAME_YEAR : FrameCount = FRAME_DAY * 365;

#[derive(Debug)]
pub struct GameObj {
	pub blueprint : &'static GameObjBlueprint,
	pub bounds : CircleBounds,
	pub durability : Amount,
	pub tasks : Vec<Task>,
	pub tmp_effects : Vec<TemporalEffect>,
}

#[derive(Debug)]
pub struct TemporalEffect { start : FrameCount, duration : FrameCount, effect : EffectType } // TODO: curse, buffs/debuffs

impl TemporalEffect {
	fn new(start : FrameCount, duration : FrameCount, effect : EffectType) -> TemporalEffect {
		TemporalEffect { start, duration, effect }
	}
}

#[derive(Debug, Eq, PartialEq)]
enum EffectType { PeriodicEffect { period : FrameCount, effect : PeriodicEffectType }, Slow(OrderedFloat<Amount>), Stun, DelayedDeath }

#[derive(Debug, Eq, PartialEq)]
enum PeriodicEffectType { Hunger(OrderedFloat<Amount>), Damage(OrderedFloat<Amount>) }

impl GameObj {
	pub fn from(blueprint : &'static GameObjBlueprint, bounds : CircleBounds, time : FrameCount) -> GameObj {
		let tmp_effects = blueprint.max_lifetime.iter().map( |lifetime| TemporalEffect {
			start: time + (*lifetime as f32 * rng_range(&(0.7..1.0))) as u64,
			duration: 1,
			effect: EffectType::DelayedDeath
		}).collect();
		GameObj { blueprint, durability : blueprint.durability, bounds, tasks : blueprint.tasks.to_vec(), tmp_effects }
	}
}

#[derive(Debug, Clone)]
pub struct GameObjBlueprint {
	pub name : &'static str,
	pub genus : Genus,
	pub min_dist : Dist, // required for worldgen
	pub radius : Range<Dist>,
	pub color : ColorTone,
	pub durability : Amount,
	pub speed : Dist,
	pub tasks : &'static [Task],
	pub max_lifetime: Option<FrameCount>
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
		tasks : &[],
		max_lifetime: None
	};

	pub const WANDERER: GameObjBlueprint = GameObjBlueprint {
		name : "Wanderer",
		genus : Genus::Animal(Size::Average, FoodPreference::Herbivore),
		min_dist : 2.0,
		radius: (4.0..4.0),
		color : ColorTone::Black,
		durability : 20.0,
		speed : 0.33,
		tasks : &[Task::Wander()],
		max_lifetime: Some(2 * FRAME_YEAR)
	};

	pub const PLAYER: GameObjBlueprint = GameObjBlueprint {
		name : "Player",
		genus : Genus::Animal(Size::Average, FoodPreference::Herbivore),
		min_dist : 2.0,
		radius: (4.0..4.0),
		color : ColorTone::Black,
		durability : 20.0,
		speed : 0.33,
		tasks : &[Task::ListenControls()],
		max_lifetime: Some(2 * FRAME_YEAR)
	};

	pub const HARE : GameObjBlueprint = GameObjBlueprint {
		name : "Hare",
		genus : Genus::Animal(Size::Small, FoodPreference::Herbivore),
		min_dist : 2.0,
		radius: (3.0..3.0),
		color : ColorTone::White,
		durability : 15.0,
		speed : 0.9,
		tasks : &[Task::Eat(Genus::Plant(Size::Small))],
		max_lifetime: Some(FRAME_YEAR)
	};

	pub const GRASS: GameObjBlueprint = GameObjBlueprint {
		name : "Grass",
		genus : Genus::Plant(Size::Small),
		min_dist : 0.0,
		radius: (2.0..2.0),
		color : ColorTone::DarkGreen,
		durability : 20.0,
		speed : 0.0,
		tasks : &[Task::Reproduce],
		max_lifetime: Some(FRAME_DAY)
	};

	pub const WOLF: GameObjBlueprint = GameObjBlueprint {
		name : "Wolf",
		genus : Genus::Animal(Size::Average, FoodPreference::Carnivore),
		min_dist : 10.0,
		radius: (6.0..6.0),
		color : ColorTone::Red,
		durability : 100.0,
		speed : 0.6,
		tasks : &[Task::Hunt(Genus::Animal(Size::Small, FoodPreference::Herbivore))],
		max_lifetime: Some(FRAME_YEAR)
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
	Hunt(Genus),
	Reproduce,
	ListenControls(),
}

pub enum Action {
	Scream { bounds : CircleBounds },
	Hit { bounds : CircleBounds, damage : f32 },
	Swallow(usize),
	Pick(usize),
	MoveTo(Point),
	Spawn(GameObj)
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
				Task::Wander() => gen_circle_bounds(&Bounds::Rect{ v : &w.map[1].bounds }, &w.objects, &self.blueprint).
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
				Task::Hunt(genus) => {
					if let Some((i, food)) = w.objects.iter().enumerate().filter( |(_, obj)| obj.blueprint.genus == *genus).
						min_by_key( |(_, obj)| OrderedFloat(obj.bounds.coords.dist(&self.bounds.coords))) {
						if food.bounds.collides_with(&self.bounds) {
							TaskAct(Action::Swallow(i))
						} else {
							let mut obstacles = w.objects.iter().filter(|o| o.blueprint.genus != Genus::Plant(Size::Small)).map(|o| &o.bounds);
							TaskAct(Action::MoveTo(self.bounds.coords + move_to_target(&self.bounds, &food.bounds.coords, &mut obstacles, self.blueprint.speed)))
						}
					} else {
						TaskPop
					}
				},
				Task::Reproduce => {
					match self.blueprint.genus {
						Genus::Plant(_) => {
							if rng_range(&(0.0..1.0)) < 0.001 {
								let new_b = Bounds::Circle { v: &CircleBounds { r: self.bounds.r * 4.0, ..self.bounds } };
								gen_circle_bounds(&new_b, &w.objects, &self.blueprint).
									map(|b| TaskAct(Action::Spawn(GameObj::from(&self.blueprint, b, w.time)))).unwrap_or(TaskWait)
							} else { TaskWait }
						},
						Genus::Animal(_, _) => { println!("TODO!"); TaskWait },
					}
				}
				Task::ListenControls() => {
					TaskWait
				}
			}
		}
	}
}

impl World {
	pub fn upd(&mut self) {
		let mut removed_objects : HashSet<usize> = HashSet::new();
		let plans : Vec<(usize, TaskUpd)> = self.objects.iter().enumerate().map( |(i, o)| (i, o.plan(&self))).collect();
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
					Action::Spawn(obj) => self.objects.push(obj)
				},
			});

		let time = self.time;
		self.objects.iter_mut().enumerate().for_each(|(i, obj)| {
// TODO: die from hunger and from damage, add hunger
			obj.tmp_effects.retain(|eff| eff.start + eff.duration >= time);
			obj.tmp_effects.iter().filter(|eff| eff.start <= time).for_each(|eff| {
				// TODO: fill missing branches
				match &eff.effect {
					EffectType::PeriodicEffect { period, effect } => if ((time - eff.start) % period) == 0 {
						match effect {
							PeriodicEffectType::Hunger(_) => {},
							PeriodicEffectType::Damage(_) => {},
						}
					},
					EffectType::Slow(_) => {},
					EffectType::Stun => {},
					EffectType::DelayedDeath => { println!("DEATH!");removed_objects.insert(i); },
				}
			});
		});
		self.time += 1;
		removed_objects.iter().sorted_by_key(|i| -(**i as i32)).for_each( |i| { self.objects.remove(*i); });
	}
}