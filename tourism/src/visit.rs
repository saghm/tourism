use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    hash::Hash,
};

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

macro_rules! impl_noop_visit {
    ( $ty1:ty, $( $ty2:ty ),+ ) => {
        impl_noop_visit!($ty1);
        impl_noop_visit!( $( $ty2 ),+ );
    };

    ( $ty:ty ) => {
            impl<W> Visit<W> for $ty {}
    };
}

impl_noop_visit!(
    (),
    bool,
    char,
    f32,
    f64,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    &str,
    String,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize
);

impl<T, W> Visit<W> for Vec<T>
where
    W: Visitor<T>,
    T: Visit<W>,
{
    fn walk(self, visitor: &mut W) -> Self {
        self.into_iter().map(|t| visitor.visit(t)).collect()
    }
}

impl<T, W> Visit<W> for BTreeSet<T>
where
    W: Visitor<T>,
    T: Ord + Visit<W>,
{
    fn walk(self, visitor: &mut W) -> Self {
        self.into_iter().map(|t| visitor.visit(t)).collect()
    }
}

impl<T, W> Visit<W> for BinaryHeap<T>
where
    W: Visitor<T>,
    T: Ord + Visit<W>,
{
    fn walk(self, visitor: &mut W) -> Self {
        self.into_iter().map(|t| visitor.visit(t)).collect()
    }
}

impl<T, W> Visit<W> for HashSet<T>
where
    W: Visitor<T>,
    T: Hash + Eq + Visit<W>,
{
    fn walk(self, visitor: &mut W) -> Self {
        self.into_iter().map(|t| visitor.visit(t)).collect()
    }
}

impl<T, W> Visit<W> for LinkedList<T>
where
    W: Visitor<T>,
    T: Visit<W>,
{
    fn walk(self, visitor: &mut W) -> Self {
        self.into_iter().map(|t| visitor.visit(t)).collect()
    }
}

impl<T, W> Visit<W> for VecDeque<T>
where
    W: Visitor<T>,
    T: Visit<W>,
{
    fn walk(self, visitor: &mut W) -> Self {
        self.into_iter().map(|t| visitor.visit(t)).collect()
    }
}

impl<K, V, W> Visit<W> for BTreeMap<K, V>
where
    W: Visitor<K> + Visitor<V>,
    K: Ord + Visit<W>,
    V: Visit<W>,
{
    fn walk(self, visitor: &mut W) -> Self {
        self.into_iter()
            .map(|(k, v)| (visitor.visit(k), visitor.visit(v)))
            .collect()
    }
}

impl<K, V, W> Visit<W> for HashMap<K, V>
where
    W: Visitor<K> + Visitor<V>,
    K: Hash + Eq + Visit<W>,
    V: Visit<W>,
{
    fn walk(self, visitor: &mut W) -> Self {
        self.into_iter()
            .map(|(k, v)| (visitor.visit(k), visitor.visit(v)))
            .collect()
    }
}
