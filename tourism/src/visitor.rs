use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    hash::Hash,
};

use super::Visit;

pub trait Visitor<U>
where
    U: Visit<Self>,
{
    fn visit(&mut self, visit: U) -> U {
        visit.walk(self)
    }
}

macro_rules! impl_noop_visitor {
    ( $ty1:ty, $( $ty2:ty ),+ ) => {
        impl_noop_visitor!($ty1);
        impl_noop_visitor!( $( $ty2 ),+ );
    };

    ( $ty:ty ) => {
         impl<U> Visitor<$ty> for U {}
    };
}

impl_noop_visitor!(
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

impl<T, U> Visitor<Vec<U>> for T
where
    T: Visitor<U>,
    U: Visit<Self>,
{
    fn visit(&mut self, visit: Vec<U>) -> Vec<U> {
        visit.into_iter().map(|t| t.walk(self)).collect()
    }
}

impl<T, U> Visitor<BTreeSet<U>> for T
where
    T: Visitor<U>,
    U: Ord + Visit<Self>,
{
    fn visit(&mut self, visit: BTreeSet<U>) -> BTreeSet<U> {
        visit.into_iter().map(|t| t.walk(self)).collect()
    }
}

impl<T, U> Visitor<BinaryHeap<U>> for T
where
    T: Visitor<U>,
    U: Ord + Visit<Self>,
{
    fn visit(&mut self, visit: BinaryHeap<U>) -> BinaryHeap<U> {
        visit.into_iter().map(|t| t.walk(self)).collect()
    }
}

impl<T, U> Visitor<HashSet<U>> for T
where
    T: Visitor<U>,
    U: Hash + Eq + Visit<Self>,
{
    fn visit(&mut self, visit: HashSet<U>) -> HashSet<U> {
        visit.into_iter().map(|t| t.walk(self)).collect()
    }
}

impl<T, V> Visitor<LinkedList<T>> for V
where
    V: Visitor<T>,
    T: Visit<Self>,
{
    fn visit(&mut self, visit: LinkedList<T>) -> LinkedList<T> {
        visit.into_iter().map(|t| t.walk(self)).collect()
    }
}

impl<T, V> Visitor<VecDeque<T>> for V
where
    V: Visitor<T>,
    T: Visit<Self>,
{
    fn visit(&mut self, visit: VecDeque<T>) -> VecDeque<T> {
        visit.into_iter().map(|t| t.walk(self)).collect()
    }
}

impl<K, V, T> Visitor<BTreeMap<K, V>> for T
where
    T: Visitor<K> + Visitor<V>,
    K: Ord + Visit<Self>,
    V: Visit<Self>,
{
    fn visit(&mut self, visit: BTreeMap<K, V>) -> BTreeMap<K, V> {
        visit
            .into_iter()
            .map(|(k, v)| (k.walk(self), v.walk(self)))
            .collect()
    }
}

impl<K, V, T> Visitor<HashMap<K, V>> for T
where
    T: Visitor<K> + Visitor<V>,
    K: Hash + Eq + Visit<Self>,
    V: Visit<Self>,
{
    fn visit(&mut self, visit: HashMap<K, V>) -> HashMap<K, V> {
        visit
            .into_iter()
            .map(|(k, v)| (k.walk(self), v.walk(self)))
            .collect()
    }
}
