# Metal integration: P0 decision experiments

Code for the experiments in `crates/rhapsody/METAL_DESIGN.md` section 5 (X1-X6) and the P0 Windows
baseline. Each experiment answers named design questions; this file says what is ready, what was
actually run (and where the output is), and what still needs a machine this was not written on.

## Status

| ID | Question it answers | Code | Run so far |
| --- | --- | --- | --- |
| P0 baseline | Dispersion of GPU time for the 0.5 performance thresholds | `p0_gpu_matrix.py` (+ test) | Not run: needs the Windows machine and a frozen build. Plan and wiring unit-tested (`--dry-run`). CPU render-phase baseline is separate (`tools/perf_headless_series.py`) and not driven here. |
| X1a | Do the production-lowered shaders have features Metal can't take, and how widespread is binding aliasing? | `crates/rhapsody/crates/shader-compiler/examples/x1_lower_all.rs`, `x1/x1_static_report.py` | **Run** on all 404 manifest pipelines with real DXC: 404/404 built and lowered. Output: `results/x1_static.md`. |
| X1b | Does SPIRV-Cross translate the lowered SPIR-V (and does the Metal compiler accept the MSL)? | `x1/x1_msl_translate.py` (+ test with a stub translator), `x1/x1_msl_layout_check.py`, `x1/build_spirv_cross.sh` | **Run (translation only).** All 638 stages translate with argument buffers tier 2 in device storage. **The Metal compiler step has not been run** (needs a Mac): translation success is not proof the MSL compiles or is correct. Output: `results/x1b_translation.md`, `results/x1b_layout.md`. |
| X2 | Apple-GPU time per pass; encoder and draw counts; MoltenVK gaps | `x2_mac_moltenvk.py` (+ test) | Not run: needs a Mac and the game assets. Environment construction is unit-tested against the Windows controller. |
| X3 | How many tests change if Vulkan-typed fields become neutral types? | `x3_test_impact.py` | **Run.** Output: `results/x3_test_impact.md` / `.json`. |
| X4 | Mobile bottleneck and budget on a phone | `x4_ios_harness/` (telemetry helper only; the harness is **not** written) | Not run. `x4_ios_harness/README.md` lists the blockers. |
| X5 | Which Metal capabilities D5-D8, D13, D18-D25 rely on actually exist | `x5_metal_probe/x5_metal_probe.swift` | Not run, **never compiled** (no Swift toolchain or Mac here). |
| X6 | Can Apple targets be type-checked from Linux/Windows CI? | `x6_apple_check.sh` | **Run here: n.a.** `rustup target add aarch64-apple-darwin` fails: the download from `static.rust-lang.org` is blocked on this machine. Needs a machine with access, or a Mac CI runner. |

**Running on a Mac:** follow [`MAC_CHECKLIST.md`](MAC_CHECKLIST.md) (ordered steps, expected results, what to send back).

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

### X1b: SPIRV-Cross translation (`results/x1b_translation.md`, `results/x1b_layout.md`)

SPIRV-Cross `vulkan-sdk-1.4.350.1` (commit `1a61695`), MSL 3.0, macOS target, all 638 lowered stages.

| profile | translated | failed |
| --- | ---: | ---: |
| `plain` (no argument buffers; control) | 124 | 514 |
| `argbuf-tier1` (control) | 124 | 514 |
| **`argbuf-tier2-device`** (what D8 proposes) | **638** | 0 |
| `argbuf-tier2-device-rich` | 638 | 0 |

- The 514 failures of the controls are the stages that have heap resources; the 124 that pass are the
  stages without any. Reasons: `Unsized array of descriptors requires argument buffer tier 2` and, with
  argument buffers on but in constant storage, `Runtime sized variables must be in device storage
  argument buffers`. My first run used the second configuration without the device-storage option and
  failed 514 stages for that reason alone; the correct flag was found in `spirv-cross --help-msl`,
  not guessed.
- **Aliased bindings are translated, not rejected**: SPIRV-Cross emits one argument-buffer member and
  re-views the same base address as each aliased type
  (`reinterpret_cast<const device spvDescriptor<texture2d<float>>*>(&spvDescriptorSet0.<first member>)`),
  commenting the others as `// Overlapping binding`. This answers the main X1a risk at the translator.
- **Every declaration uses `[[id(0)]]`** (3143 of 3143), every struct has exactly one non-overlapping
  member, and only descriptor set 0 is used (`x1b_layout.md`). The generated shader therefore assumes the
  heap is **one flat table of descriptors indexed by slot, shared by textures, samplers and buffer
  pointers**.
- No stage exceeds Metal's binding limits: at most 2 buffer slots (the argument buffer and the
  parameter block), 0 textures, 0 samplers, because everything goes through the heap.
- `--msl-runtime-array-rich-descriptor` changes a buffer descriptor to `{pointer, length, padding}`
  (16 bytes) while textures and samplers stay 8 bytes; that would break the flat re-viewing above.
  No stage needs it (the translation succeeds without it; the "buffer_size" text found in 8 outputs is
  a game struct field, `grass_buffer_size`). Do not enable it.

Not established: that the Metal compiler accepts these 638 files (run with `--compile` on a Mac, and with
`--no-fast-math` for D25); that the device-address-space argument buffer performs acceptably; that the
generated code is correct.

### X6

`n.a.` here (see table). Log: `rustup target add` error is the download being refused through the
proxy, not a rustup or workspace problem.

## Running order that avoids wasted effort

1. X1b translation is done (above). Repeat it on a Mac with `--compile` (and `--no-fast-math` for D25) to
   add the Metal compiler result: `x1_msl_translate.py <lowered> --spirv-cross <bin> --out <dir>
   --profiles argbuf-tier2-device --compile`.
2. On a Mac: `x5_metal_probe` (format/ICB/fence tests **without** validation first; see the file header),
   then `x2_mac_moltenvk.py`.
3. On the Windows machine: `p0_gpu_matrix.py` (and the CPU series separately) before the first P1 change.
4. Anywhere with access to `static.rust-lang.org`: `x6_apple_check.sh`.
