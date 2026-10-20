# Sheep startup assets: complete producer checkpoint

Candidate only. This owner did not change production files or invoke Cargo. `candidate-checkpoint.json` records six exact raw-byte base/proposed SHA256 pairs. Normal `git apply --check` passes and source candidates pass rustfmt parsing. Two binary fixtures are deliberately omitted from the textual patch; central integration must copy their exact bytes from `candidate/` and verify manifest SHA.

## Original authority and independent execution

`native-ranges.json` binds the original RNE and full8031-byte startup_sheep140a093e0 (SHA5782b2476a2d39c5ddd667a67374326877827f512781349ee2ca725d89eda3d9). Full original LLVM assembly is `../wall-gpu/sheep-startup-original.asm`; bounded decompilation is `../wall-gpu/sheep-startup-query.c`. Its hot body is only5648B, so the8031B PDB range and original assembly, including the cold error paths, were retained instead of treating the shorter Ghidra body as complete ABI evidence.

`native_animation_probe.py` executes original140a09fcf..140a0a5f4, including all thirty frame position/normal lookups and type/bounds checks, both delighted lookups, array zeroing and992-byte row writes. Original mapped image supplies all branches; the fixture std-hasher boundary returns a controlled hash for fixture table buckets, CRT memset/memcpy and preallocated Vec capacity are explicit test boundaries. No std/mesh/hash implementation is restored into production.

Thirty-four native cases passed. Synthetic raw float payloads prove exact preservation of NaN payload7fc01234, negative zero and infinities, with every fourth lane written positive zero. Missing/wrong-type first/last-frame attributes, short frames, missing delighted attributes and zero base vertices execute original error/skip behavior. Original successful bytes are compared against independently packed source input bytes. All860 vertices from the shipped30+delighted asset files were run through the original loop, producing853120 byte-exact golden bytes; each source asset SHA is recorded in `original-animation-fixtures.json`.

## Recovered algorithm and lifecycle

Native first probes the existing mesh library for sheep_mesh. A hit skips the complete base/delighted/frame load and SSBO producer. A miss reads base1.json, delighted.json, then obtains the base Vertex_Position array length before collecting all frames1..=30. Frame1 is intentionally read a second time. Base access uses the array length without a Float3 tag check; each requested frame/delighted attribute independently requires Float3 and bounds-checks its vertex. No equality-of-all-lengths prevalidation is added: extra vertices are ignored, short data fails when reached, and base zero vertices skip all frame/delighted attribute accesses after loading the files.

Rows are vertex-major: thirty[xyz,+0] positions, thirty[xyz,+0] normals, delighted position[xyz,+0], delighted normal[xyz,+0]. Canonical SheepAnimationSsbo already expresses that exact992-byte layout. There is no interpolation, normal normalization, angle conversion or value defaulting. Full typed data goes through the existing AssetSsboLibrary::add_new_with_data; a narrowly justified NoUninit implementation and compile-time size/alignment guards establish that this repr(C) float-array element contains no uninitialized padding.

Only after that upload is the unchanged base mesh watched and registered as sheep_mesh. Then the real sheep shader is compiled, and heart/umbrella/floatie meshes are loaded, watched and registered in original order. Finally existing solid_vertex_color is looked up; it is not recompiled as a second startup shader. The delighted mesh is temporary animation input and is never separately registered. Missing files/attributes/shader owners propagate failure rather than becoming a bare mesh or empty buffer.

## Interface and registration owner

New `startup::sheep_assets::load_sheep_startup_assets(installed,meshes,shaders,backend,watch,ssbos,device)` returns six real handles in SheepStartupAssets: sheep_mesh, sheep_shader, heart_mesh, umbrella_mesh, floatie_mesh, accessory_shader. wall_gpu_chain owns startup_sheep.rs facade, full ECS tuples, SheepControl/audio owner and `pub mod sheep_assets` registration. This patch does not touch those shared source files.

Central registration must replace the old startup_sheep_meshes mesh-only callback. Running it before the full producer would pre-register a bare sheep_mesh, causing the original mesh-hit guard to skip the animation SSBO forever. A complete algorithm behind that stale earlier producer would still be a broken startup chain.

## Rust acceptance candidate and limits

A separate tiny-glade integration test imports the actual public builder and uses already compiled production libraries. It compares both synthetic rows byte-for-byte to original golden bytes, checks native error/zero-vertex/longer-frame policies, and contains an ignored full860-vertex actual-asset test. The ignored test must run the compiled test binary directly with the original distribution as process cwd; Cargo normally sets the package cwd. No production/Cargo file is modified by this test candidate.

These Rust tests have not been compiled or executed by this owner. The original byte oracle proves the animation producer algorithm and validates its candidate goldens; it does not prove physical GPU upload, the actual startup App schedule, shader watcher behavior, or the full sheep/entity/audio lifecycle. Those remain with central build/acceptance and wall_gpu_chain's separate wrapper checkpoint.
