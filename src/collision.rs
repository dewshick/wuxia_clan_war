use std::ops::*;

pub type Coord = f32;
pub type Dist = f32;
#[derive(Copy, Clone, Debug)] // maybe we can get rid of that later and use reference for points
pub struct Point { pub x : Coord, pub y : Coord }
pub type Coords = Point;
pub type Size = Point;
pub type Direction = Point;
#[derive(Debug)]
pub struct CircleBounds { pub coords : Coords, pub r : Dist }
#[derive(Debug)]
pub struct RectBounds { pub coords : Coords, pub rect : Size }
#[derive(Debug)]
pub struct MovingObject { pub bounds : CircleBounds, pub target : Point }

impl Point {
	pub fn init() -> Direction { Point { x: 0.0, y : 0.0 } }
	pub fn new(x : Coord, y : Coord) -> Point { Point { x, y } }
	pub fn len(&self) -> Dist { (self.x.powi(2) + self.y.powi(2)).sqrt() }
	pub fn multf(&self, n : f32) -> Direction { Point::new(self.x * n, self.y * n) }
	pub fn mults(&self, p : &Coords) -> Dist { p.x*self.x + p.y * self.y }
	pub fn norm(&self) -> Direction {
		if self.len() <= 0.001 { Point::init() } else { self.multf(1.0 / self.len()) }
	}
	pub fn dist(&self, p : &Coords) -> Dist { (*self - *p).len() }
	pub fn ort(&self) -> Direction { Point { x: self.y, y: -self.x } }
}

impl CircleBounds {
	pub fn on_layer(&self, layer : &RectBounds, dist_from_edge : f32) -> bool {
		!(self.coords.x + self.r + dist_from_edge < layer.coords.x ||
		self.coords.x - self.r - dist_from_edge > layer.coords.x + layer.rect.x ||
		self.coords.y + self.r + dist_from_edge < layer.coords.y ||
		self.coords.y - self.r - dist_from_edge > layer.coords.y  + layer.rect.y)
	}
}

impl Sub for Point {
	type Output = Point;
	fn sub(self, p : Point) -> Point { Point { x : (self.x - p.x), y : (self.y - p.y) } }
}

impl Add for Point {
	type Output = Point;
	fn add(self, p : Point) -> Point { Point { x : self.x + p.x, y : self.y + p.y } }
}

pub fn can_add<'a, T>(obj : &CircleBounds, dist : Dist, obstacles : &mut T) -> bool where T : Iterator<Item=&'a CircleBounds> {
	!(obstacles.any(|obs| obs.coords.dist(&obj.coords) < obs.r + obj.r + dist))
}

fn avoid_collision<'a, T>(obj : &MovingObject, obstacles : &mut T) -> Direction
where T : Iterator<Item=&'a CircleBounds> {
	obstacles.filter(|obs| obs.coords.dist(&obj.bounds.coords) < obs.r + obj.bounds.r &&
		(obj.target - obj.bounds.coords).mults(&(obs.coords - obj.bounds.coords)) > 0.0
	).fold(Direction::init(), |dir, obs| dir + (obs.coords - obj.bounds.coords).norm()).norm().ort()
}

pub fn move_to_target<'a, T>(moved : &MovingObject, obstacles : &mut T) -> MovingObject
where T : Iterator<Item=&'a CircleBounds> {
	let direction = ((moved.target - moved.bounds.coords).norm().multf(0.01) + avoid_collision(&moved, obstacles).multf(0.99)).norm();
	MovingObject {
		bounds: CircleBounds { coords: moved.bounds.coords + direction, r : moved.bounds.r },
		target: moved.target
	}
}