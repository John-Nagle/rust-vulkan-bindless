# Bindless Rust rendering interfaces for higher performance rendering

John Nagle

Animats

nagle@animats.com

November, 2024

**PRELIMINARY**

## Introduction
Vulkan, Direct-X, and Metal all have features for high-performance rendering which are not available at the Rust level in the existing interface crates. This crate addresses that problem.

This crate is intended to be part of a graphics stack. It sits above Vulkan and below a renderer. It's thus a "wrapper", at roughly the same level as Vulkano and WGPU. In addition to wrapping Vulkan in Rust form, it also provides GPU allocation and updating of bindless descriptors. That needs to be at this level to present a Rust safe interface to the renderer, and to avoid locking bottlenecks.

This crate is intended to offer a safe Rust interface.

## Goals
- This crate is intended for multi-thread use but should still work for single-thread programs.
- Rendering thread work should be draw, draw, draw, with no binding delays during drawing.
- Other threads update the content concurrently with rendering.

## Background
### Game loops
#### Single thread
The usual game loop looks like this:
- Wait for vertical sync (optional).
- Draw frame, running render passes.
- Draw 2D GUI.
- Network and local file activity.
- Perform game calculations for next frame.
- Housekeeping.

This is traditional, and many games still work this way. It limits performance.

#### Multi-thread
By now, almost all computers are multiprocessors. Even the cheapest Raspberry Pi now has four cores. High performance game development requires using more than one core. Unreal Engine started doing this with Unreal Engine 3 in 2005, almost twenty years ago. Multi-thread games often look something like this:

Rendering thread(s) loop
- Wait for vertical sync (optional)
- Trigger update thread to run in parallel with rendering
- Draw frame, running render passes.
- Draw 2D GUI
- Rendering thread housekeeping

High frame rate on complex scenes requires that the rendering thread rarely be forced to wait for anything else.
This constraint dominates performance. That’s the main concern of this approach. 

Movement thread(s) loop:

- Wait for trigger from rendering thread.
- Perform game calculations for next frame
- Movement thread housekeeping

The movement thread should finish its work in less than one frame time.

Networking thread(s)
- Talk to game servers.
- Talk to local file system.
- May be asynchronous to rendering

Housekeeping threads
- Cache and LOD management
- Asset fetch control

### The Rust graphics stack
Rust has several graphics stacks, all of which have roughly the same structure.
- Application or game engine (Bevy, Fyrox, non-engine applications) – the fun part that does the game.
- Renderer (Renderling, Rend3, Bevy renderer) – Rendering passes, shadows and lighting, graphics object management.
- Rust wrapper layer (Vulkano, WGPU, Ash) – Puts a Rust API around the target graphics layer. Abstracts over multiple target graphics layers.
- Target graphics layer (Vulkan, Metal, Direct-X, OpenGL, Android, WebGPU) – C or C++.
This document is concerned about the interface between the renderer layer and the Rust wrapper layer.

#### Previous work
Most of the Rust wrapper layers offer a Vulkan-type interface to the renderer layer above.
Vulkano and WGPU try to offer a safe Rust interface, while Ash simply re-exports Vulkan’s API as unsfe Rust.
This is the obvious boundary between the layers.
It seems to be a reasonable perimeter at which to start enforcing Rust safety.

Inside the perimeter, raw pointers predominate. Outside of it, they should be encapsulated in proper Rust constructs.
This turns out to be bad for performance.
It's an architectural problem. Vulcano and WGPU try to create a Rust safety perimeter at an API that's basically a wrapper around Vulkan.
This is not the best boundary for that safety perimeter.
Moving buffer allocation inside the safety perimeter eliminates much locking and checking.
Bindless really brings this out, because somebody has to keep the descriptor table and buffer allocation in sync.
The GPU depends on that. So that has safety implications.

If this problem is partitioned differently, the locking problems for concurrent GPU content updating may become simpler.
Right now, both Vulcano and WGPU force more serialization than Vulkan itself requires. So do the renderers.
The rendering thread is too often stalled on a lock waiting for some content updating operation that should not interfere with rendering.

#### New design
Rust-based texture loading needs to work something like this:

- Application layer requests a new, empty GPU texture buffer of the appropriate size and type, and gets back an opaque buffer handle which is an index into the buffer descriptor table for bindless mode, or an out-of-memory error. It is the responsibility of the allocation system to keep the buffer allocation and the buffer descriptor table in sync. The buffer is owned by the CPU and may not be used by the GPU yet.
- Application layer provides texture data to fill that buffer, and the data is copied into the GPU buffer. (Unclear if this is done by mapping the buffer into CPU space and having the CPU do the copy, or by enqueueing the data on a transfer queue so the GPU can do the copy. I don't know enough Vulkan yet to answer that.)
- Application requests that the buffer be given to the GPU for use there. This probably queues a request.
- At the end of the frame, all those queued handover requests are processed. Shaders can now use the buffer given the index to the descriptor table.
- When the application drops a buffer handle, a request is queued to drop the buffer. At the end of the frame, all queued drops take place on the GPU side

This allocation machinery belongs inside a crate at the WGPU/Vulkano level, near the top. The point is that it's easier to wrap a safe interface around this functionality than around one that passes GPU buffer addresses around. This approach looks self-sufficient for locking. If the application owns the buffer, it can write it, if the GPU owns the buffer, it can read it, all mapping switches happen when the GPU is at end of frame, but content copying can happen while other things are going on.

The main thing that can go wrong is that a buffer index used by a shader might be invalid. This can be handled by insuring that all unused slots contain a Vulkan null handle value when not pointed at a valid buffer.


