#![no_std]

pub mod space;

pub use page_table::*;
pub use space::{AddressSpace, PageManager};

#[cfg(test)]
mod tests;
