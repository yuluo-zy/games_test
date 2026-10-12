# G5 取证：GI 与光照 compute pass

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 方法：`run-lanes --lanes 2`。database `ccgi`（helpers/strings 用 `ccgi2/3/4`）。
- 产物：`evidence/render-recovery/producer-closure/g5-gi-lighting/`（job JSON ×4、4 份 REPORT、29 份 `0x*.c`、lane 报告）。
- 仅取证，未改任何受限路径。

## 0. 关键地址

| 函数 | 地址 |
|---|---|
| `build_cwbvh_from_tris` | `0x1417c93d0` |
| `build_gpu_bvh` | `0x140b0da50` |
| `calc_terrain_lighting_texture` | `0x140a51440` |
| `calculate_ssgi` | `0x1408743b0` |
| `ray_trace_gi` | `0x140a01cf0` |
| `eval_gi_screen_radiance` | `0x140a44cd0` |
| `bin_lights` | `0x1409e7770`（`as_compute_pipeline` `0x1409e8db0`） |
| `restir_sample_direct_lights` | `0x140a6fec0` |
| `apply_screen_shadows` | `0x140a39e70` |
| `compute_shadow_mask_mean` | `0x140a3a6f0` |
| `deferred_raster` | `0x1409d20b0` |
| `extract_half_res_depth_normals` | `0x1408f6180` |
| `calculate_reprojection_map` | `0x140a0d190` |
| 访问码归并 | `0x1409dca90` |

## 1. GI 数据流

### BVH —— CPU 建、软件遍历（**再证无硬件光追**）

- `build_cwbvh_from_tris`(`0x1417c93d0`)：用 **SIMD**(`minps`/`maxps`) 算 AABB + 面积，**SAH 常数 `0x3fcccccd` / `0x3fe66666`**（≈1.6 / 1.8），调 obvhs 内部 `FUN_1420e9710`/`FUN_1420e5450`/`FUN_1420f2480`，读 config `+0x19`(bool)/`+0x1a`/`*config`(leaf)，返回**节点 + primitive-index 数组**。
- `build_gpu_bvh`(`0x140b0da50`)：把三角形 **stride 0x28 → 0x30 重排**后调用之，输出节点数组 + 重排索引。
- 上传属 `resources::rt`（`RtWorld`；系统 `upload_rt_world` 取 `RtBvhBuilderGlueResource` + `AssetSsboLibrary` → `RtBvhSsbo`/`RtTrianglesSsbo`/`RtTriangleMetaSsbo`）。
- 着色器 `_gi_trace_gi_candidates` 声明 `rt_bl_bvh`/`rt_triangles`/`rt_triangle_meta`，**无 RayTracing cap** ⇒ **确认软件 CW-BVH 遍历**。

### candidate（`ray_trace_gi` `0x140a01cf0`，建 2 个 ComputePass）

着色器 `_gi_trace_gi_candidates` 输入：
`terrain_heightmap`、`visual_terrain_heightmap`、`blue_noise`、`terrain_lighting_texture`、`color_library`、`shadow_map`、`wall_color_ids`、`rt_*`、**`taa_history_tex`**、**`reprojection_tex`**、**`half_depth_tex`**、**`half_view_normal_tex`**
→ 输出 `candidate_radiance_tex`、`candidate_hit_tex`。

### radiance 更新

`_gi_update_gi_radiance` 读 `depth_tex` + candidates → 写 `restir_radiance_out_tex` / `restir_ray_dir_out_tex` / `restir_reservoir_out_tex`。

### evaluation（`eval_gi_screen_radiance` `0x140a44cd0`，建 3 个子 dispatch，名长 20/32、12/37、9/32）

- `_gi_eval`：读 `blue_noise` + `half_depth` + `half_normal` + **`ssgi_tex`** + `restir_radiance/ray_dir/reservoir` → 输出；
- `_gi_blur`：另写 `temporal_stats_out`；
- `_gi_reproject`：读 `history` + `reprojection` + `temporal_stats`。

CPU 端资源按 **u64 key** 登记。

## 2. `restir_di`

单个 compute pass（`0x140a6fec0`）。读 ECS 资源 TypeId `0x8226cc786e679355`（与 `bin_lights`、`deferred_raster` **共用**），bind 均为 code **`0x19`**：`0x815bf5036966978c`、`0xb8a74ccd835a6be3`、`0x2a333c3a28028230`、`0x350f8dc714051d10`、`0x4da9887cf3dc26f4`；另 `0x8daeeefe7463dee0`(`0x32`)；uniform `param_7` / `param_7+0x18`。

> **reservoir 的字段 / 变体 / 权重 / 偏差修正 = unknown。** 蓄水池是 **GPU 纹理** `restir_reservoir_out_tex`，CPU 侧无结构体、PDB 无字段名。**不套论文。**

## 3. `ssgi` 与 `ray_trace_gi`：互补，不是二选一

- **`ssgi`**（`_ssgi_ssgi`：depth + half_normal → output；`_ssgi_temporal_filter`：input + history + reprojection → final/history；`_ssgi_upsample`：ssgi + depth + half_depth + half_normal + gbuffer → output）负责**屏幕空间项**；
- **`ray_trace_gi`** 负责**世界空间 BVH 追踪 + ReSTIR**；
- `eval_gi` **同时读 `ssgi_tex` 与 `restir_*`** 合成。

## 4. `bin_lights`(`0x1409e7770`)

读 TypeId `0x8226cc786e679355`；**光数为 0 → 仅建小 raw resource 并返回 3 个句柄**；否则建多个 compute 子 pass，bind SSBO key `0x63e587f19a869d94`/`0xaab861d318ad7248`/`0xc3674440971f6990`/`0xe3b8c78bd96aa31d`，raw resource 尺寸 `0x44`、`0x100000`，uniform `0x9b88329e408ff69f` = 光数，返回 5 句柄。

**分桶的字段布局 = unknown**（着色器定义，PDB 未命名）。可确定是「光源记录 + 分桶/网格索引 SSBO」。

## 5. `apply_screen_shadows` / `calc_terrain_lighting_texture`

- `calc_terrain_lighting_texture`(`0x140a51440`)：**1 个** compute pass，bind `0xb981790202418bc8`(`0x32`，输出)、`0x61b08b5e0470c5f2`(`0x19`，即 `terrain_lighting_texture`，`ray_trace_gi` 也读)、`0x297d1e4cdae8f657`(`0x19`)，`write_uniform` `0x4bbdb30b873d3990`。**在光栅 GI 之前算出地形光照纹理。**
- `apply_screen_shadows`(`0x140a39e70`)：**1 个** compute pass，bind `0x41aa4a0d2dea0b36`/`0x4861579da049fbfc`/`0x185a603c077dad19`/`0xb8a74ccd835a6be3`(`0x19`) + `0x88219a3ae69892b2`(`0x32`)，uniform `0xa9a109b760eed902`。
- `compute_shadow_mask_mean`(`0x140a3a6f0`)：把阴影掩码直方图归约为均值 SSBO（`ShadowMaskMeanSsbo` / `ShadowMaskHistogramSsbo`）。

## 6. GI 用哪套 depth/normal + 跨帧

- 两轮光栅可见性（`main_pass` → `main_pass_oc2`）及 `late_forward_and_deferred` **之后**才 `half_res`（`extract_half_res_depth_normals`，从全分辨率 depth + gbuffer 生成半分辨率深度/法线）。
- GI trace 着色器**显式声明 `half_depth_tex` / `half_view_normal_tex`** ⇒ **GI 用半分辨率 depth/normal**。
- **跨帧**：trace 读 `taa_history_tex` + `reprojection_tex`；blur/reproject 用 history + reprojection + temporal_stats。即 **GI 做时域重投影**（重投影图由 `calculate_reprojection_map` 用 depth/prev_depth/velocity 生成）。

## 7. 访问码实测

`FUN_1408ee240` / `FUN_1408edd00` / `FUN_140905260` / `FUN_1409056d0` / `FUN_14096f090` 的**第 4 参直接作为 access_code** 传给归并函数 `FUN_1409dca90(self, res, code, flag)`（**已代码验证**）。

| 码 | 出现处 |
|---|---|
| **`0x19`** | compute 读取（占绝对多数） |
| **`0x32`** | `FUN_1408edd00`，各 pass 的「输出/中间」绑定 |
| **`0x37`** | 写入；`eval_gi` 的 `FUN_1408edd00(..., 0x88219a3ae69892b2, 0x37, ..)` |
| **`0x1d`** | draw pass 读取；`deferred_raster` 的 `FUN_1408ee800`/`FUN_140908610` |
| `0x34` / `0x02` / `0x17` / `0x1a` / `0x1e` / `0x24` | 见归并逻辑与既有记录 |

归并函数实测分支：**`0x37` 覆盖 `0x19`/`0x1a`**；**`0x17` ↔ `0x1d` → `0x1e`**；**`0x1a` + `0x02` → `0x02`**。（与 R2 的复核一致。）

> ⚠️ **注意**：`FUN_14096d490`（`get_or_create_temporal_pair`）的第 4 参是**名字长度**，**不是访问码**。勿混。

**`0x32` 与 `0x19` 的确切命名（read vs read-write/storage）未定。**

## 8. 观察 / 推断 / 未知

**观察**：地址 / 体积 / 调用序列、access 归并、BVH 软件构建（含 SAH 常数）、着色器绑定集、半分辨率输入、共用的 u64 资源 key。

**推断**：u64 key ↔ 资源名的映射（跨 pass 复用，如 `0x61b08b5e0470c5f2` = `terrain_lighting_texture`）；`0x88219a3ae69892b2` = 主输出；TypeId `0x8226cc786e679355` = 光照/窗口光资源；`0x32` = storage 绑定。

**未知**：reservoir 字段 / 变体 / 权重；`bin_lights` 分桶字段；`0x32`/`0x1d` 的命名；`UNK_142ad…` 字符串未在 Ghidra 定义（`get_strings` 仅回 PE 导入）；**资源名哈希不可反推**。

依项目规则：伪代码 ≠ 原 Rust；**xref 无 CALL 边**（本批未用 xref 断言调用）；`disassemble_function` 恒 0 指令；`calls.json` 仅 450 函数、**不含本层**。**未宣称构图闭包已闭合。**
