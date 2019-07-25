use std::iter::FromIterator;

use super::Visitor;

pub trait Visit<W>
where
    W: Visitor<Self> + ?Sized,
    Self: Sized,
{
    #[allow(unused_variables)]
    fn walk(self, visitor: &mut W) -> Self {
        self
    }
}

impl<I, T, W> Visit<W> for I
where
    I: IntoIterator<Item = T> + FromIterator<T>,
    T: Visit<W> + Sized,
    W: Visitor<T>,
{
    fn walk(self, visitor: &mut W) -> Self {
        self.into_iter().map(|t| visitor.visit(t)).collect()
    }
}
