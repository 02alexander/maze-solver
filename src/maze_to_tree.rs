
use super::maze::{Maze, Coordinate};
use super::tree::TernaryTree;

#[derive(PartialEq, Eq, Debug)]
enum Direction {
	Up=1,
	Right=2,
	Down=-1,
	Left=-2,
}

#[derive(PartialEq, Eq, Debug)]
enum TreePath {
	Left,
	Right,
	Middle
}

pub fn maze_to_tree(maze: &Maze) -> TernaryTree<Coordinate> {
	let mut visited_tiles = vec![vec![0;maze.height];maze.width]; // 0 means it's not visited and 1 means it is.
	
	
	
	unimplemented!()
}

/// Gets a adjacent unvisited tile that is not a wall.
/// The direction is relative to "tile".
fn get_adjacent_unvisited_tile(tile: Coordinate, 
							   maze: &Maze,
							   visited_tiles: &Vec<Vec<u8>>) 
							  -> Option<(Coordinate, Direction)> {

	let x = tile.x as i32 - 1;
	let y = tile.y as i32;
	if maze.is_inside(x, y) && maze.grid[x as usize][y as usize]==0 && visited_tiles[x as usize][y as usize]==0 { // left
		return Some((Coordinate::new(x as usize, y as usize), Direction::Left));
	}
	let x = tile.x as i32 + 1;
	let y = tile.y as i32;
	if maze.is_inside(x, y) && maze.grid[x as usize][y as usize]==0 && visited_tiles[x as usize][y as usize]==0 { // right
		return Some((Coordinate::new(x as usize,y as usize), Direction::Right));
	}
	let x = tile.x as i32;
	let y = tile.y as i32 - 1;
	if maze.is_inside(x, y) && maze.grid[x as usize][y as usize]==0 && visited_tiles[x as usize][y as usize]==0 { // up
		return Some((Coordinate::new(x as usize, y as usize), Direction::Up));
	}
	let x = tile.x as i32;
	let y = tile.y as i32 + 1;
	if maze.is_inside(x, y) && maze.grid[x as usize][y as usize]==0 && visited_tiles[x as usize][y as usize]==0 { // down
		return Some((Coordinate::new(x as usize, y as usize), Direction::Down));
	}
	None
}

fn direction_to_path(step1: Coordinate, step2: Coordinate, direction: Direction) -> TreePath {
	use self::Direction::*;
	let dir = direction_from(step1, step2);
	if dir==direction { return TreePath::Middle }
	else if dir==Down && direction==Left { return TreePath::Right }
	else if dir==Down && direction==Right { return TreePath::Left }
	else if dir==Up && direction==Left { return TreePath::Left }
	else if dir==Up && direction==Right {return TreePath::Right }
	else if dir==Left && direction==Up { return TreePath::Right }
	else if dir==Left && direction==Down { return TreePath::Left }
	else if dir==Right && direction==Up { return TreePath::Left }
	else { return TreePath::Right }
}


/// Returns direction needed to go from a to b
fn direction_from(a: Coordinate, b: Coordinate) -> Direction {
	if a.x > b.x {
		Direction::Left
	} else if a.x < b.x {
		Direction::Right
	}  else if a.y < b.y {
		Direction::Down
	} else {
		Direction::Up
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn dir_from() {
		assert_eq!(Direction::Down, direction_from(Coordinate::new(0,0), Coordinate::new(0,1)));
		assert_eq!(Direction::Up, direction_from(Coordinate::new(1,1), Coordinate::new(1,0)));
		assert_eq!(Direction::Left, direction_from(Coordinate::new(1,1), Coordinate::new(0,1)));
		assert_eq!(Direction::Right, direction_from(Coordinate::new(1,1), Coordinate::new(2,1)));
	}

	#[test]
	fn dir_to_path() {
		assert_eq!(TreePath::Middle, direction_to_path(Coordinate::new(0,0), Coordinate::new(1,0), Direction::Right));
		assert_eq!(TreePath::Left, direction_to_path(Coordinate::new(1,0), Coordinate::new(0,0), Direction::Down));
		assert_eq!(TreePath::Left, direction_to_path(Coordinate::new(1,1), Coordinate::new(1,0), Direction::Left));
		assert_eq!(TreePath::Right, direction_to_path(Coordinate::new(1,1), Coordinate::new(2,1), Direction::Down));
	}

	#[test]
	fn get_tile() {
/*
		1, 1, 0, 1, 0,
		0, 0, 0, 1, 0,
		0, 1, 1, 1, 0,
		0, 0, 1, 0, 0,
		1, 0, 0, 0, 1, 
		0, 0, 1, 1, 1, 
*/
		let maze = Maze { 
			width:5, 
			height:6, 
			start:Coordinate::new(0,0), 
			end:Coordinate::new(0,0),
			grid:vec![
				vec![1, 0, 0, 0, 1, 0],
				vec![1, 0, 1, 0, 0, 0],
				vec![0, 0, 1, 1, 0, 1],
				vec![1, 1, 1, 0, 0, 1],
				vec![0, 0, 0, 0, 1, 1]
			]
		};
/*
		0, 0, 0, 0, 0,
		0, 1, 1, 0, 0,
		0, 0, 0, 0, 1,
		0, 0, 0, 0, 0,
		0, 1, 0, 0, 0,
		0, 0, 0, 0, 0,
*/
		let visited_tiles = vec![
			vec![0, 0, 0, 0, 0, 0],
			vec![0, 1, 0, 0, 1, 0],
			vec![1, 1, 0, 0, 1, 0],
			vec![0, 0, 0, 0, 0, 0],
			vec![0, 0, 1, 0, 0, 0]
		];

		assert_eq!(
			Some((Coordinate::new(0,2), Direction::Down)), 
			get_adjacent_unvisited_tile(Coordinate::new(0, 1),
										&maze,
										&visited_tiles)
		);
		assert_eq!(Some((Coordinate::new(4,0), Direction::Up)), 
			get_adjacent_unvisited_tile(Coordinate::new(4,1),
										&maze,
										&visited_tiles)
		);
		assert_eq!(Some((Coordinate::new(0,5), Direction::Left)), 
			get_adjacent_unvisited_tile(Coordinate::new(1,5),
										&maze,
										&visited_tiles)
		);
		assert_eq!(
			Some((Coordinate::new(3, 4), Direction::Right)), 
			get_adjacent_unvisited_tile(Coordinate::new(2, 4),
										&maze,
										&visited_tiles)
		);
		assert_eq!(
			None, 
			get_adjacent_unvisited_tile(Coordinate::new(2,0),
										&maze,
										&visited_tiles)
		);
	}
}