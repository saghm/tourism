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

macro_rules! impl_value_iterator_visit {
    ( $hkty1:ident $( ($cons1:path) )*, $( $hkty2:ident $( ($cons2:path) )*),+ ) => {
        impl_value_iterator_visit!( $hkty1 $( ($cons1) )* );
        impl_value_iterator_visit!( $( $hkty2 $( ($cons2) )*),+ );
    };

    ( $hkty:ident  $( ($cons:path) )* ) => {
            impl<T, W> Visit<W> for $hkty<T>
            where
                W: Visitor<T>,
                T:  $( $cons + )*  Visit<W>
            {
                fn walk(self, visitor: &mut W) -> Self {
                    self.into_iter().map(|t| visitor.visit(t)).collect()
                }
            }
    };
}

impl_value_iterator_visit!(
    BinaryHeap(Ord),
    BTreeSet(Ord),
    HashSet(Hash)(Eq),
    LinkedList,
    Vec,
    VecDeque
);

macro_rules! impl_key_value_iterator_visit {
    ( $hkty1:ident $( ($cons1:path) )*, $( $hkty2:ident $( ($cons2:path) )*),+ ) => {
        impl_key_value_iterator_visit!( $hkty1 $( ($cons1) )* );
        impl_key_value_iterator_visit!( $( $hkty2 $( ($cons2) )*),+ );
    };

    ( $hkty:ident  $( ($cons:path) )* ) => {
            impl<K, V, W> Visit<W> for $hkty<K,V>
            where
                W: Visitor<K> + Visitor<V>,
                K:  $( $cons + )*  Visit<W>,
                V: Visit<W>,
            {
                fn walk(self, visitor: &mut W) -> Self {
                    self.into_iter()
                        .map(|(k, v)| (visitor.visit(k), visitor.visit(v)))
                        .collect()
                }
            }
    };
}

impl_key_value_iterator_visit!(BTreeMap(Ord), HashMap(Hash)(Eq));
