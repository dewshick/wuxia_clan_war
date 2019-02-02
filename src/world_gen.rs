use super::colors::*;
use super::collision::*;
use super::std_extended::*;
use super::colors::Color;
use super::range;

//use piston_window::{rectangle, math::Matrix2d, Graphics, types::Color};
#[derive(PartialEq, Eq, Debug)]
pub enum Tile { Forest, Village, Mine, Water }

#[derive(Debug)]
pub struct RectTile { pub tile : Tile, pub bounds : RectBounds }

type Map = Vec<RectTile>;

#[derive(Debug)]
struct GameObject {
	pub bounds : CircleBounds,
	pub color : ColorTone,
	pub durability : Amount,
	pub speed : Dist,
	pub tasks : Vec<Task>
}

impl GameObject {
	fn tree(bounds : CircleBounds) -> GameObject {
		GameObject { bounds, color : ColorTone::Brown, durability : 100.0, speed : -1.0, tasks : vec![] }
	}

	fn wanderer(bounds : CircleBounds, target : Point) -> GameObject {
		GameObject { bounds, color : ColorTone::Black, durability : 20.0, speed : 0.33, tasks : vec![Task::MovingTo(target)] }
	}
}

#[derive(Debug)]
enum Task {
	MovingTo(Point)
}

#[derive(Debug)]
pub struct World { map : Map, objects : Vec<GameObject> }

pub fn generate_world(map : Map) -> World {
	let mut game_objects : Vec<GameObject> = vec![];
	let (tree_r_min, tree_r_max, tree_dist) = (8., 18., 10.);

	map.iter().for_each(|layer| {
		if layer.tile == Tile::Forest {
			while let Some(t) = try_n_times(100, &|| {
				gen_circle_bounds(&(layer.bounds), &mut game_objects.iter().map (|obj| &obj.bounds), rng_range(tree_r_min, tree_r_max), tree_dist)
			}) { game_objects.push(GameObject::tree(t)); }
		} else {
			game_objects.retain(|obj| !obj.bounds.on_layer(&layer.bounds, tree_dist));
		}
	});

	let mut world = World { map, objects: game_objects };
	for _ in 0..200 {
		gen_wanderer_bounds(&mut world).and_then(|b| gen_wanderer_bounds(&mut world).map(|dir| world.objects.push(GameObject::wanderer(b, dir.coords))));
	};
	world
}

fn gen_wanderer_bounds(w : &mut World) -> Option<CircleBounds> {
	let (wanderer_r, wanderer_dist) = (4.0, 1.0);
		w.map.iter().find(|layer| layer.tile != Tile::Water).and_then(|layer| try_n_times(100, &|| {
			gen_circle_bounds(&layer.bounds, &mut w.objects.iter().map(|o| &o.bounds), wanderer_r, wanderer_dist)
		}))
}

fn gen_circle_bounds<'a, T>(layer : &RectBounds, existing_bounds : &mut T, r : Dist, dist : Dist) -> Option<CircleBounds>
	where T : Iterator<Item = &'a CircleBounds> {
//	const tree params
		let bounds = CircleBounds {
			coords: Point::new(
				rng_range(r + layer.coords.x + dist, layer.coords.x + layer.size.x - r - dist),
				rng_range(r + layer.coords.y + dist, layer.coords.y + layer.size.y - r - dist)
			),
			r
		};
		if can_add(&bounds, dist, existing_bounds) { Some(bounds) } else { None }
}

fn tile_color(t : &Tile) -> piston_window::types::Color {
	match t {
		Tile::Forest => solid_color(&ColorTone::LawnGreen),
		Tile::Village => solid_color(&ColorTone::PaleGoldenRod),
		Tile::Mine => solid_color(&ColorTone::Gainsboro),
		Tile::Water => solid_color(&ColorTone::MediumBlue),
	}
}

pub enum Bounds<'a> {
	Rect { v : &'a RectBounds },
	Circle { v : &'a CircleBounds }
}
pub struct RenderedShape<'a> { color : Color, bounds : Bounds<'a> }

pub fn render_world<G>(world : &mut World, t : piston_window::math::Matrix2d, g : &mut G)
	where G : piston_window::Graphics  {
	for _ in 0..5 {
		for i in 0..world.objects.len() {
			let wanderer = &world.objects[i];
			if let Some(Task::MovingTo(target)) = wanderer.tasks.first() {
				let mut obstacles = world.objects.iter().map(|w| &w.bounds);
				if target.dist(&wanderer.bounds.coords) < 1.0 {
					gen_wanderer_bounds(world).map(|b| { world.objects[i].tasks = vec![Task::MovingTo(b.coords)]; });
				} else {
					world.objects[i].bounds.coords = wanderer.bounds.coords +
						move_to_target(&wanderer.bounds, target, &mut obstacles, wanderer.speed);
				}
			}
		}
	}

	let rendered = world.map.iter().map(|layer| {
		RenderedShape { bounds : Bounds::Rect { v : &layer.bounds }, color : tile_color(&layer.tile) }
	}).chain(world.objects.iter().map(|obj| RenderedShape {
		bounds: Bounds::Circle { v: &obj.bounds },
		color: solid_color(&obj.color)
	}));

	render_scene(rendered.collect(), t, g);
}

pub fn render_scene<G>(scene : Vec<RenderedShape>, t : piston_window::math::Matrix2d, g : &mut G)
where G : piston_window::Graphics {
	scene.iter().for_each(|shape| g.tri_list(&Default::default(), &shape.color, |f| match shape.bounds {
		Bounds::Rect { v } => f(&(rect_tri(v, t)[..])),
		Bounds::Circle { v } => f(&(circle_tri(v, t)[..])),
	}));
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