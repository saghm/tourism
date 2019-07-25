pub trait Visit<W>
where
    W: Visitor<Self> + ?Sized,
{
    #[allow(unused_variables)]
    fn walk(&mut self, visitor: &mut W) {}
}

pub trait Visitor<V>
where
    V: Visit<Self> + ?Sized,
{
    fn visit(&mut self, v: &mut V) {
        v.walk(self)
    }
}
