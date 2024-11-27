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



