
pub struct TernaryTree<T> {
	pub root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
	pub elem: T,
	pub left:   Link<T>,
	pub middle: Link<T>,
	pub right:  Link<T>,
}

pub struct DepthFirstIter<'a, T: 'a> {
	stack: Vec<&'a Node<T>>,
}

impl<T> Node<T> {
	pub fn new(elem: T) -> Self {
		Node { elem:elem, left:None, middle:None, right:None }
	}
}

impl<T> TernaryTree<T> {
	pub fn new() -> Self {
		TernaryTree { root:None }
	}

	pub fn depth_first(&self) -> DepthFirstIter<T> {
		let mut stack = Vec::new();
		if self.root.is_some() { stack.push(&**self.root.as_ref().unwrap())}
		DepthFirstIter { stack:stack}
	}
}

impl<'a, T> Iterator for DepthFirstIter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		if self.stack.is_empty() {
			return None;
		}
		let node = self.stack.pop().unwrap();
		if node.right.is_some() { self.stack.push(&**node.right.as_ref().unwrap()) }
		if node.middle.is_some() { self.stack.push(&**node.middle.as_ref().unwrap()) }
		if node.left.is_some() { self.stack.push(&**node.left.as_ref().unwrap()) }
		Some(&node.elem)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn depth_first() {
		let mut tree = TernaryTree::new();
		tree.root = Some(Box::new(Node::new(5)));
		tree.root.as_mut().unwrap().left = Some(Box::new(Node::new(7)));
		tree.root.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(Node::new(9)));
		tree.root.as_mut().unwrap().middle = Some(Box::new(Node::new(11)));
		tree.root.as_mut().unwrap().right = Some(Box::new(Node::new(13)));
		let mut it = tree.depth_first();
		assert_eq!(Some(&5), it.next());
		assert_eq!(Some(&7), it.next());
		assert_eq!(Some(&9), it.next());
		assert_eq!(Some(&11), it.next());
		assert_eq!(Some(&13), it.next());
	}
}
