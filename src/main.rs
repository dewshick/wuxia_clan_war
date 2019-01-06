mod world_gen;
mod colors;
mod collision;
use self::world_gen::*;
use self::collision::*;
use piston_window::*;

const WHITE : types::Color = [1.0; 4];

fn main() {
	let mut window: PistonWindow =
		WindowSettings::new("Hello Piston!", [640, 480]).exit_on_esc(true).build().unwrap();
	let world = Map::TileOnTop {
		main : Box::new([
			(Point::new(120.0, 120.0), Map::RectTile { tile : Tile::Village, size : Point::new(100.0, 100.0) }),
			(Point::new(350.0, 250.0), Map::RectTile { tile : Tile::Mine, size : Point::new(80.0, 50.0) }),
		]),
		background : Box::new(Map::RectTile { tile : Tile::Forest, size : Point::new(480.0, 320.0) }),
	};

	while let Some(event) = window.next() {
		window.draw_2d(&event, |context, graphics| {
			clear(WHITE, graphics);
			render_map(Point::init(), &world, context.transform, graphics)
		});
	}
}
