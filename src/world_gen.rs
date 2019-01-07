use super::colors::*;
use super::collision::*;
use super::{thread_rng, Rng};
use piston_window::math::Matrix2d;

//use piston_window::{rectangle, math::Matrix2d, Graphics, types::Color};
#[derive(PartialEq, Eq)]
pub enum Tile { Forest, Village, Mine }

pub struct RectTile { pub tile : Tile, pub bounds : RectBounds }
type Map = Vec<RectTile>;
pub struct World { map : Map, trees : Vec<CircleBounds> }
const TREE_WIDTH : Dist = 15.0;
const TREE_DIST : Dist = 7.0;

pub fn generate_world(map : Map) -> World {
	let mut trees = vec![];
	map.iter().filter(|layer| layer.tile == Tile::Forest).for_each(|layer| {
		while let Some(t) = gen_tree(&layer.bounds, &trees, 10) { trees.push(t); }
	});
	World { map, trees : trees }
}

fn gen_tree(layer : &RectBounds, trees : &Vec<CircleBounds>, attempts : i32) -> Option<CircleBounds> {
	if (attempts == 0) {
		None
	} else {
		let tree = CircleBounds {
			coords: Point::new(
				thread_rng().gen_range(TREE_WIDTH + layer.coords.x + TREE_DIST, layer.coords.x + layer.rect.x - TREE_WIDTH - TREE_DIST),
				thread_rng().gen_range(TREE_WIDTH + layer.coords.y + TREE_DIST, layer.coords.y + layer.rect.y - TREE_WIDTH - TREE_DIST)
			),
			r: TREE_WIDTH
		};
		if (can_add(&tree, TREE_DIST, trees)) { Some(tree) } else { gen_tree(layer, trees, attempts - 1) }
	}
}

fn tile_color(t : &Tile) -> piston_window::types::Color {
	match t {
		Tile::Forest => solid_color(Lime),
		Tile::Village => solid_color(PaleGoldenRod),
		Tile::Mine => solid_color(Gainsboro),
	}
}

// todo: check if background tile is not overlapping main tile
fn render_map<G>(p : Point, map : &Map, t : piston_window::math::Matrix2d, g : &mut G) where G : piston_window::Graphics {
	map.iter().for_each(|layer| {
		let rect = [
			layer.bounds.coords.x as f64, layer.bounds.coords.y as f64,
			layer.bounds.rect.x as f64, layer.bounds.rect.y as f64
		];
		piston_window::rectangle(tile_color(&layer.tile), rect, t, g);
	});
}

pub fn render_world<G>(world : &World, t : piston_window::math::Matrix2d, g : &mut G) where G : piston_window::Graphics  {
	render_map(Point::init(), &world.map, t, g);
	world.trees.iter().for_each( |tree| {
		let (upperx, uppery, side) = ((tree.coords.x - tree.r) as f64, (tree.coords.y - tree.r) as f64, tree.r as f64);
		piston_window::ellipse(solid_color(Brown), [upperx, uppery, side, side], t, g);
	})
}