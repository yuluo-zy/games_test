# WP9 — 屋顶内容域（roof shape/simulation 资源 + 屋顶系统）

恢复 country-core **屋顶内容域**的 CPU 侧资源与系统（B 版：ljxsj_92385，EXE
`cf048e27…`，PDB `2d9686b8…`）。方法遵循项目流程：业务逻辑（Ghidra 反编译 + 调用链 +
PDB 骨架证据）→ 合同 → 骨架实现；差分仅用于关键数值/边界裁决。

## 交付

- `contracts/`
  - `roof-shape-sim.md` — Roof/RoofShapeParams/RoofRidgeDims/RoofBoundsDims/
    RoofTriggerVolume/RoofTileSimulation/RoofAnimation/roof_noise/roof_buffers 合同
    （producer/consumer、字段+偏移、side effects、失败分支、O/I/U）。
  - `roof-tiles.md` — RoofTiles 组件 + RoofShingle 64B 记录 + `roof_tiles_gravity_compute`
    （W19 消费）+ instanced_roof 三 draw 合同。
  - `roof-systems.md` — create/edit/delete/update_roof_spatial/wall_height_dt/
    simulate_roof_tile_for_audio/ui_edit_*/replay/roof_audio/startup 系统合同。
- `src/` — 骨架对齐源码（镜像 `reconstruction/country-core` 装配布局）：
  - `resources/roofs/{roof_shape, roof_tile_sim, roof_tiles, roof_animation, roof_ridge,
    roof_visual_shape, roof_trigger_volume, roof_noise, roof_buffers, dormer_tiles,
    commands}.rs`
  - `resources/{roof_tile_ssbo, color_id_roof, roof_shape_curve, history/roof_edit}.rs`
  - `systems/roof/{create_roof, edit_roof, edit_rules, delete_roof, update_roof_spatial,
    wall_height_dt, simulate_roof_tile_for_audio, ui_edit_roof(_ridge_dir,_ridge_length),
    replay_roof, deferred_record_roof_history, highlight_roof, roof_trigger_volume,
    utils/, visual/}.rs`
  - `systems/render_loop/{compute_passes/roof_tiles_gravity, draw_passes/draw_shaders/
    instanced_roof}.rs`
  - `systems/audio/roof_audio.rs`、`startup/startup_roof.rs`
- `evidence/` — 全部 Ghidra 反编译（decomp/*.c，49 个目标）、roof 域函数/VA 映射
  （roof-domain-dump.txt / rva-map.txt）、作业/结果 JSON、解析脚本。
- `evidence.json` — 地址表、skeleton bundle SHA、prior-work 复用裁决、构建验证。

## 构建验证

`reconstruction/country-core-render/wp9/Cargo.toml`（独立证据束 crate；依赖
bevy_ecs =0.16.0、glam =0.29.3，锁定原版基线）：

```
cargo check --offline   → 0 errors / 0 warnings
cargo clippy --offline  → 0 errors / 3 doc-indentation warnings（骨架风格）
```

全部标记 `implemented_unverified`：无 runtime/GPU 验收；合同是当前可审阅依据。

## 复用裁定（prior work）

`reconstruction/roof/{surface,tiles,roof_state,parameters,edit_rules}.rs` 已机器码验证的
数值核心**复用**（本次反编译逐函数确认 VA/公式一致，未重推导）；wp5 W19 合同复用；
WP6 `resources/roofs/{roof_animation,roof_shape}.rs` 复用并扩展。

## 主要恢复成果

- **Roof 88B 组件布局**（`Roof::new` 0x1408e31c0 反编译确认）与全部形状方法
  （ty/height/pivot_3d/is_flat/is_gable/gable_sides/shape_dims/roof_tip_offset_xz/
  rotation_yaw_radians/profile_curve_normalized/get_roof_shape/…），数值与 surface.rs 一致。
- **RoofTileSimulation**（16B，`new` 0x140939390：off4=入参，off0/off8=0）+ **RoofTiles**
  组件（8 行源，含 `Option<SparseInstanceBufferAllocation<RoofTileSsbo>>` 消费证据）。
- **RoofShingle 64B** 记录（与 evidence/roof-tiles + shader `roof_tile.inc.hlsl` 一致）。
- **roof_tiles_gravity_compute**（0x140a29b10）：Query + flat_map → PushConstants →
  `read_storage_buffer<RoofTileSsbo>` + `uniform<f32>` + `dispatch_with`；RoofPushConstants
  20B（first_instance/instance_count/wall_height_delta/roof_pivot_y/animation_t）。
- **instanced_roof 三 draw**：nani（读 RoofTileSsbo/RoofColorIdSsbo/DormerHole*）、spire
  （RoofSpireSsbo）、support_beam（RoofSupportBeamSsbo）。
- **create_roof**（0x1421aaea0）：屋顶 spawn bundle 完整集合
  （Roof+RoofTileSimulation+RoofTriggerVolume+RoofAnimation+RoofTiles+RoofSpire+
  RoofSupportBeams+NaniBundle<StoneFloorInstanceData>）。
- **edit_roof**（0x1421ab7c0）：64B RoofEditEvent + apply_edit 数值核心（prior 复用）。

## 剩余缺口（typed gaps，不臆造）

- RoofTiles 具体字段形态与 `#[require]` 成员。
- RoofRidgeDims 内部字段/to_world_space 矩阵闭包、RoofTriggerVolume 集合容器细节、
  gpunoise 哈希正文、RoofSpireSsbo/RoofSupportBeamSsbo/WoodenPlankSsbo 布局。
- 屋顶枚举判别编号（RoofType/GableRoofStyle/FlatRoofStyle/FlatRoofRimStyle/FlatRoofMaterial）。
- RoofTileSimulation off0/off8 语义、roof_uv 碰撞链（rapier3d）。
- 全部 system 的 World/组件图深层遍历与关联表结构、RoofEdit/RoofReplayCmd 历史变体字段、
  UI 类型布局（RoofParamsCacheResource/DoUiEditRoof/RidgeGrabParams 等）。

## 集成状态

本束为证据包；`reconstruction/country-core` 的完整装配（wp1–wp6 + wp9）由集成 owner 组装，
本束 `src/` 镜像其布局并提供 `consume::pass_builder` / `render::*` 装配占位。
