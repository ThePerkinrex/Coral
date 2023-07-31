use std::ops::Range;

use id_arena::Arena;
use logos::SpannedIter;
use miette::{SourceSpan, SourceCode};

use crate::{fs::{FileId, File, NamedSpan}, FileArena};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span<T> {
	file: FileId,
	range: Range<usize>,
	pub data: T
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

	pub fn as_miette_span<'a>(&self, arena: &'a FileArena, lines_before: usize, lines_after: usize) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
		let source_span: SourceSpan = self.range.clone().into();
		arena[self.file].read_span(&source_span, lines_before, lines_after)
	}
}
