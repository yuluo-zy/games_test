# Shader recompile-validation pipeline

**Scope:** Tiny Glade original-game shader reconstruction. The recovery pipeline
is: reconstruct HLSL → recompile with the SAME dxc command → compare SPIR-V
structure against the shipped `.bin`. This document describes the installed
toolchain, the canonical command, the fingerprint methodology, how a
reconstruction agent runs a check, and troubleshooting.

Evidence files:
- `compile-commands.json` — per-`.bin` dxc commands (extracted from the embedded `dxc-cl-option:` strings), 404 bins / 638 modules.
- `tools/render-passes/recompile_check.py` — the validator.
- `tools/render-passes/extract_compile_commands.py` — regenerates `compile-commands.json`.
- `tools/render-passes/spirv_parse.py` — SPIR-V reflection parser (reuses `scripts/render-passes/parse_shader_catalog.py`).
- `evidence/render-passes/shaders/toolchain-embedded-debug.md` — original discovery.

---

## 1. Installed toolchain

### dxc (DirectX Shader Compiler)

| | |
|---|---|
| Tool | `dxc` (Microsoft DirectX Shader Compiler) |
| Version | v1.10.2605.37 (source tag of the official `microsoft/DirectXShaderCompiler` repo, commit `c4d8f4f99aa221da58cd540bf2099a2889632ab6`, 2026-08-06) |
| Install path | `/opt/homebrew/bin/dxc` (Homebrew formula `directx-shader-compiler` from tap `SharkyRawr/homebrew-dxc`) |
| Why this tap | `brew install directx-shader-compiler` does not exist in `homebrew-core` (only `directx-headers`), and Microsoft ships no official macOS release asset. The tap builds from the official source and provides an `arm64_tahoe` bottle. |
| Bottle | `directx-shader-compiler-1.10.2605.37.arm64_tahoe.bottle.1.tar.gz`, sha256 `557832263349f4b2a2667081f9762f7b16b6c7e5cc5ea9b5c4a03825c41d05fc` (per tap formula `Formula/directx-shader-compiler.rb`) |
| SPIR-V generator word | game bins report `0x0`, so the original dxc version is **not** recoverable from the modules; version parity is therefore not guaranteed (see §3). |

### spirv-cross (SPIR-V → HLSL decompilation aid)

| | |
|---|---|
| Tool | `spirv-cross` (base binary; HLSL output via `--hlsl`) |
| Version | 1.4.363.0 (brew bottle, timestamp 2026-09-16) |
| Install path | `/opt/homebrew/bin/spirv-cross` |
| Usage | decompile aid for later reconstruction agents, e.g. `spirv-cross --hlsl --entry <entry> <module.spv>`. Caveat: `-Zi -Qembed_debug` debug info does not reconstruct source; use it to cross-check declarations/control flow, not to recover exact HLSL. |

---

## 2. Canonical command per family

All 638 original module commands share one flag set (verified 638/638 in
`compile-commands.json`):

```
dxc -E <entry> -T <profile> -spirv -WX -Ges -fspv-target-env=vulkan1.1 -HV 2021 -Zi -Fo <out> -Qembed_debug -I <include_root> <source>
```

| family | profile | entry | modules / bin |
|---|---|---|---|
| compute | `cs_6_4` | `cs_main` (mostly; also `cs_halfres`, `cs_halfres_all`, `sample1`, `sample4`, `cs_main_multisampled`, `capture`, `compare`, `r16f`, `rgba16f`, `cs_pass1`, `cs_pass2`) | 1 |
| raster vertex | `vs_6_4` | `vs_main` | 2 |
| raster fragment | `ps_6_4` | `ps_main` | 2 |

The raster `.bin` holds two modules (vertex + fragment) compiled from the same
`.raster.hlsl` source; `recompile_check.py` compiles that source once per module
with the module's own `-E`/`-T`.

**Include-dir layout expectation.** Original includes are relative to the
`shaders/` root (`#include "inc/common.hlsl"`), so the reconstructed tree keeps
the same relative structure and the validator passes
`-I reconstruction/shader-source/shaders`. See
`reconstruction/shader-source/README.md`.

---

## 3. Fingerprint methodology

`-Zi -Qembed_debug` embeds the input file path and the temp output path into the
SPIR-V module. Recompiling the same source from a different path therefore
yields different bytes. **A byte-for-byte raw sha256 is the WRONG comparison** —
even a perfect reconstruction will differ. Two mechanisms are used instead:

### 3.1 Structural fingerprint (primary)

Parsed from both modules via `spirv_parse.py` (word-level SPIR-V reflection):

1. **Entry point + stage** — `OpEntryPoint` execution model and name.
2. **LocalSize** — `OpExecutionMode LocalSize` (compute).
3. **Capability set** — `OpCapability` values.
4. **Descriptor bindings** — set / binding / storage class / name per
   `OpVariable` + `OpDecorate(DescriptorSet, Binding)`.
5. **Opcode histogram** — count of every opcode.

The validator compares each category and reports per-category diffs
(bindings/entries missing vs. extra; histogram target-only vs. candidate-only).
A module is structurally **PASS** when all five categories match.

### 3.2 Normalized sha256 (stronger, secondary)

sha256 of the module with **source-path, temp-path and dxc-command strings
stripped** (`<HLSLPATH>`, `<TMP>`, `<DXCCMD>` placeholders), so two compilations
of the same source with the same dxc compare equal. It is an additional
signal, not the verdict: because the original dxc version is unrecoverable
(SPIR-V generator word 0x0), a version/codegen drift will make normalized hashes
differ even when the structure is identical. Treat `PASS_STRUCTURAL` +
`normalized_hash_match=false` as a likely toolchain-version difference, not a
reconstruction bug.

Sanity proof (recorded in this directory):
- `self-test-proof.json` — the same trivial HLSL (`cs_main`, LocalSize 8,8,1,
  3 bindings) compiled from two different paths gives `raw_hashes_equal=false`
  but `normalized_hashes_equal=true` and `fingerprints_equal=true`: the
  normalized hash is path-stable, the raw hash is not.
- `self-test-validator.json` — the validator run end-to-end over a trivial
  "target bin" reports `PASS_NORMALIZED` (structural + normalized identical),
  including a case where the target and candidate `-Fo` output paths differ in
  length.
- `real-bin-fail-demo.json` — the validator against a real game `.bin`
  (`_blur.cs.hlsl_cs_main_0.bin`) recompiles with the module's own
  `-E cs_main -T cs_6_4` command and reports `FAIL` with a per-category diff
  (bindings and opcode histogram), proving the oracle-comparison and diff
  reporting path.

---

## 4. Running a check

From the tiny-glade project root:

```sh
python3 tools/render-passes/recompile_check.py \
    reconstruction/shader-source/shaders/blur.cs.hlsl \
    "<game>/compiled-shaders/_blur.cs.hlsl_cs_main_0.bin"
```

- `--entry` restricts to one module (raster bins contain two).
- `--out report.json` writes the machine-readable report (fingerprints, hashes,
  diffs, verdicts).
- `--keep` keeps temp artifacts under `recompile_tmp/` (also dumps dxc
  disassembly `-Fc` for inspection).
- Exit code 0 = every checked module passes structurally.

The target module's own `-E`/`-T` are read from the embedded `dxc-cl-option:`
string, so the replay command is always the original command.

---

## 5. Troubleshooting

| Symptom | Cause / check |
|---|---|
| `FAIL` on bindings, entries | Reconstruction changed resource names, set/binding layout, or the entry function; compare the `differences` section (target_only vs candidate_only). |
| `FAIL` on opcode histogram only | Near-identical but not identical codegen — the reconstruction is semantically close; inspect dxc disassembly (`--keep`) to find the differing construct. |
| `PASS_STRUCTURAL` but normalized hash differs | Expected when the installed dxc version differs from the original build's (unrecoverable). Not a reconstruction bug. Verify the diff is only in path/command placeholders. |
| `FAIL_DXC` (dxc error) | dxc rejected the HLSL — usually a missing include (check `-I reconstruction/shader-source/shaders` and the include graph), or a reconstruction syntax error. |
| `include root not found` | run from the tiny-glade project root, or pass `-I` explicitly. |
| Phantom "hash mismatch" reports | Do **not** raw-hash `.bin` vs recompiled SPIR-V. Use `recompile_check.py`; the normalized hash is the only hash that is path-stable. |
| Which `.bin` to compare against | Use the bin whose `main_source_path` in `compile-commands.json` equals your reconstructed file's path. Raster: two bins may share the source (different 16-hex variant hashes) — check each variant. |
