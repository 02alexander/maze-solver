
use super::maze::{Maze, Coordinate};
use super::tree::{QuaternaryTree, Node};
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug)]
enum Direction {
	Up=1,
	Right=2,
	Down=-1,
	Left=-2,
}


pub fn maze_to_tree(maze: &Maze) -> QuaternaryTree<Coordinate> {
	let mut visited_tiles = vec![vec![0;maze.height];maze.width]; // 0 means it's not visited and 1 means it is.
	let mut tree = QuaternaryTree::new();
	tree.root = Some(Box::new(Node::new(maze.start)));
	visited_tiles[maze.start.x][maze.start.y] = 1;
	{ 
		let mut next = Some(tree.root.as_mut().unwrap());
		let mut junctions = VecDeque::new();
		loop {
			let cur_node = &mut**next.take().unwrap();

			let mut cors = Vec::new();
			let mut dirs = Vec::new();
			for _ in 0..4 {
				if let Some(tup) = get_adjacent_unvisited_tile(cur_node.elem, &maze, &visited_tiles) {
					visited_tiles[tup.0.x][tup.0.y] = 1;
					cors.push(tup.0);
					dirs.push(tup.1);
				}
			}

			#[cfg(debug_assertions)] {
				for i in 0..cors.len() {
					println!("({},{}), {:?}", cors[i].x,cors[i].y,dirs[i]);
				}
				println!("");
			}
			
			if let Some(tup) = dirs.iter().enumerate().find(|x| *x.1 == Direction::Left)  {
				let indx = tup.0;
				cur_node.left = Some(Box::new(Node::new(cors[indx])));
				junctions.push_back(cur_node.left.as_mut().unwrap());
			}
			if let Some(tup) = dirs.iter().enumerate().find(|x| *x.1 == Direction::Right) {
				let indx = tup.0;
				cur_node.right = Some(Box::new(Node::new(cors[indx])));
				junctions.push_back(cur_node.right.as_mut().unwrap());
			}
			if let Some(tup) = dirs.iter().enumerate().find(|x| *x.1 == Direction::Up) {
				let indx = tup.0;
				cur_node.up = Some(Box::new(Node::new(cors[indx])));
				junctions.push_back(cur_node.up.as_mut().unwrap());
			}
			if let Some(tup) = dirs.iter().enumerate().find(|x| *x.1 == Direction::Down) {
				let indx = tup.0;
				cur_node.down = Some(Box::new(Node::new(cors[indx])));
				junctions.push_back(cur_node.down.as_mut().unwrap());
			}

			if junctions.is_empty() {
				break;
			}

			next = junctions.pop_front();
		}
	}
	tree
}

/// Gets a adjacent unvisited tile that is not a wall.
/// The direction is relative to "tile".
fn get_adjacent_unvisited_tile(tile: Coordinate, 
							   maze: &Maze,
							   visited_tiles: &Vec<Vec<u8>>) 
							  -> Option<(Coordinate, Direction)> {


	let x = tile.x as i32 - 1;
	let y = tile.y as i32;
	if maze.is_inside(x, y) && maze.grid[x as usize][y as usize]==1 && visited_tiles[x as usize][y as usize]==0 { // left
		return Some((Coordinate::new(x as usize, y as usize), Direction::Left));
	}
	let x = tile.x as i32 + 1;
	let y = tile.y as i32;
	if maze.is_inside(x, y) && maze.grid[x as usize][y as usize]==1 && visited_tiles[x as usize][y as usize]==0 { // right
		return Some((Coordinate::new(x as usize,y as usize), Direction::Right));
	}
	let x = tile.x as i32;
	let y = tile.y as i32 - 1;
	if maze.is_inside(x, y) && maze.grid[x as usize][y as usize]==1 && visited_tiles[x as usize][y as usize]==0 { // up
		return Some((Coordinate::new(x as usize, y as usize), Direction::Up));
	}
	let x = tile.x as i32;
	let y = tile.y as i32 + 1;
	if maze.is_inside(x, y) && maze.grid[x as usize][y as usize]==1 && visited_tiles[x as usize][y as usize]==0 { // down
		return Some((Coordinate::new(x as usize, y as usize), Direction::Down));
	}
	None
}


#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn t_get_tile() { 

		// 0, 0, 1, 0, 1,
		// 1, 1, 1, 0, 1,
		// 1, 0, 0, 0, 1,
		// 1, 1, 0, 1, 1,
		// 0, 1, 1, 1, 0, 
		// 1, 1, 0, 0, 0, 

		let maze = Maze { 
			width:5, 
			height:6, 
			start:Coordinate::new(0,0), 
			end:Coordinate::new(0,0),
			grid:vec![
				vec![0, 1, 1, 1, 0, 1],
				vec![0, 1, 0, 1, 1, 1],
				vec![1, 1, 0, 0, 1, 0],
				vec![0, 0, 0, 1, 1, 0],
				vec![1, 1, 1, 1, 0, 0]
			]
		};

		// 0, 0, 0, 0, 0,
		// 0, 1, 1, 0, 0,
		// 0, 0, 0, 0, 1,
		// 0, 0, 0, 0, 0,
		// 0, 1, 0, 0, 0,
		// 0, 0, 0, 0, 0,

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

	#[test]
	fn t_maze_to_tree() {
		/*
		1 1 0 1 1 1
		0 1 1 1 0 1
		0 0 1 0 0 1 
		1 1 1 1 1 1
		*/
		let grid = vec![
			vec![1, 0, 0, 1],
			vec![1, 1, 0, 1],
			vec![0, 1, 1, 1],
			vec![1, 1, 0, 1],
			vec![1, 0, 0, 1],
			vec![1, 1, 1, 1]
		];
		let maze = Maze {
			width:6,
			height:4,
			start:Coordinate::new(1,0),
			end:Coordinate::new(0,0),
			grid:grid,		
		};
		let tree = maze_to_tree(&maze);
		assert_eq!(tree.root.as_ref().unwrap().elem, Coordinate::new(1,0));		
		assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().elem, Coordinate::new(0,0));
		assert_eq!(tree.root.as_ref().unwrap().right, None);
		assert_eq!(tree.root.as_ref().unwrap().down.as_ref().unwrap().right.as_ref().unwrap().elem, Coordinate::new(2,1));
		assert_eq!(tree.root.as_ref().unwrap().down.as_ref().unwrap().right.as_ref().unwrap().down.as_ref().unwrap().elem, Coordinate::new(2,2));
		assert_eq!(tree.root.as_ref().unwrap().down.as_ref().unwrap().right.as_ref().unwrap().down.as_ref().unwrap().down.as_ref().unwrap().elem, Coordinate::new(2,3));
		assert_eq!(tree.root.as_ref().unwrap().down.as_ref().unwrap().right.as_ref().unwrap().right.as_ref().unwrap().elem, Coordinate::new(3,1));
		assert_eq!(tree.root.as_ref().unwrap().down.as_ref().unwrap().right.as_ref().unwrap().right.as_ref().unwrap().up.as_ref().unwrap().elem, Coordinate::new(3,0));
	}
}