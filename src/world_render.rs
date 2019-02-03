use super::world_gen::*;
use super::colors::*;
use crate::collision::*;
use num_iter::range;
use itertools::Itertools;
use ggez::graphics::MeshBuilder;
use ggez::graphics::DrawMode;
use ggez::graphics::Rect;
use ggez::nalgebra::Point2;
use ggez::event;
use ggez::Context;
use ggez::error::GameResult;
use ggez::graphics::{clear, draw, present};
use ggez::graphics::Mesh;
use piston_window::*;
use fps_counter::FPSCounter;
use ggez::event::EventHandler;
use ggez::error::GameError;

pub struct RenderedShape<'a> { color : Color, bounds : Bounds<'a> }

pub enum Bounds<'a> {
	Rect { v : &'a RectBounds },
	Circle { v : &'a CircleBounds }
}

impl World {
	pub fn to_scene(&mut self) -> Vec<RenderedShape> {
		for _ in 0..5 {
			for i in 0..self.objects.len() {
				let wanderer = &self.objects[i];
				if let Some(Task::MovingTo(target)) = wanderer.tasks.first() {
					let mut obstacles = self.objects.iter().map(|w| &w.bounds);
					if target.dist(&wanderer.bounds.coords) < 1.0 {
						gen_wanderer_bounds(self).map(|b| { self.objects[i].tasks = vec![Task::MovingTo(b.coords)]; });
					} else {
						self.objects[i].bounds.coords = wanderer.bounds.coords +
							move_to_target(&wanderer.bounds, target, &mut obstacles, wanderer.speed);
					}
				}
			}
		}

		let rendered = self.map.iter().map(|layer| {
			RenderedShape { bounds : Bounds::Rect { v : &layer.bounds }, color : tile_color(&layer.tile) }
		}).chain(self.objects.iter().map(|obj| RenderedShape {
			bounds: Bounds::Circle { v: &obj.bounds },
			color: solid_color(&obj.color)
		}));

		rendered.collect()
//	render_scene(rendered.collect(), t, g);
	}
}

struct WorldWithDebugInfo { world : World, fps : FPSCounter }

impl event::EventHandler for WorldWithDebugInfo {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		self.world.update(ctx)
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		println!("{}", self.fps.tick());
		self.world.draw(ctx)
	}
}

// ggez-related rendering
impl event::EventHandler for World {
	fn update(&mut self, _ctx: &mut Context) -> GameResult {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		clear(ctx, [0.0, 0.0, 0.0, 0.0].into());
		let mesh: Mesh = self.to_scene().iter().fold(&mut MeshBuilder::new(), |mb, shape| match shape.bounds {
			Bounds::Rect { v : RectBounds { coords, size } } => mb.rectangle(DrawMode::Fill, Rect::new(coords.x.clone() as f32, coords.y.clone() as f32, size.x.clone() as f32, size.y.clone() as f32), shape.color.into()),
			Bounds::Circle { v : CircleBounds { coords, r } } => mb.circle(DrawMode::Fill, Point2::new(coords.x.clone() as f32, coords.y.clone() as f32), r.clone() as f32, 0.5, shape.color.into()),
		}).build(ctx)?;

		draw(ctx, &mesh, (Point2::new(0.0, 0.0),))?;
		present(ctx)?;
		Ok(())
	}
}

//static mut FPS : fps_counter::FPSCounter = fps_counter::FPSCounter::new();

pub fn ggez_loop(w : World) {
	let cb = ggez::ContextBuilder::new("super_simple", "ggez");
	let (ctx, event_loop) = &mut cb.build().unwrap();
	event::run(ctx, event_loop, &mut WorldWithDebugInfo { world : w, fps : fps_counter::FPSCounter::new() });
}



// piston-related rendering
pub fn render_scene<G>(scene : Vec<RenderedShape>, t : piston_window::math::Matrix2d, g : &mut G)
	where G : piston_window::Graphics {
	scene.iter().group_by(|shape| &shape.color).into_iter().for_each(|(color, items)| {
		let vertices : Vec<[f32;2]> = items.into_iter().flat_map(|shape| match shape.bounds {
			Bounds::Rect { v } => rect_tri(v, t),
			Bounds::Circle { v } => circle_tri(v, t),
		}).collect();

		g.tri_list(&Default::default(), color, |f| f(&vertices[..]))
	});
}

// using unstructured triples representation as in piston
pub fn circle_tri(b : &CircleBounds, t : piston_window::math::Matrix2d) -> Vec<[f32; 2]> {
	use std::f64::consts::PI;
	let center = [b.coords.x as Dist, b.coords.y as Dist];
	let slice_count = (2.0 * (b.r as Dist) * PI) as i32; // length of circle is 2 * pi * r, 2 pixels per edge ~ 4 * r
	let sector_len = (2.0 * PI) / (slice_count as Dist /*as f64*/);

	let sector_point = |i| {
		let rad_coord = (i % slice_count) as Dist * sector_len;
		[rad_coord.sin() * b.r + b.coords.x as Dist, rad_coord.cos() * b.r + b.coords.y as Dist]
	};
	range(0, slice_count).flat_map(|i| {
		let p1 = sector_point(i);
		let p2 = sector_point(i + 1);
		vec![txy(t,p1), txy(t, center), txy(t,p2), txy(t,center), txy(t,p1), txy(t,p2)]
	}).collect()
}

// using unstructured triples representation as in piston
pub fn rect_tri(b : &RectBounds, t : piston_window::math::Matrix2d) -> Vec<[f32; 2]> {
	let (x, y, xs, ys) = (b.coords.x as Dist, b.coords.y as Dist, (b.coords.x + b.size.x) as Dist, (b.coords.y + b.size.y) as Dist);
	vec![
		txy(t,[x, y]), txy(t,[xs, y]), txy(t,[x, ys]),
		txy(t, [x, ys]), txy(t,[xs, y]), txy(t,[xs, ys])
	]
}

/// Transformed x coordinate as f32.
#[inline(always)]
fn txy(m: piston_window::math::Matrix2d, xy : [f64;2]) -> [f32;2] {
	[
		(m[0][0] * xy[0] + m[0][1] * xy[1] + m[0][2]) as f32,
		(m[1][0] * xy[0] + m[1][1] * xy[1] + m[1][2]) as f32
	]
}

fn tile_color(t : &Tile) -> piston_window::types::Color {
	match t {
		Tile::Forest => solid_color(&ColorTone::LawnGreen),
		Tile::Village => solid_color(&ColorTone::PaleGoldenRod),
		Tile::Mine => solid_color(&ColorTone::Gainsboro),
		Tile::Water => solid_color(&ColorTone::MediumBlue),
	}
}

fn _piston_loop(world : &mut World) {
	let mut window: PistonWindow =
		WindowSettings::new("Hello Piston!", [640, 480]).exit_on_esc(true).build().unwrap();

	let mut fps = fps_counter::FPSCounter::new();
	while let Some(event) = window.next() {
		window.draw_2d(&event, |context, graphics| {
			render_scene(world.to_scene(), context.transform, graphics);
		});
		println!("{}", fps.tick());
	}
}