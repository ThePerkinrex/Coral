pub trait ErrorReporter {
	pub fn report<E>(&mut self, error_span: Span<E>);
}