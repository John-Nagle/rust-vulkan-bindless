# Renderer requirements for Sharpview

John Nagle

nagle@animats.com

December, 2024
## Introduction
## Basics
Sharpview needs roughly the same rendering capabilities needed to render the standard "Bistro" test scene.
All four known renderers (Rend3/WGPU, Renderling, Orbit, and the SL C++ Renderer have this capability.

## API
The API offered by the renderer needs a rather standard set of structs.
All the known renderers have roughly equivalent data structures for this.
In Rust, all of these are handles that point to an Arc. When the reference count goes to 0, they are dropped.
The GPU state is updated appropriately.

### The major types

#### Renderer
Created once at startup, All other objects are created by functions on Renderer.

#### Mesh
The usual mesh type - arrays of vertices, triangles, UVs, and normals.
Everybody does this in about the same way, since that's what the GPU wants.

Meshes are immutable. They can be replaced, but not changed.

#### Texture
A texture is just an 2D image. No 3D textures. MIP-mapping may be supported.

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

#### Transform
Transforms are the usual 4x4 transformation matrices, from object space to world space.

#### Drawable, or Object.
Creating a Drawable puts something on the screen. Dropping a drawable makes it disappear. The Drawable struct contains
handles for mesh and material, and the actual 4x4 transform.
