# reconstruction → 正式工程逐文件移植清单

- 日期：2026-10-24
- 入口：[逆向域索引](../README.md) · [正式工程](../../../tiny-glade/README.md)
- 源：`reverse-engineering/tiny-glade/reconstruction/`
- 目标：`game/tiny-glade/`
- 方法：文件路径 + `fn/struct/enum/trait/type/const` 符号比对，再人工复核代码体、依赖与正式工程对应实现。
- 范围：只列“reconstruction 有、正式工程无同名实现”的业务文件；探针、测试、工作包契约另列。

## 结论

**真正需要移植的实代码很少。** 18 个候选中：

| 状态 | 数量 | 说明 |
|---|---:|---|
| P1 部分合并，需补函数/类型 | 3 | 键位分配、墙附着类型、uniform 辅助函数 |
| P2 仅骨架/`todo!`，无逻辑可移 | 10 | 6 个 UI、`utils_intersect`、`grow_grass`、`debug_info_buffers`、`blur_exclusion_mask` |
| P3 已被正式工程通用实现覆盖 | 3 | `replay_garden/path/water` |
| P4 架构不同，不建议移植 | 2 | raw managed framebuffer/render_pass |

可直接落地的只有 **3 个文件、约 10.5 KB**，且都是函数/类型级补充，不是整模块移植。

## 状态定义

- **P0**：真实实现，正式工程完全缺失，可原样或小改移植。
- **P1**：正式工程已有同一职责的实现，只缺部分函数/类型。
- **P2**：reconstruction 里也只是骨架或 `todo!()`，没有可移植逻辑。
- **P3**：正式工程已用更通用的实现覆盖。
- **P4**：正式工程架构已重构，原文件不应按原样移植。
- **V**：验证/探针/测试资产，不进入生产代码。

## 总表

| # | reconstruction 源文件 | 大小 | 状态 | 正式工程对应 | 动作 |
|---:|---|---:|---|---|---|
| 1 | `country-core/src/game_key_bindings.rs` | 5.6 KB / 144L | P1 | `crates/country-core/src/game_profile.rs::GameKeyBindings` | 只补 `assign_key` / `assign_mouse_button` / `get_mouse_button` 等缺失函数 |
| 2 | `country-core/src/resources/walls/wall_attachment.rs` | 3.0 KB / 77L | P1 | `crates/country-core/src/resources/walls/decorator/anchors.rs`（只有 Observed 版） | 补 `WallCornerAttachment` / `WallAttachmentSide` 规范类型 |
| 3 | `country-core/src/render/uniform.rs` | 1.9 KB / 45L | P1 | `crates/country-core/src/render/pass_builder/uniform_binder.rs` | 补 `bind_lazy_uniform` / `write_uniform_to_buffer*` 辅助函数 |
| 4 | `country-core/src/systems/history/replay_garden.rs` | 2.6 KB / 66L | P3 | `crates/country-core/src/systems/history/replay_raster.rs::replay_rasters` | 不移植；`RasterDomain::Garden` 已覆盖 |
| 5 | `country-core/src/systems/history/replay_path.rs` | 4.5 KB / 119L | P3 | 同上 | 不移植；`RasterDomain::Path/Cobblestone1-3` 已覆盖 |
| 6 | `country-core/src/systems/history/replay_water.rs` | 2.6 KB / 66L | P3 | 同上 | 不移植；`RasterDomain::Water` 已覆盖 |
| 7 | `country-core/src/systems/ui_move_shapes.rs` | 1.2 KB / 41L | P2 | 无 | 仅 `todo!()` 骨架；需先取证再实现 |
| 8 | `country-core/src/systems/ui_raise_shape.rs` | 1.6 KB / 50L | P2 | 无 | 同上 |
| 9 | `country-core/src/systems/ui_edit_freehand_walls.rs` | 0.9 KB / 29L | P2 | 无 | 同上 |
| 10 | `country-core/src/systems/ui_move_freehand_walls.rs` | 0.8 KB / 29L | P2 | 无 | 同上 |
| 11 | `country-core/src/systems/ui_focus_mode_shape_height.rs` | 1.0 KB / 31L | P2 | 无 | 同上 |
| 12 | `country-core/src/systems/ui_camera_mode_gizmos.rs` | 1.2 KB / 30L | P2 | 无 | 同上 |
| 13 | `country-core/src/systems/utils_intersect.rs` | 1.3 KB / 37L | P2 | 无 | `line_x_shape_overlap` 仍是 `todo!()` |
| 14 | `country-core/src/systems/backend/grow_grass.rs` | 1.1 KB / 34L | P2 | `crates/country-core/src/startup/startup_grass.rs` | 只有计时器骨架，无生长逻辑 |
| 15 | `country-core/src/render/debug_info_buffers.rs` | 0.5 KB / 11L | P2 | 无 | 只有 `DebugInfoBuffers` typed gap |
| 16 | `country-core-render/wp5/.../compute_path_masks.rs` | 0.8 KB / 21L | P2 | `crates/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs` | `blur_exclusion_mask` 仍是 `unimplemented!()` |
| 17 | `rhapsody-rebuild/src/raw/managed/framebuffer.rs` | 0.8 KB / 22L | P4 | `crates/rhapsody/src/rg/pass_api.rs`、`rg/attachments.rs` | 正式工程无 raw Framebuffer 类型，改用 render graph 附件 |
| 18 | `rhapsody-rebuild/src/raw/managed/render_pass.rs` | 0.7 KB / 18L | P4 | 同上 | 同上 |

## 详细移植项

### P1-1 键位分配（`game_key_bindings.rs`）

正式工程已有：

- `game_profile.rs::GameKeyBindings`：37 键位 + 3 鼠标位、默认值、序列化字段名、旧档迁移。
- 已有 `keycode_to_str` / `action_desc` 的消费或等价实现。

仍缺：

| 符号 | 原地址 | 说明 |
|---|---|---|
| `KeyBindings::assign_key` | `0x14089b2a0` | 冲突/交换语义只部分恢复，不能声称完整 |
| `KeyBindings::assign_mouse_button` | `0x14089b280` | 写 `u16` 键/鼠标槽 |
| `MouseBinding::get_mouse_button` | `0x14089c880` | 读取鼠标绑定 |
| `KeyBindingDesc` / `MouseBinding` | 命名实体 | 正式工程用不同结构，按 UI 需要决定是否引入 |

建议：**扩展现有 `game_profile.rs`**，不要新建平行 `game_key_bindings` 权威类型。先补 `assign_*` 的字节/冲突语义，再按需要补描述表。

### P1-2 墙附着类型（`wall_attachment.rs`）

正式工程已有：

- `resources/walls/decorator/anchors.rs::WallAttachmentAnchorObserved`
- `decorator_on_rectangle_edited.rs` 的 `rectangle_corner_id` 等价助手。
- 源码注释明确：`WallCornerAttachment` 类型在 core 未实现。

建议：在 `resources/walls/decorator/anchors.rs` 或新 `resources/walls/wall_attachment.rs` 中补 `WallCornerAttachment` / `WallAttachmentSide` / `WallAttachmentAnchor`，并让现有 `rectangle_corner_id` / `WallAttachmentAnchorObserved` 复用同一编码；不要保留两套附着权威。

### P1-3 uniform 辅助函数（`uniform.rs`）

正式工程已有：`pass_builder/uniform_binder.rs` 的 `UniformBinder`、`UniformBytes`、`UniformReflection`。

仍缺：

- `bind_lazy_uniform`
- `write_uniform_to_buffer`
- `write_uniform_to_buffer_aligned`

建议：新增 `crates/country-core/src/render/uniform.rs`，`render/mod.rs` 注册；直接复用现有 `UniformBinder`。这是风险最低的一项。

## P3：回放文件已被通用实现覆盖

正式工程 `systems/history/replay_raster.rs::replay_rasters` 已按 `RasterDomain` 处理全部六个域：

```text
Path / Cobblestone1 / Cobblestone2 / Cobblestone3 / Water / Garden
```

并在 `systems/history.rs` 注册。reconstruction 的 `replay_garden/path/water` 是更早的按域拆分版本，**不要移植**，否则会形成第二套回放路径。需要的 API 对应：

| reconstruction | 正式工程 |
|---|---|
| `ReplayCmd` | `ReplayCommand` |
| `RasterCmdFilter` / `RasterStateBoundary` | `RasterReplayCmd` / `RasterDomain` / `RasterReplayParams` |
| `replay_raster(...)` | `replay_rasters::<S>(...)` |

## P2：只有骨架，不是可移植内容

以下文件包含 `todo!()`、`unimplemented!()` 或只有 typed gap；移植不会带来行为：

- `ui_move_shapes.rs`、`ui_raise_shape.rs`、`ui_edit_freehand_walls.rs`、`ui_move_freehand_walls.rs`、`ui_focus_mode_shape_height.rs`、`ui_camera_mode_gizmos.rs`
- `utils_intersect.rs`
- `grow_grass.rs`
- `debug_info_buffers.rs`
- `compute_path_masks.rs` 的 `blur_exclusion_mask`

如果要补这些功能，应回到 PDB/反编译取证，而不是从 reconstruction 搬空壳。

## P4：rhapsody raw 封装

reconstruction 的 `raw/managed/framebuffer.rs`、`render_pass.rs` 依赖 `Framebuffer` / `RenderPass` raw 类型；正式工程已改为 `rg/pass_api.rs`、`rg/attachments.rs`、`rg/attachment_contract.rs` 的 render graph 模型，没有对应 raw 类型。

**不要按原样移植。** 如果确认缺少某个能力，应在 `rg` 层补，而不是恢复旧 raw 封装。

## V：验证资产（保留在 reconstruction，不移植）

| 类别 | 数量 |
|---|---:|
| `src/bin/*-probe.rs` | 43 |
| `standalone/`（tg-audio、tg-tree、tg-critters-water） | 35 |
| `rhapsody-rebuild/tests/` | 46 |
| `country-core-render/wp1…wp21` 工作包契约/证据 | 21 包 |
| `shader-source` 候选 HLSL | 600（最终 415 与正式工程逐字节一致，185 个候选/变体） |

这些用于差分与复现，不是生产代码。

## 已确认已合并，不要重复移植

- startup 全家（`clearing`、`load_audio`、`startup_*` 等）
- `configwatch` / `meshwatch` / `texture_watch` → `asset_libraries/`
- `panic_reporter_startup` → `crates/tiny-glade/src/panic_reporter/`
- `sheep_pet` → `crates/systems/critters/src/sheep/`
- `camera` normal/free/body → `systems/camera_rig/`
- `terrain readback` → `crates/tiny-glade/src/world_terrain.rs`
- `place_support_beams` → `systems/wall-constructor/.../construct_lintels/`
- `water_level_at_xz`、`clear_buffer`、`triangle_triangle`、`swizzlers`、`adjust_wall_height` → 正式工程已有
- 415 个最终 HLSL 与 `human-readable-shaders` 逐字节一致

## 执行顺序建议

1. `uniform.rs` 辅助函数（最低风险）。
2. `wall_attachment` 类型统一（先确认 decorator 消费者）。
3. `game_profile` 的 `assign_*`（先补冲突语义取证）。
4. 其余 P2 项按模块优先级重新取证，不从骨架移植。

每项完成后的验证：

```bash
cargo check -p country-core --offline
cargo test -p country-core --offline
cargo clippy -p country-core --all-targets --offline -- -D warnings
```

## 限制

- 本清单是**文件/符号级**比对；正式工程经过重构，某些功能可能以不同 API 存在，不能仅凭“无同名符号”断言缺失。
- reconstruction 中的 `todo!()` 文件不是“未合并内容”，而是尚未取证的空壳。
- `country-core-render/wp*` 的 720 个 `.rs` 多数是候选/证据；本次只确认了上述具体项，未逐函数证明其余候选全部被正式工程吸收。
