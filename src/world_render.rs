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

pub struct RenderedShape<'a> { color : Color, bounds : Bounds<'a> }

impl World {
	pub fn to_scene(&mut self) -> Vec<RenderedShape> {
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

struct WorldWithDebugInfo { world : World, fps : FPSCounter }

impl EventHandler for WorldWithDebugInfo {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		self.world.update(ctx)
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		println!("{}", self.fps.tick());
		self.world.draw(ctx)
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
			Bounds::Circle { v : CircleBounds { coords, r } } => mb.circle(DrawMode::Fill, Point2::new(coords.x, coords.y), *r, 0.2, shape.color.into()),
		}).build(ctx)?;

		draw(ctx, &mesh, (Point2::new(0.0, 0.0),))?;
		present(ctx)?;
		Ok(())
	}
}

pub fn ggez_loop(w : World) {
	let cb = ggez::ContextBuilder::new("super_simple", "ggez");
	let (ctx, event_loop) = &mut cb.build().unwrap();
	run(ctx, event_loop, &mut WorldWithDebugInfo { world : w, fps : fps_counter::FPSCounter::new() }).unwrap();
}