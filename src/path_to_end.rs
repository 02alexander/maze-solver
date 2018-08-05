
use super::maze::Coordinate;
use super::tree::{QuaternaryTree, Node, Link};

/// Finds the path to take from the root of the tree to the end.
/// Returns None if end is unreachable.
pub fn path_to_end(end: Coordinate, tree: &QuaternaryTree<Coordinate>) -> Option<Vec<Coordinate>> {
	let mut path = Vec::new();
	if depth_first(end, &tree.root, &mut path) {
		return Some(path);
	} else {
		return None;
	}
}

/// Returns true if end is found.
fn depth_first(end: Coordinate, node: &Link<Coordinate>, path: &mut Vec<Coordinate>) -> bool {
	if node.is_none() {
		return false;
	}
	let node = node.as_ref().unwrap();
	path.push(node.elem);
	if node.elem == end {
		return true;
	}
	if depth_first(end, &node.left, path)  ||
	   depth_first(end, &node.right, path) ||
	   depth_first(end, &node.up, path)    ||
	   depth_first(end, &node.down, path) {
	   return true;
	}
	path.pop();
	return false;

}


#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn t_depth_first() {
		let end = Coordinate::new(1,2);
		let mut tree = QuaternaryTree::new();
		// W for wall, s for start, P for path e for end 
		// P s P
		// P W P
		// W e P
		tree.root = Some(Box::new(Node::new(Coordinate::new(1,0))));
		tree.root.as_mut().unwrap().left = Some(Box::new(Node::new(Coordinate::new(0,0))));
		tree.root.as_mut().unwrap().left.as_mut().unwrap().down = Some(Box::new(Node::new(Coordinate::new(0,1))));
		tree.root.as_mut().unwrap().right = Some(Box::new(Node::new(Coordinate::new(2,0))));
		tree.root.as_mut().unwrap().right.as_mut().unwrap().down = Some(Box::new(Node::new(Coordinate::new(2,1))));
		tree.root.as_mut().unwrap().right.as_mut().unwrap().down.as_mut().unwrap().down = Some(Box::new(Node::new(Coordinate::new(2,2))));
		tree.root.as_mut().unwrap().right.as_mut().unwrap().down.as_mut().unwrap().down.as_mut().unwrap().left = Some(Box::new(Node::new(Coordinate::new(1,2))));
		let mut path = Vec::new();
		assert_eq!(depth_first(end, &tree.root, &mut path), true);
		assert_eq!(path, vec![
			Coordinate::new(1,0),
			Coordinate::new(2,0),
			Coordinate::new(2,1),
			Coordinate::new(2,2),
			Coordinate::new(1,2)
		]);
		let end = Coordinate::new(0,2);
		assert_eq!(depth_first(end, &tree.root, &mut path), false);

	}
	#[test]
	fn t_path_to_end() {
		let mut tree = QuaternaryTree::new();
		tree.root = Some(Box::new(Node::new(Coordinate::new(1,0))));
		tree.root.as_mut().unwrap().left = Some(Box::new(Node::new(Coordinate::new(0,0))));
		tree.root.as_mut().unwrap().left.as_mut().unwrap().down = Some(Box::new(Node::new(Coordinate::new(0,1))));
		tree.root.as_mut().unwrap().right = Some(Box::new(Node::new(Coordinate::new(2,0))));
		tree.root.as_mut().unwrap().right.as_mut().unwrap().down = Some(Box::new(Node::new(Coordinate::new(2,1))));
		tree.root.as_mut().unwrap().right.as_mut().unwrap().down.as_mut().unwrap().down = Some(Box::new(Node::new(Coordinate::new(2,2))));
		tree.root.as_mut().unwrap().right.as_mut().unwrap().down.as_mut().unwrap().down.as_mut().unwrap().left = Some(Box::new(Node::new(Coordinate::new(1,2))));
		assert_eq!(path_to_end(Coordinate::new(1,2), &tree), Some(vec![
			Coordinate::new(1,0),
			Coordinate::new(2,0),
			Coordinate::new(2,1),
			Coordinate::new(2,2),
			Coordinate::new(1,2)
		]));
		assert_eq!(path_to_end(Coordinate::new(0,2), &tree), None);
	}
}