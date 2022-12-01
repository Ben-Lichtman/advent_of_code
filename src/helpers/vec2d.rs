#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
	x: usize,
	y: usize,
}

impl Coord {
	pub fn new(x: usize, y: usize) -> Self { Self { x, y } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size {
	width: usize,
	height: usize,
}

impl Size {
	pub fn new(width: usize, height: usize) -> Self { Self { width, height } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect {
	x: usize,
	y: usize,
	width: usize,
	height: usize,
}

impl Rect {
	pub fn new(Coord { x, y }: Coord, Size { width, height }: Size) -> Self {
		Self {
			x,
			y,
			width,
			height,
		}
	}

	pub fn from_points(a: Coord, b: Coord) -> Self {
		let x = a.x.min(b.x);
		let y = a.y.min(b.y);
		let width = a.x.max(b.x) - x + 1;
		let height = a.y.max(b.y) - y + 1;
		Self {
			x,
			y,
			width,
			height,
		}
	}

	pub fn contains_point(&self, point: Coord) -> bool {
		let horiz = self.x <= point.x && point.x < self.x + self.width;
		let vert = self.y <= point.y && point.y < self.y + self.height;
		horiz & vert
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2D<T> {
	width: usize,
	data: Vec<T>,
}

impl<T> Vec2D<T> {
	pub fn new(width: usize, data: Vec<T>) -> Self {
		assert!(data.len() % width == 0);
		Self { width, data }
	}

	pub fn size(&self) -> Size {
		let width = self.width;
		let height = self.data.len() / self.width;
		Size { width, height }
	}

	pub fn rect(&self) -> Rect {
		let Size { width, height } = self.size();
		Rect {
			x: 0,
			y: 0,
			width,
			height,
		}
	}

	pub fn iter_rows(&self, rect: Rect) -> impl Iterator<Item = &[T]> {
		let filtered_rows = self.data.chunks(self.width).skip(rect.y).take(rect.height);
		filtered_rows.map(move |row| &row[rect.x..rect.x + rect.width])
	}

	pub fn iter_rows_mut(&mut self, rect: Rect) -> impl Iterator<Item = &mut [T]> {
		let filtered_rows = self
			.data
			.chunks_mut(self.width)
			.skip(rect.y)
			.take(rect.height);
		filtered_rows.map(move |row| &mut row[rect.x..rect.x + rect.width])
	}

	pub fn iter_coords(&self, rect: Rect) -> impl Iterator<Item = (Coord, &T)> {
		self.iter_rows(rect).enumerate().flat_map(|(y, row)| {
			row.iter()
				.enumerate()
				.map(move |(x, item)| (Coord::new(x, y), item))
		})
	}

	pub fn iter_coords_mut(&mut self, rect: Rect) -> impl Iterator<Item = (Coord, &mut T)> {
		self.iter_rows_mut(rect).enumerate().flat_map(|(y, row)| {
			row.iter_mut()
				.enumerate()
				.map(move |(x, item)| (Coord::new(x, y), item))
		})
	}
}

impl<T> Vec2D<T>
where
	T: Clone,
{
	pub fn from_single(size: Size, item: T) -> Self {
		let width = size.width;
		let num = width * size.height;
		let data = vec![item; num];
		Self { width, data }
	}
}
