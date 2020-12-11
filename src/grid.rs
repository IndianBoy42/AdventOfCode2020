use std::convert::{TryFrom, TryInto};
use std::iter::FromIterator;
use std::ops::Deref;

use itertools::izip;
use ndarray::{azip, Array2, ArrayBase};
use rayon::collections::binary_heap::IntoIter;
use rayon::iter::FromParallelIterator;

use crate::utils::FMap;

#[derive(Debug, Clone)]
pub struct Grid2D<T> {
    pub arr: Array2<T>,
}

impl<T: num_traits::Zero + Clone> Grid2D<T> {
    pub fn from_iter_w_shape<I: IntoIterator<Item = T>>(shape: (usize, usize), iter: I) -> Self {
        let mut arr = Array2::zeros(shape);
        izip!(arr.iter_mut(), iter.into_iter()).for_each(|(out, elem)| {
            *out = elem;
        });

        // let arr: ArrayBase<_, _> = iter.into_iter().collect();
        // let arr = arr.into_shape(shape).unwrap();
        Grid2D { arr }
    }
}

impl<T: Default + std::cmp::PartialEq> Grid2D<T> {
    pub fn contains_key(&self, &(x, y): &(i32, i32)) -> bool {
        self.arr
            .get((x as usize, y as usize))
            .map_or(false, |x| x != &T::default())
    }
}

impl<T> Grid2D<T> {
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.arr.iter()
    }

    pub fn get(&self, &(x, y): &(i32, i32)) -> Option<&T> {
        self.arr.get((x as usize, y as usize))
    }

    pub fn get_mut(&mut self, &(x, y): &(i32, i32)) -> Option<&mut T> {
        self.arr.get_mut((x as usize, y as usize))
    }

    pub fn iter<I>(&self) -> impl Iterator<Item = ((I, I), &T)>
    where
        usize: TryInto<I>,
        <usize as std::convert::TryInto<I>>::Error: std::fmt::Debug,
    {
        self.arr
            .indexed_iter()
            .map(|((x, y), z)| ((x.try_into().unwrap(), y.try_into().unwrap()), z))
    }
}

impl<T> FromIterator<T> for Grid2D<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        todo!()
    }
}

impl<T: Clone + num_traits::Zero> FromIterator<((usize, usize), T)> for Grid2D<T> {
    fn from_iter<I: IntoIterator<Item = ((usize, usize), T)>>(iter: I) -> Self {
        let map = iter.into_iter().collect::<Vec<_>>();
        let xmax = map.iter().map(|&((x, y), _)| x).max().unwrap() + 1;
        let ymax = map.iter().map(|&((x, y), _)| y).max().unwrap() + 1;

        let mut arr = Array2::zeros((xmax, ymax));
        for ((x, y), z) in map {
            arr[(x, y)] = z;
        }

        Grid2D { arr }
    }
}

// TODO: collect par_iter
// impl<T: Send> FromParallelIterator<T> for Grid2D<T> {
//     fn from_par_iter<I>(par_iter: I) -> Self
//     where
//         I: rayon::iter::IntoParallelIterator<Item = T>,
//     {
//         todo!()
//     }
// }
// impl<T: Send> FromParallelIterator<((usize, usize), T)> for Grid2D<T> {
//     fn from_par_iter<I>(par_iter: I) -> Self
//     where
//         I: rayon::iter::IntoParallelIterator<Item = ((usize, usize), T)>,
//     {
//         todo!()
//     }
// }

// TODO: Should I have this?
// impl<T> Deref for Grid2D<T> {
//     type Target = Array2<T>;
//     fn deref(&self) -> &Self::Target {
//         &self.arr
//     }
// }
