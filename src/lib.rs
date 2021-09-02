#![feature(register_tool)]
#![register_tool(tarpaulin)]

use inexor_rgf_core_model as model;
use inexor_rgf_core_reactive as reactive;

#[derive(Debug)]
pub struct BehaviourCreationError;

pub mod entity;
pub use entity::*;

pub mod relation;
pub use relation::*;

#[cfg(test)]
#[tarpaulin::ignore]
pub mod tests;
