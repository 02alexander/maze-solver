
pub struct Maze {
	pub width: usize,
	pub height: usize,
	pub grid: Vec<Vec<u8>>, // walls have the value of 0 and path 1.
	pub start: Coordinate,
	pub end: Coordinate,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Coordinate {
	pub x: usize,
	pub y: usize,
}

impl Maze {
	pub fn from_image(width: usize, height: usize, pixels: &Vec<u8>) -> Self {
		let start = find_start(width, &pixels).unwrap_or(Coordinate::new(0,0));
		let end = find_end(width, &pixels).unwrap_or(Coordinate::new(width-1, height-1));

		let mut grid = vec![Vec::with_capacity(height); width];
		for j in 0..width {
			for i in 0..height {
				grid[j].push({
					let cur_pixel = &pixels[i*width*3+j*3..i*width*3+j*3+3]; 
					if cur_pixel[0] == 0 && cur_pixel[1] == 0 && cur_pixel[2] == 0 {
						0
					} else  {
						1
					}
				});
			}
		}
		
		Maze {
			width: width,
			height: height,
			grid: grid,
			start: start,
			end: end,
		}
	}

	pub fn as_image(&self) -> Vec<u8> {
		let mut image = Vec::with_capacity(self.width*self.height*3);
		for y in 0..self.height {
			for x in 0..self.width {
				if self.start.x == x && self.start.y == y {
					// red
					image.push(255); image.push(0); image.push(0);
				} else if self.end.x == x && self.end.y == y {
					// blue
					image.push(0); image.push(0); image.push(255);
				} else if self.grid[x][y] == 0 {
					// black
					image.push(0); image.push(0); image.push(0);
				} else {
					// white
					image.push(255); image.push(255); image.push(255); 
				}
			}
		}
		image
	}

	///Returns true if the x, y coordinates are inside the maze. 
	pub fn is_inside(&self, x: i32, y: i32) -> bool {
		x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
	}
}

fn find_start(width: usize, pixels: &Vec<u8>) -> Option<Coordinate> {
	for i in (0..(pixels.len()/3)).map(|x|x*3) {
		if pixels[i] == 255 && pixels[i+1] == 0 && pixels[i+2] == 0 {
			return Some(Coordinate::new( (i / 3) % width, (i/3)/width)); 
		};
	}
	None
}

fn find_end(width: usize, pixels: &Vec<u8>) -> Option<Coordinate> {
	for i in (0..(pixels.len()/3)).map(|x|x*3) {
		if pixels[i] == 0 && pixels[i+1] == 0 && pixels[i+2] == 255 {
			return Some(Coordinate::new( (i / 3) % width, (i/3)/width)); 
		};
	}
	None
}

impl Coordinate {
	pub fn new(x: usize, y: usize) -> Self {
		Coordinate { x:x, y:y }
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_find_start() {
		let pixels = vec![ 
			0, 0, 0, 0, 0, 0, 0, 0, 255,
			0, 0, 0, 0, 0, 0, 0, 0, 0,
			255, 0, 0, 0, 0, 0, 0, 0, 0, 
		];
		assert_eq!(Some(Coordinate::new(0,2)), find_start(3, &pixels));

		let pixels = vec![ 
			0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 
		];
		assert_eq!(None, find_start(3,&pixels));
	}
	#[test]
	fn test_find_end() {
		let pixels = vec![ 
			0, 0, 0, 0, 0, 0, 0, 0, 255,
			0, 0, 0, 0, 0, 0, 0, 0, 0,
			255, 0, 0, 0, 0, 0, 0, 0, 0, 
		];
		assert_eq!(Some(Coordinate::new(2,0)), find_end(3, &pixels));

		let pixels = vec![ 
			0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 
		];
		assert_eq!(None, find_end(3,&pixels));
	}

	#[test]
	fn from() {
		let pixels = vec![ 
			0, 0, 0, 0, 0, 0, 0, 0, 255,
			255, 255, 255, 255, 255, 255, 0, 0, 0,
			255, 0, 0, 0, 0, 0, 0, 0, 0,
			255, 255, 255, 0, 0, 0, 255, 255, 255,
		];
		let expected_grid = vec![
			vec![0, 1, 1, 1],
			vec![0, 1, 0, 0],
			vec![1, 0, 0, 1],
		];

		let maze = Maze::from_image(3, 4, &pixels);

		assert_eq!(&expected_grid, &maze.grid);
		assert_eq!(4, maze.height);
		assert_eq!(3, maze.width);
		assert_eq!(Coordinate::new(0,2), maze.start);
		assert_eq!(Coordinate::new(2,0), maze.end);

		let pixels = vec![
			0, 0, 0, 0, 0, 0, 
			0, 0, 0, 0, 0, 0,
		];
		let maze = Maze::from_image(2,2,&pixels);

		assert_eq!(Coordinate::new(0,0), maze.start);
		assert_eq!(Coordinate::new(1,1), maze.end);
	}
}