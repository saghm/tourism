use std::iter::FromIterator;

use super::Visit;

pub trait Visitor<W>
where
    W: Visit<Self> + Sized,
{
    fn visit(&mut self, visit: W) -> W {
        visit.walk(self)
    }
}

impl<I, T, U> Visitor<I> for U
where
    U: Visitor<T>,
    I: IntoIterator<Item = T> + FromIterator<T>,
    T: Visit<U> + Sized,
{
    fn visit(&mut self, visit: I) -> I {
        visit.walk(self)
    }
}
