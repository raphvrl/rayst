# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-12-19

### Added

- **Ray tracing engine** with realistic reflections and lighting
- **Geometry support** for spheres, planes, cubes, and pyramids
- **Soft shadows** using area light approximation with multiple shadow samples
- **Material system** with plastic, metal, and gold presets
- **PBR-based rendering** with metallic/roughness workflow
- **Multi-threaded rendering** powered by Rayon for optimal CPU utilization
- **Configurable anti-aliasing** with customizable sample count
- **TOML-based scene configuration** for human-readable scene files
- **Camera controls** with position, direction, and field-of-view settings
- **High-resolution output** with PNG export support
- **Cross-platform support** for Windows, macOS, and Linux

### Technical Features

- **Optimized ray-triangle intersection** using fast Möller-Trumbore algorithm
- **Cook-Torrance BRDF implementation** for physically accurate material response
- **Chunk-based processing** for efficient memory usage during large renders
- **Adjustable recursion depth** to control reflection bounces
- **Rust 2024 Edition** with modern systems programming practices

### Performance

- **Parallel rendering** with multi-core CPU utilization
- **Efficient shadow sampling** with configurable quality/speed balance
- **Memory efficient** streaming chunk-based rendering
- **Fast vector mathematics** using Glam crate

### Documentation

- Complete README with usage examples
- Example scene configuration included
- Cross-platform binary distribution
- Ready-to-render sample scene with multiple objects and materials

---

_A modern raytracer built from scratch in Rust as a journey from C++ to learning proper Rust development. Perfect for learning raytracing concepts, Rust programming, or generating beautiful rendered scenes!_
