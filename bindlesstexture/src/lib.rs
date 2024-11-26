//! # Bindless texture library
/*
A possible API for texture buffers
This is a sketch of an API at the Rust wrapper layer for texture buffers. It’s usable in both single-thread and multi-thread programs. This uses bindless texture handling exclusively.
Most of these operations do not block, and do not require locking up other parts of the system. This should reduce lock contention problems between rendering and updating, a serious problem with current APIs.
Texture buffers are immutable. The caller requests a new buffer, loads it with content, and gives it to the GPU. Shaders can then use the texture buffer. When the caller is done with the texture buffer, it drops the TextureBufferHandle, and the next time the GPU finishes a frame, that texture buffer is dropped. This prevents removing a buffer while the GPU is using it.

pub fn TextureBuffer::new(
	device_context: &DeviceContext, 
	attributes: TextureBufferAttributes, 
	size: usize) ->  
	Result<TextureBufferHandle, Error> 

This new call allocates a texture buffer in the GPU and a descriptor slot in the GPU for bindless mode. The returned value is an opaque handle which owns both the buffer and the descriptor slot. Dropping the TextureBufferHandle will result in the release of the texture buffer at the end of the current frame. This operation should not block.
This call can be made from any thread at any time, and should not be blocked due to operations in progress. 
impl TextureBufferHandle {

	pub fn write(&self, b: Vec<u8>) -> Result<(), Error> 

Moves the indicated data into the texture buffer. The write may not be immediate. This may create a transfer queue operation for the GPU. This operation should not block.

	pub fn wait(&self) -> Result<(), Error> 

Delays the calling thread until any pending transfer operations on the buffer are complete. This operation blocks.

	pub fn map_to_gpu(&mut self) -> Result<(), Error>

Transfers mapping of this texture buffer to the GPU. Shaders can now read the buffer. 
(Does this need to block? Or is it something you wait for later with “wait”?)

	pub fn get_index(&self) -> u32

Returns the index of the buffer in the descriptor table. This is the index number used in shaders. This never block
	pub fn wait_all(device_context: &DeviceContext) -> Result<(), Error>

Wait for all in-progress texture buffer operations for this device to complete. Single-thread programs will probably start up all their operations and then call wait_all. Multi-thread programs will probably use wait instead. This operation blocks.
*/
use std::sync::{Arc};
use anyhow::{Error};

/// Dummy objects for stub use only
pub struct Dummy {}
type DeviceContext = Dummy;
type TextureBufferAttributes = Dummy;

/// TextureBufferHandle - the CPU's handle for a texture buffer in the GPU
pub struct TextureBufferHandle {
    /// Descriptor index
    _index: u32,
    /// Device context to which this handle belongs
    _context: Arc::<DeviceContext>
}

impl TextureBufferHandle {

    /// This new call allocates a texture buffer in the GPU and a descriptor slot in the GPU for bindless mode.
    /// The returned value is an opaque handle which owns both the buffer and the descriptor slot.
    /// Dropping the TextureBufferHandle will result in the release of the texture buffer at the end of the current frame.
    ///
    /// This operation should not block.
    /// This call can be made from any thread at any time.
    pub fn new(
	    _device_context: &Arc::<DeviceContext>, 
	    _attributes: TextureBufferAttributes, 
	    _size: usize) ->  
	    Result<Self, Error> {
	        
        todo!();    // UNIMPLEMENTED
    }
    
    /// Moves the indicated data into the texture buffer. The write is not immediate.
    /// This will create a transfer queue operation for the GPU. This operation should not block.   
    pub fn write(&self, _b: Vec<u8>) -> Result<(), Error> {
        todo!();    // UNIMPLEMENTED
    }
    
    /// Delays the calling thread until any pending transfer operations on the buffer are complete.
    ///`This operation blocks.
    pub fn wait(&self) -> Result<(), Error> {
        todo!();    // UNIMPLEMENTED
    }   
}

