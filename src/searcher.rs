use std::marker::PhantomData;
use std::{
    cmp::Reverse, collections::BinaryHeap, collections::HashSet, collections::VecDeque,
    hash::BuildHasher, hash::Hash,
};

use crate::utils::{fset, BitSet, FSet};

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

impl Visited<usize> for BitSet {
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
pub struct Searcher<T, Queue, VisitSet, NeighboursFn> {
    visited: VisitSet,
    queue: Queue,
    neighbours: NeighboursFn,
    _dummy_T: PhantomData<T>,
}
pub type DFSearcher<T, V, SF> = Searcher<T, V, Vec<T>, SF>;
pub type DFSearcherInt<SF> = Searcher<usize, BitSet, Vec<usize>, SF>;
pub type BFSearcher<T, V, SF> = Searcher<T, V, VecDeque<T>, SF>;
pub type BFSearcherInt<SF> = Searcher<usize, BitSet, VecDeque<usize>, SF>;
pub type DijSearcher<T, V, SF> = Searcher<T, V, BinaryHeap<Reverse<T>>, SF>;
pub type DijSearcherInt<SF> = Searcher<usize, BitSet, BinaryHeap<Reverse<usize>>, SF>;

impl<T, VisitSet, Queue, NeighboursFn, SearchIter> Searcher<T, Queue, VisitSet, NeighboursFn>
where
    T: Hash + Clone + Eq,
    VisitSet: Visited<T>,
    Queue: SearchQueue<T>,
    NeighboursFn: FnMut(&T) -> SearchIter,
    SearchIter: IntoIterator<Item = T>,
{
    pub fn new(init: T, neighbours: NeighboursFn) -> Self {
        Self::with_capacity(0, init, neighbours)
    }
    pub fn with_capacity(cap: usize, init: T, neighbours: NeighboursFn) -> Self {
        let mut slf = Searcher {
            visited: VisitSet::newset_cap(cap),
            queue: Queue::newq_cap(cap),
            neighbours,
            _dummy_T: PhantomData,
        };
        slf.push(init);
        slf
    }

    pub fn push(&mut self, e: T) {
        if self.visited.mark(e.clone()) {
            self.queue.pushq(e);
        }
    }
}

impl<T, VisitSet, Queue, NeighboursFn, SearchIter> Iterator for Searcher<T, Queue, VisitSet, NeighboursFn>
where
    T: Hash + Clone + Eq,
    VisitSet: Visited<T>,
    Queue: SearchQueue<T>,
    NeighboursFn: FnMut(&T) -> SearchIter,
    SearchIter: IntoIterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.popq().map(|elem| {
            // Mark visited
            // self.visited.mark(elem.clone());

            // Find Neighbours
            for e in (self.neighbours)(&elem) {
                if self.visited.check(&e) {
                    self.push(e);
                }
            }

            elem
        })
    }
}
