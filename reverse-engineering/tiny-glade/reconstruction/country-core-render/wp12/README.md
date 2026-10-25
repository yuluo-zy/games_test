# WP12 — country_core::systems::wall（墙系统）恢复

Tiny Glade 原作逆向（版本 B；EXE `cf048e27…`，PDB `2d9686b8…`）的墙系统 ECS 恢复包。

## 交付

| 项 | 路径 | 内容 |
|---|---|---|
| 契约 | `contracts/` | 8 份业务逻辑契约（overview + 7 系统组；O/I/U + typed gap） |
| 骨架对齐源码 | `src/systems/wall/*.rs` | 57 个 skeleton 单元（模块路径/函数名/FunctionSystem 签名对齐 B PDB） |
| 门面 | `src/{resources,utils,ui,audio,components,render,systems}.rs` | 墙状态（WP11 属主）typed-gap 门面 + bevy_ecs 0.16 真实 API |
| 独立 crate | `Cargo.toml` + `src/lib.rs` | offline `cargo check` 0 错误；all-targets clippy 0 错误 |
| 证据 | `evidence.json`, `evidence/` | 反编译、地址映射、inventory、ghidra job/result |

## 覆盖（57 单元）

核心生命周期 13（calc_wall_bend / delete_wall / draw_wall / finish_wall / merge_walls /
set_wall_style / wall_splitter / wall_highlight / upload_instanced_wall_ssbo /
deferred_wall_history / structure_outline / ui_continue_wall / ui_draw_wall）、
adjust_wall_height 5、eraser 4、shapes 21、snapping 7、valid_enclosure 1、wall_state 6。

FunctionSystem 签名取自每个单元 `*.enriched.json.gz` 的 `file_interface_candidates`
（`native_direct_generic_caller`）；18 个代表系统经 Ghidra 只读反编译
（`evidence/decompiles/b1/`）落地业务契约。O 级路径：
- `calc_wall_bend` bend 计算（点>5 前 4 段 dot 平均 → acos → min(1.0)）
- `delete_wall` 级联 remove_entry + despawn
- `set_wall_style` 三读器/历史组/style 后 0x20B 事件
- `adjust_wall_height_impl` 高度夹取/follow-terrain/双事件分支
- `clamp_wall_height` 数值策略（复用 S02 `height_rules.rs`）
- `calc_shape_height` 形状预览高度（base 6/8、0.46 项、clamp[0.7,14]）
- `snapping_init`/`snapping_audio` 阈值 1.65/1.5/mode 分支
- `animate_wall_bulldoze`（复用 building/wall_bulldoze.rs 全链路）
- `deferred_wall_history`（复用 building/wall_pending_changed.rs）
- `finish_edit_rectangle`（复用 building/wall_rectangle_edit.rs）
- `gpu_flush_brick_buffers`（InstancedWall 脏段 → BrickTransformSsbo GPU 冲刷）

## 已知缺口（typed gap，不臆造）

- 墙状态类型布局（`InnerWalls`/`PublicWalls`/`WallState`/`PublicWallState`）属 **WP11**；
  本包只给访问器面 + Gap 字段。
- 各 UI widget/signifier/trigger-volume 精确参数（UI 层）。
- `WallSpatialHash` 段 u/terrain/normal metadata。
- 全部 `implemented_unverified`：无 runtime/GPU 验收；契约是当前可审阅依据。

## 集成说明

`src/systems/wall/*` 路径与 B skeleton 一一对应；集成 owner 并入
`reconstruction/country-core` 时，将 `crate::resources::walls::*` 门面替换为 WP11 实型，
`crate::ui::*`/`crate::utils::*` 替换为既有 country-core 模块即可。
