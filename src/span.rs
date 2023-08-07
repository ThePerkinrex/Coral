use miette::{SourceCode, SourceSpan};
use std::ops::Range;

use crate::{fs::FileId, FileArena};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    file: FileId,
    start: usize,
    end: usize,
}

impl Span {
    pub const fn new(file: FileId, range: Range<usize>) -> Self {
        Self {
            file,
            start: range.start,
            end: range.end,
        }
    }
    const fn range(&self) -> Range<usize> {
        self.start..self.end
    }
    pub fn as_miette_span<'a>(
        &self,
        arena: &'a FileArena,
        lines_before: usize,
        lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        let source_span: SourceSpan = self.range().into();
        arena[self.file].read_span(&source_span, lines_before, lines_after)
    }

    pub fn get_slice<'a>(&self, arena: &'a FileArena) -> &'a str {
        &arena[self.file].contents[self.range()]
    }

    pub fn filename<'a>(&self, arena: &'a FileArena) -> &'a str {
        &arena[self.file].name
    }

    #[must_use]
    pub fn from_ends(start: Self, end: Self) -> Option<Self> {
        if start.file != end.file {
            None
        } else {
            Some(Self {
                file: start.file,
                start: start.start,
                end: end.end,
            })
        }
    }

    pub const fn spanned<T>(self, data: T) -> Spanned<T> {
        Spanned { span: self, data }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spanned<T> {
    pub span: Span,
    pub data: T,
}

impl<T> From<(FileId, Range<usize>, T)> for Spanned<T> {
    fn from(value: (FileId, Range<usize>, T)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl<T> Spanned<T> {
    pub const fn new(file: FileId, range: Range<usize>, data: T) -> Self {
        Self {
            span: Span::new(file, range),
            data,
        }
    }

    pub fn get_slice<'a>(&self, arena: &'a FileArena) -> &'a str {
        self.span.get_slice(arena)
    }

    pub fn filename<'a>(&self, arena: &'a FileArena) -> &'a str {
        self.span.filename(arena)
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
        self.span.as_miette_span(arena, lines_before, lines_after)
    }

    #[must_use]
    pub fn from_ends<U, V>(start: Spanned<U>, end: Spanned<V>, data: T) -> Option<Self> {
        Span::from_ends(start.span, end.span).map(|span| Self { span, data })
    }

    pub const fn copy_new_data<U>(&self, data: U) -> Spanned<U> {
        self.span.spanned(data)
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, mapping: F) -> Spanned<U> {
        Spanned {
            span: self.span,
            data: mapping(self.data),
        }
    }
}
