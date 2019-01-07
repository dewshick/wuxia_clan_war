mod world_gen;
mod colors;
mod collision;
use self::world_gen::*;
use self::collision::*;
use piston_window::*;
use rand::{thread_rng, Rng};

const WHITE : types::Color = [1.0; 4];

fn main() {
	let mut window: PistonWindow =
		WindowSettings::new("Hello Piston!", [640, 480]).exit_on_esc(true).build().unwrap();
	let world = generate_world(vec![
		RectTile {
			tile: Tile::Water,
			bounds: RectBounds { coords: Point::new(0.0, 0.0), rect: Point::new(640.0, 480.0) }
		},
		RectTile {
			tile: Tile::Forest,
			bounds: RectBounds { coords: Point::new(50.0, 50.0), rect: Point::new(480.0, 320.0) }
		},
		RectTile {
			tile: Tile::Village,
			bounds: RectBounds {
				coords: Point::new(120.0, 120.0),
				rect: Point::new(100.0, 100.0),
			}
		},
		RectTile {
			tile: Tile::Mine,
			bounds: RectBounds {
				coords: Point::new(350.0, 250.0),
				rect: Point::new(120.0, 110.0),
			}
		}
	]);

	while let Some(event) = window.next() {
		window.draw_2d(&event, |context, graphics| {
			clear(WHITE, graphics);
			render_world(&world, context.transform, graphics)
		});
	}
}
