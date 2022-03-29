use std::{collections::VecDeque, rc::Rc};

pub trait ConsExt: Sized + Iterator {
	 fn each_cons(self, slice: usize) -> Cons<Self>;
}

pub fn each_cons<I: Iterator>(slice: usize, iter: I) -> Cons<I> {
	Cons::new(iter, slice)
}

pub struct Cons<I: Iterator> {
	slice: usize,
	prevs: Option<VecDeque<Rc<I::Item>>>,
	iter: I,
}

impl<I: Iterator> ConsExt for I {
	fn each_cons(self, slice: usize) -> Cons<Self> {
		Cons::new(self, slice)
	}
}

impl<I: Iterator> Cons<I> {
	pub fn new(mut iter: I, slice: usize) -> Self {
		let mut expected_prevs: VecDeque<Rc<I::Item>> = VecDeque::with_capacity(slice - 1);
		let mut finished = true;
		for _ in 0..(slice - 1) {
			if let Some(val) = iter.next() {
				expected_prevs.push_back(Rc::new(val));
			} else {
				finished = false;
				break
			}
		}
		let prevs = if finished { Some(expected_prevs) } else { None };
		Self { iter , prevs, slice }
	}
}

impl<I: Iterator> Iterator for Cons<I> {
	type Item = Vec<Rc<I::Item>>;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(prevs) = &self.prevs {
			if let Some(a) = self.iter.next() {
				let new_rc = Rc::new(a);
				let mut rv: Vec<Rc<I::Item>> = Vec::with_capacity(self.slice);
				let mut curr = VecDeque::with_capacity(self.slice - 1);
				let mut first = true;
				for v in prevs {
					if first {
						first = false
					} else {
						curr.push_back(Rc::clone(&v));
					}
					rv.push(Rc::clone(&v));
				}
				rv.push(Rc::clone(&new_rc));
				curr.push_back(new_rc);
				self.prevs = Some(curr);
				Some(rv)
			} else {
				None
			}
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

		let mut i = each_cons(2, v.iter());

		let s = i.next().unwrap();
		assert_eq!(*s[0], &1);
		assert_eq!(*s[1], &2);
		let s = i.next().unwrap();
		assert_eq!(*s[0], &2);
		assert_eq!(*s[1], &3);
		assert!(i.next().is_none());
	}

	#[test]
	fn each_cons_iterator_method() {
		let v = vec![1, 2, 3];

		let mut i = v.iter().each_cons(2);

		let s = i.next().unwrap();
		assert_eq!(*s[0], &1);
		assert_eq!(*s[1], &2);
		let s = i.next().unwrap();
		assert_eq!(*s[0], &2);
		assert_eq!(*s[1], &3);
		assert!(i.next().is_none());
	}

	#[test]
	fn non_copy_object() {
		let v = vec!["foo", "bar", "baz"];

		let mut i = v.iter().each_cons(2);

		let s = i.next().unwrap();
		assert_eq!(*s[0], &"foo");
		assert_eq!(*s[1], &"bar");
		let s = i.next().unwrap();
		assert_eq!(*s[0], &"bar");
		assert_eq!(*s[1], &"baz");
		assert!(i.next().is_none());
	}

	#[test]
	fn three_cons() {
		let v = vec!["foo", "bar", "baz"];

		let mut i = v.iter().each_cons(3);

		let s = i.next().unwrap();
		assert_eq!(*s[0], &"foo");
		assert_eq!(*s[1], &"bar");
		assert_eq!(*s[2], &"baz");
		assert!(i.next().is_none());
	}

	#[test]
	fn more_cons_than_possible() {
		let v = vec!["foo"];

		let mut i = v.iter().each_cons(2);

		assert!(i.next().is_none());
	}
}
