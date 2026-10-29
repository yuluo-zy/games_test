# Metal integration: P0 decision experiments

Code for the experiments in `crates/rhapsody/METAL_DESIGN.md` section 5 (X1-X6) and the P0 Windows
baseline. Each experiment answers named design questions; this file says what is ready, what was
actually run (and where the output is), and what still needs a machine this was not written on.

## Status

| ID | Question it answers | Code | Run so far |
| --- | --- | --- | --- |
| P0 baseline | Dispersion of GPU time for the 0.5 performance thresholds | `p0_gpu_matrix.py` (+ test) | Not run: needs the Windows machine and a frozen build. Plan and wiring unit-tested (`--dry-run`). CPU render-phase baseline is separate (`tools/perf_headless_series.py`) and not driven here. |
| X1a | Do the production-lowered shaders have features Metal can't take, and how widespread is binding aliasing? | `crates/rhapsody/crates/shader-compiler/examples/x1_lower_all.rs`, `x1/x1_static_report.py` | **Run** on all 404 manifest pipelines with real DXC: 404/404 built and lowered. Output: `results/x1_static.md`. |
| X1b | Does SPIRV-Cross translate the lowered SPIR-V (and does the Metal compiler accept the MSL)? | `x1/x1_msl_translate.py` (+ test with a stub translator), `x1/build_spirv_cross.sh` | **Not run.** SPIRV-Cross is not vendored and building it clones third-party source, which this environment refused to do automatically. The runner logic is tested against a stub. |
| X2 | Apple-GPU time per pass; encoder and draw counts; MoltenVK gaps | `x2_mac_moltenvk.py` (+ test) | Not run: needs a Mac and the game assets. Environment construction is unit-tested against the Windows controller. |
| X3 | How many tests change if Vulkan-typed fields become neutral types? | `x3_test_impact.py` | **Run.** Output: `results/x3_test_impact.md` / `.json`. |
| X4 | Mobile bottleneck and budget on a phone | `x4_ios_harness/` (telemetry helper only; the harness is **not** written) | Not run. `x4_ios_harness/README.md` lists the blockers. |
| X5 | Which Metal capabilities D5-D8, D13, D18-D25 rely on actually exist | `x5_metal_probe/x5_metal_probe.swift` | Not run, **never compiled** (no Swift toolchain or Mac here). |
| X6 | Can Apple targets be type-checked from Linux/Windows CI? | `x6_apple_check.sh` | **Run here: n.a.** `rustup target add aarch64-apple-darwin` fails: the download from `static.rust-lang.org` is blocked on this machine. Needs a machine with access, or a Mac CI runner. |

Tests for the Python tools: `cd tools/metal_experiments && python3 -B -m unittest discover -p 'test_*.py'`.

## Results obtained here

### X3: test impact (`results/x3_test_impact.md`)

Test code only; "tests in files" is an upper bound (a test in a touched file may not use the type).

| Scope | Files | `#[test]` in those files |
| --- | ---: | ---: |
| Families CORE_SPLIT_DESIGN K1 changes (stage, access, layout, aspect, queue family, `AccessScope`, `NativeState`, `vk::Result`, `Error::Vulkan`, `TimelineApi`, semaphore) | 100 | 643 (rhapsody 493 of 1008, tiny-glade 150 of 492, country-core 0 of 3099) |
| All families incl. format, usage flags, command buffer | 139 | 841 |

Reading: **without** the K1 aliases, up to 643 tests would need edits for the P1/P2 type changes;
the design's claim is that with them none do. That claim is not yet verified on the real code.

### X1a: facts about the lowered shaders (`results/x1_static.md`)

404 pipelines / 638 stages built with DXC from `shaders/manifest.json` and lowered with the production
`lower_pipeline_with_limit(.., 64)`.

- **Every** stage carries `RuntimeDescriptorArray`, `SampledImageArrayDynamicIndexing`,
  `StorageImageArrayDynamicIndexing`, `StorageBufferArrayDynamicIndexing`: the bindless lowering is
  universal, not limited to a few shaders (confirms design F6).
- **Binding aliasing: 486 of 638 stages (76%)** have at least one binding carrying more than one resource
  type (386 of 404 pipelines). It is binding 2 (storage buffers of different struct layouts) in every
  affected stage, plus binding 0 (images of different component type or depth flag) in 39 and binding 1
  in 16. The widest binding carries 7 types (`wall/generate_draw_lists.cs.hlsl`). How SPIRV-Cross
  turns this into MSL is the open question X1b exists to answer; this number says it applies to
  three quarters of the shaders, not to an edge case.
- Capabilities needing attention on Metal: `PhysicalStorageBufferAddresses` + `Int64` (7 shaders),
  quad/shuffle/vote/arithmetic/ballot group operations (15 stages use quad operations, 8 shaders,
  including 4 fragment lighting shaders), `DemoteToHelperInvocation` (3), `ComputeDerivativeGroupQuads`
  (1: `dof/analyze.cs.hlsl`), typeless storage image read/write (13 / 5 shaders). Per-shader lists are
  in the report.
- The two `final_post.raster.hlsl` pipelines initially failed only because the manifest lists the
  pixel entry before the vertex entry and the builder requires vertex first; the driver orders stages.

### X6

`n.a.` here (see table). Log: `rustup target add` error is the download being refused through the
proxy, not a rustup or workspace problem.

## Running order that avoids wasted effort

1. On any machine with the shader tools: `x1_lower_all` (already done here), then
   `x1/build_spirv_cross.sh` and `x1/x1_msl_translate.py` for X1b. On a Mac add `--compile` and, for
   D25, `--no-fast-math`.
2. On a Mac: `x5_metal_probe` (format/ICB/fence tests **without** validation first; see the file header),
   then `x2_mac_moltenvk.py`.
3. On the Windows machine: `p0_gpu_matrix.py` (and the CPU series separately) before the first P1 change.
4. Anywhere with access to `static.rust-lang.org`: `x6_apple_check.sh`.
