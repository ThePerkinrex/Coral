use std::ops::Range;
use miette::{SourceCode, SourceSpan};

use crate::{
    fs::FileId,
    FileArena,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span<T> {
    file: FileId,
    range: Range<usize>,
    pub data: T,
}

impl<T> From<(FileId, Range<usize>, T)> for Span<T> {
    fn from(value: (FileId, Range<usize>, T)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl<T> Span<T> {
    pub const fn new(file: FileId, range: Range<usize>, data: T) -> Self {
        Self { file, range, data }
    }

    pub fn get_slice<'a>(&self, arena: &'a FileArena) -> &'a str {
        &arena[self.file].contents[self.range.clone()]
    }

    pub fn filename<'a>(&self, arena: &'a FileArena) -> &'a str {
        &arena[self.file].name
    }

    // pub fn data<'a>(&self, arena: &'a FileArena) -> (&'a str, &'a str) {
    // 	(self.filename(arena), self.get_slice(arena))
    // }

    pub fn as_miette_span<'a>(
        &self,
        arena: &'a FileArena,
        lines_before: usize,
        lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        let source_span: SourceSpan = self.range.clone().into();
        arena[self.file].read_span(&source_span, lines_before, lines_after)
    }

    #[must_use]
    pub fn from_ends<U, V>(start: Span<U>, end: Span<V>, data: T) -> Option<Self> {
        if start.file != end.file {
            None
        } else {
			Some(Self {
				file: start.file,
				data,
				range: start.range.start..end.range.end
			})
        }
    }

	pub fn copy_new_data<U>(&self, data: U) -> Span<U> {
		Span { file: self.file, range: self.range.clone(), data }
	}

	pub fn map<U, F: FnOnce(T) -> U>(self, mapping: F) -> Span<U> {
		Span { file: self.file, range: self.range, data: mapping(self.data) }
	}
}
