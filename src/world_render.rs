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

pub struct RenderedShape<'a> { color : Color, bounds : Bounds<'a> }

impl World {
	pub fn to_scene(&mut self) -> Vec<RenderedShape> {
		self.objects.sort_by_key(|obj| OrderedFloat(obj.blueprint.speed));
		self.map.iter().map(|layer| {
			RenderedShape { bounds : Bounds::Rect { v : &layer.bounds }, color : tile_color(&layer.tile) }
		}).chain(self.objects.iter().map(|obj| RenderedShape {
			bounds: Bounds::Circle { v: &obj.bounds },
			color: solid_color(&obj.blueprint.color)
		})).collect()
	}
}

fn tile_color(t : &Tile) -> Color {
	match t {
		Tile::Forest => solid_color(&ColorTone::LawnGreen),
		Tile::Village => solid_color(&ColorTone::PaleGoldenRod),
		Tile::Mine => solid_color(&ColorTone::Gainsboro),
		Tile::Water => solid_color(&ColorTone::MediumBlue),
	}
}

struct WorldWithDebugInfo { world : World, fps : FPSCounter, controls : ControlsState }

struct ControlsState { up : bool, down : bool, left : bool, right : bool, superhot : bool }

fn bool2f32(b : bool) -> f32 {
	if (b) { 1.0 } else { 0.0 }
}

impl ControlsState {
	fn init() -> ControlsState { ControlsState { up : false, down : false, left : false, right : false, superhot : true } }

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
		if (direction.len() > 0.001 || !self.controls.superhot) { self.world.update(ctx) } else { Ok(()) }
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		self.world.draw(ctx)
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
}

// ggez-related rendering
impl EventHandler for World {
	fn update(&mut self, _ctx: &mut Context) -> GameResult {
		self.upd();
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mesh: Mesh = self.to_scene().iter().fold(&mut MeshBuilder::new(), |mb, shape| match shape.bounds {
			Bounds::Rect { v : RectBounds { coords, size } } => mb.rectangle(DrawMode::Fill, Rect::new(coords.x, coords.y, size.x, size.y), shape.color.into()),
			Bounds::Circle { v : CircleBounds { coords, r } } => mb.circle(DrawMode::Fill, Point2::new(coords.x, coords.y), *r, 0.4, shape.color.into()),
		}).build(ctx)?;

		draw(ctx, &mesh, (Point2::new(0.0, 0.0),))?;
		present(ctx)?;
		Ok(())
	}
}

pub fn ggez_loop(w : World) {
	let cb = ggez::ContextBuilder::new("super_simple", "ggez");
	let (ctx, event_loop) = &mut cb.build().unwrap();
	run(ctx, event_loop, &mut WorldWithDebugInfo { world : w, fps : fps_counter::FPSCounter::new(), controls: ControlsState::init() }).unwrap();
}