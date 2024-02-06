use span::{HasSpan, Span};

pub struct CstNode {
	pub span: Span,
}

impl HasSpan for CstNode {
	fn span(&self) -> Span { self.span }

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span> {
		self.span = span.into();
	}
}
