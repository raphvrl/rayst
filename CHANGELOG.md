# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-12-19

### Added

- **Core Ray Tracing Engine**

  - Real-time ray tracing with reflection support
  - Configurable recursion depth for ray bounces
  - Anti-aliasing with customizable sample count

- **Geometry Support**

  - Sphere primitives with radius and positioning
  - Infinite plane primitives with normal vectors
  - Cube primitives with rotation support
  - Pyramid primitives with configurable base and height
  - Optimized ray-triangle intersection using Möller-Trumbore algorithm

- **Lighting System**

  - Point light sources with position, color, and intensity
  - Realistic shadow casting
  - Soft shadows using area light approximation with multiple samples

- **Material System**

  - Physically-based rendering (PBR) with metallic/roughness workflow
  - Pre-configured material presets: plastic, metal, gold
  - Cook-Torrance BRDF implementation for accurate material response
  - Customizable albedo colors for all materials

- **Performance Optimizations**

  - Multi-threaded rendering using Rayon for CPU parallelization
  - Chunk-based processing for efficient memory usage
  - Fast vector mathematics powered by Glam crate

- **Configuration & I/O**

  - TOML-based scene configuration files
  - Human-readable scene format with camera, objects, and lights
  - High-resolution PNG export support
  - Flexible camera controls (position, direction, field-of-view)

- **Cross-Platform Support**
  - Native builds for Windows, macOS, and Linux
  - Rust 2024 Edition with modern language features

### Technical Implementation

- **Dependencies**

  - `glam 0.30.6` - High-performance vector mathematics
  - `rayon 1.11.0` - Data parallelism for rendering
  - `image 0.25.8` - PNG image export
  - `serde 1.0.225` - Configuration serialization
  - `toml 0.9.6` - TOML configuration parsing
  - `fastrand 2.3.0` - Fast random number generation

- **Architecture**
  - Modular design with separate geometry, lighting, and material systems
  - Trait-based primitive system for extensible geometry types
  - Error handling with custom `RaystError` type
  - Clean separation of concerns between rendering and scene management

### Documentation

- Complete README with usage examples and feature overview
- Example scene configuration demonstrating all supported features
- Cross-platform build and usage instructions
- Learning objectives documentation for Rust language features

---

_This is the initial release of Rayst - a modern raytracer built from scratch in Rust as a learning project transitioning from C++. The implementation focuses on clean, idiomatic Rust code while delivering a fully functional raytracing engine suitable for generating high-quality rendered scenes._
