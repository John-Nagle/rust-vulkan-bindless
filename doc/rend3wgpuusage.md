# Rend3 usage of WGPU features
To allow using Rend3 on top of rust-vulkan-bindless, it's necessary to document what features of WGPU Rend3 actually uses.

## Appearances of wgpu and wgt in Rend3

### In rend3-types
Re-exports:

    pub use wgt::{
        AstcBlock, AstcChannel, Backend, Backends, Color, DeviceType, PresentMode, TextureFormat,
        TextureFormatFeatureFlags, TextureUsages,
    };
    
Usages:

    wgt::FrontFace
    
### In rend3 surface.rs
    use wgpu::{CompositeAlphaMode, Device, SurfaceConfiguration};
    
### In rend3 egui interface
    internal: &mut egui_wgpu::Renderer,
    image_texture: wgpu::Texture,
    
    *EGUI has more direct use of WGPU than the rest of Rend3. There are more items related to EGUI*
    
### In rend3 upload.rs
    use wgpu::{Buffer, CommandEncoder, Device};
    
### In rend3 store.rs
    use wgpu::TextureView;

### In rend3 mipmap.rs
    use wgpu::{
        AddressMode, BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Color,
            #[source]
            source: wgpu::Error,
        },
        
    wgpu::include_wgsl!
    
### In rend3 output.rs
    use wgpu::{SurfaceTexture, TextureView};

### In rend3 util/typedefs.rs
    wgpu_profiler::GpuTimerQueryResult
    
### In rend3 encpass.rs
    use wgpu::{CommandEncoder, RenderPass};
    use wgpu_profiler::ProfilerCommandRecorder;
    
    &wgpu::QuerySet
    
### In rend3 renderer/setup.rs   
    use wgpu::TextureViewDimension;
    use wgpu_profiler::GpuProfilerSettings;

### In rend3 managers/point.rs
    use wgpu::{BufferUsages, Device, ShaderStages};
            ShaderStages::FRAGMENT,
            wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                
### In rend3 util/bind_merge.rs
    use wgpu::{
        BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
        BindingResource, BindingType, Buffer, BufferBinding, BufferBindingType, Device, Sampler, ShaderStages, TextureView,
    };
    
### In rend3 forward.rs
    use wgpu::{
        BindGroup, BindGroupLayout, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState,
        FragmentState, IndexFormat, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode,
        PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModule, StencilState,
        TextureFormat, VertexState,
    };
    
    &wgpu::BufferDescriptor
    
### In egui example
    wgpu::TextureFormat::Rgba8UnormSrgb;
    
### In skinning.rs
    use wgpu::{
        BindGroupLayout, Buffer, BufferBindingType, BufferDescriptor, BufferUsages, CommandEncoder, ComputePassDescriptor,
        
    &wgpu::Device, 
    wgpu::ShaderSource::Wgsl
    
### In texture_store.rs
    use wgpu::{Device, Extent3d, Texture, TextureDescriptor, TextureDimension};
    
### In texture.rs
    use wgpu::{
        util::DeviceExt, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
        BindGroupLayoutEntry, BindingResource, BindingType, CommandBuffer, CommandEncoder, CommandEncoderDescriptor,
        Device, Extent3d, Features, ImageCopyTexture, ImageDataLayout, Origin3d, ShaderStages, Texture, TextureAspect,
        TextureDescriptor, TextureDimension, TextureSampleType, TextureView, TextureViewDescriptor, TextureViewDimension,     

    wgpu::TextureDimension::D3,
    wgpu::TextureDimension::D2,
    wgpu::util::TextureDataOrder::LayerMajor,
    
### In rend3/lib.rs
    pub use wgpu::{PresentMode, Surface, SurfaceError};
    wgpu::TextureFormat::Depth32Float;   
    
### In mesh.rs
    use wgpu::{
        Buffer, BufferAddress, BufferDescriptor, BufferUsages, CommandBuffer, CommandEncoder, CommandEncoderDescriptor,    
        
### In skeleton.rs
    wgpu::Device
    
### In graph.rs
    use wgpu::{
        Buffer, CommandBuffer, CommandEncoder, CommandEncoderDescriptor, LoadOp, Operations, RenderPass,
        RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp, SurfaceTexture,
        Texture, TextureView, TextureViewDescriptor,
    };
        
    wgpu::Color
    
### In texture dedup
    use wgpu::{
        BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
        BindingResource, Device, ShaderStages, TextureViewDimension,
    };
    
    wgpu::BindingType::Texture
    wgpu::TextureSampleType::Float
    
### In routine.rs
    use wgpu::{BlendState, ShaderModuleDescriptor, ShaderSource};
    
### In samplers.rs
    use wgpu::{
        AddressMode, BindingType, CompareFunction, Device, FilterMode, Sampler, SamplerBindingType, SamplerDescriptor,
        ShaderStages,
    };
    
### In framework lib.rs
    use wgpu::{Instance, PresentMode, SurfaceError};
    wgpu::Texture,
    
### In managers object.rs
    use wgpu::{Buffer, CommandEncoder, Device};
    
### In rend3_routine base.rs
    use wgpu::BindGroup;
    
### In util buffer.rs
    use wgpu::{Buffer, BufferAddress, BufferDescriptor, BufferUsages, Device, Queue};

### In error.rs
    use wgpu::Features;
    use wgpu_profiler::CreationError;
    
### In examples scene_viewer.rs
    wgpu::PresentMode::Fifo
    
### In managers material.rs
    use wgpu::{BindGroup, BindGroupLayout, BindingType, Buffer, BufferBindingType, CommandEncoder, Device, ShaderStages};
    
### In instruction.rs
    use wgpu::{CommandBuffer, Device};
    
### In scatter_copy.rs
    use wgpu::util::DeviceExt;
    
    wgpu::Device,
    wgpu::Queue,
    wgpu::Backends
    wgpu::InstanceDescriptor
    
### In eval.rs
    use wgpu::CommandEncoderDescriptor;
    
### In freelist.rs
    use wgpu::{Buffer, BufferDescriptor, BufferUsages, CommandEncoder, Device};
    
### In uniforms.rs
    use wgpu::{BindGroup, BufferUsages};
    &wgpu::BufferDescriptor
    
### In error_scope.rs
    use wgpu::Device;
    
### In interfaces.rs
    use wgpu::{
        BindGroupLayout, BindingType, BufferBindingType, Device, ShaderStages, TextureSampleType, TextureViewDimension,
    };
    
### In renderer mod.rs
    use wgpu::{Device, DownlevelCapabilities, Features, Limits, Queue};
    use wgpu_profiler::GpuProfiler;
    
### In graph mod.rs
    use wgpu::{Extent3d, TextureDimension, TextureView};
    
### In setup.rs
    use wgpu::{
        Adapter, AdapterInfo, Backend, Backends, BufferAddress, Device, DeviceDescriptor, DeviceType, Features,
        Gles3MinorVersion, Instance, InstanceFlags, Limits, Queue,
    };

### In directional.rs
    use wgpu::{
        BindingType, BufferBindingType, BufferUsages, Device, Extent3d, ShaderStages, TextureDescriptor, TextureDimension,
        TextureUsages, TextureView, TextureViewDescriptor,
    };
    
### In examples tests.rs
    wgpu::TextureFormat
    
### In rend_tests runner.rs
    use wgpu::{
        Extent3d, ImageCopyBuffer, ImageDataLayout, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    };
    
### In helpers.rs
    use wgpu::Device;
    
### In tests simple.rs
    use wgpu::FrontFace;
    
## Initial classification
These types from WGPU can be classified as

- Can be copied literally from WGPU
- Maps directly to some type in Ash/Vulkan
- Needs new implementation
- Needs redesign due to movement of allocation to the *rust-vulkan-bindless* level.

## The major types from WGPU
These are the ones which have significant implementations behind them.

### Device
Ref: https://github.com/gfx-rs/wgpu/blob/1a643291c2e8854ba7e4f5445a4388202731bfa1/wgpu/src/api/device.rs#L32

*Device* has the creation functions for most of the major structs - Buffer, Texture, PipelineLayout, etc.
It's mostly a container for *Data*, which is an *Any*.

There's extensive indirection here, because WGPU supports so many back ends.

Textures seem to be created at https://github.com/gfx-rs/wgpu/blob/trunk/wgpu-core/src/device/resource.rs#L726

### Buffer
Different for different back ends.
Encapulates wgt::BufferAddress
Not much bookeeping info.

Seems to be a Vulkan-level big buffer, not a user-level buffer.
Has raw GPU addresses.

### CommandEncoder

### Texture
Texture seems to be a synonym for Resource.

### TextureDescriptor
Ref: https://github.com/gfx-rs/wgpu/blob/5e52a313b996344b67838c459b3e5f4ca0c3b322/wgpu-types/src/lib.rs#L6087

### RenderPass

### PipelineLayout

### BindGroup

## Memory allocation
### Heap questions
(asked in r/vulkan)

Ref: https://media.gdcvault.com/gdc2018/presentations/Sawicki_Adam_Memory%20management%20in%20Vulkan.pdf

The Vulkan spec is very general in this area. There are a huge number of options.

The Vulkan spec says there's some number of heaps for each implementation. There's no indication in the spec of how many. One? Two? 65535? I gather from this 2018 GDC presentation that there are very few, rarely more than three. Apparently there is rarely if ever more than one heap of a given type. Is that correct? The main types seem to be unshared CPU memory, unshared device memory, and various slow shared variants which may or may not be supported. Or the other extreme, the integrated graphics case, where everything is in one memory system. Are those pretty much the real world options, or are there other variants?

The Vulkan spec describes allocate and free functions. But the GDC presentation indicates these are very limited, or at least were back in 2018. The number of allocations is limited; that presentation suggests 4K. (Where does that number come from? Can it be read from the Vulkan API?) So you can't just allocate space for each texture with its own Vulkan allocate call. I think. The general idea seems to be to allocate big blocks (256MB was suggested) and then subdivide them with some kind of suballocator. Is that correct? Any comments on memory fragmentation problems.

Finding out how much device local memory is available was apparently hard back in 2018. Is that fixed? What's best practice today on getting a lot of device memory but not locking up the system because you grabbed all of it and nothing else can run?

Spilling from device memory to slower CPU memory accessed via the PCI bus is apparently something some Vulkan implementations can do. Or will do without being asked. When that happens, there's a big performance drop. How is that detected, prevented, or managed?

Is there something I should read that's more current than that 2018 presentation but covers the same material? Thanks. 

## Implementation plan
Get something working and advance to more complex things working.
### Phase I 
- Start with the simplest cube example from WGPU tests.
- Create stubs for everything it calls.
- Implement and test allocation without actually doing graphics.
- Run example to test and log allocation
- Add actual Ash/Vulkan graphics and test.

### Phase II
- Make phase I example bindless.
- Test with large number of objects.

### Phase III
- Expand to more complex examples.

### Phase IV
- Integrate with Rend3.
- Get cube example working.
- Get scene-viewer example working.

### Phase V
- Get render-bench working.
- Get render-bench working in bindless mode.
- Benchmark

### Phase VI
- Convert Sharpview
- Benchmark


