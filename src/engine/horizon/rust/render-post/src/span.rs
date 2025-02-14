use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

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
