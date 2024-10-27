//! A 6502 CPU emulator written in Rust
//!
//! This crate provides a 6502 CPU emulator that can be used to run 6502 machine
//! code. It provides a simple API for executing instructions and accessing the
//! CPU's registers and memory.

#![warn(missing_docs)]
pub mod addressing_modes;
pub mod bus;
pub mod cpu;
pub mod instructions;
pub mod registers;

#[cfg(test)]
mod tests;
