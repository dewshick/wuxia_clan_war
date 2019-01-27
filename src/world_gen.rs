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
pub struct World { map : Map, trees : Vec<CircleBounds>, wanderers : Vec<MovingObject> }

pub fn generate_world(map : Map) -> World {
	let mut trees : Vec<CircleBounds> = vec![];
	let (tree_r_min, tree_r_max, tree_dist) = (8., 18., 10.);

	map.iter().for_each(|layer| {
		if layer.tile == Tile::Forest {
			while let Some(t) = try_n_times(100, &|| {
				gen_circle_bounds(&(layer.bounds), &mut trees.iter(), rng_range(tree_r_min, tree_r_max), tree_dist)
			}) { trees.push(t); }
		} else {
			trees.retain(|tree| !tree.on_layer(&layer.bounds, tree_dist));
		}
	});

	let mut world = World { map, trees, wanderers : vec![] };
	for _ in 0..200 {
		gen_wanderer_bounds(&mut world).and_then(|b| gen_wanderer_bounds(&mut world).map(|dir| {
			MovingObject { bounds: b, target: dir.coords }
		})).map(|w| world.wanderers.push(w));
	};
	world
}

fn gen_wanderer_bounds(w : &mut World) -> Option<CircleBounds> {
	let (wanderer_r, wanderer_dist) = (4.0, 1.0);
		w.map.iter().find(|layer| layer.tile != Tile::Water).and_then(|layer| try_n_times(100, &|| {
			gen_circle_bounds(&layer.bounds, &mut w.trees.iter().chain(w.wanderers.iter().map(|mo| &mo.bounds)), wanderer_r, wanderer_dist)
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
		Tile::Forest => solid_color(ColorTone::LawnGreen),
		Tile::Village => solid_color(ColorTone::PaleGoldenRod),
		Tile::Mine => solid_color(ColorTone::Gainsboro),
		Tile::Water => solid_color(ColorTone::MediumBlue),
	}
}

pub enum Bounds<'a> {
	Rect { v : &'a RectBounds },
	Circle { v : &'a CircleBounds }
}
pub struct RenderedShape<'a> { color : Color, bounds : Bounds<'a> }

pub fn render_world<G>(world : &mut World, t : piston_window::math::Matrix2d, g : &mut G)
	where G : piston_window::Graphics  {
	for _ in 0..15 {
		for i in 0..world.wanderers.len() {
			let wanderer = &world.wanderers[i];
			let mut obstacles = world.trees.iter().chain(world.wanderers.iter().map(|w| &w.bounds));
			if wanderer.target.dist(&wanderer.bounds.coords) < 1.0 {
				gen_wanderer_bounds(world).map(|b| { world.wanderers[i].target = b.coords; });
			} else {
				world.wanderers[i].bounds.coords = wanderer.bounds.coords + move_to_target(wanderer, &mut obstacles);
			}
		}
	}

	let rendered = world.map.iter().map(|layer| {
		RenderedShape { bounds : Bounds::Rect { v : &layer.bounds }, color : tile_color(&layer.tile) }
	}).chain(world.trees.iter().map(|tree| RenderedShape {
		bounds: Bounds::Circle { v: tree }, color: solid_color(ColorTone::Brown)
	})).chain(world.wanderers.iter().map( |w| RenderedShape {
		bounds : Bounds::Circle { v : &w.bounds }, color:  solid_color(ColorTone::Black)
	}));

	render_scene(rendered.collect(), t, g);
}

pub fn render_scene<G>(scene : Vec<RenderedShape>, t : piston_window::math::Matrix2d, g : &mut G)
where G : piston_window::Graphics {
	scene.iter().for_each(|shape| match shape.bounds {
		Bounds::Rect { v : RectBounds { coords, size } } => {
			piston_window::rectangle(shape.color, [coords.x, coords.y, size.x, size.y], t, g);
		},
		Bounds::Circle { v : CircleBounds { coords, r },  } => {
			piston_window::ellipse(shape.color, [coords.x - r, coords.y - r, 2.0 * r, 2.0 * r], t, g);
		},
	})
}

// using unstructured triples representation as in piston
pub fn render_circle(b : CircleBounds) -> Vec<[f64; 2]> {
	use std::f64::consts::PI;
	let center = [b.coords.x, b.coords.y];
	let slice_count = (2.0 * b.r * PI) as i32; // length of circle is 2 * pi * r, 2 pixels per edge ~ 4 * r
	let sector_len = (2.0 * PI) / (slice_count as f64);

	let sector_point = |i| {
		let rad_coord = (i % slice_count) as f64 * sector_len;
		[rad_coord.sin() * b.r, rad_coord.cos() * b.r]
	};
	range(0, slice_count).flat_map(|i| {
		let p1 = sector_point(i);
		let p2 = sector_point(i + 1);
		vec![p1, center, p2,center, p1, p2]
	}).collect()
}

// using unstructured triples representation as in piston
pub fn render_rect(b : RectBounds) -> Vec<[f64; 2]> {
	let (x, y, xs, ys) = (b.coords.x, b.coords.y, b.coords.x + b.size.x, b.coords.y + b.size.y);
	vec![
		[x, y], [xs, y], [x, ys],
		[x, ys], [xs, y], [xs, ys]
	]
}