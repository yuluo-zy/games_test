# R6C shipped shader interfaces

## Observed

The shipped compiled-shaders directory has 404 files: 170 compute containers and 234 raster containers. Binary word-count-prefixed SPIR-V payloads produce 638 modules/entry points: 170 GLCompute, 234 Vertex and 234 Fragment. All 638 reflect successfully using the original spirq0.6.2 fork at commit 4f22e6f149750401705d47ae15a60e77cd5a6905. Every file and payload has a SHA256 and exact offset/word count in catalog.json. No database or game process was opened.

Declaration-only reflection yields 2577 descriptor variables: 327 samplers, 778 uniform buffers, 719 sampled images, 116 storage images and 637 storage buffers. Four variables use runtime-sized descriptor counts (count0, not zero allocation). Forty-three push-constant variables have minimum extents of 4/8/12/16/20/24 bytes. Runtime-sized buffer members are not proven buffer-size requirements by spirq's minimum_bytes.

Two deterministic signatures are provided: 240 layout families (set/binding/descriptor class/count plus push layout), and 402 reflected interface families (stage, detailed descriptors, push types, location/component IO). Debug names are removed from signature types; declared debug names and full member offsets/types remain in the catalog. BuiltIn/other IO decorations are retained separately as raw declarations; family membership does not prove complete Vulkan interstage compatibility.

All 404 per-container descriptor merges validate, all 234 Vertex/Fragment pairs match location/component type declarations, and all reflected push ranges meet alignment/overflow checks. These checks do not validate physical device limits, descriptor lifetime/image layout or built-in linking. Rebuilding the six catalog/index artifacts produces byte-identical SHA256s.

PDB/workgraph evidence indexes pipeline, binding and descriptor-set functions, country-core pass-building/native consumers and observed native direct edges. Monomorphized bind_descriptor_set argument types identify draw_stream and compute_pass_builder::dispatch_impl closure associations; update_persistent_descriptor_set identifies asset texture-library bindless table association. These are type/closure associations, not a live dispatch trace. PDB void signatures remain placeholders; alias-collision functions retain all original names.

## Inferred

Only three conservative shader-name token matches are proposed: generate_hzb to the similarly named PDB consumer, and two text raster variants to the text consumer. They are marked name-based candidates, not native string-xrefs or verified shader-to-pass bindings. A family defines declaration compatibility, not identical shader behavior, GPU usage or outputs.

## Unknown / exclusions

Actual shader-library lookup-to-pass bindings, specializations, runtime descriptor counts, binding-to-game-field ABI, device-feature constraints, image/buffer state and complete BuiltIn linking remain unresolved. No shader algorithm was reverse engineered. No Vulkan pipeline/scene/image/runtime equivalence is claimed.

## Reproduction

Build reconstruction/render/work-packages/pipeline-interfaces-r6 with cargo build --bins --locked --offline. From project root, run node evidence/render-recovery/parallel-batch-04/pipeline-shader-interfaces/catalog.mjs, then validate.mjs. catalog.mjs accepts an alternate compiled-shaders directory as its first argument. validate.mjs repeats catalog generation for determinism, checks source/payload hashes, runs the Rust validator and compares dependency versions/sources/checksums to tracking/original-Cargo.lock.
