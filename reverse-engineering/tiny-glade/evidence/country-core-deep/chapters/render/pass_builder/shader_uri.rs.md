# render/pass_builder/shader_uri.rs

> 文件作用：ShaderUri 解析（三次 memrchr：':'→entry、'/'→文件名、'.'→stem，无点 unwrap）+ AsComputeShaderSource trait（&str/String 通用实现 + 18 个游戏实现：9 个 nani cull shaders + DoF×5 + bin_lights + GaussBlur + TgsrMain + GenerateStrokes）。

函数总计 52：业务 29 / 内联无独立符号 17 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::render::pass_builder::shader_uri::ShaderUri::parse` | 0x140968960 | 20 | 48B sret = 3×&str 对：rfind(':')→path/entry（无冒号 entry=None）、rfind('/')→文件名、rfind('.')→stem；无点 → unwrap panic | O | render-top/decomp/decomp-0x140968960.txt（原 B_i_api；wp1/contracts/shader_uri.md §1；src/shader_uri.rs（0x140968960；纠正旧 wp1 的 whole/name/entry 语义）） |
| `country_core::render::pass_builder::shader_uri::impl$1::source_hash<ref$<str$> >` | 0x1408a4300 | 48 | calculate_hash(&(self_str, &()))：SipHash-1-3 零密钥，str 写字节后补 0xff | O | render-top/decomp/decomp-0x1408a4300.txt（原 B_n_sys cull_shaders §1；wp1/contracts/shader_uri.md §2（0x1408a4300）） |
| `country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<alloc::string::String>` | 0x140056d30 | 52 | ShaderUri::parse → ComputePipelineDesc{path: String, entry: "cs_main", defines: []}.with_entry(uri.entry) | O | render-top/decomp/decomp-0x140056d30.txt（原 B_i_api §3.3；wp1/contracts/shader_uri.md §2（0x140056d30）） |
| `country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> >` | 0x1405cf0f0 | 52 | 同上（&str 实例；0x1405cf0f0，ICF 同体） | O | render-top/decomp/decomp-0x1405cf0f0.txt（原 B_i_api §3.3；wp1/contracts/shader_uri.md §2） |
| `country_core::render::nani::cull_shaders::impl$30::source_hash` | 0x141624770 | 129 | source_hash = calculate_hash(&("<path>:cs_main", &self))（round3 round2 修正：第二元为 &self 非 &i32；SipHash-1-3 零密钥；串+0xff+4 字节） | O | render-top/decomp/decomp-0x141624770.txt（原 B_n_sys cull_shaders §1（0x141624770/0x141624c10/0x1416250c0/0x141625560）） |
| `country_core::render::nani::cull_shaders::impl$34::source_hash` | 0x141624c10 | 129 | source_hash = calculate_hash(&("<path>:cs_main", &self))（round3 round2 修正：第二元为 &self 非 &i32；SipHash-1-3 零密钥；串+0xff+4 字节） | O | render-top/decomp/decomp-0x141624c10.txt（原 B_n_sys cull_shaders §1（0x141624770/0x141624c10/0x1416250c0/0x141625560）） |
| `country_core::render::nani::cull_shaders::impl$38::source_hash` | 0x1416250c0 | 129 | source_hash = calculate_hash(&("<path>:cs_main", &self))（round3 round2 修正：第二元为 &self 非 &i32；SipHash-1-3 零密钥；串+0xff+4 字节） | O | render-top/decomp/decomp-0x1416250c0.txt（原 B_n_sys cull_shaders §1（0x141624770/0x141624c10/0x1416250c0/0x141625560）） |
| `country_core::render::nani::cull_shaders::impl$42::source_hash` | 0x141625560 | 129 | source_hash = calculate_hash(&("<path>:cs_main", &self))（round3 round2 修正：第二元为 &self 非 &i32；SipHash-1-3 零密钥；串+0xff+4 字节） | O | render-top/decomp/decomp-0x141625560.txt（原 B_n_sys cull_shaders §1（0x141624770/0x141624c10/0x1416250c0/0x141625560）） |
| `country_core::systems::render_loop::compute_passes::dof::impl$11::source_hash` | 0x140a5de00 | 129 | source_hash 同构 = calculate_hash(&(path:entry, &self)) | O | render-top/decomp/decomp-0x140a5de00.txt（原 B_n_sys cull_shaders §1 同法；wp1/contracts/shader_uri.md §2） |
| `country_core::systems::render_loop::compute_passes::dof::impl$3::source_hash` | 0x140a5d4c0 | 129 | source_hash 同构 = calculate_hash(&(path:entry, &self)) | O | render-top/decomp/decomp-0x140a5d4c0.txt（原 B_n_sys cull_shaders §1 同法；wp1/contracts/shader_uri.md §2） |
| `country_core::systems::render_loop::compute_passes::dof::impl$7::source_hash` | 0x140a5d960 | 129 | source_hash 同构 = calculate_hash(&(path:entry, &self)) | O | render-top/decomp/decomp-0x140a5d960.txt（原 B_n_sys cull_shaders §1 同法；wp1/contracts/shader_uri.md §2） |
| `country_core::render::nani::cull_shaders::impl$10::as_compute_pipeline` | 0x141623120 | 136 | NaniCullAndBucket 族 as_compute_pipeline（9 个同构）：ComputePipelineDesc{path: 各变体 URI, entry: "cs_main", defines: [("PERM_oc_pass", i32 Display)]} | O | render-top/decomp/decomp-0x141623120.txt（原 B_n_sys cull_shaders §1（0x141623120..0x141625590；路径长 29/39/37/34/42/47/37/35/42）） |
| `country_core::render::nani::cull_shaders::impl$14::as_compute_pipeline` | 0x141623590 | 136 | NaniCullAndBucket 族 as_compute_pipeline（9 个同构）：ComputePipelineDesc{path: 各变体 URI, entry: "cs_main", defines: [("PERM_oc_pass", i32 Display)]} | O | render-top/decomp/decomp-0x141623590.txt（原 B_n_sys cull_shaders §1（0x141623120..0x141625590；路径长 29/39/37/34/42/47/37/35/42）） |
| `country_core::render::nani::cull_shaders::impl$18::as_compute_pipeline` | 0x141623a10 | 136 | NaniCullAndBucket 族 as_compute_pipeline（9 个同构）：ComputePipelineDesc{path: 各变体 URI, entry: "cs_main", defines: [("PERM_oc_pass", i32 Display)]} | O | render-top/decomp/decomp-0x141623a10.txt（原 B_n_sys cull_shaders §1（0x141623120..0x141625590；路径长 29/39/37/34/42/47/37/35/42）） |
| `country_core::render::nani::cull_shaders::impl$22::as_compute_pipeline` | 0x141623e90 | 136 | NaniCullAndBucket 族 as_compute_pipeline（9 个同构）：ComputePipelineDesc{path: 各变体 URI, entry: "cs_main", defines: [("PERM_oc_pass", i32 Display)]} | O | render-top/decomp/decomp-0x141623e90.txt（原 B_n_sys cull_shaders §1（0x141623120..0x141625590；路径长 29/39/37/34/42/47/37/35/42）） |
| `country_core::render::nani::cull_shaders::impl$26::as_compute_pipeline` | 0x141624300 | 136 | NaniCullAndBucket 族 as_compute_pipeline（9 个同构）：ComputePipelineDesc{path: 各变体 URI, entry: "cs_main", defines: [("PERM_oc_pass", i32 Display)]} | O | render-top/decomp/decomp-0x141624300.txt（原 B_n_sys cull_shaders §1（0x141623120..0x141625590；路径长 29/39/37/34/42/47/37/35/42）） |
| `country_core::render::nani::cull_shaders::impl$30::as_compute_pipeline` | 0x1416247a0 | 136 | NaniCullAndBucket 族 as_compute_pipeline（9 个同构）：ComputePipelineDesc{path: 各变体 URI, entry: "cs_main", defines: [("PERM_oc_pass", i32 Display)]} | O | render-top/decomp/decomp-0x1416247a0.txt（原 B_n_sys cull_shaders §1（0x141623120..0x141625590；路径长 29/39/37/34/42/47/37/35/42）） |
| `country_core::render::nani::cull_shaders::impl$34::as_compute_pipeline` | 0x141624c40 | 136 | NaniCullAndBucket 族 as_compute_pipeline（9 个同构）：ComputePipelineDesc{path: 各变体 URI, entry: "cs_main", defines: [("PERM_oc_pass", i32 Display)]} | O | render-top/decomp/decomp-0x141624c40.txt（原 B_n_sys cull_shaders §1（0x141623120..0x141625590；路径长 29/39/37/34/42/47/37/35/42）） |
| `country_core::render::nani::cull_shaders::impl$38::as_compute_pipeline` | 0x1416250f0 | 136 | NaniCullAndBucket 族 as_compute_pipeline（9 个同构）：ComputePipelineDesc{path: 各变体 URI, entry: "cs_main", defines: [("PERM_oc_pass", i32 Display)]} | O | render-top/decomp/decomp-0x1416250f0.txt（原 B_n_sys cull_shaders §1（0x141623120..0x141625590；路径长 29/39/37/34/42/47/37/35/42）） |
| `country_core::render::nani::cull_shaders::impl$42::as_compute_pipeline` | 0x141625590 | 136 | NaniCullAndBucket 族 as_compute_pipeline（9 个同构）：ComputePipelineDesc{path: 各变体 URI, entry: "cs_main", defines: [("PERM_oc_pass", i32 Display)]} | O | render-top/decomp/decomp-0x141625590.txt（原 B_n_sys cull_shaders §1（0x141623120..0x141625590；路径长 29/39/37/34/42/47/37/35/42）） |
| `country_core::systems::render_loop::compute_passes::bin_lights::impl$2::as_compute_pipeline` | 0x1409e8db0 | 136 | 游戏 compute pass 结构体（DoF×5/BinLights/GaussBlur/TgsrMain/GenerateStrokes）的 as_compute_pipeline：同构（path=各自 URI、entry=cs_main 或 cs_halfres(_all)、defines 经 pipeline_defines 注入 PERM_*） | O | render-top/decomp/decomp-0x1409e8db0.txt（原 B_i_api §3.3/§5.6；wp1/contracts/shader_uri.md §3（URI 表；0x140a5d4f0..0x1408aab70）） |
| `country_core::systems::render_loop::compute_passes::dof::impl$11::as_compute_pipeline` | 0x140a5de30 | 136 | 游戏 compute pass 结构体（DoF×5/BinLights/GaussBlur/TgsrMain/GenerateStrokes）的 as_compute_pipeline：同构（path=各自 URI、entry=cs_main 或 cs_halfres(_all)、defines 经 pipeline_defines 注入 PERM_*） | O | render-top/decomp/decomp-0x140a5de30.txt（原 B_i_api §3.3/§5.6；wp1/contracts/shader_uri.md §3（URI 表；0x140a5d4f0..0x1408aab70）） |
| `country_core::systems::render_loop::compute_passes::dof::impl$15::as_compute_pipeline` | 0x140a5e290 | 136 | 游戏 compute pass 结构体（DoF×5/BinLights/GaussBlur/TgsrMain/GenerateStrokes）的 as_compute_pipeline：同构（path=各自 URI、entry=cs_main 或 cs_halfres(_all)、defines 经 pipeline_defines 注入 PERM_*） | O | render-top/decomp/decomp-0x140a5e290.txt（原 B_i_api §3.3/§5.6；wp1/contracts/shader_uri.md §3（URI 表；0x140a5d4f0..0x1408aab70）） |
| `country_core::systems::render_loop::compute_passes::dof::impl$19::as_compute_pipeline` | 0x140a5e700 | 136 | 游戏 compute pass 结构体（DoF×5/BinLights/GaussBlur/TgsrMain/GenerateStrokes）的 as_compute_pipeline：同构（path=各自 URI、entry=cs_main 或 cs_halfres(_all)、defines 经 pipeline_defines 注入 PERM_*） | O | render-top/decomp/decomp-0x140a5e700.txt（原 B_i_api §3.3/§5.6；wp1/contracts/shader_uri.md §3（URI 表；0x140a5d4f0..0x1408aab70）） |
| `country_core::systems::render_loop::compute_passes::dof::impl$3::as_compute_pipeline` | 0x140a5d4f0 | 136 | 游戏 compute pass 结构体（DoF×5/BinLights/GaussBlur/TgsrMain/GenerateStrokes）的 as_compute_pipeline：同构（path=各自 URI、entry=cs_main 或 cs_halfres(_all)、defines 经 pipeline_defines 注入 PERM_*） | O | render-top/decomp/decomp-0x140a5d4f0.txt（原 B_i_api §3.3/§5.6；wp1/contracts/shader_uri.md §3（URI 表；0x140a5d4f0..0x1408aab70）） |
| `country_core::systems::render_loop::compute_passes::dof::impl$7::as_compute_pipeline` | 0x140a5d990 | 136 | 游戏 compute pass 结构体（DoF×5/BinLights/GaussBlur/TgsrMain/GenerateStrokes）的 as_compute_pipeline：同构（path=各自 URI、entry=cs_main 或 cs_halfres(_all)、defines 经 pipeline_defines 注入 PERM_*） | O | render-top/decomp/decomp-0x140a5d990.txt（原 B_i_api §3.3/§5.6；wp1/contracts/shader_uri.md §3（URI 表；0x140a5d4f0..0x1408aab70）） |
| `country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$8::as_compute_pipeline` | 0x1408ca1c0 | 136 | 游戏 compute pass 结构体（DoF×5/BinLights/GaussBlur/TgsrMain/GenerateStrokes）的 as_compute_pipeline：同构（path=各自 URI、entry=cs_main 或 cs_halfres(_all)、defines 经 pipeline_defines 注入 PERM_*） | O | render-top/decomp/decomp-0x1408ca1c0.txt（原 B_i_api §3.3/§5.6；wp1/contracts/shader_uri.md §3（URI 表；0x140a5d4f0..0x1408aab70）） |
| `country_core::systems::render_loop::compute_passes::taa::impl$1::as_compute_pipeline` | 0x140a3e100 | 136 | 游戏 compute pass 结构体（DoF×5/BinLights/GaussBlur/TgsrMain/GenerateStrokes）的 as_compute_pipeline：同构（path=各自 URI、entry=cs_main 或 cs_halfres(_all)、defines 经 pipeline_defines 注入 PERM_*） | O | render-top/decomp/decomp-0x140a3e100.txt（原 B_i_api §3.3/§5.6；wp1/contracts/shader_uri.md §3（URI 表；0x140a5d4f0..0x1408aab70）） |
| `country_core::systems::render_loop::vermeer::impl$7::as_compute_pipeline` | 0x1408aab70 | 136 | 游戏 compute pass 结构体（DoF×5/BinLights/GaussBlur/TgsrMain/GenerateStrokes）的 as_compute_pipeline：同构（path=各自 URI、entry=cs_main 或 cs_halfres(_all)、defines 经 pipeline_defines 注入 PERM_*） | O | render-top/decomp/decomp-0x1408aab70.txt（原 B_i_api §3.3/§5.6；wp1/contracts/shader_uri.md §3（URI 表；0x140a5d4f0..0x1408aab70）） |

## 内联/无独立符号函数

- `country_core::render::pass_builder::shader_uri::impl$1::source_hash`（行 48）— source_hash<&str> 泛型（行 48）—— O（见实例）
- `country_core::render::pass_builder::shader_uri::impl$2::source_hash`（行 61）— 通用 T 实现（行 61，inline 1）—— I
- `country_core::render::pass_builder::shader_uri::impl$2::as_compute_pipeline`（行 65）— 通用 T 实现（行 65）—— I
- `country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value`（行 75）— AsShaderDefValue：编译期特性值（行 75）—— I
- `country_core::render::pass_builder::shader_uri::impl$4::as_shader_def_value`（行 81）— AsShaderDefValue（行 81）—— I
- `country_core::render::nani::cull_shaders::impl$10::source_hash`（行 129）— ICF/内联（as_compute_pipeline 同址取 hash）—— O
- `country_core::render::nani::cull_shaders::impl$14::source_hash`（行 129）— ICF/内联（as_compute_pipeline 同址取 hash）—— O
- `country_core::render::nani::cull_shaders::impl$18::source_hash`（行 129）— ICF/内联（as_compute_pipeline 同址取 hash）—— O
- `country_core::render::nani::cull_shaders::impl$22::source_hash`（行 129）— ICF/内联（as_compute_pipeline 同址取 hash）—— O
- `country_core::render::nani::cull_shaders::impl$26::source_hash`（行 129）— ICF/内联（as_compute_pipeline 同址取 hash）—— O
- `country_core::systems::render_loop::compute_passes::bin_lights::impl$2::source_hash`（行 129）— 同构 source_hash —— O
- `country_core::systems::render_loop::compute_passes::dof::impl$15::source_hash`（行 129）— 同构 source_hash —— O
- `country_core::systems::render_loop::compute_passes::dof::impl$19::source_hash`（行 129）— 同构 source_hash —— O
- `country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$8::source_hash`（行 129）— 同构 source_hash —— O
- `country_core::systems::render_loop::compute_passes::taa::impl$1::source_hash`（行 129）— 同构 source_hash —— O
- `country_core::systems::render_loop::vermeer::impl$7::source_hash`（行 129）— 同构 source_hash —— O
- `country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute`（行 237）— 计算源码 hash 的辅助（inline 22；calculate_hash 转发）—— I

## 派生胶水（6 项：eq、fmt、hash 等）

6 项 glue = impl$18::fmt（0x140939420 Debug，O）+ impl$5::hash/impl$7::eq（ShaderUri 作 HashMap 键）+ taa/vermeer/rasterize_terrain_stroke 的 impl$2/8/9::hash（ShaderDefValue 派生）。

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `ShaderUri::parse`（0x140968960）—— 三段 URI 解析
算法（O）：三次 `memrchr`（0x140968960 系）：
1. `rfind(':')` → path / entry（**entry 可选**，无冒号 → None）；
2. `rfind('/')` → 文件名；
3. `rfind('.')` → stem（**无 '.' → `Option::unwrap` panic**）。
48B sret = 3×&str 胖指针。旧 wp1 合同把 entry 当必选、三段含义全错（纠错，B 阶段）。
- 数据流：`ComputePass::new(name, uri)` 与 `startup_*` 的 `AsComputeShaderSource::as_compute_pipeline` 都经此解析；`path` 进 `ComputePipelineDesc{path: String, entry: "cs_main"}.with_entry(uri.entry)`。

### 2. `source_hash` 族 —— 着色器缓存键
算法（O）：`calculate_hash(&(uri_str, &self))` = SipHash-1-3 **零密钥**（DefaultHasher::new()），str 写字节后补 0xff；`impl$1::source_hash<&str>`（0x1408a4300）第二元是 `&()`；9 个 cull shader（0x141624770 系）与 DoF×5（0x140a5de00 系）第二元是 `&self`（round3 修正：非 &i32）。与 `render::utils::calculate_hash`（0x1405bdda0）逐位一致。
- 用途：`ComputePass::new` 的 `get_compute_by_name` 缓存键（着色器只编译一次）+ `source_hash()` 公共接口。

### 3. `as_compute_pipeline` 族 —— 18 个游戏实现
- 通用 `&str`/`String`（0x1405cf0f0/0x140056d30）：`ComputePipelineDesc{path, entry:"cs_main", defines:[]}`。
- 9 个 `NaniCullAndBucket*`（0x141623120..0x141625590）：`entry:"cs_main"`、`defines:[("PERM_oc_pass", i32 Display)]`（oc_pass 值 0..4 由 `NaniCullAndBucketFamily::for_kind` 决定）。
- DoF×5（`FullResAllLod0/Lod1`、`FullRes`、`HalfRes`、`HalfResAll`）+ `BinLights`（PERM_write）+ `GaussBlur{fmt}`（PERM_fmt/PERM_ty="float" I）+ `TgsrMain{wave32,sharpen}`（PERM_wave32/PERM_sharpen）+ `GenerateStrokes{photomode}`（PERM_photomode）：`defines` 经 `pipeline_defines()` 注入（B_i_api §3.3/§5.6），entry = `cs_main` 或 `cs_halfres(_all)`。
- 形态：带 PERM_* 的源在 `as_compute_pipeline` 直接产出带 defines 的 `ComputePipelineDesc`（2311B 形态）；不带的是 1943B（B_i_api §3.3）。

## 证据注记（tg_rn1）
- **parse 语义纠错**（B 阶段 vs 旧 wp1 合同）：三段 = `path`（':' 前全部）/ `stem`（文件名去末扩展名）/ `entry`（':' 后，**可选**）；旧合同把 entry 当必选、三段含义全错；无 '.' → `Option::unwrap` panic。
- **source_hash**（O）：SipHash-1-3 零密钥（`DefaultHasher::new()`），str 写字节后补 0xff；`calculate_hash` 0x1405bdda0 与 `render::utils::calculate_hash` 逐位一致。
- **18 个游戏实现**（O，URI 全表见 wp1/contracts/shader_uri.md §3）：9 个 `NaniCullAndBucket*`（/nani/cull_and_bucket{,_bush,_curved_halftimber,_flowers,_plaster,_roof_tile,_smoke,_stairs_floor,_wooden_plank}.cs.hlsl）+ DoF×5 + BinLights + GaussBlur + TgsrMain + GenerateStrokes；PERM 变体在 `as_compute_pipeline`（带 defs）而非 `source_hash`。
- `ComputePass::new` 的 2311B（带 PERM_* 源）与 1943B（&str）两种形态（B_i_api §3.3）。
