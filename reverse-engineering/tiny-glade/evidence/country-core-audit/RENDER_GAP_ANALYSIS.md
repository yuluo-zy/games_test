# 渲染模块未实现项分析

日期：2026-10-14。基线：B 阶段提交 `b5965593` 之后，并已同步到远端 `a205bd8a`（含 foundation 七库与 415 个 HLSL 的新提交）。
本文只做分析，没有改任何源码。数据来源和复现方法见 §9。标注：O 实测 / I 推断 / U 未测量。

## 0. 结论

1. **函数级逻辑覆盖已经很高**：渲染范围 445 个去泛型单元里 426 个（96%，按机器码体积 99%）已在源码里引用原地址。
   但“地址被引用”≠ 实现正确，更不等于可运行。
2. **真正没做完的是四件事**：整体编译与接线；底层边界仍是 stub（其中一部分现在可以换成已恢复的真实库）；
   少量机器码没吃透的 U 项；渲染依赖、但位于渲染目录之外的模块。
3. **整个 crate 的编译状态没有测量到（U）**：`dolly` 离线缺失，网络被代理拦截（403）；在副本里去掉 `dolly` 做实测时，
   `/tmp` 只剩 1.4GB，一次拷贝就写满磁盘，已中止并清理。所有“通过”都来自隔离测试台（785 项）。
4. **有一处重复劳动需要先处理**：共享基础库 `utils` 现在已有 `Aabb3`、`Rectangle2d`、`NameHash`、`float_grid`、
   `IndexCondenser`、`Transform`、`geometry`，而 country-core 里仍保留自己的一套桩和恢复版。

## 1. 依赖现状（对照原版 `tracking/original-Cargo.lock`）

### 1.1 分类

| 类别 | 包 | 在原 lock 中的身份 | 本仓库现状 |
|---|---|---|---|
| 作者本地 crate | `frozen`、`toolbar`、`shader-pipeline`、`shader-variants`、`shader-compiler` | 版本 0.1.0，无 `source` | 渲染侧只有占位；`frozen` 依赖 `stable_deref_trait`，PDB 骨架里函数体“未恢复” |
| 作者本地 crate（已恢复） | `utils`、`asset`、`paths`、`lang-lib`、`theme-defs`、`tabloid` | 0.1.0，无 `source` | 在 `reconstruction/shared-foundation/crates/` |
| 作者本地 crate（已有重建） | `rhapsody` | 0.1.0 | `reconstruction/rhapsody-rebuild`（约 2 万行），country-core 的 `rhapsody_contract` **没有接到它** |
| 作者 fork | `vk-sync-fork` 0.4.0、`hotwatch` 0.4.6（git `h3r2tic/hotwatch`） | fork | 渲染侧只有占位 |
| 第三方，离线缓存缺失 | `puffin 0.14.1`、`notify 4.0.17`、`nohash-hasher 0.2.0`、`enum-iterator 1.2.0`、`dolly 0.6.0`、`mint 0.5.9` | crates.io | 只能 stub / 无法整体编译 |
| 第三方，已直接使用 | `bevy_ecs 0.16.0`、`glam 0.29.3`、`ash 0.38.0`、`slotmap 1.0.6`、`hashbrown`、`parking_lot 0.12.1`、`xxhash-rust 0.8.10`、`log`、`uuid`、`bumpalo` | crates.io | 直接 `use`，未重造 |

### 1.2 版本偏差（环境所迫，需在有原版缓存的机器上收回）

| 依赖 | country-core 当前 | 原 lock | 说明 |
|---|---|---|---|
| `half` | 2.7.1 | 2.4.1 | 离线缓存只有 2.7.1；共享基础库 lock 用 2.4.1 |
| `anyhow` | 1.0.103 | 1.0.86 | 同上 |
| `bytemuck` | 1.21.0（无 derive） | 1.16.0 | 缓存里没有 `bytemuck_derive` |
| `itertools` | 0.13.0 | 共享 `utils` 用 0.14.0 | 需要核对原版 |
| `glam` | 0.29.3，带 `mint` feature | 0.29.3 | `mint` 离线缺失，整个 crate 因此无法解析依赖 |

### 1.3 `dolly` 的影响范围（O）

`dolly` 只被 `src/systems/camera_rig/{walk,mod,butterfly,free}.rs` 使用，**不在渲染代码里**。它阻塞的是整个 crate 的编译，不是渲染本身。

## 2. 实测：原函数地址覆盖率

按组件（`coverage.py`，去泛型归并后的单元数；“已引用”= 该单元任一实例地址出现在 `country-core/src` 里）：

| 组件 | 单元 | 已引用 | KiB | 已引用 KiB |
|---|---|---|---|---|
| A1 mesh/vao/杂项 | 41 | 36 | 18 | 13（71%） |
| A1 shader_uri/shader 程序 | 8 | 8 | 6 | 6 |
| A1 ssbo/buffer 基元 | 10 | 9 | 8 | 8 |
| A1 uniform/frame/view 常量 | 3 | 3 | 3 | 3 |
| A2 mesh_atlas | 1 | 1 | 4 | 4 |
| A2 sparse_instance_buffer | 11 | 10 | 10 | 10 |
| A3 资产库 | 82 | 74 | 56 | 54（97%） |
| A4 pass_builder | 65 | 65 | 55 | 55 |
| A4 帧 pass 辅助 | 4 | 4 | 3 | 3 |
| C compute 帧 pass | 67 | 65 | 187 | 187 |
| D draw_passes | 61 | 60 | 120 | 119 |
| H 帧编排 | 3 | 3 | 20 | 20 |
| N nani | 68 | 67 | 103 | 103 |
| S 风格化/LUT/菜单背景 | 21 | 21 | 48 | 48 |
| **合计** | **445** | **426（96%）** | **642** | **634（99%）** |

源码中共引用 1911 个不同的原地址。

### 2.1 未引用的 19 个单元（逐个）

| 大小 B | 地址 | 单元 | 实例数 | 判断 |
|---|---|---|---|---|
| 1776 | 0x140555a90 | `asset_libraries::impl$21::register_required_components` | 1 | **真实逻辑，需补**（bevy Resource/组件注册钩子，I） |
| 1598 | 0x14003ae40 | `render_config` 反序列化 `visit_str` | 3 | serde derive 生成，无独立逻辑（I）；字符串枚举名可核对 |
| 1538 | 0x140039c10 | `render_config` `serialize` | 7 | serde derive |
| 944 | 0x1404d7d50 | `render_config::impl$38::clone` | 1 | 多半是 derive(Clone)（I），待核对 |
| 668 | 0x140322a10 | `atmosphere` `serialize` | 1 | serde derive |
| 520 | 0x140968c40 | `render_config::impl$7::from` | 1 | **U**：可能是 `From` 转换，需反汇编 |
| 235 | 0x1409bdc60 | `build_draw_lists::impl$1::fmt` | 1 | Debug/Display，低优先 |
| 94 / 56 | 0x1412e9e10 / 0x1412ea160 | `asset_libraries::impl$11/12::fmt` | 各 1 | fmt，低优先 |
| 86 | 0x1402b15c0 | `ssbo::impl$22::fmt` | 15 | fmt |
| 56 | 0x140a241d0 | `nani::gpu_world::impl$44::fmt` | 1 | n_world 已记录“机器码与 PDB 归属不符”（U） |
| 46 / 46 | 0x140a5de00 / 0x140a5d960 | `dof::impl$11/$7::source_hash` | 各 1 | **需补**（dof 着色器源哈希，c1 漏标） |
| 11×5 | `as_any` / `as_any_mut` / `handle` 若干 | `ssbo_library`、`sparse_instance_buffer` | 4–34 | 平凡 getter，已在 `ssbo_library.rs` 里以其他形式实现，只是没标地址 |

## 3. 实测：源码里残留的未实现标记

### 3.1 渲染目录内（O）

- `todo!/unimplemented!`：仅 `resources/grading_lut.rs` 7 处（行 34、58、64、69、74、80、98）。
- `Gap` 类型占位（非注释）：`resources/render_ssbo.rs` 5、`compute_water.rs` 3、`indirect_params.rs` 2、`leaf_particles.rs` 2、
  `compute_garden.rs` 2、`apply_screen_shadows.rs` 2、`render/nani/systems.rs` 2，以及 `stairs`、`spawn_boats`、`roof_tiles_gravity`、
  `read_bush_positions_for_cpu`、`rasterize_terrain_stroke`、`mouse_read_terrain_stroke_index`、`grass`、`nani/mod.rs` 各 1。
- 含“stub/占位/placeholder”字样最多的文件：`render/ssbo.rs`（12）、`bin_lights.rs`（9）、`texture_library.rs`（7）、
  `session_selection/mod.rs`（6）、`restir_di.rs`（6）、`bind_resources.rs`（6）、`texture_watch.rs`（6）。

### 3.2 渲染依赖、但在渲染目录之外（O，`todo!/unimplemented!` 计数）

| 文件 | 数 | 与渲染的关系 |
|---|---|---|
| `startup/load_prefabs.rs` | 7 | 网格/预制体入库的生产者 |
| `resources/logo_animation.rs` | 7 | logo 叠加 pass 的输入 |
| `resources/grading_lut.rs` | 7 | 调色 LUT，`final_post` / `lut_gallery` 依赖 |
| `utils/dirty_tiles.rs` | 6 | 地形脏区，C 层若干 pass 依赖 |
| `utils/mod.rs` / `debug_viz.rs` / `geo.rs` | 4 / 4 / 3 | 工具函数（应改用共享 `utils`，见 §4） |
| `resources/terrain_height.rs` | 4 | `TerrainHeightsData::sample_at_world_xz` 等 |
| `startup/{mod,startup_walls,startup_session,startup_nani,load_mesh_atlases,clearing}.rs` | 4/3/3/3/3/3 | 着色器/网格图集入库、nani 初始化 |
| `systems/…/main_camera` | 6 | `MainCamera` 在 crate 里是 `Gap`，回调只能用 `*View` 顶替 |

全 crate 的 `todo!/unimplemented!` 共约 227 处（B 阶段提交前为 475），其余是 UI、地形、历史、音频等非渲染模块。

## 4. 现在可以替换的占位（依赖更新带来的机会）

### 4.1 country-core 的 `utils` → 共享 `utils`（优先级最高）

共享 `utils`（`reconstruction/shared-foundation/crates/utils`，约 1.3 万行）已包含：
`geometry::{aabb::Aabb3, rectangle::Rectangle2d, ray, circle, plane, intersect::Triangle, …}`、`name_hash::NameHash`、
`float_grid`（含 `bilinear_sample_at_world_xz`、`FloatGrid`）、`index_condenser::IndexCondenser`、`components::transform::Transform`、
`rdp`、`swizzlers`、`comparative_space`、`curve`、`camera`。

country-core 中与之重复的位置：

- `lib.rs::utils_contract`（约 376 行起）：`Aabb3`、`Rectangle2d`、`Triangle`、`load_json_as_mesh`。
- `src/name_hash.rs`（本轮新增，`xxh3_64`）。
- `src/utils/float_grid.rs`（x_out 恢复，含 `CENTERED_PATH`、`(130,130)` 世界范围参数）。
- `src/utils/index_condenser.rs`（n_world 恢复）。
- `src/utils/components/transform.rs`（i_infra 按机器码修正了布局）。
- `src/utils/` 其余 1476 行桩（`dirty_tiles`、`debug_viz`、`geo`、`load_json` 等）。

必须先做的核对：两边行为是否一致（尤其 `float_grid` 的双路径与 `CENTERED_PATH`、`Transform` 的 `{rotation@0, translation@0x10, scale@0x1c}`
布局、`IndexCondenser::remove`）。不一致时以机器码为准，再决定谁修。**共享 `utils` 里没有 `load_json_as_mesh`**（68 个调用方依赖它）。

### 4.2 其他可对接的已恢复产物

- `lang-lib`：A1 层有 3 处对 `lang_lib` 的调用边。
- `asset` / `asset-pipe`：纹理交换格式、prefab 继承、DDS/mip，可与 `texture_library`、`load_texture`、`TextureDesc::from(preset)` 对照。
- `human-readable-shaders`（415 个 HLSL）与 `shader-source`：与 `shader_library`、`shader_uri` 的着色器键对照；
  C/D 各组用“重建 HLSL 字典”还原了约 95% 的绑定名，可固化成数据表。
- `rhapsody-rebuild`：评估是否可以替代 `rhapsody_contract`（见 §5.1）。

## 5. 未实现项清单（按影响排序）

### 5.1 接线与边界（决定能不能跑）

| # | 项 | 状态 | 出处 |
|---|---|---|---|
| 1 | 整个 crate 编译 | **U**：`dolly`、glam `mint` 缺失；仅有隔离测试台 | B_SUMMARY |
| 2 | `render_world`/`render` 接真实 pass 入口 | 未做。D/C 的 pass 是 `(PassEnv/DrawPassCtx, 显式输入)`，原版是 `(rg, world, handles) -> handles` | B_s_h |
| 3 | 水/冰的 `SurfaceCompute` 实现 | 未做，需由集成者提供并从 `render_world` 接线 | B_d_water |
| 4 | `PassResources::new_from(&World)` | 按原 128 位 TypeId 查资源是 typed gap，**调用必然 panic**。`typeid_resolve.py` 已能反查真名，可闭合 | B_a4_compute、B_d_cb_b |
| 5 | `PassHost` 缺 World 访问器 | `bind_brick_erase_highlight` 等 4 个资源值只能由调用方传入 | B_i_nani |
| 6 | `rhapsody_contract`（lib.rs:82–330，约 250 行） | stub；`rhapsody-rebuild` 未对接 | lib.rs |
| 7 | `shader_pipeline_contract`、`shader_variants_contract`、`puffin_contract`、`ustr_contract` | stub | lib.rs |
| 8 | `rg_boundary`（rg_helpers.rs）、`rhapsody_boundary`（ssbo.rs）、`shader.rs`/`load_texture.rs` 的 `boundary` | stub | 各文件 |
| 9 | `frozen_map.rs`（`RwLock<HashMap>` 占位） | `frozen` 是本地 crate，非外部库 | B_A3 |
| 10 | `reused.rs` 的 hotwatch/notify/nohash 替身；`hotwatch_contract` 与 `reused::watch` 两份未统一 | stub | A3 遗留 |

### 5.2 渲染逻辑上的 U 项（机器码没吃透）

| 项 | 说明 | 出处 |
|---|---|---|
| `shader_refactoring` 状态机 disc 6..=9 | 暂当 no-op | B_c3 |
| `reset_indirect_args` 的 0x1e0B 暂存区 | 机器码里无任何写入 | B_c3 |
| `CENTERED_PATH`（`*0x14362c308`） | 写入者没找到，默认 false（对应 .bss 初值 0） | B_x_out |
| 3 个只有 TypeId 的组件 | `0x66476a13`（BackgroundTerrain?）、`0xa6c0fc01`（DontReceiveContactShadows?）、`0x192600af`（OverwriteRoughness?） | B_d_cb_a |
| 水/冰计数缓冲初值 `[0,1,1,0]` | 观测到，含义是推断 | B_d_water |
| 全局 `*0x14362c0d8`、`*0x14362c098` | 含义未知 | B_s_h |
| `roof_tiles_gravity` 的 f32 uniform 键 `0x601a45ce43af912f` | 名字未还原 | B_c2 |
| `compute_path_masks` 的资源 E、grass 的主题资源类型名 | 未知 | B_c2 |
| `Flower03Ssbo` 的 NAME | 未能反推 | B_c2 |
| `ui_2d` 的 4 个组件字段真名 | 组件名取自 PDB（`CustomizationIcon`、`UiInteractionState`、`UiDrawOrder`、`UiInactive`），字段真名未知 | B_d_gap |

### 5.3 类型缺失或仅有视图替身

- `MainCamera`、`ActiveSession`、`Weather`、`EditingHighlight`、`DebugGui`、`LogoAnimation`、`RoofFlagCloth`、`PrevTransform`、
  `RockyTerrainMeshVertCount`：在 crate 里是 `Gap` 或无 `Resource` 派生，`draw_callbacks_a/b` 用只含被读字段的 `*View` 顶替，偏移来自机器码。
- `Duck`、`DuckState`、`CritterControl`、`EraserSizeMultiplier`、`RoofHighlight`、`InstancedDrawFromPlacementData`：crate 里不存在，按 PDB 名放了 stub。
- `SparkleSsbo` 是 `Gap`；`Flower03Ssbo`、`GardenStoneSsbo` 仍是载体。
- `nani_draw_world` 用一次性 `RenderGraph::default()`，因为 `RenderGraph` 不是 `Resource`。
- `Transform` 的 `look_at`/`abs_diff_eq` 仍是 `todo!`（i_infra 范围限定未改）。

### 5.4 公共 API 缺口

- `rhapsody_types::TextureDesc` 缺 `sampled_with_different_format`、`color_attachment` 字段（水/冰目前只能设 `sampled`）。
- `TypedStorageBuffer::new_storage_buffer` 没建模 `BUFFER_LEN>0` 的固定大小形态（`pass_kit::fixed_len_typed_storage` 绕过）。
- `MappedStorageBuffer` 的对齐参数与 `MouseDepthValue` 机器码不符。
- `TemporalTexturePair.fresh` 的命名是推断。
- `ComputeShaderSource.defs` 不能装 `Vec<(String,String)>`，PERM define 只能经 `pipeline_defines()` 传。
- `TextureDesc::from(preset)`（0xa59890，1933B，`resources/texture_presets.rs`）仍没恢复。

### 5.5 去重与清理

- `draw_callbacks_a/b` 重复的 `nani_draw_world`、`LibView`、`res`/`comp`、`cull_state`；同名 `*View` 字段不是同一子集，需取并集
  （`LogoAnimation` 一边读 +0、一边读 +8；`Weather` 一边读 +0xc、一边读 +4）。
- `water_planes_draw` 闭包入口与 `*Tag` 类型别名待删；`draw_callbacks_b` 应改调 `water_planes_draw_rt`。
- `draw_lists.rs` 里“其余是 typed gap”的措辞已过时；`d_main_support::raster_pass` 对 tag2 的补丁可删。
- `PrefabName`/`PrefabNameRef`、`ComputeShaderSourceHash` 的重复定义（x_out 已统一前者，后者待查）。
- 超 800 行文件（多为内联测试）：`render/nani/systems.rs`（3089）、`draw_builder.rs`（1973）、`compute_pass_builder.rs`（1512）、
  `vermeer.rs`（1294）、`render_world.rs`（1293）、`session_selection/mod.rs`（1169）、`painterly.rs`（1128）、`gpu_world.rs`（1116）、
  `draw_callbacks_a.rs`（1112）、`draw_stream.rs`（1027）等。

### 5.6 渲染范围之外、但渲染必需

`startup` 的网格/着色器入库（`load_mesh_into_library`、`load_shader_into_library`、`load_prefabs`、`load_mesh_atlases`、`startup_nani`）、
`grading_lut` 加载、`TerrainHeightsData::sample_at_world_xz`、`TerrainRockMask::sample_at_world_xz`（现返回 false）、
`TerrainHeightFloatGrid` 的 `FloatGridCells` 实现、`resources::walls`（A4 引用 2 处）、`utils::load_json_as_mesh`。
没有这些，资产库是空的，pass 没有输入。

## 6. 验证状态与可信度限制

- 统一测试台 `tools/country-core-audit/l0-harness`：**785 通过 / 0 失败 / 0 ignore**（O）。测试台只挂载渲染相关文件，
  个别测试台里 nani 是同签名替身；`render_world`、`render` 与真实 pass 入口之间没有联测。
- 可信度 = 机器码对照 + 隔离测试。**没有 GPU 或原版行为对照**。
- 手写轮子：唯一已知的是 `binding_hash`（`const fn` FNV-1a-64，原因是 `fnv` crate 的哈希器不能用于常量表达式；
  测试里用真实 `fnv` 交叉校验）。其余哈希用真实 `xxhash-rust`。我没有做全量轮子审计，只抽查了 FNV 和 cast。
- 地址覆盖统计的局限：只数“地址是否出现在源码里”；ICF 合并的函数、没写地址的实现会被低估或误判。

## 7. 建议的推进顺序

1. **切到共享 `utils`**（§4.1）：先写对比测试，裁决 `float_grid`/`Transform`/`IndexCondenser` 差异，再删重复。成本最低、收益最大。
2. **补小缺口**：`register_required_components`、`dof` 两个 `source_hash`、`render_config::impl$7::from`、`grading_lut` 的 7 处。
3. **闭合 `new_from(&World)`**：用 `typeid_resolve.py` 的 22+33 个真名建立 TypeId→类型表，去掉必然 panic 的路径。
4. **评估 `rhapsody_contract` ↔ `rhapsody-rebuild`**：先列出游戏侧实际调用面，再决定适配层。
5. **恢复 `TextureDesc::from(preset)` 与 `startup` 入库**，让资产库有内容。
6. **整体编译**：在有网络或 `dolly`/`mint` 缓存的机器上做第一次，修类型问题，再做 `render_world` 接线与 `SurfaceCompute`。
7. 去重与拆文件（§5.5）。

## 8. 本轮没有做的事

没有改任何源码；没有恢复新函数；没有提交（本文和 `coverage.py` 是工作区里的新文件）。

## 9. 数据来源与复现

- 地址覆盖：`python3 -I tools/country-core-audit/coverage.py [--list]`（依赖 `calls.py` 先生成 `data/scope_calls.json`，
  以及 `data/procs.tsv`；未引用单元写入 `/tmp/uncovered_units.json`）。
- 残留标记：`grep -rn "todo!\|unimplemented!"`、`grep -rnE "\bGap\b"`（排除注释行）作用于 `render`、`systems/render_loop`、
  `asset_libraries`、`resources/render*`、`rhapsody_types.rs`。
- 依赖身份：`tracking/original-Cargo.lock`（有无 `source` 字段区分本地 crate 与 crates.io）、`FOUNDATION_BUSINESS_FUNCTIONS.md`、
  `SHARED_FOUNDATION_README.md`、`reconstruction/shared-foundation/`。
- 各项 U 与 stub 的逐条来源：`evidence/country-core-audit/B_*.md`（共 20 份）。

## 10. U 项闭合记录（2026-10-15 round3，逐条证据见 `evidence/country-core-deep/round3/RENDER-U-CLOSURE.md`）

§5.2 / §5.4 各项状态（本文为分析快照，闭合以 round3 文档为准）：

| §5.2/§5.4 项 | 状态 | 一句话结论 |
|---|---|---|
| 全局 `*0x14362c0d8` / `*0x14362c098` | **O 闭合** | `freeze_occlusion_culling` 调试开关（do_gui_debug 按钮绑定；门控 wall draw list pass2+gpu_generate_draw_lists）与进程级 demo 开关（main 写 0；门控 refraction_copy+refraction source+water/ice surface 链）；两名字分别经 uniform 哈希 `0x6d7707561ec0da2b` 与全游戏读取语义证实 |
| CENTERED_PATH `*0x14362c308` | **O 闭合** | **写入者 = load_game 0x14146dc92**（`setge [rax]`，值=配置字段+0xc0 >= 0x1a，与楼梯开关 0x14362c011 同 cmp 共写）——render-gap"无写入者、恒向量路径"结论反转 |
| 3 个只有 TypeId 的组件 | **O 闭合** | `0x66476a13…`=BackgroundTerrain、`0xa6c0fc01…`=DontReceiveContactShadows、`0x192600af…`=OverwriteRoughness（bundle component_ids 处理序 + rrc call 配对，128 位全对） |
| shader_refactoring disc 6..=9 | **O 闭合** | 6/7 = 第二表 idx2/idx3-4 同一转移码（第一表入口 UB），5/8/9 = 指令中部死条目；不可达结论维持，reconstruction 注释应从 no-op 改"死变体" |
| reset_indirect_args 0x1e0 暂存区 | **O 闭合** | `memcpy(staging, 0x142aaaa78, 0x1e0)`（24×20B DrawIndexedIndirectCommand index_count 初值表）——"无写入"判据不成立（漏 memcpy 调用），真实初始化 |
| roof_tiles_gravity uniform `0x601a45ce43af912f` | **O 闭合** | 名字 = **`delta_seconds`**（fnv1a 吻合）；资源 = `GameTime`（TypeId 53 处证实），值 = delta_f32@+0x28，mode!=8 才写 |
| TextureDesc::from(preset) 0xa59890 | **O 闭合** | 0..0x18 全映射表 O（尺寸/格式/用法链/key）；key = xxh3(预设名)，11/25 名字还原（brdf_lut/terrain_lighting/ShadowMap/water_mask/garden_mask/rocky_terrain/…）；0x19..0x1f 死变体 |
| Flower03Ssbo NAME | **U 卡点** | SsboId `0xF78C9C2EB9E1F6FB` O；NAME 穷举（全二进制串/SPIR-V/HLSL/花名组合）无命中；同态 U：IvyFlowerSsbo/LilypadFlowerSsbo |
| compute_path_masks 资源 E | **O 闭合** | = `Events<RecalculateHeightMapCmd>`（全 EXE 9 处命中）；"非空" = events_a/b 长度 |
| grass 主题资源 | **O 闭合** | = `systems::mode_manager::AppMode`（34 处命中）；`GladeThemeId` 占位作废 |
