# Renderer requirements for Sharpview

John Nagle

nagle@animats.com

December, 2024
## Introduction
Right now, the Second Life/Open Simulator clients use various back ends.

- Sharpview uses Rend3/WGPU/Vulkan.
- The Linden Lab desktop viewer uses a custom C++ renderer and OpenGL.
- The Linden mobile viewer uses Unity, as does another desktop client.

All have major performance problems. 
## Basics
Sharpview needs roughly the same rendering capabilities needed to render the standard "Bistro" test scene.
All five known renderers (Rend3/WGPU, Renderling, Orbit, the SL C++ Renderer, and Unity) have this capability.
None of them really get the performance out of the GPU that the GPU is capable of delivering.

## API
The API offered by the renderer needs a rather standard set of structs.
All the known renderers have roughly equivalent data structures for this.
In Rust, all of these are handles that point to an Arc. When the reference count goes to 0, they are dropped.
The GPU state is updated appropriately.

### The major types

#### Renderer
Created once at startup, All other objects are created by functions on Renderer.

The application has to make various calls on every frame to make the renderer go. 
The renderer thread also has the window event loop, which is the Rust Winit crate.

#### Mesh
The usual mesh type - arrays of vertices, triangles, UVs, and normals.
Everybody does this in about the same way, since that's what the GPU wants.

Rigged meshes should support up to 4 weights per vertex.
Non-rigged meshes should use a data structure with no space wasted on weights.

Meshes are immutable. They can be replaced, but not changed.

#### Texture
A texture is just an 2D image. An alpha channel is optional. No 3D textures. MIP-mapping may be supported.

Textures are immutable. They can be replaced, but not changed.

#### Material
A material is a structure which references textures and contains other information, such as base color.
At least the standard glTF texture layers should be suppported.
Each layer has some parameters and, optionally, a handle to a texture.

The standard layers:
- Albedo - usual glTF meaning. RGB base color, with optional alpha.
- Roughness - usual glTF meaning
- Emissive - usual glTF meaning
- Metallic - usual glTF meaning
- Ambient occlusion - usual glTF meaning, although rarely used.
- Normals - usual glTF meaning.

Someday it would be nice to have subsurface scattering, for better skin tone, but that's still an optional glTF extension.
No rush on that.

Materials are mutable. They can be changed, and this affects further rendering of the object.

It's very common to change texture offsets on every frame, to animate textures. So that should be cheap.
Other changes are less frequent and can be more expensive.

#### Transform
Transforms are the usual 4x4 transformation matrices, from object space to world space.

Sharpview has a hierarchy internally, but what it sends the renderer is transforms to world space, not to parent objects.
#### Drawable, or Object
Creating a Drawable puts something on the screen. Dropping a drawable makes it disappear. The Drawable struct contains
handles for mesh and material, an optional skeleton, and the actual 4x4 transform.

#### Light
The usual glTF punctual lights, plus a sun, are needed.  Punctual lights have a distance, falloff, and cone angle, as usual.

The sun should throw shadows. 
If a few lights can throw shadows, that's nice.
The OpenGL tradition is eight lights - sun, moon, and the nearest six others.

Large numbers of non-shadow-casting lights should be supported.
When shadow resources are limited, make lights other than the sun non shadow casting and limit their range.

Second Life also has "projectors" - cone lights that project a texture. This is not used much, and isn't a high priority.

#### Skeleton
Usual glTF type skeletons. 

#### Camera
Usual 4x4 camera matrix with a viewing frustum.

## Performance
These are the known bottlenecks.

### Bindless, or lack thereof.
For this application, each mesh stands alone, as a Drawable. So the draw loop is bind, draw, unbind, repeat.
Maybe half the drawing time is going into binding.

My current thinking on bindless textures is that they should work roughly like this:

- Application requests a descriptor slot. It gets back an opaque handle.
- Application requests a texture buffer and provides data to fill it. Buffer is created and filled.
Knowing when this is completed is important. Completed means the data is in the GPU, ready for use.
The requestor could either be synchronous (thread blocks) or async (some kind of callback or async notification).
These requests will generally be made from asset loading threads, of which there are 8 to 16. So it's OK if they block.
That won't stall other requests. 
- Application requests a descriptor slot and provides a texture It gets back an opaque handle.
Descriptor manager takes ownership of the buffer, preventing it from being changed or deleted by the application. This queues insertion into the descriptor array.
- Dropping the descriptor slot handle queues deletion from the descriptor array.
- All descriptor array updates are processed at a point in the render cycle where the GPU is not rendering.

Draw calls refer to textures by index number, obtained from the descriptor slot handle.
Unused slots are filled either Vulkan's NULL_HANDLE or a link to some ugly error texture so bugs are visible.

The idea is to use Rust ownership to manage most of the interlocking, and to avoid barriers that can stall rendering.
If you let the application mess with the descriptor array, you need more checking in the lower layers.
More machinery to implement.

Bindless mode exists to improve performance by spending less time doing binding. It's only useful if it provides a big reduction in binding overhead.

### Thread support.
None of the renderers do multi-thread very well. Sharpview requests all updates from threads other than the render thread.
But within Rend3, they all go into an instruction queue, and the render thread does all the work.
Updating thus impacts the frame rate. During periods of heavy updating, frame rate can drop by 3x. 

Vulkan has the concurrency to do this right, but that's lost at the Rend3 level.

A transfer queue to the GPU might help, to get content loading completely off the render thread.

True multi-thread rendering, where several threads are making draw calls, would be nice, but is not essential at this point.
If content loading, binding, and translucency stop impacting rendering, performance should improve enough for now.

### Lighting and shadows.
Rend3's lighting system compares every light with every object. This is far too slow.
So, at present, the only light in Sharpview is the sun. 
All objects in Sharpview are currently given some emissive output so you can see indoors.

A more efficient lighting system is needed. If the Bistro test scene looks good, then that's been solved.

### Translucency
Too much time is going into the depth sort for translucency. Order-independent translucency is desirable.
Second Life and Open Simulator content has a lot of windows. For those, simple order-independent translucncy is good enough.
As long as it works well on translucent windows that are close to grey, that's fine.
Looking through layers of colored glass bottles does not need to be color-accurate.
Don't need the fancy systems which have a list stored for every pixel. Go for a one-pass commutative operator system.

### Culling
Frustum culling seems to be essential. Occlusion culling was put into Rend3 and slowed things down, and introduced bugs.

### Depth of field
Not really necessary. But if present, it will be used.

## 2D layer integration
It's necessary to bolt a 2D layer, for menus and such, onto all this. EGUI seems to be the way to go. Rend3 uses EGUI.
So an EGUI integration is needed.

## Additional renderers
- Water (the camera can go underwater)
- Fog - fade out with distance
- Skybox

## Mirrors
The Second Life C++ renderer now has full mirror and reflective object support.
This includes automatic and manual reflection probes so that objects reflect the environment properly.
It slows down rendering.
It's a nice feature to have someday, is but not needed immediately.

## Conclusion
So that's what it takes to do a good-looking metaverse.
It's been done, but the existing pre-Vulkan implementations are just too slow.
So the goal here is to use modern GPUs and computers with many CPUs effectively.

