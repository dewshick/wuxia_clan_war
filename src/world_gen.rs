use super::colors::*;
use super::collision::*;
use super::std_extended::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Tile { Forest, Village, Mine, Water }

#[derive(Debug)]
pub struct RectTile { pub tile : Tile, pub bounds : RectBounds }

type Map = Vec<RectTile>;

#[derive(Debug)]
pub struct GameObject {
	pub bounds : CircleBounds,
	pub color : ColorTone,
	pub durability : Amount,
	pub speed : Dist,
	pub tasks : Vec<Task>
}

impl GameObject {
	fn tree(bounds : CircleBounds) -> GameObject {
		GameObject { bounds, color : ColorTone::Brown, durability : 100.0, speed : -1.0, tasks : vec![] }
	}

	fn wanderer(bounds : CircleBounds, target : Point) -> GameObject {
		GameObject { bounds, color : ColorTone::Black, durability : 20.0, speed : 0.33, tasks : vec![Task::MovingTo(target)] }
	}
}

#[derive(Debug)]
pub enum Task {
	MovingTo(Point)
}

#[derive(Debug)]
pub struct World { pub map : Map, pub objects : Vec<GameObject> }

pub fn generate_world(map : Map) -> World {
	let mut game_objects : Vec<GameObject> = vec![];
	let (tree_r_min, tree_r_max, tree_dist) = (8., 18., 10.);

	map.iter().for_each(|layer| {
		if layer.tile == Tile::Forest {
			while let Some(t) = try_n_times(100, &|| {
				gen_circle_bounds(&(layer.bounds), &mut game_objects.iter().map (|obj| &obj.bounds), rng_range(tree_r_min, tree_r_max), tree_dist)
			}) { game_objects.push(GameObject::tree(t)); }
		} else {
			game_objects.retain(|obj| !obj.bounds.on_layer(&layer.bounds, tree_dist));
		}
	});

	let mut world = World { map, objects: game_objects };
	for _ in 0..200 {
		gen_wanderer_bounds(&mut world).and_then(|b| gen_wanderer_bounds(&mut world).map(|dir| world.objects.push(GameObject::wanderer(b, dir.coords))));
	};
	world
}

pub fn gen_wanderer_bounds(w : &mut World) -> Option<CircleBounds> {
	let (wanderer_r, wanderer_dist) = (4.0, 1.0);
		w.map.iter().find(|layer| layer.tile != Tile::Water).and_then(|layer| try_n_times(100, &|| {
			gen_circle_bounds(&layer.bounds, &mut w.objects.iter().map(|o| &o.bounds), wanderer_r, wanderer_dist)
		}))
}

fn gen_circle_bounds<'a, T>(layer : &RectBounds, existing_bounds : &mut T, r : Dist, dist : Dist) -> Option<CircleBounds>
	where T : Iterator<Item = &'a CircleBounds> {
//	const tree params
		let bounds = CircleBounds {
			coords: Point::new(
				rng_range(r + layer.coords.x + dist, layer.coords.x + layer.size.x - r - dist),
				rng_range(r + layer.coords.y + dist, layer.coords.y + layer.size.y - r - dist)
			),
			r
		};
		if can_add(&bounds, dist, existing_bounds) { Some(bounds) } else { None }
}
