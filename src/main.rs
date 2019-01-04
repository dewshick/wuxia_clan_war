use piston_window::*;

mod world_gen;
use self::world_gen::*;

const WHITE : types::Color = [1.0; 4];

fn main() {
	let mut window: PistonWindow =
		WindowSettings::new("Hello Piston!", [640, 480]).exit_on_esc(true).build().unwrap();
	let world = Map::TileOnTop {
		main : Box::new([
			(120, 120, Map::RectTile { tile : Tile::Village, width : 100, height : 100 }),
			(350, 250, Map::RectTile { tile : Tile::Mine, width : 80, height : 50 }),
		]),
		background : Box::new(Map::RectTile { tile : Tile::Forest, width : 480, height : 320 }),
	};

	while let Some(event) = window.next() {
		window.draw_2d(&event, |context, graphics| {
			clear(WHITE, graphics);
			render_map(0, 0, &world, context.transform, graphics)
		});
	}
}
