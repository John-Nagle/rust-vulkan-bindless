//! stubs.rs -- dummy stubs to be replaced with real code.

/// These are types that WGPU defines and which must be emulated.

/// Instance
use winit::window::{Window};
use anyhow::{Error};
#[derive(Default)]
pub struct Instance{}

impl Instance {
    pub fn create_surface(&self, _window: &Window) -> Result<Surface, Error> {
        Ok(Surface {})
    }
}

pub struct Surface {}

/// PowerPreference
#[derive(Default)]
pub struct PowerPreference{}

/// Features
pub struct Features{}

impl Features {
    pub fn empty() -> Self {
        Self {}
    }
}

/// Limits
pub struct Limits{}

impl Limits {
    pub fn downlevel_webgl2_defaults() -> Self {
        Self {}
    }
}

/// PrimitiveState
#[derive(Default)]
pub struct PrimitiveState{}

/// MultisampleState
#[derive(Default)]
pub struct MultisampleState{}


/*  --> examples/src/hello_triangle/mod.rs:14:26
   |
14 |     let instance = wgpu::Instance::default();
   |                          ^^^^^^^^ could not find `Instance` in `wgpu`

error[E0433]: failed to resolve: could not find `PowerPreference` in `wgpu`
  --> examples/src/hello_triangle/mod.rs:19:37
   |
19 |             power_preference: wgpu::PowerPreference::default(),
   |                                     ^^^^^^^^^^^^^^^ could not find `PowerPreference` in `wgpu`

error[E0433]: failed to resolve: could not find `Features` in `wgpu`
  --> examples/src/hello_triangle/mod.rs:32:42
   |
32 |                 required_features: wgpu::Features::empty(),
   |                                          ^^^^^^^^ could not find `Features` in `wgpu`

error[E0433]: failed to resolve: could not find `Limits` in `wgpu`
  --> examples/src/hello_triangle/mod.rs:34:40
   |
34 |                 required_limits: wgpu::Limits::downlevel_webgl2_defaults()
   |                                        ^^^^^^ could not find `Limits` in `wgpu`

error[E0433]: failed to resolve: could not find `ShaderSource` in `wgpu`
  --> examples/src/hello_triangle/mod.rs:45:23
   |
45 |         source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
   |                       ^^^^^^^^^^^^ could not find `ShaderSource` in `wgpu`

error[E0433]: failed to resolve: could not find `PrimitiveState` in `wgpu`
  --> examples/src/hello_triangle/mod.rs:72:26
   |
72 |         primitive: wgpu::PrimitiveState::default(),
   |                          ^^^^^^^^^^^^^^ could not find `PrimitiveState` in `wgpu`

error[E0433]: failed to resolve: could not find `MultisampleState` in `wgpu`
  --> examples/src/hello_triangle/mod.rs:74:28
   |
74 |         multisample: wgpu::MultisampleState::default(),
   |                            ^^^^^^^^^^^^^^^^ could not find `MultisampleState` in `wgpu`

error[E0433]: failed to resolve: could not find `TextureViewDescriptor` in `wgpu`
   --> examples/src/hello_triangle/mod.rs:112:49
    |
112 | ...                   .create_view(&wgpu::TextureViewDescriptor::default());
    |                                           ^^^^^^^^^^^^^^^^^^^^^ could not find `TextureViewDescriptor` in `wgpu`

error[E0433]: failed to resolve: could not find `LoadOp` in `wgpu`
   --> examples/src/hello_triangle/mod.rs:125:57
    |
125 | ...                   load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
    |                                   ^^^^^^ could not find `LoadOp` in `wgpu`

error[E0433]: failed to resolve: could not find `StoreOp` in `wgpu`
   --> examples/src/hello_triangle/mod.rs:126:58
    |
126 | ...                   store: wgpu::StoreOp::Store,
    |                                    ^^^^^^^ could not find `StoreOp` in `wgpu`

error[E0433]: failed to resolve: could not find `WindowBuilder` in `window`
   --> examples/src/hello_triangle/mod.rs:151:38
    |
151 |     let mut builder = winit::window::WindowBuilder::new();
    |                                      ^^^^^^^^^^^^^ could not find `WindowBuilder` in `window`

error[E0422]: cannot find struct, variant or union type `RequestAdapterOptions` in crate `wgpu`
  --> examples/src/hello_triangle/mod.rs:18:33
   |
18 |         .request_adapter(&wgpu::RequestAdapterOptions {
   |                                 ^^^^^^^^^^^^^^^^^^^^^ not found in `wgpu`

error[E0422]: cannot find struct, variant or union type `DeviceDescriptor` in crate `wgpu`
  --> examples/src/hello_triangle/mod.rs:30:20
   |
30 |             &wgpu::DeviceDescriptor {
   |                    ^^^^^^^^^^^^^^^^ not found in `wgpu`

error[E0422]: cannot find struct, variant or union type `ShaderModuleDescriptor` in crate `wgpu`
  --> examples/src/hello_triangle/mod.rs:43:52
   |
43 |     let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
   |                                                    ^^^^^^^^^^^^^^^^^^^^^^ not found in `wgpu`

error[E0422]: cannot find struct, variant or union type `PipelineLayoutDescriptor` in crate `wgpu`
  --> examples/src/hello_triangle/mod.rs:48:64
   |
48 |     let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
   |                                                                ^^^^^^^^^^^^^^^^^^^^^^^^ not found in `wgpu`

error[E0422]: cannot find struct, variant or union type `RenderPipelineDescriptor` in crate `wgpu`
  --> examples/src/hello_triangle/mod.rs:57:64
   |
57 |     let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
   |                                                                ^^^^^^^^^^^^^^^^^^^^^^^^ not found in `wgpu`

error[E0422]: cannot find struct, variant or union type `VertexState` in crate `wgpu`
  --> examples/src/hello_triangle/mod.rs:60:23
   |
60 |         vertex: wgpu::VertexState {
   |                       ^^^^^^^^^^^ not found in `wgpu`

error[E0422]: cannot find struct, variant or union type `FragmentState` in crate `wgpu`
  --> examples/src/hello_triangle/mod.rs:66:30
   |
66 |         fragment: Some(wgpu::FragmentState {
   |                              ^^^^^^^^^^^^^ not found in `wgpu`

error[E0422]: cannot find struct, variant or union type `CommandEncoderDescriptor` in crate `wgpu`
   --> examples/src/hello_triangle/mod.rs:114:66
    |
114 | ...                   device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^ not found in `wgpu`

error[E0422]: cannot find struct, variant or union type `RenderPassDescriptor` in crate `wgpu`
   --> examples/src/hello_triangle/mod.rs:119:66
    |
119 | ...                   encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    |                                                        ^^^^^^^^^^^^^^^^^^^^ not found in `wgpu`

error[E0422]: cannot find struct, variant or union type `RenderPassColorAttachment` in crate `wgpu`
   --> examples/src/hello_triangle/mod.rs:121:69
    |
121 | ...                   color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `wgpu`

error[E0422]: cannot find struct, variant or union type `Operations` in crate `wgpu`
   --> examples/src/hello_triangle/mod.rs:124:52
    |
124 | ...                   ops: wgpu::Operations {
    |                                  ^^^^^^^^^^ not found in `wgpu`

error[E0433]: failed to resolve: could not find `Color` in `wgpu`
   --> examples/src/hello_triangle/mod.rs:125:77
    |
125 | ...                   load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
    |                                                       ^^^^^ could not find `Color` in `wgpu`
    |
help: consider importing this enum
    |
1   + use env_logger::fmt::Color;
    |
help: if you import `Color`, refer to it directly
    |
125 -                                             load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
125 +                                             load: wgpu::LoadOp::Clear(Color::GREEN),
    |

warning: use of deprecated method `winit::event_loop::EventLoop::<T>::run`: use `EventLoop::run_app` instead
  --> examples/src/hello_triangle/mod.rs:86:10
   |
86 |         .run(move |event, target| {
   |          ^^^
   |
   = note: `#[warn(deprecated)]` on by default

Some errors have detailed explanations: E0422, E0433.
For more information about an error, try `rustc --explain E0422`.
*/
