use piston_window::*;

pub enum Tile { Forest, Village, Mine }
pub enum Map {
	RectTile { tile : Tile, width : u16, height : u16 },
	TileOnTop { main: Box<[(u16, u16, Map)]>, background : Box<Map> }
}

//World { map : Map, trees : [] }

//fn generate_world(map : Map) -> World {
//	World {
//		map : map,
//		trees : [],
//	}
//}

fn tile_color(t : &Tile) -> types::Color {
	match t {
		Tile::Forest => [0.3, 1.0, 0.3, 1.0],
		Tile::Village => [0.8, 0.6, 0.6, 1.0],
		Tile::Mine => [0.8, 0.8, 0.8, 1.0],
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
			main.as_ref().iter().for_each ( |(x_off, y_off, submap)| render_map(x + x_off, y + y_off, submap, t, g) );
		}
	}
}
