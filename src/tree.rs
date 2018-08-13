
use std::mem::drop;

pub struct QuaternaryTree<T> {
	pub root: Link<T>,
}

pub type Link<T> = Option<Box<Node<T>>>;

#[derive(PartialEq, Eq, Debug)]
pub struct Node<T> {
	pub elem: T,
	pub left:   Link<T>,
	pub right:  Link<T>,
	pub up: Link<T>,
	pub down: Link<T>
}


impl<T> Node<T> {
	pub fn new(elem: T) -> Self {
		Node { elem:elem, left:None, up:None, right:None, down:None }
	}
}

impl<T> QuaternaryTree<T> {
	pub fn new() -> Self {
		QuaternaryTree { root:None }
	}

}

impl<T> Drop for QuaternaryTree<T> {
	fn drop(&mut self) {
		if self.root.is_none() {
			return;
		}
		let mut next = Some(self.root.take().unwrap());
		let mut stack = Vec::new();
		loop {
			let mut cur = &mut *next.take().unwrap();
			if cur.left.is_some() {
				stack.push(cur.left.take().unwrap());
			}
			if cur.right.is_some() {
				stack.push(cur.right.take().unwrap());
			}
			if cur.up.is_some() {
				stack.push(cur.up.take().unwrap());
			}
			if cur.down.is_some() {
				stack.push(cur.down.take().unwrap());
			}
			if stack.is_empty() {
				break;
			}
			next = stack.pop();
		}
	}
}

#[cfg(test)]
mod test {
}
