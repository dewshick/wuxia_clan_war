use super::collision::*;
use super::std_extended::*;
use crate::world_update::GameObj;
use crate::world_update::GameObjBlueprint;
use ordered_float::OrderedFloat;
use crate::world_update::FrameCount;

pub enum Bounds<'a> {
  Rect { size : &'a Size },
  Circle { v : &'a CircleBounds },
}

// entities on different layers do not collide
#[derive(PartialEq, Eq, Debug)]
pub enum Tile { Floor = 0, Middle = 1, Sky = 2 }

#[derive(Debug)]
pub struct RectTile { pub tile : Tile, pub bounds : RectBounds }

#[derive(Debug)]
pub struct World { pub size : Size, pub objects : Vec<GameObj>, pub time : u64 }

pub fn generate_world(size : Size, wanderers : i32) -> World {
	let mut world = World { size, objects: vec![], time : 0 };
	add_objects(&mut world, Tile::Middle, &GameObjBlueprint::TREE, None);
	add_objects(&mut world, Tile::Floor, &GameObjBlueprint::HARE, Some(100));
	add_objects(&mut world, Tile::Middle, &GameObjBlueprint::WOLF, Some(10));
	add_objects(&mut world, Tile::Floor, &GameObjBlueprint::GRASS, Some(1000));
	add_objects(&mut world, Tile::Middle, &GameObjBlueprint::WANDERER, Some(wanderers));
	add_objects(&mut world, Tile::Middle, &GameObjBlueprint::PLAYER, Some(1));
	world
}

fn add_object(size : &Size, objects : &mut Vec<GameObj>, blueprint : &'static GameObjBlueprint, time : FrameCount) -> bool {
	let bounds = gen_circle_bounds(size, None, objects, blueprint);
	if bounds.is_some() { objects.push(GameObj::from(&blueprint, bounds.unwrap(), time)); true } else { false }
}

pub fn add_objects(w : &mut World, tile : Tile, blueprint : &'static GameObjBlueprint, count : Option<i32>) {
	let objects = &mut w.objects;
	let time = w.time;
	if count.is_none() {
		loop { if !add_object(&w.size, objects, blueprint, time) { break } }
	} else {
		for _ in 0..count.unwrap() { add_object(&w.size, objects, blueprint, time); }
	}
}

//maybe support circular bounds too
pub fn gen_circle_bounds(
	size : &Size,
	center_bounds : Option<&CircleBounds>,
	objects : &Vec<GameObj>,
	blueprint : &'static GameObjBlueprint
) -> Option<CircleBounds> {
	for _ in 0..100 {
		let r = rng_range(&blueprint.radius);
		let coords = match center_bounds {
			None => Point::new(
				rng_range(&(r + blueprint.min_dist..size.x - r - blueprint.min_dist)),
				rng_range(&(r + blueprint.min_dist..size.y - r - blueprint.min_dist))
			),
			Some(v) => {
				let (angle, dist) = (rng_range(&(0.0..2.0*std::f32::consts::PI)), rng_range(&(0.0..v.r)));
				Point::new(v.coords.x + angle.sin() * dist, v.coords.y + angle.cos() * dist)
			}
		};
		let bounds = CircleBounds { coords, r };
		if !(objects.iter().any(|obs| { obs.bounds.coords.dist(&bounds.coords) < obs.bounds.r + bounds.r + blueprint.min_dist})) { return Some(bounds) }
	}
	return None
}
