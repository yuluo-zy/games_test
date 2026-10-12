# R6C interface candidate and tooling

This crate reuses original spirq0.6.2 commit4f22e6f149750401705d47ae15a60e77cd5a6905, serde1.0.200 and serde_json1.0.85. All resolved dependency versions/sources/checksums match original Cargo.lock. It does not reimplement shader reflection or recover shader algorithms.

The library exposes descriptor merge, push-range and location/component type checks. reflect-shaders reflects word-count-prefixed SPIR-V modules from shipped containers; validate-catalog checks per-container interfaces. Neither binary launches the game, creates Vulkan pipelines or validates physical-device limits.

Build: cargo build --bins --locked --offline. Tests: cargo test --locked --offline; cargo fmt --check; cargo clippy --locked --offline --all-targets -- -D warnings.

From Tiny Glade analysis root, run evidence/render-recovery/parallel-batch-04/pipeline-shader-interfaces/catalog.mjs then validate.mjs with Node. Catalog includes all404 containers/638 modules, stage IO and raw built-in decorations. Reflection resources include declarations even when unused; descriptor count0 is runtime sized. Full built-in Vulkan linking and runtime bindings remain unknown.
