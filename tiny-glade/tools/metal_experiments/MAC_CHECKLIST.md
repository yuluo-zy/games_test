# Mac execution checklist (P0 experiments that need Apple hardware)

Run these in order on an Apple Silicon Mac. Each step says what it needs, the exact command, what a good
result looks like, and what to send back. Everything here is written but **has never run on a Mac**: the
first run of each step may need small fixes, and a fix is itself a result worth reporting.

Steps 1-3 need no game assets. Step 5 (X2) needs the original game install and may not build at all yet.
If you only have time for a few steps, do 1 → 2 → 3: they decide the biggest open design questions.

| Step | Experiment | Decides | Needs |
| --- | --- | --- | --- |
| 0 | Setup | - | Xcode, Rust, Python 3 |
| 1 | X1b compile | Does the Metal compiler accept the 638 translated shaders? (D8, D9) | step 0 + lowered SPIR-V |
| 2 | X5 part A (no validation) | Format/usage support, depth filtering (D24) | step 0 |
| 3 | X5 part B (validation on) | Fences, ICB, residency, counters, ordering (D5, D18, D19, D22, D23) | step 0 |
| 4 | X1b with fast math off | D25 | step 1 |
| 5 | X2 | Per-pass Apple GPU time, MoltenVK gaps (D13, D15) | game assets, MoltenVK, a buildable tiny-glade |

## Step 0: setup

```bash
git clone <repo> && cd tiny-glade && git checkout codex/perf-windows-20261026   # branch carrying these tools
xcode-select -p                      # Xcode (not only Command Line Tools) is needed for `xcrun metal`
xcrun -sdk macosx metal --version    # must print a version
```

If `xcrun metal` reports a missing Metal toolchain, newer Xcode versions ship it as a separate download
(`xcodebuild -downloadComponent MetalToolchain`); I have not verified that command, check Xcode's own message.

Record for every result: `sw_vers`, `system_profiler SPDisplaysDataType | grep -E 'Chipset|Metal'`, and
`xcrun metal --version`. A result without the machine and OS is not comparable later.

## Step 1: X1b with the Metal compiler

**Goal:** turn "SPIRV-Cross translates all 638 stages" into "the Metal compiler accepts them", or find out which
fail and why.

### 1a. Get the lowered SPIR-V

The input is the output of the `x1_lower_all` example (one folder per pipeline with `lowered-<i>.spv` and
`meta.json`). Two ways:

- **Copy it from the Linux machine** (recommended, it is the exact input of the committed results):
  ```bash
  # on the Linux machine, from the directory holding the x1_lower_all output (here /tmp/x1)
  cd /tmp/x1 && find . -name 'lowered-*.spv' -o -name meta.json | tar czf ~/x1_lowered.tgz -T -
  # on the Mac
  mkdir -p ~/x1 && tar xzf x1_lowered.tgz -C ~/x1
  ```
- Regenerate on the Mac: needs DXC (`dxc`, for example from the Vulkan SDK) and
  ```bash
  cargo run -p shader-compiler --example x1_lower_all --release -- \
    --manifest shaders/manifest.json --shaders shaders --dxc /path/to/dxc --out ~/x1 --jobs 4
  ```
  Expect `404 pipelines, 404 ok, 0 failed`. A different DXC build can change the SPIR-V, so prefer the copy.

### 1b. Build SPIRV-Cross (same pinned tag as the Linux run)

```bash
CXX=clang++ tools/metal_experiments/x1/build_spirv_cross.sh ~/tools/spirv-cross
cat ~/tools/spirv-cross/SPIRV-Cross.tag        # expect vulkan-sdk-1.4.350.1
```

### 1c. Translate and compile

```bash
python3 tools/metal_experiments/x1/x1_msl_translate.py ~/x1 \
  --spirv-cross ~/tools/spirv-cross/spirv-cross --out ~/x1b_mac \
  --profiles argbuf-tier2-device --compile --jobs 4
python3 tools/metal_experiments/x1/x1_msl_layout_check.py ~/x1b_mac/argbuf-tier2-device
```

**Good:** `argbuf-tier2-device | 638 | 0 | 0 | 0 | 0` (all ok, none `metal_compile_fail`).
**If some fail:** `results.md` groups failures by normalised message. Read the top group first. The likely
suspects are the shaders from the static report: buffer-address shaders (7), quad operations in fragment
shaders (8), `DemoteToHelperInvocation` (3), the compute-derivative shader (`dof/analyze`). Send back
`~/x1b_mac/results.md` and, for each failure group, one `detail` from `results.json`.

**Report back:** `~/x1b_mac/results.md`, `results.json`, the `x1_msl_layout_check` output, machine info.

## Step 2: X5 part A, without validation

**Why first and unvalidated:** with the Metal validation layer on, an unsupported texture usage or format
raises an exception that aborts the whole process. These tests probe exactly such combinations.

```bash
cd tools/metal_experiments/x5_metal_probe
swiftc -O -o x5_metal_probe x5_metal_probe.swift
```

**Expect compile errors on the first build** (the file has never been compiled; Metal API names moved between
macOS 13/14/15). Fix them in place and note each fix: they are the SDK differences. If a whole test cannot
compile on your SDK, delete or comment out that `test(...)` block and say so.

```bash
./x5_metal_probe --only device --json ~/x5_device.json
./x5_metal_probe --only format-usage,format-risks,storage-write-rg11b10,depth32-linear-filter --json ~/x5_formats.json
```

If the process crashes, the crash itself is the result: note the last printed `[status] name` line.

| Test | Good | Decision |
| --- | --- | --- |
| `device` | informational: record the whole block | D14 tiers (families, argument-buffer tier, unified memory, counters) |
| `format-usage` | every game format has `sample` and (colour) `rt-pipeline`; look at which have `write` | D24 |
| `format-risks` | `r32Uint->rgb9e5Float view: true`, `rgb9e5Float 3D texture: true`, `x32_stencil8 view: true` | D24 fallbacks if any is false |
| `storage-write-rg11b10` | `pass` | D24: otherwise use RGBA16F for those outputs |
| `depth32-linear-filter` | `pass` (sampled 0.5) | D24: otherwise copy depth to R32F before linear sampling; nearest is not an acceptable substitute |

## Step 3: X5 part B, with validation

```bash
MTL_DEBUG_LAYER=1 ./x5_metal_probe --only fence-cross-commandbuffer,shared-event,icb-gpu-range,buffer-gpu-address,residency-set,counter-timestamps,heap-placement-alias,memoryless --json ~/x5_sync_validated.json
./x5_metal_probe --only dispatch-order,fast-math-nan,simd-quad-compile,simd-width,icb-gpu-range,counter-timestamps --json ~/x5_misc.json
```

(`icb-gpu-range` and `counter-timestamps` are run both with and without validation on purpose.)

| Test | Good | Decision |
| --- | --- | --- |
| `fence-cross-commandbuffer` | `pass` with validation **on** (a pass with it off proves little) | D5: if it fails, cross-command-buffer sync uses events only (the design already avoids fences there) |
| `shared-event` | `pass` | D5 |
| `dispatch-order` | `serial` and `concurrent+barrier`: 0 mismatches. `concurrent no barrier` may show mismatches; **zero does not prove ordering** | D5: serial is the default; concurrent only if it is measurably faster |
| `icb-gpu-range` | `pass` (lit pixels > 0) | D19: **the key test.** Fail means count-path draws cannot use ICBs on this device |
| `buffer-gpu-address` | `pass` | D23 |
| `residency-set` | `pass` on macOS 15+ | D22: unsupported means the heap/useResource fallback |
| `counter-timestamps` | `pass`, positive delta | D18 |
| `heap-placement-alias` | `pass` | D6 |
| `memoryless` | `pass` (Apple Silicon) | D13 b |
| `fast-math-nan` | `fastMath=false` must give `1,1`; `fastMath=true` may give `0` | D25: shows why shipping fast math needs a per-shader allow-list |
| `simd-quad-compile` | all `ok` | D9: each `FAIL` line names a construct the Metal compiler rejects |
| `simd-width` | informational | D9 (the `tgsr_reproject` lane mapping assumes a simdgroup width) |

**Report back:** the four JSON files and the console output of any run that crashed or printed `FAIL`.

## Step 4: X1b with fast math off (D25)

```bash
python3 tools/metal_experiments/x1/x1_msl_translate.py ~/x1 \
  --spirv-cross ~/tools/spirv-cross/spirv-cross --out ~/x1b_mac_nofm \
  --profiles argbuf-tier2-device --compile --no-fast-math --jobs 4
```

Same pass count expected; the point is to confirm the flag is accepted and to produce the MSL/metallib build
that the verification builds (D25) will use. Report the summary table.

## Step 5: X2, the current backend through MoltenVK

**Needs:** the original game install (`TG_INSTALL_ROOT`), MoltenVK (Vulkan SDK or `brew install molten-vk`),
and a tiny-glade build that works on macOS, which has **not** been exercised. Try the build first:

```bash
cargo test -p tiny-glade --release --features cpu-probe --test original_gameplay_chain --no-run
```

If it does not build, stop and send the first error: that is useful on its own (it is the X6 question for
the real tree). If it builds:

```bash
python3 tools/metal_experiments/x2_mac_moltenvk.py --install-root "<game install dir>" \
  --output ~/x2 --scene static --quality quality --warmup 200 --samples 120 --isolate --stats
python3 docs/performance/evidence-gpu-pass-20261027/analyze_gpu_passes.py \
  ~/x2/static-quality-r1/session.json --output ~/x2/static-quality-r1/ranking.json
```

Then, once, the lower-resolution preset to see how time scales: repeat with `--quality performance`.
`--validation` is optional (needs the Vulkan SDK layer). Compare `ranking.json` with the Windows evidence in
`docs/performance/evidence-gpu-pass-20261027/ec3ea62da-static-isolated-gpu-pass-ranking.json`; the numbers are
**not** comparable across GPUs, the pass *shares* and the geometry vs pixel split are what matter.

**Report back:** `run.json`, `fixture.log` (tail), `session.json`, `ranking.json` for each run.

## Sending results back

Put everything under one folder, for example `mac-results-<date>/` with `machine.txt` (the step 0 record),
and tell me the folder. I will compare against the design's expectations and update `METAL_DESIGN.md`
(F33 onwards, the section 6.2 capability table, and the decisions each result moves).

## If a step cannot be done

Say which and why; "did not build" and "crashed here" are findings. Do not work around a failure in a way that
changes what is being measured (for example, turning validation off to get a pass in step 3).
