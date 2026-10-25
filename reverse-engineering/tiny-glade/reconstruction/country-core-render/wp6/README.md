# WP6 — nani ECS systems + consumed component/resource graph

恢复 `country_core::render::nani::systems.rs`（22 个系统，含 WP3 未列出的
`nani_flush_instance_updates`）+ 这些系统消费的组件/资源类型，并按 PDB 源码骨架
实现 Rust、装配进 `reconstruction/country-core`。

## 交付

- `contracts/systems.md` — 22 系统逐条业务逻辑（ECS in/out、GPU 世界效果、条件、失败分支、
  O/I/U）。
- `contracts/components.md` — drawable + 消费组件族契约。
- `contracts/resources.md` — 资源族契约（color_id/window_lights/walls/roofs/critters/
  stairs/world_raster/mesh_atlas_library/draw_path/transform）。
- `src/` — 骨架对齐源码（与 country-core 装配同步）：
  - `src/render/nani/systems.rs`（中心件：maintain + 20 update + flush）
  - `src/components/{mod,drawable}.rs`
  - `src/resources/*`（color_id, window_lights, critters, world_raster(_defs),
    render/mesh_atlas_library, walls/{mod,public_walls,plaster,curved_halftimber},
    roofs/{roof_shape,roof_animation}, stairs/visual）
  - `src/utils/components/transform.rs`（游戏自有 Transform，bevy 兼容布局）
  - `src/systems/paths/draw_path.rs`（PathRastersResources SystemParam）
- `evidence.json` — 反编译地址、skeleton bundle SHA、候选复用裁决、装配补丁。

## 集成状态（reconstruction/country-core）

- `src/render/nani/systems.rs` 已声明（`render/nani/mod.rs`）；`components`/`resources`
  子模块、`utils::components::transform`、`systems::paths` 已接线（`lib.rs`）。
- `cargo check --offline`：**0 错误**（141 警告，均为骨架 marker/未使用）。
- `cargo clippy --all-targets --offline`：**0 错误**。
- 装配补丁（记录于 evidence.json）：`InstanceDataMarker<D>`/`GpuWorld`/`Asset*Library`
  补 `Component`/`Resource`；`owned_subset_mut` 返回 `&RwLock<...>`；`CurvedHalfTimberInstanceData`
  补 `Component`；`BufferState` 补 `Send/Sync`。

## 方法（按项目流程）

1. 业务逻辑分析：反编译（wp3 evidence + 本轮 Ghidra）+ PDB 骨架（ecs-system-signatures、
   structures.json.gz、pdb.json、enriched.json）+ 调用链。
2. 整理：contracts/*（Observed/Inferred/Unknown + typed gap）。
3. 骨架实现：src/*，ECS 用真实 bevy_ecs 0.16 API，未知字段 → typed gap。
4. 差分仅作关键数值/边界裁决（如 Transform 布局经 compute_affine 反编译确认）。

## 本轮新发现

- **`nani_flush_instance_updates`（0x142080640，行 1011）**：GPU 冲刷系统（无 Query 的
  `ResMut<GpuWorld>` 系统，故不在 wp3 的 24 条 FunctionSystem 签名中）；30 槽脏区间遍历
  + `ShadowedBuffer<GpuInstance>::gpu_flush` + 帧计数搬移（+0x1440→+0x1448）。
- **Transform 布局 O**：`compute_affine` 反编译确认 `{translation@0x0, rotation@0xC,
  scale@0x1C}` = 40B（utils crate 游戏自有 util；数学复用 glam）。
- **preplaced_flower 水判定 O**：512×512 花位图 bit + 水位 `≤0.01` 双线性采样，否则隐藏。
- **stone_floor 屋顶切线 O**：动态组件位 + `Circle2d::get_tangent_at_u` 分支。
- **curved_halftimber cap 掩码 O**：16 段 u128 位提取直写实例数据。

## 剩余缺口（typed gaps，不臆造）

- 各 `nani_update_*` 的组件→`T` 字段映射（除 3 个 O 级路径）。
- 每系统 `GpuWorldSubsetKind` 常量（除 shader 家族确定的 4 个）。
- `NaniActiveInstance` 的 chunk_id 宽度（u32 vs u64）与 from/into key 打包。
- `nani_flush_instance_updates` 的 30 槽 vs 27 subset 布局。
- `PathRastersResources` SystemParam 内部形状。
- `NaniInstanceDesc` 精确尺寸（24B 候选 vs 16B 行 stride 证据）。

全部标记 `implemented_unverified`：无 runtime/GPU 验收；合同是当前可审阅依据。
