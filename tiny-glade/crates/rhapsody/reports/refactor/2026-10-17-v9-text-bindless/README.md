# Text consumer: shared bindless production path

`GpuTextRenderer` now lowers the original text shader through `BindlessShaderPackage`, uses the renderer-owned shared four-array heap, binds typed resource leases through `BindlessParameters`, and uploads its parameter index through the formal 4-byte push ABI. It uses dynamic rendering and a bounded background pipeline compiler; only construction prewarms, while rendering polls `PipelineTicket::try_get`. Per-frame descriptor pools and sets are removed. `new_with_heap` shares an existing host heap.

Strict real MoltenVK GPU acceptance: **2/2 PASS, 9 frames, zero validation errors/VUID/SYNC-HAZARD, no skips**. Solid/glyph pixels and runtime sampled-image-array element 1 (with a black element 0 decoy) match the preserved original dynamic-rendering path byte for byte. Each RGBA reference is 4096 bytes. The first fixture repeats three frames across two renderers sharing one heap and checks fixed pipeline compile count, stable live slots, one dynamic constants backing page, and exact completed command-pool reset counts.

`result.json` identifies the actual copied executable, source subset and original references by SHA-256. Original-path provenance is retained in `../2026-10-17-v6/tg-text-dynamic-reference-provenance.json`; those old production rlibs were matched to Cargo fingerprints before compiling only the bounded pixel-export fixture. Existing references were read, never overwritten, by this new run.

The adapter is an actual tg-text consumer of the formal rhapsody paths. This does not claim recovery of the original full-process toolbar scheduling. Later shared command/frame-storage cleanup must be covered by the integrating final source matrix; this run binds the executable that was actually used.
