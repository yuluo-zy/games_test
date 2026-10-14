# R2：nani 实例载荷构建函数迁入 `tiny-glade` 工作区

日期：2026-10-14。标注：O 实测 / I 推断 / U 未测量。

## 背景
用户要求以根目录 `tiny-glade/` 工作区为基础继续还原，并把此前在旧树 `reverse-engineering/tiny-glade/reconstruction/country-core` 里
实现的内容迁过去。盘点（旧树 596 文件 vs `render-source-map.json` 181 条）发现，**渲染范围里唯一有实质体量、尚未迁入的是
`render/nani/systems.rs`**（3103 行）。source-map 把它整体排除，理由是依赖未恢复的游戏组件。

## 关键发现（O）
文件里每个 `nani_update_*` 系统都配有一个**纯函数 `build_*`**（输入是普通值或小型 `*View` 结构，输出 `InstanceUpdate<N>` 字载荷）。
只有系统包装层（Query/Res/Commands）依赖缺失的组件类型。所以不需要整份排除，可以按函数拆。

## 迁入（`crates/country-core/src/render/nani/instance_updates.rs`，约 1.8k 行，含 50 项测试）
- `build_*` 构建函数（plant、solid_vertex_color 系列、albedo_map、window_glass、bricks、wren、gothic_window_bricks、preplaced_flower、
  curved_halftimber、stone_floor、lantern_emissive、plaster、cloth、smoke、balcony_door、uber、velocity_only、stairs_floor）。
- 打包辅助：`InstanceUpdate<N>`、`trs_words`、`trs_words12`、`gb_affine_words`、`pack_2x16f_uint`、`compact_key_for` 等；常量与 `*View`/`*Inputs` 结构。
- 原文的 O/I/U 注释、地址、运算顺序保持不动。

## 没迁入（留在旧树，等组件恢复）
| 项 | 缺什么 |
|---|---|
| `nani_maintain_instances`、`nani_flush_instance_updates`、全部 `nani_update_*`、`NaniActiveInstanceCollection` | `NaniActiveInstance`、`NaniInstanceDesc`、`Wren`、`Plaster`、`StairsFloorMeshChunk`、`GpuWorld` 的 ECS 接线等组件/资源 |
| `resolve_mesh_part`、`resolve_changed_mesh_part`、`push_update`、`pack_instance`、`gap_words`、`kind_from_u8`、`warn_once` | 同上 / `Gap` 占位 |
| `terrain_rock_mask_words` | `TerrainRockMaskSsbo`（本工作区没有 `resources::terrain_rock_mask`） |
| `NaniPathRasters` | `world_raster_defs::{Path,Cobblestone1..3}Raster` |
| `animated_peel_lookup`、`albedo_name_hash`、`albedo_bindless_index`、`prev_transform_of` | `AnimatedPlasterPeel`、`AlbedoMap`、`PrevTransform` |
| `ecs_tests`、`core_tests`（旧树 ECS 级测试） | 上述系统与 `World` |

## 一处适配
`PlasterWallInputs::from_state(&PublicWallState)` → `from_bytes(&[u8])`：`PublicWallState` 不在本工作区，偏移/语义不变；
旧树的 `from_state` 只是对 `state.raw_bytes()` 的转发。

## 未做的去重（刻意）
同文件内的重复实现没有合并：`trs_words`/`trs_words12`/`gb_affine_words`（参数顺序不同）、`pack_2x16f_uint`/`gb_pack_2x16f`、
`max_abs_scale`/`gb_max_abs_scale`（`max` 链顺序不同）。它们各自对应机器码里不同的调用点，合并要逐点核对运算顺序，属于后续去重任务。
另：`rasterize_terrain_stroke.rs` 里还有第三份 `pack_2x16f_uint`，共享 `utils` 里没有。

## 验证（真实工作区）
先前这台机器解析不了工作区（git 依赖不在离线缓存），曾用 `/tmp` 垫片工作区验证过一轮（结果相同）。之后改用 USTC 的 crates.io sparse 镜像
（临时 `--config` 文件，全局 cargo 配置未改，`Cargo.lock` 未变）`cargo fetch --locked`，**在你的真实工作区里**验证：
- `cargo check -p country-core --all-targets`：通过。
- 库内测试：默认特性与 `native-physics` 均 **1210 通过 / 0 失败**（基线 1142，与 README 一致；+R1 18 +R2 50）。
- 严格 Clippy（`-D warnings`，`--no-deps`，全部目标）：默认与 `native-physics` 均通过；`cargo fmt -p country-core --check` 通过。
- 不带 `--no-deps` 的 Clippy 会失败于 `crates/tg-history/src/lib.rs:334`（Clippy 1.96 新增的 `unnecessary_map_or`，`Option::map_or(true, ..)`）。
  与本次改动无关；你的记录是在 Rust 1.88 上验证的，此处工具链为 1.96，未处理。
- 网络：阿里云镜像被本机代理拒绝（CONNECT 403）；USTC 镜像（走代理）和 rsproxy（直连）可用；清华同样被拒。

## 顺带发现
`render-source-map.json` 里有 5 个条目的 `destination_sha256` 与 HEAD 时的实际文件不一致（`render/indirect.rs`、`render/nani/gpu_world.rs`、
`utils/float_grid.rs`、`utils/index_condenser.rs`、`utils_contract.rs`），在我改动之前就已如此。我没有修改这些记录；
`verify_inventory.py` 默认不比对目标文件哈希，所以不会报错。建议你确认是有意为之还是漏同步。
