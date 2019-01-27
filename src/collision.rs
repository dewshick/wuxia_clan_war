use std::ops::*;

pub type Coord = f64;
pub type Dist = f64;
#[derive(Copy, Clone, Debug)] // maybe we can get rid of that later and use reference for points
pub struct Point { pub x : Coord, pub y : Coord }
pub type Coords = Point;
pub type Size = Point;
pub type Direction = Point;
#[derive(Debug)]
pub struct CircleBounds { pub coords : Coords, pub r : Dist }
#[derive(Debug)]
pub struct RectBounds { pub coords : Coords, pub size: Size }
#[derive(Debug)]
pub struct MovingObject { pub bounds : CircleBounds, pub target : Point }

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
		!(self.coords.x + self.r + dist_from_edge < layer.coords.x ||
		self.coords.x - self.r - dist_from_edge > layer.coords.x + layer.size.x ||
		self.coords.y + self.r + dist_from_edge < layer.coords.y ||
		self.coords.y - self.r - dist_from_edge > layer.coords.y  + layer.size.y)
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

	let active_obs = obstacles.filter(|obs| obs.coords.dist(&obj.bounds.coords) < obs.r + obj.bounds.r &&
		(obj.target - obj.bounds.coords).mults(&(obs.coords - obj.bounds.coords)) > -0.1
	);

	let (avoid, count) = active_obs.fold((Direction::init(), 0), |(dir, count), obs| {
		(dir + (obs.coords - obj.bounds.coords).norm().multf(obj.bounds.r + obs.r - obj.bounds.coords.dist(&obs.coords)), count + 1)
	});

	(avoid.norm().ort() - avoid).multf(1.0/ count.max(1) as f64 )
}

pub fn move_to_target<'a, T>(moved : &MovingObject, obstacles : &mut T) -> Point
where T : Iterator<Item=&'a CircleBounds> {
	let avoid_direction = avoid_collision(&moved, obstacles);
	let direction = if avoid_direction.len() < 0.0001 {
		(moved.target - moved.bounds.coords).norm()
	} else {
		avoid_direction
	};

	direction.multf(0.33)
}