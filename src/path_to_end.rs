
use super::maze::Coordinate;
use super::tree::{QuaternaryTree, Node, Link};

/// Finds the path to take from the root of the tree to the end.
/// Returns None if the end is unreachable.
pub fn path_to_end(end: Coordinate, tree: &QuaternaryTree<Coordinate>) -> Option<Vec<Coordinate>> {
	let mut path = Vec::<&Link<Coordinate>>::new();
	let mut stack = Vec::new();
	let mut next = &tree.root;
	loop {
		path.push(next);
		if next.as_ref().unwrap().elem == end {
			return Some(path.iter().map(|x| x.as_ref().unwrap().elem).collect());
		}
		let cur = next.as_ref().unwrap();
		if cur.left.is_some() {
			stack.push(&cur.left);
		}
		if cur.right.is_some() {
			stack.push(&cur.right);
		}
		if cur.up.is_some() {
			stack.push(&cur.up);
		}
		if cur.down.is_some() {
			stack.push(&cur.down);
		}

		if stack.is_empty() {
			return None;
		}
		while !is_child(path.last().unwrap(), stack.last().unwrap()) {
			path.pop();
		}
		next = stack.pop().unwrap();
	}
}

/// Checks if suspected child is any of the parents children.
fn is_child(parent: &Link<Coordinate>, suspected_child: &Link<Coordinate>) -> bool {
	let parent = parent.as_ref().unwrap();
	&parent.left as *const Link<Coordinate> == suspected_child as *const Link<Coordinate> || 
	&parent.right as *const Link<Coordinate> == suspected_child as *const Link<Coordinate> || 
	&parent.up as *const Link<Coordinate> == suspected_child as *const Link<Coordinate> || 
	&parent.down as *const Link<Coordinate> == suspected_child as *const Link<Coordinate>
}

#[cfg(test)]
mod test {
	use super::*;
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