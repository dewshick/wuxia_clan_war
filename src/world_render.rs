use super::world_gen::*;
use super::colors::*;
use crate::collision::*;
use ggez::graphics::MeshBuilder;
use ggez::graphics::DrawMode;
use ggez::graphics::Rect;
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::error::GameResult;
use ggez::graphics::{draw, present};
use ggez::graphics::Mesh;
use fps_counter::FPSCounter;
use ggez::graphics::Color;
use ggez::event::{EventHandler, run};
use ordered_float::OrderedFloat;
use ggez::input::keyboard::KeyCode;
use itertools::Itertools;
use ggez::input::keyboard::KeyMods;
use ggez::event::EventsLoop;
use ggez::graphics::{Text, TextFragment};

pub struct RenderedShape<'a> { color : Color, bounds : Bounds<'a> }

impl World {
	pub fn to_scene(&mut self) -> Vec<RenderedShape> {
		self.objects.sort_by_key(|obj| OrderedFloat(obj.blueprint.speed));
		Some(RenderedShape {
			bounds : Bounds::Rect { size : &self.size },
			color: solid_color(&ColorTone::LimeGreen)
		}).into_iter().
			chain(self.objects.iter().map(|obj| RenderedShape {
				bounds: Bounds::Circle { v: &obj.bounds },
				color: solid_color(&obj.blueprint.color)
			})).collect()
	}
}

struct WorldWithDebugInfo { world : World, fps : FPSCounter, controls : ControlsState }

struct ControlsState { up : bool, down : bool, left : bool, right : bool, superhot : bool, mouse : Coords }

fn bool2f32(b : bool) -> f32 {
	if b { 1.0 } else { 0.0 }
}

impl ControlsState {
	fn init() -> ControlsState { ControlsState { up : false, down : false, left : false, right : false, superhot : true, mouse : Coords { x : 0.0, y : 0.0 } } }

	fn upd_key(&mut self, keycode : KeyCode, down : bool) {
		match keycode {
			KeyCode::Up => { self.up = down },
			KeyCode::Down =>  { self.down = down },
			KeyCode::Left =>  { self.left = down },
			KeyCode::Right =>  { self.right = down },
			KeyCode::Space => if down { self.superhot = !self.superhot; }
			_ => {},
		}
	}

	fn direction(&self) -> Point {
		Point { x: bool2f32(self.right) - bool2f32(self.left), y: bool2f32(self.down) - bool2f32(self.up) }
	}
}


impl EventHandler for WorldWithDebugInfo {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
//    update player
		let i = self.world.objects.iter().find_position( |item| item.blueprint.name == "Player").unwrap().0;
		let mut player = &self.world.objects[i];
		let speed = player.blueprint.speed;
		let direction = self.controls.direction();
		let upd_coords = player.move_to(&self.world, &(player.bounds.coords + direction.multf(speed)));
		let mut player_mut = &mut self.world.objects[i];
		player_mut.bounds.coords = upd_coords;
//    update world
		if (direction.len() > 0.001 || !self.controls.superhot) { /*println!("{}", self.fps.tick());*/ self.world.update(ctx) } else { Ok(()) }
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let result = self.world.draw(ctx);
		let selected = self.world.objects.iter().for_each( |obj|
			if obj.bounds.collides_with(&CircleBounds { coords : self.controls.mouse, r : 0.1 }) {
				let text = Text::new(format!("durability: {}", &obj.durability));
				let coverRect = match text.dimensions(ctx) { (x, y) => {
//					println!("{} {}", x, y);
					let mb = MeshBuilder::new().rectangle(
						DrawMode::Fill,
						Rect::new(10.0, 10.0, x as f32, y as f32),
						solid_color(&ColorTone::Black)
					).build(ctx).unwrap();
					draw(ctx, &mb, (point2(&self.controls.mouse),));
					draw(ctx, &text, (point2(&(self.controls.mouse + Point::new(10.0, 10.0))),));

				}};
			}
		);
		let result = present(ctx);
		result
	}

	fn key_down_event(
		&mut self,
		ctx: &mut Context,
		keycode: KeyCode,
		_keymods: KeyMods,
		_repeat: bool,
	) {
		self.controls.upd_key(keycode, true);
	}

	fn key_up_event(
		&mut self,
		ctx: &mut Context,
		keycode: KeyCode,
		_keymods: KeyMods,
	) {
		self.controls.upd_key(keycode, false);
	}

	fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
		self.controls.mouse = Coords{ x, y };
	}
}

// ggez-related rendering
impl EventHandler for World {
	fn update(&mut self, _ctx: &mut Context) -> GameResult {
		self.upd();
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mesh: Mesh = self.to_scene().iter().fold(&mut MeshBuilder::new(), |mb, shape| match shape.bounds {
			Bounds::Rect { size } => mb.rectangle(DrawMode::Fill, Rect::new(0.0, 0.0, size.x, size.y), shape.color.into()),
			Bounds::Circle { v : CircleBounds { coords, r } } => mb.circle(DrawMode::Fill, point2(coords), *r, 0.4, shape.color.into()),
		}).build(ctx)?;

		draw(ctx, &mesh, (Point2::new(0.0, 0.0),))?;
		Ok(())
	}
}

pub fn ggez_loop(w : World) {
	let cb = ggez::ContextBuilder::new("super_simple", "ggez");
	let (ctx, event_loop) = &mut cb.build().unwrap();
	run(ctx, event_loop, &mut WorldWithDebugInfo { world : w, fps : fps_counter::FPSCounter::new(), controls: ControlsState::init() }).unwrap();
}

pub fn point2(coords : &Coords) -> Point2<f32> {
	Point2::new(coords.x, coords.y)
}