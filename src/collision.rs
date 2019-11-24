use std::ops::*;

pub type Coord = f32;
pub type Dist = f32;
pub type Amount = f32;
#[derive(Copy, Clone, Debug)] // maybe we can get rid of that later and use reference for points
pub struct Point { pub x : Coord, pub y : Coord }
pub type Coords = Point;
pub type Size = Point;
pub type Direction = Point;
#[derive(Debug, Clone)]
pub struct CircleBounds { pub coords : Coords, pub r : Dist }
#[derive(Debug)]
pub struct RectBounds { pub coords : Coords, pub size: Size }

const WORLD_SIZE : (usize, usize) = (1024, 768);
const GRID_CELL_SIZE : usize = 30;
const GRID_SIZE : (usize, usize) = (WORLD_SIZE.0 / GRID_CELL_SIZE, WORLD_SIZE.1 / GRID_CELL_SIZE );

pub struct CollisionGrid { objects : [[Box<[i32]>; GRID_SIZE.1] ; GRID_SIZE.0] }

pub trait Traverser<T> {
	fn each(&self, closure : &Fn(&T)) {
		self.reduce(Void{}, &|_ : Void, val| {
			closure(val);
			Void{}
		});
	}
	fn reduce<U>(&self, default : U, closure : &Fn(U, &T) -> U) -> U;
}

struct Void {}

impl<T> Traverser<T> for [T] {
	fn reduce<U>(&self, default: U, closure: &Fn(U, &T) -> U) -> U {
		let mut result = default;
		for i in 0..self.len() {
			result = closure(result, &self[i])
		}
		result
	}
}

struct GridRange {
	xmin : usize, xmax : usize,
	ymin : usize, ymax :usize
}

impl Traverser<(usize, usize)> for GridRange {
	fn reduce<U>(&self, default: U, closure: &Fn(U, &(usize, usize)) -> U) -> U {
		let mut result = default;
		for x in self.xmin..=self.xmax {
			for y in self.ymin..=self.ymax {
				result = closure(result, &(x, y))
			}
		}
		result
	}
}

impl CollisionGrid {
//	fn addObj(&mut self, obj : CircleBounds) {
//		obj.rectBounds.each { |(x,y)|
//
//		}
//	}
//
//	fn rmObj() {
//
//	}
//
//	fn updObj() {
//
//	}
}

impl RectBounds {
	pub fn from_circle(b : CircleBounds) -> RectBounds {
		RectBounds {
			coords : Point { x : b.coords.x - b.r, y : b.coords.y - b.r },
			size : Point { x : 2.0 * b.r, y : 2.0 * b.r }
		}
	}
}

impl Point {
	pub fn init() -> Direction { Point { x: 0.0, y : 0.0 } }
	pub fn new(x : Coord, y : Coord) -> Point { Point { x, y } }
	pub fn len(&self) -> Dist { (self.x.powi(2) + self.y.powi(2)).sqrt() }
	pub fn multf(&self, n : Dist) -> Direction { Point::new(self.x * n, self.y * n) }
	pub fn mults(&self, p : &Coords) -> Dist { p.x*self.x + p.y * self.y }
	pub fn norm(&self) -> Direction {
		if self.len() <= 0.001 { Point::init() } else { self.multf(1.0 / self.len()) }
	}
	pub fn dist(&self, p : &Coords) -> Dist { (*self - *p).len() }
	pub fn ort(&self) -> Direction { Point { x: self.y, y: -self.x } }
}

impl CircleBounds {
	pub fn on_layer(&self, layer : &RectBounds, dist_from_edge : Dist) -> bool {
		let v = [1,2,3];
		!(self.coords.x + self.r + dist_from_edge < layer.coords.x ||
		self.coords.x - self.r - dist_from_edge > layer.coords.x + layer.size.x ||
		self.coords.y + self.r + dist_from_edge < layer.coords.y ||
		self.coords.y - self.r - dist_from_edge > layer.coords.y  + layer.size.y)
	}

	pub fn collides_with(&self, target : &CircleBounds) -> bool {
		target.coords.dist(&self.coords) <= self.r + target.r
	}

//	pub fn rectBounds(&self : CircleBounds) -> GridRange {
//		GridRange {
//			xmin: (obj.coords.x - obj.r) as usize / GRID_CELL_SIZE,
//			xmax: (obj.coords.x + obj.r) as usize / GRID_CELL_SIZE,
//			ymin: (obj.coords.y - obj.r) as usize / GRID_CELL_SIZE,
//			ymax: (obj.coords.y + obj.r) as usize / GRID_CELL_SIZE,
//		}
//	}
}

impl Sub for Point {
	type Output = Point;
	fn sub(self, p : Point) -> Point { Point { x : (self.x - p.x), y : (self.y - p.y) } }
}

impl Add for Point {
	type Output = Point;
	fn add(self, p : Point) -> Point { Point { x : self.x + p.x, y : self.y + p.y } }
}

fn avoid_collision<'a, T>(bounds : &CircleBounds, target : &Point, obstacles : &mut T) -> Direction
where T : Iterator<Item=&'a CircleBounds> {
	let active_obs = obstacles.filter(|obs| obs.coords.dist(&bounds.coords) < obs.r + bounds.r &&
		(*target - bounds.coords).mults(&(obs.coords - bounds.coords)) > -0.1
	);

	let (avoid, count) = active_obs.fold((Direction::init(), 0), |(dir, count), obs| {
		(dir + (obs.coords - bounds.coords).norm().multf(bounds.r + obs.r - bounds.coords.dist(&obs.coords)), count + 1)
	});

	(avoid.norm().ort() - avoid).multf(1.0/ count.max(1) as Dist)
}

pub fn move_to_target<'a, T>(bounds : &CircleBounds, target : &Point, obstacles : &mut T, speed : Dist) -> Point
where T : Iterator<Item=&'a CircleBounds> {
	let avoid_direction = avoid_collision(bounds, target, obstacles);
	let direction = if avoid_direction.len() < 0.1 { (*target - bounds.coords).norm() } else { avoid_direction };

	direction.multf(speed)
}