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
	add_objects(&mut world, Tile::Forest, &GameObjBlueprint::TREE, -1);
	add_objects(&mut world, Tile::Forest, &GameObjBlueprint::GRASS, -1);
	add_objects(&mut world, Tile::Village, &GameObjBlueprint::WANDERER, 50);
	add_objects(&mut world, Tile::Forest, &GameObjBlueprint::HARE, 200);
	world
}

pub fn add_objects(w : &mut World, tile : Tile, blueprint : &'static GameObjBlueprint, count : i32) {
	let map = &w.map;
	let objects = &mut w.objects;
	map.iter().for_each(|layer| {
		if layer.tile == tile {
			if count > 0 {
				while let Some(b) = gen_circle_bounds(&(layer.bounds), objects, &blueprint) {
					objects.push(GameObj::from(&blueprint, b));
				}
			} else {
				for _ in 0..count {
					gen_circle_bounds(&(layer.bounds), objects, &blueprint).map( |b| objects.push(GameObj::from(&blueprint, b)));
				}
			}
		} else {
			objects.retain(|obj| !obj.bounds.on_layer(&layer.bounds, blueprint.min_dist));
		}
	});
}

pub fn gen_circle_bounds(layer : &RectBounds, objects : &Vec<GameObj>, blueprint : &'static GameObjBlueprint) -> Option<CircleBounds> {
	try_n_times(100, &|| {
		let r = rng_range(&blueprint.radius);
		let bounds = CircleBounds {
			coords: Point::new(
				rng_range(&(r + layer.coords.x + blueprint.min_dist..layer.coords.x + layer.size.x - r - blueprint.min_dist)),
				rng_range(&(r + layer.coords.y + blueprint.min_dist..layer.coords.y + layer.size.y - r - blueprint.min_dist))
			),
			r
		};
		if can_add(&bounds, blueprint.min_dist, &mut objects.iter().map(|obj| &obj.bounds)) { Some(bounds) } else { None }
	})
}
