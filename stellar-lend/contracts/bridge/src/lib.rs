#![no_std]
#![allow(deprecated)]
mod bridge;

pub use bridge::{BridgeContract, ContractError};

#[cfg(test)]
mod test;
#[cfg(test)]
mod math_safety_test;
