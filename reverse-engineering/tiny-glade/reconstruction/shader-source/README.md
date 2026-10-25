# reconstruction/shader-source — HLSL reconstruction tree (expected layout)

This directory holds the reconstructed HLSL source tree for Tiny Glade's shaders,
mirroring the on-disk layout the original game compiled from. The layout is
extracted from the debug info embedded in the 404 game `.bin` shaders (see
`evidence/render-passes/shaders/embedded-shader-source-paths.txt` and
`evidence/render-passes/validation/compile-commands.json`).

## Layout

```
reconstruction/shader-source/
  shaders/
    <pass>.cs.hlsl              # compute shaders (entry cs_main, cs_halfres, ...)
    <pass>.raster.hlsl          # raster shaders (entries vs_main + ps_main)
    inc/                        # shared includes (common.hlsl, samplers.hlsl, ...)
    <family>/<pass>.cs.hlsl     # family subdirectories (dof/, gi/, blur_pyramid/, ...)
```

Expectation (contract with `tools/render-passes/recompile_check.py`):

- The **include root** passed to dxc as `-I` is
  `reconstruction/shader-source/shaders`. Original sources use includes relative
  to that root, e.g. `#include "inc/common.hlsl"`, so a reconstructed file must
  keep the same relative include paths for recompilation to resolve.
- Each `.cs.hlsl` / `.raster.hlsl` path must equal the `main_source_path`
  recorded in `evidence/render-passes/validation/compile-commands.json` for the
  `.bin`(s) that embed it (the filename pass-stem roundtrip, verified 404/404).
- 419 unique embedded source paths exist in the original debug info; 68 are
  `.inc.hlsl` includes. All of them should be reconstructed for full coverage.

## How a reconstruction agent validates a shader

```sh
python3 tools/render-passes/recompile_check.py \
    reconstruction/shader-source/shaders/<pass>.cs.hlsl \
    "<game>/compiled-shaders/_<pass>.cs.hlsl_<entry>_0.bin"
```

See `evidence/render-passes/validation/validation-pipeline.md` for the full
workflow, the structural fingerprint / normalized-hash methodology, and
troubleshooting.

## Rules

- The 404 `.bin` files are the read-only oracle; never modify them.
- Do not "fix" a `.bin` to make a check pass — the validator compares
  structure, and a failing shader means the reconstruction or the include
  graph differs from the original.

## 重建状态总表
每个 HLSL 文件的类别、功能、实现方式（提升/手写）、结构门禁与差分执行结果和备注见 [SHADER_STATUS.md](SHADER_STATUS.md)（机器可读版 `SHADER_STATUS.json`），由 `tools/render-passes/build_status_table.py` 从 `evidence/render-passes/integration/*.json` 生成。
