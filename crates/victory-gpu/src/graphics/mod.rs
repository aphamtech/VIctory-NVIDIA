//! High-performance rendering subsystems for running Xbox Series X|S builds on
//! discrete NVIDIA RTX GPUs:
//!
//! - [`contracts`]: Shared value types, error enums, and Rust trait contracts.
//! - [`uma_bridge`]: Subsystem A — UMA-to-Discrete Memory & Barrier Translator.
//! - [`velocity_io`]: Subsystem B — Velocity I/O (`DStorageX`) Async GPU Pipeline.

pub mod contracts;
pub mod uma_bridge;
pub mod velocity_io;
