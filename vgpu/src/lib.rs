//! # Bindless texture library
//!
//! John Nagle
//! Animats
//! November, 2024
//!
mod testdummies;
pub mod stubs;
pub mod wgputypes;

//  Exports
pub use stubs::{Instance, PowerPreference, Features, Limits, PrimitiveState, MultisampleState };

pub use wgputypes::{Color, LoadOp, StoreOp, Operations, MemoryHints};
