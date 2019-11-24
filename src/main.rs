mod world_gen;
mod world_update;
mod colors;
mod collision;
mod std_extended;
mod world_render;
use self::world_gen::*;
use self::collision::*;
use self::world_render::*;
use rand::{thread_rng, Rng};
extern crate fps_counter;

fn main() {
	let world = generate_world(Point::new(1024.0, 768.0), 400);
	ggez_loop(world);
}
