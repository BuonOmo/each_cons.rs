#![doc(html_playground_url = "https://play.rust-lang.org/")]
#![doc(issue_tracker_base_url = "https://github.com/BuonOmo/each_cons.rs/issues/")]

//! Port of ruby's [`Enumerable#each_cons`](https://rubydoc.info/stdlib/core/Enumerable:each_cons).
//!
//! You can use this crate in two flavors:
//!
//! 1. `iter.each_cons(N)` (See [`ConsIterator`])
//! 2. `each_cons(N, iter)` (See [`each_cons`])
//!
//! Both will have the same behaviour: returning a `Cons` struct that is
//! an [`Iterator`] of `Vec<Rc<Item>>`, where `Vec` size is the given `N`
//! and `Item` correspond to the item of the previous iterator given.

/// Add this into scope to give your iterators the `each_cons(N)` method.
///
/// # Example
///
pub trait ConsGroupExt<T>
where T: Eq {
	fn cons_group(&self) -> ConsGroup<'_, T> ;
}

/// If you don't like `iter.each_cons(N)`, use this.
///
/// # Example
///

#[doc(hidden)]
pub struct ConsGroup<'a, T> {
	remaining: &'a [T]
}

impl<'a, T> ConsGroup<'a, T>
where T: Eq {
	fn new(slice: &'a [T]) -> Self {
		Self {
			remaining: slice
		}
	}
}

impl<T> ConsGroupExt<T> for [T]
where T: Eq {
	fn cons_group(&self) -> ConsGroup<'_, T> {
		ConsGroup::new(self)
	}
}

impl<'a, T> Iterator for ConsGroup<'a, T>
where T: Eq {
	type Item = &'a [T];
	fn next(&mut self) -> Option<Self::Item> {
		let len = self.remaining.len();
		if len == 0 { return None; }
		let val = &self.remaining[0];
		let mut i = 1;
		while i < len - 1 && &self.remaining[i] == val {
			i += 1;
		}
		let slice_to_return = &self.remaining[0..i];
		self.remaining = &self.remaining[i..];
		Some(slice_to_return)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn groups_by_identical_values() {
		let slice = [1, 1, 2, 3, 3, 3, 4, 5];
		let mut cons = slice.cons_group();
		assert!(matches!(cons.next(), Some(&[1, 1])));
		assert!(matches!(cons.next(), Some(&[2])));
		assert!(matches!(cons.next(), Some(&[3, 3, 3])));
		assert!(matches!(cons.next(), Some(&[4])));
		assert!(matches!(cons.next(), Some(&[5])));
		assert!(matches!(cons.next(), None));
		assert!(matches!(cons.next(), None));
	}


}
