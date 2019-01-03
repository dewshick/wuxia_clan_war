extern crate piston_window;

use piston_window::*;

const WHITE : types::Color = [1.0; 4];
//
//struct Forest { }
//struct Village { }
//struct Mine { }

enum Tile { Forest, Village, Mine }
enum Map {
	RectTile { tile : Tile, width : u16, height : u16 },
	TileOnTop { main: Box<Map>, background : Box<Map>, x_off : u16, y_off : u16 }
}

fn tile_color(t : &Tile) -> types::Color {
	match t {
		Tile::Forest => [0.0, 1.0, 0.0, 1.0],
		Tile::Village => [1.0, 0.0, 0.0, 1.0],
		Tile::Mine => [0.0, 0.0, 1.0, 1.0],
	}
}

// todo: check if background tile is not overlapping main tile
fn render_map<G>(x : u16, y :u16, map : &Map, t : math::Matrix2d, g : &mut G) where G : Graphics {
	match map {
		Map::RectTile { tile, width, height } => {
			let rect = [x as f64, y as f64, *width as f64, *height as f64];
			rectangle(tile_color(tile), rect, t, g)
		},
		Map::TileOnTop { main, background, x_off, y_off } => {
			render_map(x, y, background.as_ref(), t, g);
			render_map(x + x_off, y + y_off, main.as_ref(), t, g)
		}
	}
}

fn main() {
	let mut window: PistonWindow =
		WindowSettings::new("Hello Piston!", [640, 480]).exit_on_esc(true).build().unwrap();
	let world = Map::TileOnTop {
		main : Box::new(Map::RectTile { tile : Tile::Village, width : 100, height : 100 }),
		background : Box::new(Map::RectTile { tile : Tile::Forest, width : 480, height : 320 }),
		x_off : 120,
		y_off : 120
	};

	while let Some(event) = window.next() {
		window.draw_2d(&event, |context, graphics| {
			clear(WHITE, graphics);
			render_map(0, 0, &world, context.transform, graphics)
		});
	}
}
