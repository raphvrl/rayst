# 🦀 Rayst v0.1.0 - First Release

A modern raytracer built from scratch in Rust as a journey from C++ to learning proper Rust development.

## ✨ Core Features

### 🎯 **Geometry Support**

- **Spheres** - Perfect for reflective objects and basic shapes
- **Planes** - Ideal for floors, walls, and infinite surfaces
- **Cubes** - Solid rectangular objects with full rotation support
- **Pyramids** - Complex multi-face geometry with configurable base size and height
- **Triangles** - Foundation primitive powering complex geometries

### 🎨 **Material System**

- **Plastic materials** - Non-metallic surfaces with configurable roughness
- **Metal materials** - Reflective surfaces with controllable metallic properties
- **Gold preset** - Pre-configured luxury metal material
- **PBR-based rendering** - Physically Based Rendering with metallic/roughness workflow
- **Customizable properties** - Albedo, metallicness, roughness, and ambient occlusion

### 💡 **Advanced Lighting**

- **Point lights** - Omnidirectional light sources with position and intensity control
- **Soft shadows** - Area light approximation using multiple shadow samples for realistic penumbra
- **Realistic reflections** - Ray-traced reflections with proper energy conservation
- **Physically accurate** - Cook-Torrance BRDF implementation for realistic material response

### 🖼️ **Rendering Pipeline**

- **Multi-threaded rendering** - Powered by Rayon for optimal CPU utilization
- **Configurable anti-aliasing** - Customizable sample count for smooth edges
- **Adjustable recursion depth** - Control reflection bounces for performance/quality balance
- **High-resolution output** - Support for any resolution with PNG export
- **Chunk-based processing** - Efficient memory usage for large renders

### ⚙️ **Configuration System**

- **TOML-based scenes** - Human-readable configuration files
- **Camera controls** - Position, direction, and field-of-view settings
- **Flexible object placement** - Easy positioning and rotation of all geometry types
- **Material assignment** - Per-object material configuration
- **Render settings** - Resolution, quality, and output customization

## 🚀 **Performance Features**

- **Parallel rendering** - Multi-core CPU utilization
- **Optimized ray-triangle intersection** - Fast Möller-Trumbore algorithm
- **Efficient shadow sampling** - Configurable sample count for quality/speed balance
- **Memory efficient** - Streaming chunk-based rendering for large images

## 📦 **What's Included**

- Cross-platform binary (`rayst` / `rayst.exe`)
- Example scene configuration (`example.toml`)
- Complete documentation (`README.md`)
- Ready-to-render sample scene with multiple objects and materials

## 🛠️ **Technical Stack**

- **Rust 2024 Edition** - Modern systems programming
- **Glam** - High-performance vector mathematics
- **Rayon** - Data parallelism and work stealing
- **Image crate** - PNG output with color management
- **Serde + TOML** - Type-safe configuration parsing

Perfect for learning raytracing concepts, Rust programming, or generating beautiful rendered scenes!
