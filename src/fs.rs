use std::borrow::Cow;

use id_arena::Id;
use miette::{SourceCode, SpanContents};

pub struct File {
    pub name: Cow<'static, str>,
    pub contents: Cow<'static, str>,
}

impl SourceCode for File {
    fn read_span<'a>(
        &'a self,
        span: &miette::SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        self.contents
            .read_span(span, context_lines_before, context_lines_after)
            .map(|span| NamedSpan::new(&self.name, span) as Box<dyn miette::SpanContents<'a> + 'a>)
    }
}

pub struct NamedSpan<'a> {
    name: &'a str,
    inner: Box<dyn miette::SpanContents<'a> + 'a>,
}

impl<'a> NamedSpan<'a> {
    pub fn new(name: &'a str, inner: Box<dyn miette::SpanContents<'a> + 'a>) -> Box<Self> {
        Box::new(Self { name, inner })
    }
}

impl<'a> SpanContents<'a> for NamedSpan<'a> {
    fn data(&self) -> &'a [u8] {
        self.inner.data()
    }

    fn span(&self) -> &miette::SourceSpan {
        self.inner.span()
    }

    fn line(&self) -> usize {
        self.inner.line()
    }

    fn column(&self) -> usize {
        self.inner.column()
    }

    fn line_count(&self) -> usize {
        self.inner.line_count()
    }

    fn name(&self) -> Option<&str> {
        Some(self.name)
    }
}

pub type FileId = Id<File>;
