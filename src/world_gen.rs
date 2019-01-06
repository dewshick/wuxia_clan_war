use super::colors::*;
use super::collision::*;
use piston_window::math::Matrix2d;

//use piston_window::{rectangle, math::Matrix2d, Graphics, types::Color};

pub enum Tile { Forest, Village, Mine }

pub enum Map {
	RectTile { tile : Tile, size : Size },
	TileOnTop { main: Box<[(Coords, Map)]>, background : Box<Map> }
}
pub struct World { map : Map, trees : Vec<CircleBounds> }
const TREE_WIDTH : Dist = 5.0;

pub fn generate_world(map : Map, tree_dist : Dist) -> World { World { map, trees : vec![] } }

fn tile_color(t : &Tile) -> piston_window::types::Color {
	match t {
		Tile::Forest => solid_color(Lime),
		Tile::Village => solid_color(PaleGoldenRod),
		Tile::Mine => solid_color(Gainsboro),
	}
}

// todo: check if background tile is not overlapping main tile
fn render_map<G>(p : Point, map : &Map, t : piston_window::math::Matrix2d, g : &mut G) where G : piston_window::Graphics {
	match map {
		Map::RectTile { tile, size } => {
			let rect = [p.x as f64, p.y as f64, size.x as f64, size.y as f64];
			piston_window::rectangle(tile_color(&tile), rect, t, g)
		},
		Map::TileOnTop { main, background } => {
			render_map(p, background.as_ref(), t, g);
			main.as_ref().iter().for_each ( |(offset, submap)| render_map(*offset + p, submap, t, g) );
		}
	}
}

pub fn render_world<G>(world : &World, t : piston_window::math::Matrix2d, g : &mut G) where G : piston_window::Graphics  {
	render_map(Point::init(), &world.map, t, g);
	world.trees.iter().for_each( |tree| {
		let (upper, side) = ((tree.coords.x - tree.r) as f64, tree.r as f64);
		piston_window::ellipse(solid_color(Brown), [upper, upper, side, side], t, g);
	})
}