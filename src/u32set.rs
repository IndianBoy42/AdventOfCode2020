use core::iter::FromIterator;

#[derive(Debug, Clone, Copy, Default)]
pub struct U32Set {
    val: u32,
}
impl U32Set {
    pub const fn new(val: u32) -> U32Set {
        U32Set { val }
    }
    pub fn intersect_with(&mut self, other: U32Set) {
        self.val &= other.val;
    }
    pub const fn intersect(self, other: U32Set) -> U32Set {
        U32Set {
            val: self.val & other.val,
        }
    }
    pub fn union_with(&mut self, other: U32Set) {
        self.val |= other.val;
    }
    pub const fn union(self, other: U32Set) -> U32Set {
        U32Set {
            val: self.val | other.val,
        }
    }
    pub const fn len(self) -> usize {
        self.val.count_ones() as _
    }
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }
}
impl FromIterator<usize> for U32Set {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        U32Set {
            val: iter.into_iter().fold(0, |acc, bit| acc | (1 << bit)),
        }
    }
}
