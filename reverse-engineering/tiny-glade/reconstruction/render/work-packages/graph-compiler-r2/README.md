# Tiny Glade R2 CPU graph projection

This crate composes the retained first-batch ResourceInfo, registry/pool, and BatchCompiler candidates. `compile_graph` produces an owned CPU state plus a deterministic snapshot. Vulkan creation/debug names are external `ResourceBackend` calls; `EventBackend` records fixture events and its numeric tokens are not GPU handles. `AccessDependency` must be supplied by the original dependency or an explicit test table. No vk-sync access enum or classifier is reconstructed.

Run `cargo test --offline --locked`. Feed the DSL to `cargo run --offline --locked --bin graph-snapshot-oracle`. It emits JSON `candidate_static`, `backend_error`, or `panic`. The oracle is a candidate projection, not an original-entry differential harness.

DSL:

```
access-info 1 0x20 false
access-info 2 0x40 true
access-info 3 0x80 true
access-info 4 0x800 false
batch per-pass
resource 0 7 image
resource 1 9 buffer import-unowned 4
pass upload 0:0:1 1:0:2
pass empty
pass draw 0:0:3
export 0 4
```

Resource columns are contiguous logical index, fixture descriptor word, kind, optional import ownership and initial access. Pass declarations are `resource:group:access`. `access-info` explicitly supplies external mask/write outcomes. `fail-create ATTEMPT TAG` configures the event backend's zero-based creation failure.

Image descriptor fields remain offset-named; usage is the first word, buffer usage is word +8. Names, tokens and DSL shorthand are candidate interface choices. Original structural bounds are projected with explicit resource/pass packing-domain checks. ResourceInfo's existing panic order remains reused. Population failure is not atomic: earlier pool takes/backend calls remain, the current origin is consumed, and the returned `PopulationFailure.partial` exposes resulting CPU state. Actual compiler retirement/restoration remains outside this crate.

Initial transitions process pending resources with `reused=false`. First-access pre-pass transitions instead require `reused=true` and initial access zero. Pool hits still produce `reused=false`; no graph-local alias producer is added. Generated pre/export transitions are access decisions only; Vulkan layouts, stages, masks, aspect, queue families and device-generation validation require external recovered layers.

Shared first-batch crates and production files are referenced read-only. Evidence and residuals are under `evidence/render-recovery/parallel-batch-02/graph-compiler/`.
