use super::colors::*;
use super::collision::*;
use super::std_extended::*;

//use piston_window::{rectangle, math::Matrix2d, Graphics, types::Color};
#[derive(PartialEq, Eq, Debug)]
pub enum Tile { Forest, Village, Mine, Water }

#[derive(Debug)]
pub struct RectTile { pub tile : Tile, pub bounds : RectBounds }

type Map = Vec<RectTile>;

#[derive(Debug)]
pub struct World { map : Map, trees : Vec<CircleBounds>, wanderers : Vec<MovingObject> }

pub fn generate_world(map : Map) -> World {
	let mut trees : Vec<CircleBounds> = vec![];
	let (tree_r_min, tree_r_max, tree_dist) = (8.0, 18., 10.0);

	map.iter().for_each(|layer| {
		if layer.tile == Tile::Forest {
			while let Some(t) = try_n_times(100, &|| {
				gen_circle_bounds(&(layer.bounds), &mut trees.iter(), rng_range(tree_r_min, tree_r_max), tree_dist)
			}) { trees.push(t); }
		} else {
			trees.retain(|tree| !tree.on_layer(&layer.bounds, tree_dist));
		}
	});

	let mut wanderers : Vec<MovingObject> = vec![];
	let (wanderer_r, wanderer_dist) = (4.0, 1.0);
	map.iter().find(|layer| layer.tile != Tile::Water).iter().for_each( |layer| {
		for _ in 0..20 {
			let gen_wanderer = || try_n_times(100, &|| {
				gen_circle_bounds(&layer.bounds, &mut trees.iter().chain(wanderers.iter().map(|mo| &mo.bounds)), wanderer_r, wanderer_dist)
			});

			let wanderer = gen_wanderer().and_then(|b| gen_wanderer().map(|dir| MovingObject { bounds: b, target: dir.coords }));
			wanderer.map(|w| wanderers.push(w));
		}
	});

	World { map, trees, wanderers }
}

fn gen_circle_bounds<'a, T>(layer : &RectBounds, existing_bounds : &mut T, r : f32, dist : f32) -> Option<CircleBounds>
	where T : Iterator<Item = &'a CircleBounds> {
//	const tree params
		let bounds = CircleBounds {
			coords: Point::new(
				rng_range(r + layer.coords.x + dist, layer.coords.x + layer.rect.x - r - dist),
				rng_range(r + layer.coords.y + dist, layer.coords.y + layer.rect.y - r - dist)
			),
			r
		};
		if can_add(&bounds, dist, existing_bounds) { Some(bounds) } else { None }
}

fn tile_color(t : &Tile) -> piston_window::types::Color {
	match t {
		Tile::Forest => solid_color(LawnGreen),
		Tile::Village => solid_color(PaleGoldenRod),
		Tile::Mine => solid_color(Gainsboro),
		Tile::Water => solid_color(MediumBlue),
	}
}

// todo: check if background tile is not overlapping main tile
fn render_map<G>(map : &Map, t : piston_window::math::Matrix2d, g : &mut G) where G : piston_window::Graphics {
	map.iter().for_each(|layer| {
		let rect = [
			layer.bounds.coords.x as f64, layer.bounds.coords.y as f64,
			layer.bounds.rect.x as f64, layer.bounds.rect.y as f64
		];
		piston_window::rectangle(tile_color(&layer.tile), rect, t, g);
	});
}

pub fn render_world<G>(world : &World, t : piston_window::math::Matrix2d, g : &mut G) where G : piston_window::Graphics  {
	render_map(&world.map, t, g);
	let mut render_bounds = |bounds : &CircleBounds, color| {
		let (upperx, uppery, side) = ((bounds.coords.x - bounds.r) as f64, (bounds.coords.y - bounds.r) as f64, 2.0 * bounds.r as f64);
		piston_window::ellipse(color, [upperx, uppery, side, side], t, g);
	};
	world.trees.iter().for_each( |tree| render_bounds(tree, solid_color(Brown)));
	world.wanderers.iter().for_each( |w| render_bounds(&w.bounds, solid_color(Black)));
}