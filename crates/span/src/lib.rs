use core::fmt;
use std::{
	cmp::{max, min, Ordering},
	error::Error,
	fmt::{Debug, Display, Formatter},
	ops::{Add, Range},
};

use miette::SourceSpan;

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

	pub const fn start(&self) -> usize { self.start }

	pub const fn end(&self) -> usize { self.end }

	pub const fn range(&self) -> Range<usize> { self.start..self.end }

	pub const fn tuple(&self) -> (usize, usize) { (self.start, self.end) }
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

impl Display for Span {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}..{}", self.start, self.end)
	}
}

impl PartialOrd for Span {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

// TODO: handle DUMMY_SPAN properly
impl Ord for Span {
	// usize implements Ord
	fn cmp(&self, other: &Self) -> Ordering {
		match self.start.cmp(&other.start) {
			Ordering::Equal => other.end.cmp(&self.end),
			o => o,
		}
	}
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

impl From<Span> for (usize, usize) {
	fn from(value: Span) -> Self { value.tuple() }
}

impl From<Span> for Range<usize> {
	fn from(value: Span) -> Self { value.range() }
}

impl From<Span> for SourceSpan {
	fn from(value: Span) -> Self { Self::from(value.range()) }
}

#[derive(Debug, PartialEq)]
pub struct Spanned<T> {
	span: Span,
	value: T,
}

impl<T> Display for Spanned<T>
where T: Display
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}({})", self.value, self.span)
	}
}

impl<T> Error for Spanned<T> where T: Error {}

pub trait HasSpan {
	fn span(&self) -> Span;

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span>;

	fn with_span<S>(mut self, span: S) -> Self
	where
		S: Into<Span>,
		Self: Sized, {
		self.set_span(span);
		self
	}
}

impl<T> Spanned<T> {
	pub fn new<S>(span: S, value: T) -> Self
	where S: Into<Span> {
		Self {
			span: span.into(),
			value,
		}
	}

	pub fn get(&self) -> &T { &self.value }

	pub fn get_mut(&mut self) -> &mut T { &mut self.value }

	pub fn into_inner(self) -> T { self.value }
}

impl<T> HasSpan for Spanned<T> {
	fn span(&self) -> Span { self.span }

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span> {
		self.span = span.into()
	}
}

impl<T, S> From<(S, T)> for Spanned<T>
where S: Into<Span>
{
	fn from((span, value): (S, T)) -> Self {
		Self {
			span: span.into(),
			value,
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
