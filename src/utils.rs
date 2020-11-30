pub use lazy_static;

pub use boolinator::Boolinator;
// pub use fnv::FnvHasher;
use fxhash::FxHashMap;
use fxhash::FxHashSet;
pub use itertools::Itertools;
pub use std::collections::HashSet;
pub use std::fs::File;
pub use std::hash::BuildHasherDefault;
pub use std::io;
pub use std::io::Read;
pub use std::path::Path;
pub use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub type FSet<T> = FxHashSet<T>;
pub type FMap<K, V> = FxHashMap<K, V>;
// type FSet<T> = HashSet<T, BuildHasherDefault<FnvHasher>>;
// type FMap<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;

pub fn fmap<K, V>(cap: usize) -> FMap<K, V> {
    FMap::with_capacity_and_hasher(cap, Default::default())
}

pub fn fset<V>(cap: usize) -> FSet<V> {
    FSet::with_capacity_and_hasher(cap, Default::default())
}

pub fn read_input<P: AsRef<Path>>(p: P) -> io::Result<String> {
    let mut out = String::with_capacity(100_000);
    File::open(p)?.read_to_string(&mut out)?;
    Ok(out)
}

pub fn pause() {
    io::stdin().read_line(&mut String::new()).unwrap();
}

macro_rules! blackhole {
    ($($l:expr),+) => {};
}

type MinHeap<T> = BinaryHeap<Reverse<T>>;

pub fn firstlast<T>(vec: &[T]) -> Option<(&T, &T)> {
    vec.split_first().map(|(first, rest)| {
        rest.split_last()
            .map_or((first, first), |(last, _)| (first, last))
    })
}

pub fn firstlastex<T>(vec: &[T]) -> (Option<&T>, Option<&T>) {
    if let Some((first, rest)) = vec.split_first() {
        (Some(first), rest.split_last().map(|(last, _)| last))
    } else {
        (None, None)
    }
}
