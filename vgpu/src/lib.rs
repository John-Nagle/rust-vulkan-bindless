//! # Bindless texture library
//!
//! John Nagle
//! Animats
//! November, 2024
//!
pub mod stubs;
mod testdummies;
pub mod wgputypes;

//  Exports
pub use stubs::{Features, Instance, Limits, MultisampleState, PowerPreference, PrimitiveState};

pub use wgputypes::{Color, LoadOp, MemoryHints, Operations, StoreOp};
