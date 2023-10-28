use core::fmt;
use std::{
	cmp::{max, min, Ordering},
	fmt::{Debug, Formatter},
	ops::{Add, Range},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
	start: usize,
	end: usize,
}

impl Span {
	pub const DUMMY_SPAN: Self = Self { start: 0, end: 0 };

	pub const fn new(start: usize, end: usize) -> Self { Self { start, end } }

	// TODO: use Option<T> later?
	pub const fn is_dummy(&self) -> bool { self.start == 0 && self.end == 0 }
}

impl From<Range<usize>> for Span {
	fn from(Range { start, end }: Range<usize>) -> Self { Self { start, end } }
}

impl From<(usize, usize)> for Span {
	fn from((start, end): (usize, usize)) -> Self { Self { start, end } }
}

impl Debug for Span {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_tuple("Span")
			.field(&(self.start..self.end))
			.finish()
	}
}

impl PartialOrd for Span {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		// usize implements Ord
		Some(match self.start.cmp(&other.start) {
			Ordering::Equal => other.end.cmp(&self.end),
			o => o,
		})
	}
}

// TODO: handle DUMMY_SPAN properly
impl Ord for Span {
	// usize implements Ord
	fn cmp(&self, other: &Self) -> Ordering { self.partial_cmp(other).unwrap() }
}

impl Add for Span {
	type Output = Span;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			start: min(self.start, rhs.start),
			end: max(self.end, rhs.end),
		}
	}
}

#[cfg(test)]
mod tests {
	use pretty_assertions::assert_eq;

	use super::*;

	#[test]
	fn whoever_starts_first_should_be_less() {
		assert!(Span::from(10..20) < Span::from(11..100));
	}

	#[test]
	fn wider_should_be_less() {
		assert!(Span::from(10..100) < Span::from(10..20));
	}

	#[test]
	fn should_be_equal_if_and_only_if_start_and_end_are_same() {
		assert_eq!(Span::from(10..20), Span::from(10..20));
	}

	#[test]
	fn span_addition_should_be_symmetric() {
		assert_eq!(
			Span::from(10..20) + Span::from(30..40),
			Span::from(10..40)
		);

		assert_eq!(
			Span::from(30..40) + Span::from(10..20),
			Span::from(10..40)
		);
	}
}