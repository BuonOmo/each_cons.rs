use std::{iter::Peekable, rc::Rc};

pub trait ConsExt: Sized + Iterator {
	 fn each_cons(self) -> Cons<Self>;
}

pub fn each_cons<I: Iterator>(iter: I) -> Cons<I> {
	Cons::new(iter)
}

pub struct Cons<I: Iterator> {
	prev: Rc<I::Item>,
	iter: Peekable<I>,
}

impl<I: Iterator> ConsExt for I {
	fn each_cons(self) -> Cons<Self> {
		Cons::new(self)
	}
}

impl<I: Iterator> Cons<I> {
	pub fn new(iter: I) -> Self {
		let mut peekable = iter.peekable();
		let prev = if let Some(n) = peekable.next() {
			Rc::new(n)
		} else {
			todo!("There may be a better solution for that");
		};
		Self { iter: peekable , prev }
	}
}

impl<I: Iterator> Iterator for Cons<I> {
	type Item = (Rc<I::Item>, Rc<I::Item>);

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(a) = self.iter.next() {
			let x = Rc::clone(&self.prev);
			let y = Rc::new(a);
			self.prev = Rc::clone(&y);
			Some((x, y))
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{each_cons, ConsExt};

	#[test]
	fn each_cons_function() {
		let v = vec![1, 2, 3];

		let mut i = each_cons(v.iter());

		let (a, b) = i.next().unwrap();
		assert_eq!((&1, &2), (*a, *b));
		let (a, b) = i.next().unwrap();
		assert_eq!((&2, &3), (*a, *b));
		assert!(i.next().is_none());
	}

	#[test]
	fn each_cons_iterator_method() {
		let v = vec![1, 2, 3];

		let mut i = v.iter().each_cons();

		let (a, b) = i.next().unwrap();
		assert_eq!((&1, &2), (*a, *b));
		let (a, b) = i.next().unwrap();
		assert_eq!((&2, &3), (*a, *b));
		assert!(i.next().is_none());
	}

	#[test]
	fn non_copy_object() {
		let v = vec!["foo", "bar", "baz"];

		let mut i = v.iter().each_cons();

		let (a, b) = i.next().unwrap();
		assert_eq!((&"foo", &"bar"), (*a, *b));
		let (a, b) = i.next().unwrap();
		assert_eq!((&"bar", &"baz"), (*a, *b));
		assert!(i.next().is_none());
	}
}
