use super::colors::*;
use super::collision::*;
use super::std_extended::*;
use super::colors::Color;

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

	let mut wanderers : Vec<MovingObject> = vec![];
	let (wanderer_r, wanderer_dist) = (4.0, 1.0);
	map.iter().find(|layer| layer.tile != Tile::Water).iter().for_each( |layer| {
		for _ in 0..200 {
			let gen_wanderer = || try_n_times(100, &|| {
				gen_circle_bounds(&layer.bounds, &mut trees.iter().chain(wanderers.iter().map(|mo| &mo.bounds)), wanderer_r, wanderer_dist)
			});

			let wanderer = gen_wanderer().and_then(|b| gen_wanderer().map(|dir| MovingObject { bounds: b, target: dir.coords }));
			wanderer.map(|w| wanderers.push(w));
		}
	});

	World { map, trees, wanderers }
}

fn gen_circle_bounds<'a, T>(layer : &RectBounds, existing_bounds : &mut T, r : f32, dist : f32) -> Option<CircleBounds>
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
			let mut obstacles = world.trees.iter().chain(world.wanderers.iter().map(|w| &w.bounds));
			world.wanderers[i] = move_to_target(&world.wanderers[i], &mut obstacles);
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
	let vec2f64 = |vec : [f32; 4]| [vec[0] as f64, vec[1] as f64, vec[2] as f64, vec[3] as f64];

	scene.iter().for_each(|shape| match shape.bounds {
		Bounds::Rect { v : RectBounds { coords, size } } => {
			piston_window::rectangle(shape.color, vec2f64([coords.x, coords.y, size.x, size.y]), t, g);
		},
		Bounds::Circle { v : CircleBounds { coords, r },  } => {
			piston_window::ellipse(shape.color, vec2f64([coords.x - r, coords.y - r, 2.0 * r, 2.0 * r]), t, g);
		},
	})
}