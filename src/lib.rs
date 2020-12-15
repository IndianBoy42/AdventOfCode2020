#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(incomplete_features)]
#![allow(dead_code)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::inline_always)]
#![allow(clippy::must_use_candidate)]
#![feature(try_blocks)]
#![feature(array_value_iter)]
#![feature(associated_type_bounds)]
#![feature(const_generics)]
// #![feature(const_generic_impls_guard)]
#![feature(split_inclusive)]
#![feature(iter_partition_in_place)]
#![feature(map_entry_replace)]
#![feature(specialization)]
#![feature(impl_trait_in_bindings)]
#![feature(try_trait)]
#![feature(bindings_after_at)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_uninit_array)]
#![feature(array_chunks)]
#![feature(array_windows)]
#![feature(peekable_next_if)]
#![feature(str_split_once)]
#![feature(test)]
#![feature(const_panic)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day111;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
pub mod day2;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub mod grid;
pub mod searcher;
pub mod u32set;
pub mod utils;
