//! # Bindless texture library
//!
//! John Nagle
//! Animats
//! November, 2024
//!
use anyhow::Error;
use std::sync::Arc;

/// Dummy objects for stub use only
pub struct Dummy {}
type DeviceContext = Dummy;
type TextureBufferAttributes = Dummy;

/// TextureBufferHandle - the CPU's handle for a texture buffer in the GPU
pub struct TextureBufferHandle {
    /// Descriptor index
    _index: u32,
    /// Device context to which this handle belongs
    _context: Arc<DeviceContext>,
}

impl TextureBufferHandle {
    /// This new call allocates a texture buffer in the GPU and a descriptor slot in the GPU for bindless mode.
    /// The returned value is an opaque handle which owns both the buffer and the descriptor slot.
    /// Dropping the TextureBufferHandle will result in the release of the texture buffer at the end of the current frame.
    ///
    /// This operation should not block.
    /// This call can be made from any thread at any time.
    pub fn new(
        _device_context: &Arc<DeviceContext>,
        _attributes: TextureBufferAttributes,
        _size: usize,
    ) -> Result<Self, Error> {
        todo!(); // UNIMPLEMENTED
    }

    /// Moves the indicated data into the texture buffer. The write is not immediate.
    /// This will create a transfer queue operation for the GPU. This operation should not block.   
    pub fn write(&self, _b: Vec<u8>) -> Result<(), Error> {
        todo!(); // UNIMPLEMENTED
    }

    /// Delays the calling thread until any pending transfer operations on the buffer are complete.
    ///`This operation blocks.
    pub fn wait(&self) -> Result<(), Error> {
        todo!(); // UNIMPLEMENTED
    }
}
