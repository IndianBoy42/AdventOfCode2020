use std::{
    cmp::Reverse, collections::BinaryHeap, collections::HashSet, collections::VecDeque,
    hash::BuildHasher, hash::Hash,
};

use crate::utils::{fset, FSet, BitSet};

pub trait SearchQueue<T> {
    fn newq() -> Self;
    fn newq_cap(cap: usize) -> Self;
    fn pushq(&mut self, e: T);
    fn popq(&mut self) -> Option<T>;
}

pub trait Visited<T> {
    fn newset() -> Self;
    fn newset_cap(cap: usize) -> Self;
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
    fn newq_cap(cap: usize) -> Self {
        Self::with_capacity(cap)
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
    fn newq_cap(cap: usize) -> Self {
        Self::with_capacity(cap)
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
    fn newq_cap(cap: usize) -> Self {
        Self::with_capacity(cap)
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
    fn newset_cap(cap: usize) -> Self {
        Self::with_capacity_and_hasher(cap, B::default())
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
    fn newset_cap(cap: usize) -> Self {
        Self::with_capacity(cap)
    }
    fn mark(&mut self, e: usize) -> bool {
        self.insert(e)
    }
    fn check(&self, &e: &usize) -> bool {
        !self.contains(e)
    }
}

#[derive(Clone, Debug)]
pub struct Searcher<Queue, VisitSet, NeighboursFn> {
    visited: VisitSet,
    queue: Queue,
    neighbours: NeighboursFn,
}
pub type DFSearcher<T, V, SF> = Searcher<V, Vec<T>, SF>;
pub type BFSearcher<T, V, SF> = Searcher<V, VecDeque<T>, SF>;
pub type DijSearcher<T, V, SF> = Searcher<V, BinaryHeap<Reverse<T>>, SF>;

impl<T, Queue, NeighboursFn, SearchIter> Searcher<Queue, FSet<T>, NeighboursFn>
where
    T: Hash + Clone + Eq,
    Queue: SearchQueue<T>,
    NeighboursFn: FnMut(&T) -> SearchIter,
    SearchIter: IntoIterator<Item = T>,
{
    pub fn new(init: T, neighbours: NeighboursFn) -> Self {
        Self::with_capacity(0, init, neighbours)
    }
    pub fn with_capacity(cap: usize, init: T, neighbours: NeighboursFn) -> Self {
        let mut s = Searcher {
            visited: fset(cap),
            queue: Queue::newq_cap(cap),
            neighbours,
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

impl<T, Queue, NeighboursFn, SearchIter> Iterator for Searcher<Queue, FSet<T>, NeighboursFn>
where
    T: Hash + Clone + Eq,
    Queue: SearchQueue<T>,
    NeighboursFn: FnMut(&T) -> SearchIter,
    SearchIter: IntoIterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.popq().map(|elem| {
            // Mark visited
            self.visited.insert(elem.clone());

            // Find Neighbours
            for e in (self.neighbours)(&elem) {
                if self.visited.contains(&e) {
                    continue;
                }
                self.push(e);
            }

            elem
        })
    }
}

// impl<T, Queue, NeighboursFn> Iterator for Searcher<T, Queue, NeighboursFn>
// where
//     T: Hash + Clone + Eq,
//     Queue: SearchQueue<T>,
//     NeighboursFn: FnMut(&mut Self),
// {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.queue.popq().map(|elem| {
//             // Mark visited
//             self.visited.insert(elem.clone());

//             // Find Neighbours
//             (self.neighbours)(&elem);

//             elem
//         })
//     }
// }