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

macro_rules! impl_value_iterator_visitor {
    ( $hkty1:ident $( ($cons1:path) )*, $( $hkty2:ident $( ($cons2:path) )*),+ ) => {
        impl_value_iterator_visitor!( $hkty1 $( ($cons1) )* );
        impl_value_iterator_visitor!( $( $hkty2 $( ($cons2) )*),+ );
    };

    ( $hkty:ident  $( ($cons:path) )* ) => {
            impl<T, U> Visitor<$hkty<U>> for T
            where
                T: Visitor<U>,
                U:  $( $cons + )*  Visit<Self>
            {
                fn visit(&mut self, visit: $hkty<U>) -> $hkty<U> {
                    visit.walk(self)
                }
            }
    };
}

impl_value_iterator_visitor!(
    BinaryHeap(Ord),
    BTreeSet(Ord),
    HashSet(Hash)(Eq),
    LinkedList,
    Vec,
    VecDeque
);

macro_rules! impl_key_value_iterator_visitor {
    ( $hkty1:ident $( ($cons1:path) )*, $( $hkty2:ident $( ($cons2:path) )*),+ ) => {
        impl_key_value_iterator_visitor!( $hkty1 $( ($cons1) )* );
        impl_key_value_iterator_visitor!( $( $hkty2 $( ($cons2) )*),+ );
    };

    ( $hkty:ident  $( ($cons:path) )* ) => {
            impl<K, V, T> Visitor<$hkty<K, V>> for T
            where
                T: Visitor<K> + Visitor<V>,
                K:  $( $cons + )*  Visit<Self>,
                V: Visit<Self>,
            {
                fn visit(&mut self, visit: $hkty<K, V>) -> $hkty<K, V> {
                    visit.walk(self)
                }
            }
    };
}

impl_key_value_iterator_visitor!(BTreeMap(Ord), HashMap(Hash)(Eq));
