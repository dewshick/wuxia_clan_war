use super::collision::*;
use super::std_extended::*;
use crate::world_update::GameObj;
use crate::world_update::GameObjBlueprint;

#[derive(PartialEq, Eq, Debug)]
pub enum Tile { Forest, Village, Mine, Water }

#[derive(Debug)]
pub struct RectTile { pub tile : Tile, pub bounds : RectBounds }

type Map = Vec<RectTile>;

#[derive(Debug)]
pub struct World { pub map : Map, pub objects : Vec<GameObj> }

pub fn generate_world(map : Map, wanderers : i32) -> World {
	let mut world = World { map, objects: vec![] };
	add_objects(&mut world, Tile::Forest, &GameObjBlueprint::TREE, None);
	add_objects(&mut world, Tile::Forest, &GameObjBlueprint::HARE, Some(10));
	add_objects(&mut world, Tile::Forest, &GameObjBlueprint::GRASS, None);
	add_objects(&mut world, Tile::Village, &GameObjBlueprint::WANDERER, Some(wanderers));
	world
}

fn add_object(layer : &RectTile, objects : &mut Vec<GameObj>, blueprint : &'static GameObjBlueprint) -> bool {
	let bounds = gen_circle_bounds(&layer.bounds, objects, blueprint);
	if bounds.is_some() { objects.push(GameObj::from(&blueprint, bounds.unwrap())); true } else { false }
}

pub fn add_objects(w : &mut World, tile : Tile, blueprint : &'static GameObjBlueprint, count : Option<i32>) {
	let objects = &mut w.objects;
	let map = &w.map;
	map.iter().for_each( |layer| {
		if layer.tile == tile {
			if count.is_none() {
				loop { if !add_object(&layer, objects, blueprint) { break } }
			} else {
				for _ in 0..count.unwrap() { add_object(&layer, objects, blueprint); }
			}
		} else {
			objects.retain(|obj| !obj.bounds.on_layer(&layer.bounds, blueprint.min_dist) || obj.blueprint.name != blueprint.name);
		}
	});
}

pub fn gen_circle_bounds(layer : &RectBounds, objects : &Vec<GameObj>, blueprint : &'static GameObjBlueprint) -> Option<CircleBounds> {
	for _ in 0..100 {
		let r = rng_range(&blueprint.radius);
		let bounds = CircleBounds {
			coords: Point::new(
				rng_range(&(r + layer.coords.x + blueprint.min_dist..layer.coords.x + layer.size.x - r - blueprint.min_dist)),
				rng_range(&(r + layer.coords.y + blueprint.min_dist..layer.coords.y + layer.size.y - r - blueprint.min_dist))
			),
			r
		};
		if !(objects.iter().any(|obs| { obs.bounds.coords.dist(&bounds.coords) < obs.bounds.r + bounds.r + blueprint.min_dist})) { return Some(bounds) }
	}
	return None
}
