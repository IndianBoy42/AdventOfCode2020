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

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod utils;

use utils::*;