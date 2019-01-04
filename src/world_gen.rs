use super::custom_colors::*;
use piston_window::*;
use graphics::math::*;

type Coord = u16;
type Dist = f32;
type Coords = Vec2d<Coord>;
type Size = Vec2d<Dist>;
struct Circle { coords : Coords, r : Dist }

pub enum Tile { Forest, Village, Mine }

pub enum Map {
	RectTile { tile : Tile, width : Coord, height : Coord },
	TileOnTop { main: Box<[(Coords, Map)]>, background : Box<Map> }
}
pub struct World { map : Map, trees : Vec<Circle> }
const TREE_WIDTH : Dist = 5.0;

pub fn generate_world(map : Map, tree_dist : Dist) -> World {
	World { map : map, trees : vec![] }
}

fn tile_color(t : &Tile) -> types::Color {
	match t {
		Tile::Forest => solid_color(Lime),
		Tile::Village => solid_color(PaleGoldenRod),
		Tile::Mine => solid_color(Gainsboro),
	}
}

// todo: check if background tile is not overlapping main tile
pub fn render_map<G>(x : u16, y :u16, map : &Map, t : math::Matrix2d, g : &mut G) where G : Graphics {
	match map {
		Map::RectTile { tile, width, height } => {
			let rect = [x as f64, y as f64, *width as f64, *height as f64];
			rectangle(tile_color(tile), rect, t, g)
		},
		Map::TileOnTop { main, background } => {
			render_map(x, y, background.as_ref(), t, g);
			main.as_ref().iter().for_each ( |([x_off, y_off], submap)| render_map(x + x_off, y + y_off, submap, t, g) );
		}
	}
}
