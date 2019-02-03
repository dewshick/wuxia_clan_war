mod world_gen;
mod colors;
mod collision;
mod std_extended;
mod world_render;
use self::world_gen::*;
use self::collision::*;
use self::world_render::*;
use piston_window::*;
use rand::{thread_rng, Rng};
extern crate fps_counter;


fn main() {
	let mut window: PistonWindow =
		WindowSettings::new("Hello Piston!", [640, 480]).exit_on_esc(true).build().unwrap();
	let mut world = generate_world(vec![
		RectTile {
			tile: Tile::Water,
			bounds: RectBounds { coords: Point::new(0.0, 0.0), size: Point::new(640.0, 480.0) }
		},
		RectTile {
			tile: Tile::Forest,
			bounds: RectBounds { coords: Point::new(50.0, 50.0), size: Point::new(480.0, 320.0) }
		},
		RectTile {
			tile: Tile::Village,
			bounds: RectBounds {
				coords: Point::new(120.0, 120.0),
				size: Point::new(100.0, 100.0),
			}
		},
		RectTile {
			tile: Tile::Mine,
			bounds: RectBounds {
				coords: Point::new(350.0, 250.0),
				size: Point::new(120.0, 110.0),
			}
		}
	]);
	let mut fps = fps_counter::FPSCounter::new();
	while let Some(event) = window.next() {
		window.draw_2d(&event, |context, graphics| {
			render_world(&mut world, context.transform, graphics)
		});
		println!("{}", fps.tick());
	}
}