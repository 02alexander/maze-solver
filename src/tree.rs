
pub struct QuaternaryTree<T> {
	pub root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

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
		Node { elem:elem, left:None, middle:None, right:None }
	}
}

impl<T> QuaternaryTree<T> {
	pub fn new() -> Self {
		QuaternaryTree { root:None }
	}

}

#[cfg(test)]
mod test {
}
