# M02 地形业务函数清单（2026-10-30）

本清单沿用 M06 的调用链盘点口径，但 M02 直接函数组很少，因此保留被库存排除的机械实现和通用 owner 方法，并对 5 个词法未匹配项逐个核对当前源码。它用于定位人工审计对象，不表示原版算法等价或 M2 整模块验收。

- 源码提交：`aa37abb24`
- B 函数索引 SHA256：`61a747815e31f7f080bf88b95d307c15750ea2090aec2226c0198673f4a9f711`
- 生成入口：[`tools/audit_m02_business_functions.py`](../../../tools/audit_m02_business_functions.py)
- 基础库存：[`B-MODULE-INVENTORY.md`](B-MODULE-INVENTORY.md)
- 机器可读结果：[`M02-BUSINESS-FUNCTIONS-2026-10-30.json`](M02-BUSINESS-FUNCTIONS-2026-10-30.json)

## 口径与统计

当前基础库存已预先排除 trait/serde/SystemParam 机械实现，以及 `new/get/insert/remove` 这类无法只凭名称确定 owner 的通用方法。本报告把 M02 的这些排除项单独保留，避免丢失原业务 ID。

| 指标 | 数量 |
|---|---:|
| M02 原始函数组 | 19 |
| 基础库存直接保留 | 13 |
| 唯一非空体候选 | 8 |
| 词法未匹配 | 5 |
| 机械派生排除 | 5 |
| 通用 owner 方法单列 | 1 |
| 业务相关函数组 | 14 |

5 个词法未匹配项经源码核对后：3 个是 `TerrainSystems` 同一文件 trait impl，2 个迁移到共享 render 回读文件；没有继续处于“源码位置未知”的业务函数。但 `MouseActivityTimer::new` 仍属于通用构造语义待证项。

## 业务功能分类

| 功能域 | 数量 |
|---|---:|
| GPU 回读与延迟发布 | 3 |
| 地形系统注册与资源生命周期 | 3 |
| 地形绘制与编辑 UI | 3 |
| 笔画编辑与变更传播 | 2 |
| 地形 hover 与拾取 | 1 |
| 地形历史录制 | 1 |
| 编辑资源构造 | 1 |

## 业务函数明细

| 功能域 | 函数 | 词法状态 | 当前源码/结论 |
|---|---|---|---|
| GPU 回读与延迟发布 | `country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback::on_gpu_copy_issued` | `unmatched_review` | `crates/country-core/src/systems/render_loop/compute_passes/read_terrain_data.rs:138`<br>由地形回读资源的跨模块定义承载；写入 changed_tick，随后 swap(0) 取走 pending，并以 OR 语义保留 copy_issued。 |
| GPU 回读与延迟发布 | `country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback::on_terrain_modified` | `unmatched_review` | `crates/country-core/src/systems/render_loop/compute_passes/read_terrain_data.rs:134`<br>由地形回读资源的跨模块定义承载；对 pending 做 fetch_add(1)，由 draw/edit 等真实编辑消费者共同调用。 |
| GPU 回读与延迟发布 | `country_core::systems::terrain::delayed_terrain_changed_msg::delayed_terrain_changed_msg` | `body_candidate` | `tiny-glade/crates/country-core/src/systems/terrain/delayed_terrain_changed_msg.rs:21`<br>两帧软件延迟读取真实 GPU 发布副本，并分别提交权威高度场与视觉高度场；中间发送 OnTerrainChange。 |
| 地形 hover 与拾取 | `country_core::systems::terrain::terrain_hover_idx::terrain_hover_idx` | `body_candidate` | `tiny-glade/crates/country-core/src/systems/terrain/terrain_hover_idx.rs:43`<br>从真实鼠标地形射线与 GPU 回读选择 hover 笔画，生产 HoveredTerrainStrokeId。 |
| 地形历史录制 | `country_core::systems::terrain::deferred_terrain_history::history_record_terrain` | `body_candidate` | `tiny-glade/crates/country-core/src/systems/terrain/deferred_terrain_history.rs:32`<br>地形编辑历史录制系统；需要继续核对完整 typed history、游标与失败语义。 |
| 地形系统注册与资源生命周期 | `country_core::systems::terrain::impl$0::add_terrain_events` | `unmatched_review` | `crates/country-core/src/systems/terrain/mod.rs:238`<br>实现在 TerrainSystems trait impl 中；按原观察顺序注册 6 个地形事件。 |
| 地形系统注册与资源生命周期 | `country_core::systems::terrain::impl$0::add_terrain_systems` | `unmatched_review` | `crates/country-core/src/systems/terrain/mod.rs:205`<br>实现在 TerrainSystems trait impl 中；挂载工具 UI/编辑/回读链和历史录制系统。 |
| 地形系统注册与资源生命周期 | `country_core::systems::terrain::impl$0::insert_terrain_resources` | `unmatched_review` | `crates/country-core/src/systems/terrain/mod.rs:226`<br>实现在 TerrainSystems trait impl 中；初始化编辑状态、双高度场、hover 与 ScheduledTerrainReadback。 |
| 地形绘制与编辑 UI | `country_core::systems::terrain::ui_draw_terrain::ui_draw_terrain` | `body_candidate` | `tiny-glade/crates/country-core/src/systems/terrain/ui_draw_terrain.rs:74`<br>真实画布交互到地形绘制命令/编辑路径的 UI 生产端。 |
| 地形绘制与编辑 UI | `country_core::systems::terrain::ui_edit_terrain::ui_edit_terrain` | `body_candidate` | `tiny-glade/crates/country-core/src/systems/terrain/ui_edit_terrain.rs:180`<br>地形坡度、高度、falloff、焦点、历史哨兵、提示和装饰剔除等编辑交互主系统。 |
| 地形绘制与编辑 UI | `country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode` | `body_candidate` | `tiny-glade/crates/country-core/src/systems/terrain/ui_terrain_focus_mode.rs:173`<br>地形 focus 模式的真实 UI 输入与模式切换衔接。 |
| 笔画编辑与变更传播 | `country_core::systems::terrain::draw_terrain_stroke::terrain_stroke_changed` | `body_candidate` | `tiny-glade/crates/country-core/src/systems/terrain/draw_terrain_stroke.rs:15`<br>地形笔画变更后发出重算、修改通知与 StrokeChanged 声音事件。 |
| 笔画编辑与变更传播 | `country_core::systems::terrain::edit_terrain_stroke::edit_terrain_stroke` | `body_candidate` | `tiny-glade/crates/country-core/src/systems/terrain/edit_terrain_stroke.rs:27`<br>消费笔画添加/修改命令，更新地形权威数据，并请求重算和延迟回读。 |
| 编辑资源构造 | `country_core::systems::terrain::ui_edit_terrain::MouseActivityTimer::new` | `generic_owner_review` | `crates/country-core/src/systems/terrain/ui_edit_terrain.rs:83`<br>没有同名 new；当前生产通过 Default 构造，字段初值已记录，但业务消费者和 new 是否对应独立原步骤仍待原证。 |

## 排除项

### 机械派生或序列化

- `country_core::systems::terrain::deferred_terrain_history::impl$0::default`（`crates/country-core/src/systems/terrain/deferred_terrain_history.rs`，B ID `ffbf66ecc5fa9919e9`）
- `country_core::systems::terrain::delayed_terrain_changed_msg::impl$3::default`（`crates/country-core/src/systems/terrain/delayed_terrain_changed_msg.rs`，B ID `b6c6616e0188d46c25`）
- `country_core::systems::terrain::delayed_terrain_changed_msg::impl$1::default`（`crates/country-core/src/systems/terrain/delayed_terrain_changed_msg.rs`，B ID `1f717b099cde121984`）
- `country_core::systems::terrain::ui_edit_terrain::impl$2::default`（`crates/country-core/src/systems/terrain/ui_edit_terrain.rs`，B ID `ee17ba64be74378c0b`）
- `country_core::systems::terrain::ui_terrain_focus_mode::impl$1::default`（`crates/country-core/src/systems/terrain/ui_terrain_focus_mode.rs`，B ID `05ab4a38bbb5c621e2`）

### 通用 owner 方法

- `country_core::systems::terrain::ui_edit_terrain::MouseActivityTimer::new`（`crates/country-core/src/systems/terrain/ui_edit_terrain.rs`，B ID `3f74db2d6b867cf4da`）：已保留为业务相关项，当前源码映射见上表。

## 模块边界

B-MODULE-INVENTORY 的 M02 主体只覆盖 systems::terrain 直接路径。地形高度资源、GPU compute pass、光标射线、渲染与共享历史仍位于 SHARED 或其它模块路径，不能据本清单宣称整个 M2 已做全函数审计。

至少还要把以下跨模块地形职责单独纳入 M2 语义审计：

- `resources/terrain_height.rs`
- `resources/cursor_terrain_raycast.rs`
- `resources/mouse_terrain_raycast.rs`
- `systems/render_loop/compute_passes/read_terrain_data.rs`
- `systems/render_loop/compute_passes/rasterize_terrain_stroke.rs`
- `systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs`
- `tiny-glade/src/world_terrain.rs`

## 审计要求

1. 非空函数体只证明源码位置存在，不能证明与原版算法、顺序、失败语义一致。
2. `ScheduledTerrainReadback` 两个方法和三个 `TerrainSystems` impl 已定位，后续应直接审查其算法与真实 producer/consumer，不再列为名称缺失。
3. `MouseActivityTimer::new` 当前无同名实现；需确认原版构造是否就是现有 `Default` 行为，并定位三个字段的真实消费者。
4. 完整 M2 验收仍以真实输入、同一地形权威、GPU/碰撞/画面、历史、保存恢复和卸载链为准；本清单不改变该项状态。
