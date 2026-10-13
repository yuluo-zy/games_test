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


---

## 保真度门槛与批量工具（2026-10-13 增补）

结构指纹的硬门（entry/local_size/bindings）不能发现"编造的实现"。增补两项指标并提供批量工具（均在 `tools/render-passes/`）：

| 工具 | 作用 |
|---|---|
| `sweep_all.py` | 并行重编译全部 404 个 bin 的 main，输出每模块 `ratio`（实指令数比）、`hist_dist`（opcode 直方图距离/目标指令数）、硬门。每个 bin 自动传 `-D TGV_<id>`。`$DXC`、`LD_LIBRARY_PATH` 需设置。 |
| `batch_lift.py` | 用 spirv-cross 提升 bin 并替换低保真手写。整文件模式；`--patch` 只提升失败模块并保留其余手写代码。验收：全部模块可编译、硬门通过、ratio ≥ 0.8 且（距离降低或 < 0.3），否则自动回滚。 |
| `variant_defines.json` | 旧手写变体宏（f0ad/NO_HZB 等）。 |

典型流程：`sweep_all.py` → `batch_lift.py --patch`（或整文件）→ `sweep_all.py` 复核。

提升时的已知修补（见 `batch_lift.py`）：`type_*` cbuffer 名还原、push constant 转 `[[vk::push_constant]]`、`_alias_` 双下划线、嵌套数组拷贝 bug、`float(bool)` 转换（dxc 1.9 生成无效 SPIR-V）、`frexp` 绕过、PhysicalStorageBuffer 改 Logical 并用 `vk::RawBufferLoad`。已知例外：final_post f0ad ps（无输出空壳，dxc DCE）。


### 差分执行（2026-10-13 增补，比指令量更强的验证）

`sweep_all.py` 的比值/直方图距离只是规模代理，**不能证明算法一致**：当时通过该门槛的手写 compute 中有 22 个模块输出与原作不同。`tools/render-passes/diff_exec.py` + `tools/spv-diff`（Rust/ash，lavapipe）实际执行：同一份种子随机输入（4 种 profile）分别跑原作 bin 与当前重建的 SPIR-V，比较全部可写 SSBO/存储图像（浮点容差 2e-4，f16 2e-3，unorm ±1）。

- 构建：`cd tools/spv-diff && CARGO_TARGET_DIR=/tmp/spvdiff-target cargo build --offline --release`。运行：`DXC=... LD_LIBRARY_PATH=... python3 tools/render-passes/diff_exec.py`（驱动自动设 `LP_NUM_THREADS=0`，否则 lavapipe 的原子/前缀和顺序不确定；开启 robustBufferAccess，否则随机 UBO 当下标会使 lavapipe 段错误）。
- 对照：`--self-test`（原作对原作）必须全部 SAME，才说明差异可归因于重建（基线 140 SAME / 0 DIFF）。
- 结论：整文件提升的 13 个 compute 全部 SAME；手写的 22 个 DIFF（dof ×10、ssgi ×2、taa、tgsr ×2、leaf_particles、path_pebbles、ice_refl_march/resolve、spawn_lilypads、read_mouse_depth_value、wall pass2）已用 `batch_lift.py --from-diff` 提升，复跑 140 SAME / 0 DIFF。
- 未覆盖：14 个 SKIP（阵列/多重采样图像、描述符数组）、12 个 VACUOUS（随机输入下输出从未被写入）、4 个 ERROR（grass 前缀和、rasterize_buildings 超时、rasterize_terrain_stroke 崩溃）；**全部 raster VS/PS（468 模块）尚无执行级验证**。

**画面程序（PS）差分执行**（`diff_exec_ps.py`）：生成一个全屏三角形 VS，给 PS 的每个输入 location 喂常量 varying，原版 PS 与重建 PS 经同一管线渲染，比较全部颜色目标与可写 SSBO/存储图像（8 组 profile）。基线（原作对原作）：b903 完整变体 117 个中 105 个有效（其余无输出/不支持），f0ad 变体几乎全为无输出空壳。结论：手写 PS 里发现 5 个文件差异（ice_plane、session_selection/background、skydome、text、solid_vertex_color），除 ice_plane 外已提升替换并复跑一致。ice_plane（已是提升版）仅在"整数填充"profile（1、5）下 rt2 的法线量化字节不同，其余 6 组一致；输入为非规格化浮点，判为输入伪影，仍需关注。未覆盖：2 个 bindless 描述符崩溃（nani_cloth、window_glass）、1 个 MSAA、**全部 VS**。


### 差分执行汇总（2026-10-13 终稿，16 组输入）

输入 16 档（`gen_word`）：0 单位浮点 / 1 小整数 / 2 混合 / 3 有符号 / 4 温和范围 / 5 温和整数 / 6 单位矩阵 / 7 全 0 / 8 全 1 / 9 极大 / 10 2 的幂 / 11 稀疏 / 12 近似常数 / 13 递增序列 / 14 很大浮点 / 15 极小有符号。判定：**SAME**（全部相同）、**DIFF**（在"数据正常"档 0/2/3/4/8/12 不一致，真问题）、**EDGE**（只在边界档不一致，多为未定义行为：除零、NaN/溢出转整数）、VACUOUS（原版没写任何输出）、SKIP/CRASH/ERROR（工具限制或 lavapipe 对原版本身也崩溃）。VS 比较时忽略位置为 NaN 的顶点；PS 用生成的 VS 喂输入；VS 经 SPIR-V 改写把输出写入捕获缓冲。

结果（638 模块）：SAME 488（手写 256 + 反推 232）、VACUOUS 125、EDGE 8、DIFF 1、SKIP 8、CRASH 5、ERROR 3。

验证中发现并修复的工具问题（说明这类检查为何必要）：`float(x & 255u)` 被误改成布尔转换（模板尖括号被当比较符）；dxc 会压缩 stage IO 的 location，需 `[[vk::location]]`；`--reflect-io` 参数检查错误曾使 PS 验证没有输入；descriptor set 15 超出 lavapipe 上限。


#### 差分执行最终结论（并行收尾后）

638 模块：**SAME 505**、空壳 125、EDGE 7、DIFF 1；结构门槛 637/638（唯一例外 final_post f0ad ps，dxc DCE）。运行器已补多重采样图像（`msfill.hlsl`）、无绑定纹理（1024 描述符）、物理地址（写入名字含 `addr` 的成员，含数组元素）、1-D 前缀和 shader 只调度一个线程组（原先越界冲坏 lavapipe 堆）。

SAME 中有 6 个在"数据正常"档（0/2/3/4/8/12）全部崩溃、只在边界档比较成功，可信度较弱：rasterize_buildings、rasterize_terrain_stroke、nani_cloth PS、nani_solid_vertex_color_window_glass PS、nani_plaster VS（2 个变体）。

剩余 8 项判定：
- **bin_lights_to_cells n_14dca8ba（DIFF）**：不是手写/反推 bug；仅 profile 0 一个单元计数差 1，疑为灯光球-单元边界比较的浮点翻转（推断，无直接证据）。
- **path_pebbles（EDGE）**：原版代码对 `$Globals` 值做整数除法，值为 0 时除零，属未定义行为；把除数强制为 1..256 后两个差异档均 SAME。
- **ice_plane PS ×2（EDGE）**：剩余差异档里 rt2 低 16 位是 half(NaN)，原版编码 0x7e00、候选 0xfe00，仅符号位不同；NaN 来自退化数据上的 length/normalize。部分档把矩阵置为单位阵后 SAME，其余档未能构造出有限输入，证据是 NaN 位型一致。
- **lilypad / reed VS ×4（EDGE）**：浮点融合乘加（FMA）舍入差异，被 floor 与哈希放大；给反推文本的 float 局部变量加 `precise` 后在 8250~1e6 尺度全部一致。原版 SPIR-V 无 NoContraction，两种结果均合法。
- **dof/analyze（工具链导致的真实差异）**：dxc 1.9 对用到 quad 操作的 compute shader 额外加 `DerivativeGroupQuadsKHR` 执行模式，改变 quad 成员定义。独立 Python 模型与原版完全一致，dxc 1.9 版本不一致；去掉该模式后 SAME（`spv_strip_dgq.py`，已接入 `diff_exec.py`）。HLSL 本身无法规避（`-fspv-extension` 白名单会产生非法 SPIR-V）。

## 覆盖导向输入搜索（compute 的 9 个"无写入"模块）
- spv-diff 新增 profile ≥16（`gen_mixed`）：每个 32 位字独立取"小整数 / 单位浮点 / 0"之一，同一块 buffer 里既有计数/索引字段也有浮点数据。16–31 为良态混合（计入 CORE_PROFILES，不一致即 DIFF）；32+ 含 0/超大值（仅在此处不一致 → EDGE）。
- 结果：9 个中 8 个原版真正写出输出并与重建版一致（SAME）；`water/spawn_waterwheels` 仅 3 条指令，是真空壳。其中 5 个手写版（garden/spawn_bushes_buildings、spawn_flowers_buildings、spawn_small_flowers_garden、water/spawn_boats、spawn_reeds）在新输入下暴露真差异（DIFF），已用 batch_lift 提升原版 SPIR-V 替换，重测 profile 0–63 全部 SAME。garden/marching_squares 需 64–400 号种子才触发写入（coverage 33967），SAME；nani/cull_and_bucket 系列 18 个变体全部 SAME。

## PS 深度附件验证（discard 可观察）
- `spv-diff --gfx` 增加 D32_SFLOAT 深度附件（清为 1.0，ALWAYS + 写深度，VS z=0.5），读回后作为 `name=depth` 比较；被 discard 的像素保持 1.0。`diff_exec_ps.py` 结果新增 `depth_written`（每档 profile 写入像素数，满屏 4096）。
- 原先 115 个 VACUOUS PS 中 113 个变为有信息：107 个直接 SAME，另有 6 个手写 f0ad/b903 PS 暴露为 DIFF/EDGE（lilypad、nani_solid_vertex_color、roof_snow、solid_vertex_color、splash_particle、water_snow_plane，手写 discard 逻辑与原版不同），已用 batch_lift --from-diff 提升替换，复跑后全部 SAME。
- 最终 PS：SAME 232 / VACUOUS 2（painterly/debug_target，原版真空壳）。
