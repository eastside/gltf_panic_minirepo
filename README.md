# gltf_panic_minirepo

Demonstrates a panic! caused by wgpu when importing a presumably valid gltf package.

The .gltf file can be imported into Blender, and can be viewed with bablylon.js, cesium, filament and three.js, without errors.

The glTF Tools plugin for VS Studio Code also seems to report no errors (but there are some warnings).

I'm not sure what's going wrong.

See: https://github.com/bevyengine/bevy/issues/9021

## Full error

PS C:\Users\EASTE\gltf_panic_minirepo> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target\debug\gltf_panic_minirepo.exe`
2023-08-01T05:46:14.620039Z  INFO bevy_winit::system: Creating new window "Bevy App" (0v0)
2023-08-01T05:46:15.060903Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3090", vendor: 4318, device: 8708, device_type: DiscreteGpu, driver: "NVIDIA", driver_info: "535.98", backend: Vulkan }
2023-08-01T05:46:15.283194Z  WARN bevy_gltf::loader: Vertex attribute WEIGHTS_0 has format Unorm8x4 but expected Float32x4 for target attribute Vertex_JointWeight
2023-08-01T05:46:15.342455Z  INFO bevy_diagnostic::system_information_diagnostics_plugin::internal: SystemInfo { os: "Windows 11 Home", kernel: "22621", cpu: "AMD Ryzen 9 5900X 12-Core Processor", core_count: "12", memory: "31.9 GiB" }
2023-08-01T05:46:16.436764Z ERROR wgpu::backend::direct: Handling wgpu errors as fatal by default
thread '<unnamed>' panicked at 'wgpu error: Validation Error

Caused by:
    In a RenderPass
      note: encoder = `<CommandBuffer-(0, 79, Vulkan)>`
    In a draw command, indexed:true indirect:false
      note: render pipeline = `pbr_opaque_mesh_pipeline`
    The pipeline layout, associated with the current render pipeline, contains a bind group layout at index 2 which is incompatible with the bind group layout associated with the bind group at 2

', C:\Users\EASTE\.cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.16.3\src\backend\direct.rs:3019:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Encountered a panic in exclusive system `bevy_render::renderer::render_system`!
thread 'Compute Task Pool (6)' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', C:\Users\EASTE\.cargo\registry\src\index.crates.io-6f17d22bba15001f\bevy_render-0.11.0\src\pipelined_rendering.rs:135:45
error: process didn't exit successfully: `target\debug\gltf_panic_minirepo.exe` (exit code: 101)
