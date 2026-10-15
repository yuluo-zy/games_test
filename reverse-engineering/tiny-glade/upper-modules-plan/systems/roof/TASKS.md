# systems/roof 任务清单

## 元信息

| 项 | 值 |
|---|---|
| B 原路径 | `crates/systems/roof` |
| package | `system-roof` |
| 规模 | 26 文件 / 37 本 ns 函数 / 6 闭包 / **27 业务函数** |
| 难度 | 中 |
| 批次 | 1 |
| 上游依赖 | `country-core`（Roof 组件/权威状态）、`rhapsody`、`bracket-noise`、`glam`、`half`、`utils` |
| 复现工程现状 | MODULE_PROGRESS：**部分接线**（22 恢复文件引用）；CPU 铺瓦核心完整（矩形 33 输入/19024 记录、圆形 37 输入/11467 记录）；create/edit/delete/spatial/replay 与类型/动画已接 World；`reconstruction/roof/*` 已有 surface/ridge/pipeline/edit_rules |

## 已有基础（复用，不重复恢复）

- `Roof::new/ty/is_gable`（M02，256 构造 + 1348 输入）
- 屋顶曲面 / Curve2（M03，23810 用例）
- 脊线 / tip（M04，14776 用例）
- 矩形/圆形铺瓦 CPU（M05）
- `edit_rules.rs` 编辑后段 / 变化量 / 反馈（M06，4514 用例）
- core 侧：`recovered_building/roof_{components,creation,customization,deletion,direction,lifecycle,spatial,associations,invalid_notifications}.rs`

## 任务表（按 B src 层级）

### 生命周期主干

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-ROOF-01 | `src/create_roof.rs` | 恢复 `create_roof`：从编辑命令创建屋顶（权威状态→曲面/脊/瓦链）。 | core 权威、M02–M05 | 部分接线 | 原入口对照 + 端到端产出与 core 一致 | 部分 |
| UM-ROOF-02 | `src/edit_roof.rs` | 恢复 `edit_roof`：编辑后段已解析，补 ECS 前段（查询/调度/失败路径）。 | edit_rules、core | 部分（后段已做） | 原入口对照 + ECS 前段消费者接线 | 部分 |
| UM-ROOF-03 | `src/delete_roof.rs` | 恢复 `mark_roof_for_deletion`、`delete_roofs`、`process_invalid_roof_notifications`。 | core 生命周期 | 部分接线 | 原指令样例 + 无效通知条件与原一致 | 部分 |
| UM-ROOF-04 | `src/update_roof_spatial.rs` | 恢复 `update_roof_spatial`：屋顶空间索引更新。 | core 空间索引 | 部分接线 | 原指令样例 | 部分 |

### 历史与回放

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-ROOF-05 | `src/deferred_record_roof_history.rs` | 恢复 `roof_record_history`：屋顶编辑的历史记录（延迟提交）。 | core 历史事务（NR2） | 无 | 原入口对照 + 命令 schema 一致 | 未开始 |
| UM-ROOF-06 | `src/replay_roof.rs` | 恢复 `replay_roof` 及其 `process`：屋顶编辑回放。 | core 历史回放 | 部分接线 | 原入口对照 | 部分 |

### 触发体 / 音频 / 派生

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-ROOF-07 | `src/roof_trigger_volume.rs` | 恢复 `calculate_roof_trigger_volumes`：屋顶触发体几何（未被其他模块闭合）。 | 屋顶几何 | 未闭合 | 原指令样例 + 触发体几何与原一致 | 未开始 |
| UM-ROOF-08 | `src/simulate_roof_tile_for_audio.rs` | 恢复 `simulate_roof_tile_for_audio`：为音频模拟瓦片落点（音频跨切面）。 | 音频胶水 | 无 | 原指令样例 | 未开始 |
| UM-ROOF-09 | `src/wall_height_dt.rs` | 恢复 `wall_height_dt`：墙高变化对屋顶的增量传播。 | core 墙高权威 | 无 | 原指令样例 | 未开始 |

### 视觉族

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-ROOF-10 | `src/visual/assemble_roof_tiles.rs` | 恢复 `assemble_roof_tiles`：铺瓦结果→视觉实体。 | CPU 铺瓦、rhapsody | 部分接线 | 原入口对照 + 瓦实例变换一致 | 部分 |
| UM-ROOF-11 | `src/visual/generate_roof_stone_floor_and_roof_bottom.rs` | 恢复 `generate_roof_stone_floor_and_roof_bottom`：石地板与屋顶底面生成。 | 屋顶几何 | 无 | 原指令样例 | 未开始 |
| UM-ROOF-12 | `src/visual/place_spires.rs` | 恢复 `place_roof_spires`：尖顶放置。 | 屋顶几何 | `evidence/building-roof-spires/` 有局部 | 原指令样例 + 尖顶参数与原一致 | 部分 |
| UM-ROOF-13 | `src/visual/switch_roof_type.rs` | 恢复 `switch_roof_type`：屋顶类型切换。 | 屋顶几何 | 无 | 原指令样例 | 未开始 |
| UM-ROOF-14 | `src/visual/roof_animation.rs` | 恢复 `roof_animation`：屋顶动画（季节/雪）。 | 时间/天气 | 无 | 原指令样例 | 未开始 |

### UI 编辑

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-ROOF-15 | `src/ui_edit_roof.rs` | 恢复 `ui_edit_roof`、`do_ui_edit_roof`：屋顶高度/编辑 UI。 | toolbar 交互 | 无 | 原指令样例 | 未开始 |
| UM-ROOF-16 | `src/ui_edit_roof_ridge_dir.rs` | 恢复 `ui_edit_roof_ridge_dir`、`do_ui_edit_roof_ridge_dir`、`switch_roof_ridge_direction`、`record_switch_roof_ridge_history`。 | 脊线、历史 | 无 | 原指令样例 + 历史记录一致 | 未开始 |
| UM-ROOF-17 | `src/ui_edit_roof_ridge_length.rs` | 恢复 `ui_edit_roof_ridge_length`、`do_ui_edit_roof_ridge_length`、`create_ui_widget`。 | 脊线 | 无 | 原指令样例 | 未开始 |
| UM-ROOF-18 | `src/ui_focused/ui_edit_roof_height_focused.rs` | 恢复 `ui_edit_roof_ridge_and_height_focused`：聚焦编辑。 | 脊线、高度 | 无 | 原指令样例 | 未开始 |

### 插件 / 组件

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-ROOF-19 | `src/lib.rs` | 恢复 `add_roof_systems`、`add_roof_events_and_resources`：系统/事件/资源注册。 | 全部上面 | 仅索引 | 与原注册顺序对照 | 未开始 |
| UM-ROOF-20 | `src/utils/components.rs` | 恢复 `register_required_components`（Roof 组件合同）。 | core Roof 组件 | 部分 | 字段/required-components 策略与原一致 | 部分 |

> `src/utils/assemble_rectangular_roof.rs`、`src/utils/assemble_circular_roof.rs` 无独立符号，CPU 链已在 core/M05 完成；此处只做消费者接线，不重复恢复。

## 卡点

- `create/edit/delete` 依赖 core 权威状态（`PublicWallState::max_y`/`flat_roof_y` 等），core 侧未闭合前用冻结契约。
- `roof_record_history` / `replay_roof` 依赖 core 历史事务（NR2）。
- UI 编辑任务依赖 toolbar 的 widget 框架（批次 3），本批次只恢复编辑逻辑，UI 挂接留到批次 3。

## 验收标准

- 生命周期主干 ≥ 原入口对照；视觉族 ≥ 原指令样例；UI 编辑 ≥ 原指令样例。
- 复用 M02–M06 已有算法，不重新推导曲面/脊线/铺瓦。
