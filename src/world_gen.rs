use super::colors::*;
use super::collision::*;
use super::{thread_rng, Rng};
use piston_window::math::Matrix2d;

//use piston_window::{rectangle, math::Matrix2d, Graphics, types::Color};
#[derive(PartialEq, Eq, Debug)]
pub enum Tile { Forest, Village, Mine, Water }

#[derive(Debug)]
pub struct RectTile { pub tile : Tile, pub bounds : RectBounds }

type Map = Vec<RectTile>;

#[derive(Debug)]
pub struct World { map : Map, trees : Vec<CircleBounds> }

pub fn generate_world(map : Map) -> World {
	let mut trees : Vec<CircleBounds> = vec![];
	map.iter().for_each(|layer| {
		if (layer.tile == Tile::Forest) {
			while let Some(t) = gen_tree(&layer.bounds, &trees, 100) { trees.push(t); }
		} else {
			trees.retain(|tree| {
				tree.coords.x + tree.r + TREE_DIST < layer.bounds.coords.x ||
				tree.coords.x - tree.r - TREE_DIST > layer.bounds.coords.x + layer.bounds.rect.x ||
				tree.coords.y + tree.r + TREE_DIST < layer.bounds.coords.y ||
				tree.coords.y - tree.r - TREE_DIST > layer.bounds.coords.y  + layer.bounds.rect.y
			});
		}
	});
	World { map, trees }
}

const TREE_DIST : f32 = 10.0;

fn gen_tree(layer : &RectBounds, trees : &Vec<CircleBounds>, attempts : i32) -> Option<CircleBounds> {
//	const tree params
	let (tree_r_min, tree_r_max) = (8.0, 18.0);
	if (attempts == 0) {
		None
	} else {
		let tree_r = thread_rng().gen_range(tree_r_min, tree_r_max);
		let tree = CircleBounds {
			coords: Point::new(
				thread_rng().gen_range(tree_r + layer.coords.x + TREE_DIST, layer.coords.x + layer.rect.x - tree_r - TREE_DIST),
				thread_rng().gen_range(tree_r + layer.coords.y + TREE_DIST, layer.coords.y + layer.rect.y - tree_r - TREE_DIST)
			),
			r: tree_r
		};
		if (can_add(&tree, TREE_DIST, trees)) { Some(tree) } else { gen_tree(layer, trees, attempts - 1) }
	}
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
		let (upperx, uppery, side) = ((tree.coords.x - tree.r) as f64, (tree.coords.y - tree.r) as f64, 2.0 * tree.r as f64);
		piston_window::ellipse(solid_color(Brown), [upperx, uppery, side, side], t, g);
	})
}