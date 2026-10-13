# Shader recompile-validation toolchain — installed tools evidence

Date: 2026-10-13. Host: macOS 26 (Tahoe, arm64), Homebrew 7.0.9.
Purpose: reproduce the original game's dxc compile commands
(`dxc-cl-option: /tmp/.tmpXXXX -E <entry> -T <profile> -spirv -WX -Ges
-fspv-target-env=vulkan1.1 -HV 2021 -Zi -Fo <out> -Qembed_debug`) for the
reconstruct→recompile→compare pipeline over the 404 read-only `.bin` shaders.

## dxc (DirectX Shader Compiler)

| Field | Value |
|---|---|
| Tool | `dxc` |
| `dxc --version` | `libdxcompiler.dylib: 1.10(5373-c4d8f4f9)(1.10.0.5373)` |
| Source version | tag `v1.10.2605.37` of official `microsoft/DirectXShaderCompiler`, commit `c4d8f4f99aa221da58cd540bf2099a2889632ab6` (2026-08-06) — verified present upstream |
| Install path | `/opt/homebrew/bin/dxc` → `../Cellar/directx-shader-compiler/1.10.2605.37/bin/dxc` |
| Installation | Homebrew formula `directx-shader-compiler` from tap `SharkyRawr/homebrew-dxc` (bottle, no source build) |
| Bottle file | `directx-shader-compiler-1.10.2605.37.arm64_tahoe.bottle.1.tar.gz` (212,999,807 bytes) |
| Bottle sha256 | `557832263349f4b2a2667081f9762f7b16b6c7e5cc5ea9b5c4a03825c41d05fc` — matches the tap formula's declared bottle sha256 |
| Binary sha256 | `9bf95a5bf29a5994325860a2fa68690846c8db721f6798ce45c9fc5ddfb5e853` |

Why this source: `brew install directx-shader-compiler` does not exist in
homebrew-core (only `directx-headers`), and Microsoft ships no official macOS
release asset (the `v1.9.2609` release provides Windows + Linux binaries only).
The tap builds from the pinned official source and provides an arm64_tahoe
bottle; the tag and commit were verified against the upstream repo.

Original-game dxc version: not recoverable — all 638 game modules report SPIR-V
generator word `0x0`, so the shipped dxc version is unknown. Comparisons rely on
the structural fingerprint; normalized-hash equality additionally requires the
same dxc version (see `validation-pipeline.md`).

## spirv-cross (SPIR-V → HLSL decompilation aid)

| Field | Value |
|---|---|
| Tool | `spirv-cross` |
| Version | 1.4.363.0 (brew bottle; built 2026-09-16) |
| Install path | `/opt/homebrew/bin/spirv-cross` |
| Binary sha256 | `242d2c694be3ec9191002a2d066e068323a79479a59453ecd2e14b2e1b461d0f` |

Usage (verified on a real game module):
```sh
spirv-cross --hlsl --shader-model 64 --entry cs_main module0.spv > out.hlsl
```
The default (legacy) HLSL mode rejects the game's separate image/sampler
bindings (`Separate image and samplers not supported in legacy HLSL`); the
`--shader-model 64` flag is required. Output preserves resource names (e.g.
`$Globals`, `img_in`, `img_out`) and is a decompilation aid only — it is
compiler-shaped, not the original HLSL source.

Fallback if unusable: manual / `rspirv`-based analysis (none built; the base
`spirv-cross` binary proved sufficient for HLSL output on real modules).

## Installer notes

- Only two packages were installed (dxc + spirv-cross); no unrelated packages.
- The original game `.bin` shaders were not modified; the extraction reads them
  read-only.
- Tool self-checks and proof runs are recorded in:
  - `self-test-proof.json` (structural-identity sanity + compile-command extraction)
  - `self-test-validator.json` (validator PASS_NORMALIZED on identical source at two paths)
  - `real-bin-fail-demo.json` (validator FAIL with diff against a real game `.bin`)
