extern crate imagefmt;

use imagefmt::{read, write, ColFmt, ColType};
use maze_to_tree::maze_to_tree;
use maze::{Coordinate, Maze};
use path_to_end::path_to_end;

pub mod tree;
pub mod maze;
pub mod maze_to_tree;
pub mod path_to_end;


fn main() {
	let image = imagefmt::read("maze.bmp", ColFmt::RGB).unwrap();
	let maze = Maze::from_image(image.w, image.h, &image.buf);
	let tree = maze_to_tree(&maze);
	let path = path_to_end(maze.end, &tree).unwrap();
	let pixels = to_pixels(&path, &maze);
	imagefmt::write("out.bmp", maze.width, maze.height, ColFmt::RGB, &pixels, ColType::Auto);
	
}


/// Creates an image from a maze where the solution is green,  the start is blue, the end is red, the walls are black and the path is white.
fn to_pixels(path: &Vec<Coordinate>, maze: &Maze) -> Vec<u8> {
	let mut pixels = maze.as_image();
	for cor in path {
		if *cor != maze.end && *cor != maze.start {
			let pixel = &mut pixels[maze.width*cor.y*3+cor.x*3..maze.width*cor.y*3+cor.x*3+3];
			pixel[0] = 0;
			pixel[1] = 255;
			pixel[2] = 0;
		}
	}
	pixels
}