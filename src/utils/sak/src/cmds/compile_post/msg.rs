use anyhow::Result;
use ariadne::{Color, Label, Report, ReportKind, Source};
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::io::Write;
use std::ops::{Deref, DerefMut};

pub type MessageResult<T> = Result<T, Message>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub beg: usize,
    pub end: usize,
}

impl Span {
    pub fn char(pos: usize) -> Self {
        Self { beg: pos, end: pos }
    }
}

impl Default for Span {
    fn default() -> Self {
        Self {
            beg: usize::MAX,
            end: usize::MAX,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Spanned<T> {
    inner: T,
    span: Span,
}

impl<T> Spanned<T> {
    pub fn new(inner: T, span: Span) -> Self {
        Self { inner, span }
    }

    pub fn dummy(inner: T) -> Self {
        Self::new(inner, Span::default())
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> PartialEq for Spanned<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> Eq for Spanned<T>
where
    T: Eq,
{
    //
}

impl<T> PartialOrd for Spanned<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T> Ord for Spanned<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

#[derive(Debug)]
pub struct Message {
    body: String,
    span: Span,
    labels: Vec<(String, Span)>,
}

impl Message {
    pub fn new(body: impl ToString, span: Span) -> Self {
        Self {
            body: body.to_string(),
            span,
            labels: Default::default(),
        }
    }

    pub fn with_label(mut self, body: impl ToString, span: Span) -> Self {
        self.labels.push((body.to_string(), span));
        self
    }
}

impl Error for Message {
    //
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body)
    }
}

pub struct MessageReporter<'a, 'b> {
    out: &'a mut dyn Write,
    src: &'b str,
}

impl<'a, 'b> MessageReporter<'a, 'b> {
    pub fn new(out: &'a mut dyn Write, src: &'b str) -> Self {
        Self { out, src }
    }

    pub fn report(&mut self, msg: Message) -> Result<()> {
        let mut report = Report::build(
            ReportKind::Custom("error", Color::Red),
            "input",
            msg.span.beg,
        )
        .with_message(&msg.body)
        .with_label(
            Label::new(("input", msg.span.beg..(msg.span.end + 1)))
                .with_message(&msg.body),
        );

        for (label_body, label_span) in msg.labels {
            report = report.with_label(
                Label::new(("input", label_span.beg..(label_span.end + 1)))
                    .with_message(label_body),
            );
        }

        #[cfg(test)]
        {
            report = report
                .with_config(ariadne::Config::default().with_color(false));
        }

        report
            .finish()
            .write(("input", Source::from(self.src)), &mut self.out)?;

        Ok(())
    }
}
