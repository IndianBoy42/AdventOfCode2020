use std::{
    cmp::Reverse, collections::BinaryHeap, collections::HashSet, collections::VecDeque,
    hash::BuildHasher, hash::Hash,
};

use crate::utils::{fset, FSet, BitSet};

pub trait SearchQueue<T> {
    fn newq() -> Self;
    fn pushq(&mut self, e: T);
    fn popq(&mut self) -> Option<T>;
}

pub trait Visited<T> {
    fn newset() -> Self;
    fn mark(&mut self, e: T) -> bool;
    fn check(&self, e: &T) -> bool;
}

impl<T> SearchQueue<T> for BinaryHeap<Reverse<T>>
where
    Reverse<T>: Ord,
{
    fn newq() -> Self {
        Self::new()
    }
    fn pushq(&mut self, e: T) {
        self.push(Reverse(e));
    }
    fn popq(&mut self) -> Option<T> {
        self.pop().map(|Reverse(e)| e)
    }
}

impl<T> SearchQueue<T> for VecDeque<T> {
    fn newq() -> Self {
        Self::new()
    }
    fn pushq(&mut self, e: T) {
        self.push_back(e);
    }
    fn popq(&mut self) -> Option<T> {
        self.pop_front()
    }
}

impl<T> SearchQueue<T> for Vec<T> {
    fn newq() -> Self {
        Self::new()
    }
    fn pushq(&mut self, e: T) {
        self.push(e);
    }
    fn popq(&mut self) -> Option<T> {
        self.pop()
    }
}

impl<T, B: Default> Visited<T> for HashSet<T, B>
where
    T: Eq + Hash,
    B: BuildHasher,
{
    fn newset() -> Self {
        Self::with_hasher(B::default())
    }
    fn mark(&mut self, e: T) -> bool {
        self.insert(e)
    }
    fn check(&self, e: &T) -> bool {
        !self.contains(e)
    }
}

impl Visited<usize> for BitSet
{
    fn newset() -> Self {
        Self::new()
    }
    fn mark(&mut self, e: usize) -> bool {
        self.insert(e)
    }
    fn check(&self, &e: &usize) -> bool {
        !self.contains(e)
    }
}

#[derive(Clone, Debug)]
pub struct Searcher<T, Q, SearchF> {
    visited: FSet<T>,
    queue: Q,
    searcher: SearchF,
}
pub type DFSearcher<T, SF> = Searcher<T, Vec<T>, SF>;
pub type BFSearcher<T, SF> = Searcher<T, VecDeque<T>, SF>;
pub type DijSearcher<T, SF> = Searcher<T, BinaryHeap<Reverse<T>>, SF>;

impl<T, Q, SearchF, SearchIter> Searcher<T, Q, SearchF>
where
    T: Hash + Clone + Eq,
    Q: SearchQueue<T>,
    SearchF: FnMut(&T) -> SearchIter,
    SearchIter: IntoIterator<Item = T>,
{
    pub fn new(init: T, searcher: SearchF) -> Searcher<T, Q, SearchF> {
        Self::with_capacity(0, init, searcher)
    }
    pub fn with_capacity(cap: usize, init: T, searcher: SearchF) -> Searcher<T, Q, SearchF> {
        let mut s = Searcher {
            visited: fset(cap),
            queue: Q::newq(),
            searcher,
        };
        s.push(init);
        s
    }

    pub fn push(&mut self, e: T) {
        if self.visited.insert(e.clone()) {
            self.queue.pushq(e);
        }
    }
}

impl<T, Q, SearchF, SearchIter> Iterator for Searcher<T, Q, SearchF>
where
    T: Hash + Clone + Eq,
    Q: SearchQueue<T>,
    SearchF: FnMut(&T) -> SearchIter,
    SearchIter: IntoIterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.popq().map(|elem| {
            // Mark visited
            self.visited.insert(elem.clone());

            // Find Neighbours
            for e in (self.searcher)(&elem) {
                if self.visited.contains(&e) {
                    continue;
                }
                self.push(e);
            }

            elem
        })
    }
}

// impl<T, Q, SearchF> Iterator for Searcher<T, Q, SearchF>
// where
//     T: Hash + Clone + Eq,
//     Q: SearchQueue<T>,
//     SearchF: FnMut(&mut Self),
// {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.queue.popq().map(|elem| {
//             // Mark visited
//             self.visited.insert(elem.clone());

//             // Find Neighbours
//             (self.searcher)(&elem);

//             elem
//         })
//     }
// }
