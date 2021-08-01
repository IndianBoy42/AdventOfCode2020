use core::iter::FromIterator;
use std::fmt::Debug;

#[derive( Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct USet {
    val: u32,
}
impl Debug for USet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.val)
    }
}
pub type U32Set = USet;
impl USet {
    pub const fn new(val: u32) -> USet {
        USet { val }
    }
    pub fn intersect_with(&mut self, other: USet) {
        self.val &= other.val;
    }
    pub const fn intersect(self, other: USet) -> USet {
        USet {
            val: self.val & other.val,
        }
    }
    pub fn union_with(&mut self, other: USet) {
        self.val |= other.val;
    }
    pub const fn union(self, other: USet) -> USet {
        USet {
            val: self.val | other.val,
        }
    }
    pub const fn len(self) -> usize {
        self.val.count_ones() as _
    }
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }
    /// Reverse the first `i` bits
    pub const fn revn(self, i: usize) -> USet {
        Self::new(self.val.reverse_bits() >> (32 - i))
    }
    pub const fn rev(self) -> USet {
        Self::new(self.val.reverse_bits())
    }
}
impl FromIterator<usize> for USet {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        USet {
            val: iter.into_iter().fold(0, |acc, bit| acc | (1 << bit)),
        }
    }
}

impl FromIterator<bool> for USet {
    fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
        USet {
            val: iter
                .into_iter()
                .fold(0, |acc, bit| (acc << 1) | (bit as u32)),
        }
    }
}
