#![allow(dead_code)]

enum Formula {
	True, False,
	And(Box<Formula>, Box<Formula>), Or(Box<Formula>, Box<Formula>),
}
impl Formula {
	fn resolve(&self) -> bool {
		match self {
			&Formula::True => true,
			&Formula::False => false,
			&Formula::And(ref left, ref right) => left.resolve() && right.resolve(),
			&Formula::Or(ref left, ref right) => left.resolve() || right.resolve(),
		}
	}
}

fn and(left: Formula, right: Formula) -> Formula {
	Formula::And(Box::new(left), Box::new(right))
}
fn or(left: Formula, right: Formula) -> Formula {
	Formula::Or(Box::new(left), Box::new(right))
}

mod formula_should {
	use super::{Formula::{self, True, False}, and, or};

	fn assert_resolve(expected: bool, f: Formula) {
		assert_eq!(expected, f.resolve());
	}

	#[test]
	fn resolve_true_when_true() {
		assert_resolve(true, True);
	}

	#[test]
	fn resolve_true_when_true_and_true() {
		assert_resolve(true, and(True, True));
	}

	#[test]
	fn resolve_false_when_true_and_false() {
		assert_resolve(false, and(True, False));
	}

	#[test]
	fn resolve_false_when_false_or_false() {
		assert_resolve(false, or(False, False));
	}

	#[test]
	fn resolve_true_when_true_or_false() {
		assert_resolve(true, or(True, False));
	}
}

/// # spy
/// Implements a very simple spy which counts application calls.
mod spy {
	use std::cell::RefCell;

	#[derive(Default,Clone,Copy)]
	pub struct Stats {
		hello: u32,
		goodbye: u32,
	}

	pub struct Api {
		stats: RefCell<Stats>,
	}

	impl Api {
		pub fn new() -> Self {
			Api { stats: RefCell::default(), }
		}

		pub fn stats(&self) -> Stats {
			*self.stats.borrow()
		}

		pub fn hello(&self) {
			self.stats.borrow_mut().hello += 1;

		}
		pub fn goodbye(&self) {
			self.stats.borrow_mut().goodbye += 1;
		}
	}

	impl Stats {
		pub fn hello(&self) -> u32 {
			self.hello
		}

		pub fn goodbye(&self) -> u32 {
			self.goodbye
		}
	}

	mod should {

		use super::Api;
		
		fn assert_stats(hello: u32, goodbye: u32, api: Api) {
			let stats = api.stats();
			assert_eq!(hello, stats.hello(), "hello");
			assert_eq!(goodbye, stats.goodbye(), "goodbye");
		}

		#[test]
		fn register_hello_0_goodbye_0_after_init() {
			let api = Api::new();

			assert_stats(0, 0, api);
		}

		#[test]
		fn register_hello_1_goodbye_0_after_hello() {
			let api = Api::new();
			api.hello();

			assert_stats(1, 0, api);
		}

		#[test]
		fn register_hello_0_goodbye_1_after_goodbye() {
			let api = Api::new();
			api.goodbye();

			assert_stats(0, 1, api);
		}

		#[test]
		fn register_hello_1_goodbye_1_after_hello_goodbye() {
			let api = Api::new();
			api.hello();
			api.goodbye();

			assert_stats(1, 1, api);
		}

		#[test]
		fn register_hello_2_goodbye_0_after_hello_hello() {
			let api = Api::new();
			api.hello();
			api.hello();

			assert_stats(2, 0, api);
		}
	}
}

/// Optional
/// Try to implement a Tree structure. A template is provided but it's not working
mod tree {
	use std::rc::{Rc, Weak};
	use std::cell::{Ref, RefCell};
	use std::fmt::{Debug, Formatter, Result as FmtResult};

	pub struct Tree<T>(Rc<RefCell<TreeNode<T>>>);

	struct TreeNode<T> {
		parent: Weak<RefCell<TreeNode<T>>>,
		value: T,
		children: Vec<Rc<RefCell<TreeNode<T>>>>,
	}


	fn collect<T: Copy>(node: Ref<TreeNode<T>>, values: &mut Vec<T>) {
		values.push(node.value);
		for child in node.children.iter() {
			collect(child.borrow(), values);
		}
	}

	impl<T: Copy> Tree<T> {
		pub fn new(value: T) -> Self {
			let parent = Weak::new();
			Tree(Rc::new(RefCell::new(TreeNode { parent, value, children: vec![] })))
		}

		pub fn push(&mut self, value: T) -> Self {
			let parent = Rc::downgrade(&self.0);
			let child = Rc::new(RefCell::new(TreeNode { parent, value, children: vec![] }));
			self.0.borrow_mut().children.push(child.clone());
			Tree(child)
		}

		pub fn parent(&self) -> Option<Self> {
			let parent = &self.0.borrow().parent;
			parent.upgrade().map(|rc| Tree(rc))
		}

		pub fn value(&self) -> T {
			self.0.borrow().value
		}


		pub fn values_from_root(&self) -> Vec<T> {
			let mut values = vec![];
			values.push(self.value());
			let mut current = self.parent();
			while let Some(tree) = current {
				values.push(tree.value());
				current = tree.parent();
			}
			values.reverse();
			values
		}

		pub fn to_vec(&self) -> Vec<T> {
			let mut values = vec![];
			collect(self.0.borrow(), &mut values);
			values
		}
	}
}

mod tree_should {
	use super::tree::Tree;

	#[test]
	fn contains_1() {
		let tree = Tree::new("1");
		assert_eq!(vec!["1"], tree.to_vec());
	}

	#[test]
	fn contains_1_1a() {
		let mut tree = Tree::new("1");
		tree.push("1a");
		assert_eq!(vec!["1", "1a"], tree.to_vec());
	}

	#[test]
	fn contains_1_1a_1b() {
		let mut tree = Tree::new("1");
		tree.push("1a");
		tree.push("1b");
		assert_eq!(vec!["1", "1a", "1b"], tree.to_vec());
	}

	#[test]
	fn contains_1_1a_1a1_1b() {
		let mut tree = Tree::new("1");
		let mut t1a = tree.push("1a");
		t1a.push("1a1");
		tree.push("1b");
		assert_eq!(vec!["1", "1a", "1a1", "1b"], tree.to_vec());
	}

	#[test]
	fn contains_1_1a_1a1_1b_1b1() {
		let mut tree = Tree::new("1");
		let mut t1a = tree.push("1a");
		t1a.push("1a1");
		let mut t1b = tree.push("1b");
		t1b.push("1b1");
		assert_eq!(vec!["1", "1a", "1a1", "1b", "1b1"], tree.to_vec());
	}

	#[test]
	fn have_no_more_parent_after_drop() {
		let mut tree = Tree::new("1");
		let mut t1a = tree.push("1a");
		let t1a1 = t1a.push("1a1");
		let mut t1b = tree.push("1b");
		let t1b1 = t1b.push("1b1");

		assert_eq!(vec!["1", "1a", "1a1"], t1a1.values_from_root());
		assert_eq!(vec!["1", "1b", "1b1"], t1b1.values_from_root());

		drop(tree);

		assert_eq!(vec!["1a", "1a1"], t1a1.values_from_root());
		assert_eq!(vec!["1b", "1b1"], t1b1.values_from_root());
	}
}
