# country-core B 版 `systems/wall/` 逐文件还原度复核

- 审计日期：2026-10-19
- 审计对象：`D:/game/tiny-glade/crates/country-core/src/systems/wall/**`（81 个 `.rs`，18,720 行；不含测试外文件）
- 骨架：`D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/systems/wall`（57 个 `.rs`，PDB 元数据）
- 机械统计：`D:/game/reverse-engineering/tiny-glade/evidence/audit-2026-10-19/fn_audit.json`（`systems/wall/*` 137 条）
- 基线：`D:/game/tiny-glade/crates/country-core/recovery/STUBS-2026-10-19.md`
- 方式：只读。未修改任何源码（含用户重构区 `wall`）。未运行 cargo/差分测试；全部结论来自源码、骨架元数据、函数/注册表机械扫描与定点抽读。

---

## 1. 结论摘要

| 维度 | 估计 | 依据 | 置信度 |
|---|---:|---|---|
| 文件存在/搬移覆盖率 | **约 90%**（51/57 骨架路径；若计入 GPU 上传搬移约 91%） | 57 个骨架文件中 5 个 adjust_wall_height 合并为 1，`spatial_hash_update` 并入 `wall_spatial_hash.rs:354`，`gpu_flush_brick_buffers` 功能搬移到 `geometry/runtime.rs:40 sync_wall_bricks`；`merge_walls`、`wall_splitter`、`set_wall_style`、2 个 debug 文件无对应 | 高 |
| 具名函数覆盖率 | **约 89%**（92–93/104 有业务含义的骨架符号） | 137 条中 in_file 87、moved 5、name_only 2、no_trace 43；剔除 29 个 closure 与 4 个 serde 派生后，真正无落点约 11 条 | 中-高 |
| CPU 权威状态/几何 | **约 82%** | 发布、空间哈希、曲线预处理、形状创建/编辑、砖/拱/栅栏/平台楼梯构造均有主体代码；缺口集中在 merge/split 拓扑、墙体完整 corner/skirt/signifier owner | 中 |
| CPU 历史 | **约 76%** | 新增/删除/擦除/矩形/拉伸历史主体在；合并墙历史、墙体高度历史、普通删除历史生产者缺失 | 中 |
| 交互 UI 系统 | **约 45%** | 多个 UI 状态机有真实代码，但大量系统未注册、输入生产者（trigger volume/画布 action）缺失 | 中 |
| signifier/渲染 widget | **约 20%** | 大量 signifier 只写 `UiSignifierStream` 记录，实体 spawn/despawn、mesh、widget layer 为空或宿主待接线 | 中-高 |
| GPU 砖实例上传 | **约 60%（搬移）** | wall 目录内 `upload_instanced_wall_ssbo.rs` 缺失，但等价功能在 `geometry/runtime.rs` 并已注册；渲染 pass/着色器不在本范围 | 中 |
| **wall 模块综合** | **约 70%（65%–75%）** | 上述加权；CPU 主体明显强于 UI/渲染 | 中 |

一句话：**wall 的 CPU 权威/几何/历史主干是真实恢复，绝非签名壳；主要缺口是合并/拆分墙整条链、墙高交互 UI/历史、signifier/widget 渲染实体链，以及若干未注册的编辑系统。**

---

## 2. 文件级状态表（81 文件）

性质口径：
- **真实行为**：有可执行主体、会改变权威/几何/历史或输出领域事件。
- **数据合同**：类型、事件载荷、serde/布局合同；行为不是重点。
- **声明**：模块根、资源/命令/事件声明，无算法。
- **混合**：CPU 行为真实，但存在明确的 render/UI/宿主 seam 或未接线分支。
- **壳/水分**：空体、只返回常量、重复实现等。

### 2.1 顶层（16）

| 文件 | 行/函数/测试 | 性质 | 关键判定/缺口 |
|---|---:|---|---|
| `adjust_wall_height.rs` | 576/9/10 | 混合 | `adjust_wall_height` 核心消费真实（39–97）；`UiWallHeight` 提升/命名/signifier 判定真实（150–249）；`get_closest_point_u_to_cursor` 真实（253）。但 `ui_adjust_wall_height`、`ui_adjust_wall_height_mode`、`adjust_wall_height_signifier_animation`、`history_record_adjust_wall_height` 系统缺失。 |
| `calc_wall_bend.rs` | 45/2/0 | 真实行为 | 末尾 4 个地形映射方向求弯折角，`compute_bend` 主体完整。 |
| `construct_wall_bricks.rs` | 278/4/0 | 混合 | 砖生产、`WallTriggerVolume::recompute`、RT 网格更新、实体 spawn 真实；文件头自述完整 corner/skirt/signifier owner 另属外部（1–5）。 |
| `decorator_wall_events.rs` | 103/2/2 | 混合 | `OnWallDeleted → on_wall_removed` 真实；`decorator_wall_change_pending` **空体**（36）；split/merge 迁移未接线（11–16）。 |
| `deferred_wall_history.rs` | 114/4/0 | 真实行为 | 自由手墙扩展 pending 历史真实；合并历史分支缺失。 |
| `delete_wall.rs` | 28/1/0 | 真实行为 | 删除屋顶关联、墙/灰泥实体 despawn。 |
| `draw_wall.rs` | 813/3/7 | 混合 | 自由手墙采样/继续/历史提交真实；`snap.mode > 1` 明确拒绝（163–168），即合并吸附拓扑未实现。 |
| `finish_wall.rs` | 423/2/5 | 混合 | 新建/继续完成、无效删除、历史记录真实； snapped/merge 分支明确 `UnsupportedFinish`（72–79、129–136）。 |
| `height_command.rs` | 162/3/0 | 真实行为 | 高度命令策略、Rust/原生除法、clamp 输入真实；本身不记录历史。 |
| `history_recording.rs` | 126/7/0 | 真实行为 | `RecordWallHistory` 消费真实；`deleted_wall` 构造函数无调用者（35）。 |
| `mod.rs` | 50/0/0 | 声明 | 3 个 `Set*Cmd` 均为 `_gap: u64` typed gap（20–39）。 |
| `registration.rs` | 499/5/1 | 接线 | CPU 发布/空间/砖/删除链注册真实（175–278）；形状编辑、signifier、wall_highlight 等未注册。 |
| `structure_outline.rs` | 43/1/0 | 声明/数据合同 | 仅事件声明；`OutlineWallCmd` 载荷 `_gap`（12–13），consumer 未在本模块。 |
| `ui_continue_wall.rs` | 884/3/9 | 混合 | 端点继续/抑制状态机与历史事务真实；signifier renderer closure 属宿主。 |
| `ui_draw_wall.rs` | 655/1/2 | 真实行为 | 表驱动墙/栅栏输入状态机（180 行附近大跳表）真实；water-hint signifier closure 为渲染边界。 |
| `wall_highlight.rs` | 50/1/0 | 真实行为 | 高亮命令消费真实，但**未在任何 schedule 注册**。 |

### 2.2 `construction/`（15）

| 文件 | 行/函数/测试 | 性质 | 关键判定/缺口 |
|---|---:|---|---|
| `mod.rs` | 19/0/0 | 声明 | 子模块根。 |
| `arches.rs` | 310/12/0 | 真实行为 | 规则墙拱 mask→profile→walker→96B CPU 记录。 |
| `bricks.rs` | 660/20/0 | 真实行为 | 洞/砖切分、行拆分、proposal 消费主体完整。 |
| `fences.rs` | 413/9/0 | 真实行为 | 栅栏 CPU owner；文件头与 117 行明确 **UNVERIFIED**。 |
| `freehand_arches.rs` | 324/9/0 | 真实行为 | 自由手拱端点 profile/walker/记录。 |
| `gates.rs` | 681/17/0 | 真实行为 | 门 CPU 消费者；文件头标明 under recovery、整体差分验收 pending（3）。 |
| `parameters.rs` | 292/13/0 | 真实行为 | 构造 kind、种子、顶部策略、圆/矩形构造选项真实。 |
| `platform_collision.rs` | 288/14/0 | 混合 | 平台楼梯碰撞 key/hash/更新策略真实；真实物理后端是 trait seam。 |
| `platform_rapier_backend.rs` | 192/8/0 | 真实行为 | 真实 Rapier `ColliderBuilder::trimesh`/SlotMap 适配（159–170）。 |
| `platform_segmentation.rs` | 142/1/0 | 真实行为 | 平台楼梯阈值化。 |
| `platform_stair_rows.rs` | 324/14/0 | 真实行为 | 平台楼梯渲染曲线/共享 96B 生产者。 |
| `platform_stairs.rs` | 443/6/0 | 真实行为 | 台阶分段/几何/孔洞。 |
| `production.rs` | 582/14/0 | 真实行为 | `WallConstructor` 生产者、噪声/perturb/行边界。 |
| `segmentation.rs` | 449/16/0 | 真实行为 | `WallPathSegmentationMask`、LoopRange、坐标投影。 |
| `structures.rs` | 725/20/0 | 真实行为 | mask→materialized segments、lintel 支撑/走线。 |

### 2.3 `eraser/`（6）

| 文件 | 行/函数/测试 | 性质 | 关键判定/缺口 |
|---|---:|---|---|
| `mod.rs` | 44/0/0 | 声明 | 命令/音频事件类型；音频 host 待接线。 |
| `animate_wall_bulldoze.rs` | 123/8/0 | 真实行为 | 0.2s CubicInOut 阶段推进、切片、phase=1 删除权威。 |
| `archivist.rs` | 799/22/7 | 混合 | 3-map 装配、shrink/split/full-erase、finish 五组历史真实；0xd8/0x80 载荷字节映射是结构近似（25–33）。 |
| `bulldoze.rs` | 153/4/0 | 真实行为 | 推土历史快照 + 动画数据。 |
| `ui_bulldozer.rs` | 171/5/2 | 混合 | 模式判定/命令入队/signifier 记录真实；Rapier raycast 用代理、动作分派待接线（72）、光标图标 host（164）。 |
| `wall_eraser.rs` | 95/3/0 | 混合 | 切片/高度 clamp/平顶补偿真实；stroke u 来源待取证（35）、ChildWall split/rewire 待取证（8）。 |

### 2.4 `shapes/`（27）

| 文件 | 行/函数/测试 | 性质 | 关键判定/缺口 |
|---|---:|---|---|
| `mod.rs` | 223/5/0 | 声明 | 形状资源/命令声明、`to_authority_curve`、屋顶请求构造。 |
| `authoring.rs` | 466/16/0 | 混合 | 创建外观、InnerWalls 变更、放置请求真实；UI 接受/选中颜色属其他 owner。 |
| `calc_shape_height.rs` | 214/2/2 | 混合 | 地形采样/平均/夹取真实；但双重 `-= max` 与 `lerp_rate` 忽略 dt 需人工复核（103–146）。 |
| `shape_placement.rs` | 392/10/3 | 混合 | 空间查询/法向真实；Rapier raycast、屋顶脊几何未接入；`snap_to_roof_ridge` 已写但未被调用；系统未注册。 |
| `shape_placement_signifiers.rs` | 218/4/2 | 混合 | `preview_snapping_line` 投影/切片真实；outline mesh/spawn host；系统未注册。 |
| `utils/mod.rs` | 4/0/0 | 声明 | 子模块根。 |
| `utils/shape_signifier_lerp.rs` | 115/4/2 | 真实行为 | 指数收敛+线性钳项、跨形状 panic 合同真实。 |
| `circle/mod.rs` | 8/0/0 | 声明 | 子模块根。 |
| `circle/create_circle.rs` | 211/2/2 | 真实行为 | 圆曲线/围合/屋顶参数/历史真实。 |
| `circle/signifiers/mod.rs` | 5/0/0 | 声明 | 子模块根。 |
| `circle/signifiers/edit_arrow.rs` | 68/4/0 | 混合 | 箭头记录真实；`register_required_components` 返回空（53–55）、`clone_behavior` 返回 0（60）。 |
| `circle/signifiers/edit_line.rs` | 56/3/0 | 混合 | 半径线记录真实；组件注册为空（51–53）。 |
| `circle/ui_create_circle.rs` | 318/4/4 | 混合 | 画布输入状态机/半径累积真实；输入资源是代理（38–50）、signifier env/marker 为渲染 seam。 |
| `circle/ui_edit_circle.rs` | 104/3/0 | 混合 | 半径编辑/0.01 历史阈值真实；装饰恢复 host（90）、trigger hit-test 缺失；未注册。 |
| `circle/ui_rotate_circle.rs` | 47/2/0 | 混合 | CPU 旋转写权威真实；屏幕↔世界角度源缺失、未注册。 |
| `rectangle/mod.rs` | 20/1/0 | 声明 | 子模块根 + 朝向 helper。 |
| `rectangle/create_rectangle.rs` | 305/3/3 | 真实行为 | 矩形曲线/围合/屋顶参数/历史真实。 |
| `rectangle/edit_decision.rs` | 199/3/1 | 真实行为 | 完成编辑决策/历史/enclosure 判定真实。 |
| `rectangle/edit_rectangle.rs` | 31/1/0 | 真实行为 | 事件→曲线/围合更新。 |
| `rectangle/finish_edit_rectangle.rs` | 54/2/0 | 混合 | 决策/历史真实；装饰恢复 host。 |
| `rectangle/signifier_edit_dims_line.rs` | 52/3/0 | 混合 | 四边记录真实；组件注册为空（47–49）。 |
| `rectangle/ui_create_rectangle.rs` | 185/4/2 | 混合 | 拖拽预览真实；`rectangle_dims_from_cursor_pos` 投影待取证（7）、朝向硬编码 `get_rectangle_orientation(0.0)`（114）、`previous_wall: None`（119）、signifier env host。 |
| `rectangle/ui_edit_rectangle_dims.rs` | 102/3/0 | 混合 | 边距/扩展/EditRectangleCmd 真实；输入资源无生产者；未注册。 |
| `rectangle/ui_edit_rectangle_orientation.rs` | 243/5/3 | 混合 | fast acos/斜箱 trigger volume 真实；输入资源无生产者；未注册。 |
| `rectangle/ui_modes/mod.rs` | 5/0/0 | 声明 | 子模块根。 |
| `rectangle/ui_modes/edit_dims.rs` | 50/2/0 | 混合 | signifier 记录真实；`ui_rectangle_dims_edit_widget` **空体**（49）。 |
| `rectangle/ui_modes/edit_orientation.rs` | 24/2/0 | 壳 | widget 仅 `let _ = (...)（16）`，signifier **空体**（23）。 |

### 2.5 `snapping/`（7）

| 文件 | 行/函数/测试 | 性质 | 关键判定/缺口 |
|---|---:|---|---|
| `mod.rs` | 105/4/0 | 数据合同 | `SnappingMode` 模式/发布区/new/is_active/position_to_snap_to 合同。 |
| `snapping_audio.rs` | 97/1/3 | 真实行为 | mode2/3 音效去重与 `(snap.y, snap.x, snap.z)` 位置真实。 |
| `snapping_init.rs` | 126/1/2 | 真实行为 | 曲线长度门、首段投影走查、snapped 判定真实；debug 资源身份待取证（25）。 |
| `snapping_maintain.rs` | 138/3/2 | 混合 | 全状态发布/禁用通知真实；signifier 只写记录、实体流待接线（54–57、91）。 |
| `snapping_to_ends.rs` | 131/1/0 | 真实行为 | 1.5 距离、freehand tag、起点/终点极性、3D outline 插值真实。 |
| `snapping_to_segments.rs` | 133/2/1 | 真实行为 | ±8 AABB→最近候选排序→1.5 门→地形高度采样真实；Snap 区 swizzle 需人工复核（78–83）。 |
| `telegraph_walls_are_merged.rs` | 129/3/1 | 混合 | easing 计时/切片范围真实；despawn/insert/网格重建 host（12–18、74–78、90–101）。 |

### 2.6 `valid_enclosure/`（1）

| 文件 | 行/函数/测试 | 性质 | 关键判定/缺口 |
|---|---:|---|---|
| `mod.rs` | 699/42/11 | 数据合同 + 真实行为 | 28B ShapeParameters、包含/中心/周长/法向/expand、serde 新旧格式合同真实；4 个 `no_trace` 是 serde 单态符号，非业务缺口。 |

### 2.7 `wall_state/`（9）

| 文件 | 行/函数/测试 | 性质 | 关键判定/缺口 |
|---|---:|---|---|
| `mod.rs` | 14/0/0 | 声明 | 子模块根。 |
| `height_authority.rs` | 48/2/0 | 真实行为 | 高度写回 + dirty 标记真实。 |
| `hole_invalidation.rs` | 7/1/0 | 真实行为 | 极小的洞重算转发（等价包装）。 |
| `path_preparation.rs` | 280/11/0 | 真实行为 | 4 路 path raster 归一化、法向/衰减/平滑/阈值交点/去重/Curve3 重建真实。 |
| `pending_changed.rs` | 120/4/0 | 混合/重复 | `approximate_curve_u` 被 `wall_trigger_volume.rs:6` 使用；但本文件另有一套 `PendingWallChanged`/`try_pending_wall_changed`/`take_pending_wall_changed`（13–119）未被使用，与 `deferred_wall_history.rs` 重复。 |
| `publication_policy.rs` | 80/4/0 | 真实行为 | 最小/最大高度、约束 extra、672B 投影字段真实。 |
| `publish_public_wall_state.rs` | 297/11/2 | 真实行为 | path/terrain 订阅、dirty 收集、公开几何/事件发布真实。 |
| `wall_roof_hashmap.rs` | 74/1/2 | 真实行为 | 墙-屋顶 1:1 关联与重复 key panic 真实。 |
| `wall_spatial_hash.rs` | 526/20/3 | 真实行为 | 段存储/cell 覆盖/AABB/圆/线段/矩形查询/排序去重/`spatial_hash_update` 真实。 |

---

## 3. 空壳与水分（file:line）

### 3.1 `todo!` / `unimplemented!`
- **0 处**。wall 目录没有 `todo!`/`unimplemented!`。

### 3.2 空体/平凡体（4 处）
| 位置 | 函数 | 说明 |
|---|---|---|
| `systems/wall/decorator_wall_events.rs:36` | `decorator_wall_change_pending` | 空体，仅消费 `OnWallChanged` 参数；split/merge 迁移占位。 |
| `systems/wall/shapes/rectangle/ui_modes/edit_dims.rs:49` | `ui_rectangle_dims_edit_widget` | 空体；widget/trigger-volume 层未恢复。 |
| `systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs:16` | `ui_rectangle_orientation_edit_widget` | 仅 `let _ = (常量...)`。 |
| `systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs:23` | `ui_rectangle_orientation_edit_signifier` | 空体。 |

### 3.3 只返回占位/空注册（不算行为）
| 位置 | 函数 | 说明 |
|---|---|---|
| `shapes/circle/signifiers/edit_arrow.rs:53` | `register_required_components` | 返回 `([], [])`；原组件注册面未恢复。 |
| `shapes/circle/signifiers/edit_line.rs:51` | `register_required_components` | 同上。 |
| `shapes/rectangle/signifier_edit_dims_line.rs:47` | `register_required_components` | 同上。 |
| `shapes/circle/signifiers/edit_arrow.rs:60` | `clone_behavior` | 返回 0（原 0x140002370 即返回 0，属原作平凡，不计缺口）。 |

### 3.4 typed gap / 未知载荷
| 位置 | 内容 |
|---|---|
| `systems/wall/mod.rs:20-39` | `SetWallStyleCmd` / `SetFreehandHeightTypeCmd` / `SetPlasterPeelAmountCmd` 全为 `_gap: u64`。 |
| `systems/wall/structure_outline.rs:12-13` | `OutlineWallCmd { _gap: u64 }`，载荷语义 U。 |
| `shapes/calc_shape_height.rs:207-208` | `TimberframeConstructionParamsCache._gap: u64`（仅类型合同）。 |

### 3.5 注释标记密度（占位/待取证/待接线/未实现/未恢复/缺类型）
- 全目录 **47 处、23 个文件**。
- 最集中：`eraser/archivist.rs`（8）、`snapping/snapping_maintain.rs`（3）、`ui_create_rectangle.rs`（3）、`ui_modes/edit_orientation.rs`（3）、`ui_edit_circle.rs`（3）。

### 3.6 重复/死代码水分
- `wall_state/pending_changed.rs:13-119`：与 `deferred_wall_history.rs` 重复的 `PendingWallChanged` 类型与 `take_pending_wall_changed` 无调用者；只有 `approximate_curve_u`/`approximate_curve_coordinate` 被 `resources/walls/wall_trigger_volume.rs` 和 `decorator/anchors.rs` 使用。
- `history_recording.rs:35 RecordWallHistory::deleted_wall`：构造函数存在但全 crate 无调用者；普通删除历史链无生产者。
- `wall_state/hole_invalidation.rs` 7 行：真实但极薄（原逻辑本身就小，不计水分）。

---

## 4. 骨架对照：`no_trace` 分类

`fn_audit.json` 中 `systems/wall/*` 共 **137** 条：`in_file` 87、`moved` 5、`name_only` 2、`no_trace` 43。

### 4.1 非 closure 的 `no_trace`（14 条）
| 骨架文件:函数 | 地址 | 分类 | 依据 |
|---|---|---:|---|
| `merge_walls.rs:merge_walls` | 0x1408…（5378775328） | **真缺失** | 全 crate 无 `merge_walls`/`MergeWallsCmd`；骨架参数含 `FinishWall`、`SnappingMode`、`MergeWallsCmd`、`PlayLineAcrossWall`、`History`、`PendingWallHistory`。 |
| `deferred_wall_history.rs:history_record_merge_walls` | 5378003376 | **真缺失** | 无合并墙历史记录函数；`history_recording.rs` 只有 new/deleted/height。 |
| `deferred_wall_history.rs:wall_deleted` | 5379749536 | **真缺失** | 无 `OnWallDeleted → 删除历史` 系统；`RecordWallHistory::deleted_wall` 无调用者。 |
| `upload_instanced_wall_ssbo.rs:gpu_flush_brick_buffers` | 5377278320 | **搬移/改名** | 功能在 `geometry/runtime.rs:40 sync_wall_bricks`，并在 `asset_libraries/runtime.rs:68` 注册；名字/地址未对上，自动审计误判。 |
| `wall_splitter.rs:wall_splitter` | 5379673120 | **真缺失** | 全 crate 无 splitter/系统；骨架参数含 `PublicWalls`、`UiSignifierStream`、`WallEraserArchivist`、`DecoratorStorage`、`History`、地形与颜色事件。 |
| `adjust_wall_height/signifier_animation.rs:ui_adjust_wall_height_signifier_animation` | 5377473360 | **真缺失（UI/渲染）** | 无 wall 专用 signifier 动画系统；通用 `systems/ui/signifiers/animation.rs` 不能替代该 4 参数 wall 系统。 |
| `adjust_wall_height/ui_adjust_wall_height.rs:ui_adjust_wall_height` | 5380439040 | **真缺失（UI）** | 全 crate 无 `ui_adjust_wall_height`、`FinishAdjWallHeightMode`、`InitAdjWallHeightMode`、`AdjustWallHeightUi`。 |
| `adjust_wall_height/ui_adjust_wall_height_mode.rs:history_record_adjust_wall_height` | 5380012512 | **真缺失（历史）** | `WallHeightHistoryEdit` 仅在 `resources/history/building_transactions.rs:23` 定义，无生产者。 |
| `valid_enclosure/mod.rs:Value>` / `Error>` / `Global>` / `Serializer>` | 5392743152 等 | **serde 派生单态** | 名称明显是 serde 泛型单态；不计业务缺口。 |
| `wall_state/debug_viz_threshold_curve.rs:debug_viz_threshold_curve` | 5379764416 | **真缺失（debug-only）** | 无对应系统；骨架参数 `DebugGui/PublicWalls/UiSignifierStream`。 |
| `wall_state/spatial_hash_debug.rs:spatial_hash_debug` | 5378064608 | **真缺失（debug-only）** | 无对应系统；骨架参数 `WallSpatialHash/MouseDepthValue/UiSignifierStream/DebugGui`。 |

### 4.2 closure `no_trace`（29 条）
集中在 `structure_outline.rs`（7）、`ui_edit_circle.rs`（2）、`ui_modes/edit_dims.rs`（2）、`shape_placement_signifiers.rs`（2）、`adjust_wall_ui_mode.rs`（2）、`eraser/ui_bulldozer.rs`（2）等。
- 判定：**不能直接算缺口**。这些是闭包/内联单态，很多对应 render-host 的 `SignifierEntityBuilder`/vtable/动画闭包；其宿主函数已存在但只写记录或明确登记 seam。
- 需人工确认的具体项：`ui_rotate_circle.rs:closure$0`、`ui_edit_circle.rs:closure$0/2`、`ui_modes/edit_orientation.rs:closure$0`、`structure_outline.rs:closure$1/2/12-15/19`。

### 4.3 `name_only` / `moved`
| 项 | 结论 |
|---|---|
| `set_wall_style.rs:set_wall_style`（name_only） | **实际缺失**：全 crate 只在 `wall/mod.rs:20-38` 注释/typed gap 中出现，未实现系统。 |
| `wall_state/spatial_hash_update.rs:spatial_hash_update`（name_only） | **搬移/已实现**：`wall_spatial_hash.rs:354`。 |
| `adjust_wall_height_impl`（moved） | 合并进 `adjust_wall_height.rs:39` 核心系统。 |
| `adjust_wall_ui_mode::{signifier,widget}`（moved） | 仅部分落为 `UiWallHeight` 方法（198–249）；对应系统仍未接线。 |
| `ui_adjust_wall_height_mode`（moved） | **仅注释提及**（`adjust_wall_height.rs:107-114`），系统本身缺失；自动 `moved` 会高估。 |
| `get_closest_point_u_to_cursor`（moved） | 真实搬移：`adjust_wall_height.rs:253`。 |

### 4.4 骨架文件 vs 实现文件
- 骨架 57 → 实现 81。
- **骨架有、实现无（12 路径）**：
  - 合并/已实现 6：`adjust_wall_height/*` 5 个（合入 `adjust_wall_height.rs`）、`wall_state/spatial_hash_update.rs`。
  - 真缺失 6：`merge_walls.rs`、`set_wall_style.rs`、`upload_instanced_wall_ssbo.rs`、`wall_splitter.rs`、`wall_state/debug_viz_threshold_curve.rs`、`wall_state/spatial_hash_debug.rs`。
- **实现新增 35 路径**：`construction/*`（15）、`shapes/authoring.rs`、`shapes/*/mod.rs`、`wall_state/{height_authority,hole_invalidation,path_preparation,pending_changed,publication_policy}.rs`、`registration.rs`、`height_command.rs`、`history_recording.rs`、`decorator_wall_events.rs`、`eraser/{mod,bulldoze}.rs` 等。多为把 B 中跨文件/上层 owner 以本 crate 组织方式重组，**不是新增业务数**。

---

## 5. 抽读关键逻辑结论（12 个深读文件 + 定向核对）

1. **端点吸附是真实实现，不是壳。** `snapping_to_ends.rs:41-129` 有 freehand tag 门、cursor flag bit0、1.5 距离、端点极性（起点→end=1/u=0，终点→end=0/u=1）、简化 2D 曲线 `coord_at_u` + outline 3D 插值。细节与 fresh disasm 注释一致。**需人工**：`mode.snapped` 取 `building_projection().byte_0x29e()` 的语义未在本轮重验。
2. **段吸附也是真实实现。** `snapping_to_segments.rs:54-86`：±8 AABB、按候选到参考点距离排序、1.5 门、取地形高度并写 `(height,x,z)`。**需人工**：`mode.snap` 的 swizzle 与 `SnappingMode::position_to_snap_to`（`mod.rs:92-97`）、`snapping_maintain` 预览（68/93）存在 `(y,x,z)`/`(y,z,x)` 不一致嫌疑，必须与原生调用点复核。
3. **吸附初始化/音效真实。** `snapping_init.rs:43-83` 的长度 1.65 门与首段 1.5 走查；`snapping_audio.rs:20-40` 的 mode→kind 与发布区去重；`telegraph_walls_are_merged.rs:62-101` 的 0.75s easing 与 `[2e-1,2e]` 切片真实。**缺口**：telegraph 的实体 despawn/insert/`update_curve_mesh` 未恢复。
4. **signifier 层是“记录层”，不是渲染层。** `snapping_maintain.rs:58-101`、`shape_placement_signifiers.rs:151-185`、`edit_arrow.rs:21-48`、`edit_line.rs:25-44`、`signifier_edit_dims_line.rs:19-41` 都只写 `UiSignifierStream`；没有实体 spawn、mesh、`SignifierEntityBuilder`、`DrawTopology`、`DashedLineLength`。这是明确的 GPU/渲染缺口，不是业务算法缺口。
5. **曲线取样与路径预处理是真实的 CPU 算法。** `wall_state/path_preparation.rs:84-146` 做 4 路 raster 归一化、弧长 `points_u`、逐段强度；`167-279` 做法向/4 层衰减、5 点平滑、0.3 阈值交点、中点插入、连续去重。`shape_placement_signifiers.rs:34-76` 用 `get_closest_u_from_pos` + 对称切片中点取投影点；`telegraph` 用 `coord_at_u` + outline 3D 插值。这些不是占位。
6. **墙碰撞/触发体：CPU 触发体真实，墙体 collider 不在本目录。** `construct_wall_bricks.rs:190-200` 用 `WallTriggerVolume::default_tris()` 更新 RT 网格；`248-269` 对实体调用 `trigger.recompute(wall)`、更新 RT/生成 `InstancedWall` 实体。`WallTriggerVolume::recompute` 在 `resources/walls/wall_trigger_volume.rs:50-...` 是真实四边 Quad/三角形生成（120B Quad、法向/UV/端点）。平台楼梯的 Rapier 碰撞是真实适配（`platform_rapier_backend.rs:159-170`）。**缺口**：完整墙体 corner/skirt/signifier owner（`construct_wall_bricks.rs:1-5` 自述）与 merge/split 拓扑未恢复。
7. **历史主干真实，但分支不全。** `deferred_wall_history.rs:48-113` 真实（严格子区间、first-tie 投影、单次 drain）；`ercaser/archivist.rs:129-577` 的 3-map、shrink/split/full-erase、5 组历史真实；`finish_wall.rs:97-160` 的新建/无效继续历史真实。**缺口**：`history_record_merge_walls`、`wall_deleted`、`history_record_adjust_wall_height` 无落点。
8. **墙高：核心命令真实，交互与历史缺失。** `adjust_wall_height.rs:39-97` 完整消费 `AdjustWallHeightCmd`（mode 0 skip、mode!=1 panic、InnerWalls/PublicWalls 探针、roof height、`apply_resolved_height_command`）。但 `ui_adjust_wall_height`/`ui_adjust_wall_height_mode`/signifier 动画/高度历史四个系统全无；`FinishAdjWallHeightMode` 等事件全 crate 零引用。`get_closest_point_u_to_cursor`（253）是唯一保留的 UI 数学 helper。
9. **形状创建/编辑 CPU 路径真实。** `circle/create_circle.rs:51-129`（`circle_roof_height_01` 51、`create_circle` 58）有 SineInOut 屋顶高度插值、屋顶命令、历史；`rectangle/create_rectangle.rs:58-84` 有 ridge/height 双插值；`ui_create_circle.rs:115-200` 有 action code 3/4/2 状态机与半径累积；`ui_edit_circle.rs:61-103` 有 0.01 半径阈值历史（88–101）；`edit_rectangle.rs:17-30` 与 `finish_edit_rectangle.rs:29-54` 真实。**但**这些系统的输入生产者/trigger volume 和多个注册缺失。
10. **构造系统（bricks/arches/fences/gates/platform stairs）是真实 CPU 代码。** `construction/` 15 文件均有实质函数体（例如 `production.rs` 14 函数、`structures.rs` 20 函数、`bricks.rs` 20 函数），不是签名壳。**置信度限制**：`fences.rs:1/117` 标注 UNVERIFIED，`gates.rs:3` 标注整体差分验收 pending；本轮未做机器码差分。
11. **形状放置/屋顶脊：部分。** `shape_placement.rs:53-113` 的 AABB 查询/法向真实；`snap_to_roof_ridge`（164-274）有完整 316 指令重建，但**没有任何调用者**；Rapier raycast 分支明确未接入（9-16）。`shape_placement_params` 与 `shape_placement_signifiers` 未注册。
12. **注册表只覆盖 CPU 主链。** `registration.rs:175-278` 注册了 draw/finish/continue/height/publish/spatial/bricks/delete 等；以下有实现的系统**全 crate 无注册**：`shape_placement_params`、`shape_placement_signifiers`、`ui_edit_circle`、`ui_rotate_circle`、`ui_edit_rectangle_dims`、`ui_edit_rectangle_orientation`、`finish_edit_rectangle`、全部 signifier 系统、`wall_highlight`、`snap_to_roof_ridge` 相关。它们是否由上层宿主注册**需人工确认**，但 country-core 内确实未接线。
13. **`calc_shape_height` 有两处需人工的语义风险。** `calc_shape_height.rs:103-146`：先 `height -= max + HEIGHT_OFFSET`，再 `target = height + max - HEIGHT_OFFSET`；且 `lerp_rate` 的 `t = (rate*0.1).clamp(0,1)` 对 rate=10 恒为 1，完全没有 dt/时间连续性。可能是有意的机器码复刻，也可能是重建偏差。
14. **GPU 砖上传没有丢，只是搬走了。** `geometry/runtime.rs:40-113 sync_wall_bricks` 做 CPU 影子→稀疏 SSBO→`try_gpu_flush`，`asset_libraries/runtime.rs:68` 注册；`geometry/instanced_wall.rs:118-139` 有真实 `upload_to_storage_buffer`。因此 wall 目录的 `upload_instanced_wall_ssbo.rs` 缺失不应算“GPU 完全缺失”，但 wall-scope 的 debug viz/上传符号确实没对上。

---

## 6. 仍未复现清单（18 条，带证据）

1. **合并墙系统整条链缺失**。`merge_walls` 无实现；骨架签名依赖 `FinishWall`、`InnerWalls`、`InProgressWallResource`、`SnappingMode`、`ContinueWallOriginalState`、`MergeWallsCmd`、`PlayLineAcrossWall`、`History`、`PendingWallHistory`、颜色表。`draw_wall.rs:163-168`、`finish_wall.rs:72-79/129-136` 明确拒绝 `snap.mode > 1`。
2. **合并墙历史缺失**。`history_record_merge_walls` 为 no_trace；`history_recording.rs` 无合并记录路径；`resources/walls/decorator_storage.rs:743 on_walls_merged` 有消费者但无生产者。
3. **普通删除历史缺失**。`deferred_wall_history.rs:wall_deleted` no_trace；`RecordWallHistory::deleted_wall`（`history_recording.rs:35`）无调用者；只有 `finish_wall` 的无效继续路径记删除。
4. **墙高编辑 UI 整条模式缺失**。`ui_adjust_wall_height`（0x140a….）与 `ui_adjust_wall_height_mode` 系统无实现；`FinishAdjWallHeightMode`/`InitAdjWallHeightMode`/`AdjustWallHeightUi`/`AdjWallHeightModeCache`/`WallHeightSignifier` 全 crate 零引用。
5. **墙高历史缺失**。`history_record_adjust_wall_height` no_trace；`WallHeightHistoryEdit`（`resources/history/building_transactions.rs:23`）只有定义无生产者；`height_command.rs:1-3` 自述不记录历史。
6. **墙高 signifier 动画缺失**。`ui_adjust_wall_height_signifier_animation` no_trace；通用 `systems/ui/signifiers/animation.rs` 不是该 wall 系统。
7. **墙 splitter 工具缺失**。`wall_splitter` 3 个符号 no_trace；全 crate 无 splitter 系统；骨架参数含 `PublicWalls`/`UiSignifierStream`/`WallEraserArchivist`/`DecoratorStorage`/`History`/地形/颜色事件。
8. **`set_wall_style` 真实系统缺失**。仅有 `wall/mod.rs:20-39` 的 3 个 `_gap` 事件；骨架 `set_wall_style` 是 Query `CustomizationIcon`/`WallStyleIcon`/`FlagConnectionIcon`/`FreehandHeightTypeToggleIcon`/… 的大系统。
9. **signifier 实体/mesh 链缺失**。`snapping_maintain.rs:54-57/91`、`telegraph_walls_are_merged.rs:12-18/74-78/90-101`、`shape_placement_signifiers.rs:144-150` 均只写记录；`SignifierEntityBuilder`、`update_curve_mesh`、despawn/insert、`DrawTopology`、`DashedLineLength` 未恢复。
10. **dims/orientation widget 层缺失**。`ui_modes/edit_dims.rs:49` 空体；`ui_modes/edit_orientation.rs:16/23` 空体/仅常量；trigger volume widget 未恢复。
11. **组件注册面缺失**。`edit_arrow.rs:53-55`、`edit_line.rs:51-53`、`signifier_edit_dims_line.rs:47-49` 的 `register_required_components` 返回空切片。
12. **装饰器的 split/merge 迁移未接线**。`decorator_wall_events.rs:11-16/36` 明确只接通 `on_wall_removed`；`on_wall_split`/`on_walls_merged` 需子墙 id+u+0x28B 语义，未接。
13. **形状放置系统未接线**。`shape_placement_params`/`shape_placement_signifiers` 全 crate 无注册；`snap_to_roof_ridge`（`shape_placement.rs:164-274`）无调用者；Rapier raycast 分支未接（9-16）。
14. **形状编辑 UI 未接线**。`ui_edit_circle`、`ui_rotate_circle`、`ui_edit_rectangle_dims`、`ui_edit_rectangle_orientation`、`finish_edit_rectangle` 均无 schedule 注册；输入资源 `CircleCanvasInput`/`RectangleCanvasInput`/`RectangleDimsEditInput`/`RectangleOrientationEditInput` 无生产者（trigger volume/画布 action 缺失）。
15. **矩形创建输入不完整**。`ui_create_rectangle.rs:7` 投影 `0x140c99520` 待取证；`:114` 朝向硬编码 `get_rectangle_orientation(0.0)`；`:119` `previous_wall: None`；clamped 分支未建模。
16. **圆旋转屏幕↔世界角度缺失**。`ui_rotate_circle.rs:6` 明确 render/UI 域待接线；只有 CPU 角度应用 `rotate_circle_wall`。
17. **Debug 可视化系统缺失**。`debug_viz_threshold_curve`（5379764416）与 `spatial_hash_debug`（5378064608）无实现；均为 debug-only，不影响业务主链，但确属未复现。
18. **墙体完整 corner/skirt/signifier owner 缺失**。`construct_wall_bricks.rs:1-5` 自述：完整外部 corner/skirt/signifier owner 另属其他模块。
19. **擦除细节待取证**。`wall_eraser.rs:35` 的 stroke u 来源、`:8` 的 ChildWall split/rewire、`archivist.rs:25-33` 的 0xd8/0x80 载荷字节映射（以快照结构近似）。
20. **`calc_shape_height` 插值语义需人工**。`:103-146` 的双重 `max/HEIGHT_OFFSET` 与恒定 `t=1` 的 `lerp_rate`。

（以上 20 条，满足 ≥10。）

---

## 7. CPU / GPU / UI 真缺矩阵

| 层 | 已确认真实 | 缺失/仅 seam |
|---|---|---|
| **CPU 权威状态** | `InnerWalls` 高度写回、`PublicWalls` 发布、`WallSpatialHash` 增删查、`WallPublicationDirty`/事件、形状参数合同 | merge/split 拓扑；普通删除历史；墙高历史 |
| **CPU 几何** | 曲线预处理/衰减/平滑/阈值交点；圆/矩形曲线/围合；砖/拱/栅栏/门/平台楼梯；wall trigger volume（resources 域，被 wall 调用）；平台楼梯 Rapier collider | 完整 corner/skirt；merge/split 后几何；矩形投影 `0x140c99520`；圆形旋转角度源 |
| **CPU 历史** | 新建、删除（finish 路径）、擦除 shrink/split/full-erase、矩形 morph、墙扩展 pending、颜色 | 合并历史、普通删除历史、墙高历史、splitter 历史 |
| **GPU** | 砖实例 `InstancedWall`/`BrickTransformSsbo`/稀疏 SSBO/`sync_wall_bricks`（geometry 域）；RT 网格 `RtWorld` 更新；平台楼梯 collider 非 GPU | wall 目录内 `gpu_flush_brick_buffers` 符号；渲染 pass/着色器不在本范围；signifier mesh 未生成 |
| **渲染/UI widget** | 输入状态机（`ui_draw_wall` 大跳表、`ui_continue_wall`、`ui_create_circle/rectangle`、`ui_edit_*` 核心数学）；signifier **记录** | widget layer（2 处空体）、signifier 实体 spawn/despawn、mesh/材质、trigger volume、光标图标、装饰恢复、屏幕空间角度/投影 |

---

## 8. 完成度、依据与置信度

### 8.1 计算口径
- **文件覆盖**：57 骨架文件 → 51 有落点（含 5 合 1、1 搬移）；6 无落点（其中 `upload_instanced_wall_ssbo` 功能已搬到 geometry 域）。
- **函数覆盖**：137 骨架符号；剔除 29 closure + 4 serde 后 104 条业务符号；`in_file` 87 + `moved` 5 + `name_only` 2 = 94，扣除 `set_wall_style`（name_only 但未实现）与 `ui_adjust_wall_height_mode`（moved 仅注释）后约 **92/104 ≈ 88.5%**。
- **语义覆盖**：按上述矩阵加权；CPU 主干权重高，UI/渲染权重低。

### 8.2 结论
| 子域 | 完成度估计 | 依据 | 置信度 |
|---|---:|---|---|
| construction（bricks/arches/fences/gates/platform） | **82%–88%** | 15 文件均有实质算法；fences UNVERIFIED、gates pending、corner/skirt 缺 | 中 |
| shapes CPU（创建/编辑/合同） | **75%–82%** | 圆/矩形创建与编辑、ShapeParameters、lerp 真实；投影/触发体/部分输入缺 | 中 |
| snapping | **82%–88%** | 端点/段/初始化/音效/telegraph 真实；swizzle 与 host 实体链待复核 | 中-高 |
| eraser | **72%–80%** | 动画/archivist/历史/高度策略真实；stroke u、ChildWall rewire、载荷映射待取证 | 中 |
| wall_state（发布/空间/路径） | **85%–90%** | 发布、dirty、空间查询、path preparation 均为真实主体 | 中-高 |
| history | **72%–80%** | 主干真实；merge/delete/height 历史缺 | 中 |
| UI input/edit | **40%–50%** | 状态机/数学真实，但大量未注册、输入生产者缺失 | 中 |
| signifier/widget render | **15%–25%** | 只有记录写入；实体/mesh/widget 空或 host | 中-高 |
| GPU（本范围视角） | **55%–65%** | 砖上传搬移到 geometry 域且已注册；wall 内符号缺失、signifier mesh 缺 | 中 |
| **wall 综合** | **约 70%（65%–75%）** | 加权估算，不是机器可验证覆盖率 | 中 |

**置信度说明**：
- 文件/函数存在性与空壳定位：**高**（机械扫描 + 骨架签名）。
- 行为分类（真实/混合/壳）：**中**（深读 12 个文件 + 其余按函数体/注册表定向核对）。
- 语义等价性：**低-中**。本轮未运行 cargo test/clippy，未做机器码差分；测试多为结构/代理测试，不能证明与原版逐语句等价。

---

## 9. 与基线 STUBS-2026-10-19 的偏差（基线不可全信）

1. 基线“`systems/wall` 骨架函数 152、对得上 113、完全没迹象 23”与 `fn_audit.json`（137 条、in_file 87、no_trace 43）口径不同；差异主要来自 closure/泛型/serde 单态与“弱匹配”定义。
2. 基线把若干 `moved` 当作已对上，但本次发现 **`ui_adjust_wall_height_mode` 只在注释中出现**；自动 `moved` 会高估。
3. 基线“空壳 5 个”是整 crate；wall 内实际是 **4 个空/平凡体 + 3 个空组件注册 + 3 组 typed gap**，另有一批“函数存在但未注册/输入缺失”的系统；空壳数量不能代表真实缺口。
4. 基线未列出 `merge_walls`、`wall_splitter`、`set_wall_style`、墙高 UI/历史、普通删除历史、signifier 实体链等系统性缺口；本报告把它们单列。
5. 基线称“大部分函数已恢复（79%）”；若剔除 closure/serde 后 wall 的骨架落点约 89%，但**语义验收未做**；两者不能混用。

---

## 10. 需人工复核项（不编造结论）

1. `snapping_to_segments.rs:78-83` / `snapping/mod.rs:92-97` / `snapping_maintain.rs:68,93` 的 `(x,y,z)` swizzle 是否与原生调用点一致。
2. `calc_shape_height.rs:103-146` 的高度双重修正与 `lerp_rate` 无 dt 是否符合原机器码。
3. `snapping_to_ends.rs:126` 的 `mode.snapped = byte_0x29e != 0` 语义。
4. `wall_eraser.rs:35` 的 `wall_height_based_on_length` u/长度来源；`eraser/archivist.rs:25-33` 的载荷字节映射。
5. `shape_placement.rs:164-274` 的 `snap_to_roof_ridge` 未接线是宿主接线还是遗漏；`0x140980db0` 的多载荷记录是否还需额外字段。
6. `pending_changed.rs` 的重复 `PendingWallChanged` 是否应删除/合并（避免双源）。
7. 未注册的 UI 系统是否由上层 crate 注册；若是，需要在集成清单中登记，否则就是死代码。
8. `fences.rs`/`gates.rs` 的 UNVERIFIED/pending 状态需原指令差分后才能升级置信度。
9. `history_recording.rs:35 deleted_wall` 是否应由 `wall_deleted` 系统消费；当前是死代码 + 缺口。

---

## 11. 审计产物

- 本文件：`D:/game/reverse-engineering/tiny-glade/evidence/audit-2026-10-19/WALL-AUDIT.md`
- 机械统计：`D:/game/reverse-engineering/tiny-glade/evidence/audit-2026-10-19/fn_audit.json`
- 骨架：`D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/systems/wall`
- 实现：`D:/game/tiny-glade/crates/country-core/src/systems/wall`
