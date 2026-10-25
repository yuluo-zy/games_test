# country-core B：resources/walls/** 与 resources/roofs/** 逐文件还原度复核

- 日期：2026-10-19
- 范围：`D:/game/tiny-glade/crates/country-core/src/resources/walls/**`、`resources/roofs/**`，共 **72 个 .rs 文件 / 16,099 行 / 943 个 fn（含测试辅助）/ 114 个 `#[test]`**。
- 骨架：`D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/resources/{walls,roofs}`。机械台账：`evidence/audit-2026-10-19/fn_audit.json`（仅 31 个骨架文件、284 个具名函数落在本范围）。
- 方法：静态只读。逐文件通读 72 个文件；对照 `fn_audit.json` 的 `in_file/name_only/moved/va_only/no_trace`；对最大 12 个文件逐段抽读并核对注释中的 O/I/U 标记与机器地址。**未编译、未跑测试、未做与原指令的逐句差分**，因此“完成度”是静态保真度估计，不是语义验收。
- 约束：未修改任何 walls/roofs 源码；本文件是本次唯一新增产物。

## 0. 结论先行

- 范围内 **没有任何 `todo!()` / `unimplemented!()`**；主要“水分”不是 panic 占位，而是 `Gap` 空类型、`_marker: ()` / `_opaque: ()`、零返回访问器，以及“有类型无生产者/消费者”的接缝。
- 骨架具名函数 **284 个**：`in_file` 198、`name_only` 24、`moved` 13、`va_only` 3、`no_trace` 46（**83.8% 有名字或地址迹象**）。`no_trace` 46 个中 **24 个是 serde/hash/闭包单态化噪声**，真实未落名 **22 个**。
- 整文件未复现：`walls/wall_attachment.rs`、`walls/decorator_outline.rs`（骨架有、还原树无对应文件）。
- 最大缺口集中在：`material_stats`（18 个函数中 14 个无实现或空体）、`wall_attachment`（3 个真实函数）、`decorator_outline`（3 个闭包 + display）、`wall_trigger_volume` 的 `get_collision_geo/get_tris`、`RoofBuffers`/`RoofTile`/`DormerTiles`/`EmptyRoofTriggerVolume` 的数据面，以及屋顶两级 UV 索引与 PublicWalls 密集表两条跨域接缝。
- 字段级存在一处**明确冲突**：`publication_fields` 把公共墙记录当 `0x2a0`、shape tag `+0x258`、style `+0x299`；`color_selection` 的机器码注释则是 stride `0x2a8`、shape tag `+0x260`、style `+0x2a1`。两者至少一侧偏移/记录身份有误，**需人工裁决**。
- 综合估计：**约 66%**（结构/函数面约 84%，CPU 行为面约 75%，字段/序列化面约 55%，集成/GPU 消费面约 35%）。**置信度：中**（文件性质/占位定位为高置信；行为保真与原版等价性为中低置信）。

## 1. 文件级状态表

列说明：
- 性质：声明 / 数据合同 / 真实行为 / 混合。
- 骨架 fn：`可追溯/总数` = `in_file+name_only+moved+va_only` / 骨架具名函数数；“—”表示该文件不在 `fn_audit` 的 31 个骨架文件内（多为还原时拆分或从 systems 迁入）。
- 估计%：静态保真度，非等价验证；空壳/纯声明按文件职责给低分是刻意的。

### 1.1 walls/

| 文件 | 行 | 性质 | 骨架 fn | 测试 | 空壳 / 缺口 | 估计% |
|---|---:|---|---:|---:|---|---:|
| arches.rs | 32 | 声明（占位） | — | 0 | `Gap`：`:12,16,24,28`；无任何拱/过梁布局与行为 | 5 |
| cached_decorator_transforms.rs | 218 | 混合（行为+数据） | 5/5 | 4 | 双缓冲读/写册归属为 I（`:75`） | 88 |
| color_selection.rs | 610 | 真实行为 | — | 9 | `RoofColorComponentTables` 生产者缺失（`:70-73,225`）；记录偏移与 publication_fields 冲突 | 80 |
| curved_halftimber.rs | 12 | 声明（占位） | 1/1 | 0 | `Gap` `:12`；`remove_wall_id` 已搬到 systems/wall（非本文件） | 10 |
| decorator.rs | 1335 | 混合（行为为主，字段 U） | 45+5泛型+1/51 | 12 | `DecoratorInfo` 多字段 U；判别字节改存 dst_payload 高位（`:669-678`）；disc 2/3 未知（`:715-719`） | 86 |
| decorator/anchors.rs | 384 | 混合（行为，布局未验证） | — | 0 | 文件头自标 **UNVERIFIED**（`:1`）；bottom-door 消费者显式 gap（`:220-221`） | 70 |
| decorator/placement.rs | 269 | 真实行为 | — | 0 | 屋顶/角落/天窗附着选择未纳入本 API（`:2,57-59`） | 82 |
| decorator_archivist.rs | 667 | 混合（行为，collision 缺） | 12+4moved+1va+2no/19 | 7 | collider/mesh 重建未实现（`:400-403`）；before/after 配对为推断 U（`:240-243`） | 72 |
| decorator_archivist/changes.rs | 149 | 真实行为 | — | 0 | 与 DecoratorInfo 72B 原始字节强耦合（`:1295-1296`） | 85 |
| decorator_collision.rs | 13 | 声明（cfg 资源） | — | 0 | 仅 `native-physics` 下两张 map | 60 |
| decorator_storage.rs | 1869 | 真实行为 | 40+4泛型/44 | 22 | 字段更新尾部为 U 近似（`:947-995`）；`verify_integrity` 空体为原作空函数（`:1319`）；wire 格式 U（`:30`） | 90 |
| decorator_subtype.rs | 339 | 混合（谓词真，网格 U） | 16+moved+name/18 | 4 | `DerivedDecoratorInfo` 字段语义 I/U（`:100-113`）；碰撞/交互网格数据 U（`:149-164`）；outline 专项文件缺失（`:197-198`） | 60 |
| decorator_subtype/placement.rs | 247 | 真实行为 | — | 0 | 角落子类型 3 的视觉/洞口消费者仍未解（`:692-693`） | 82 |
| dormers.rs | 347 | 数据合同（GPU SSBO） | 3/3（name_only） | 4 | 12×u32 洞记录语义未解（`:8-11`）；骨架的 cottage/dummy/gothic 已迁到 `systems/decorator` | 75 |
| elevation_support_stats.rs | 16 | 声明（占位） | — | 0 | `Gap`：`:12,16` | 5 |
| freeform_height_cache.rs | 338 | 真实行为 | — | 0 | 输入位置/计数生产者属调用方（`:43-47`） | 85 |
| gable_heights.rs | 127 | 真实行为 | — | 0 | 无独立测试；公式/常量以机器码注释支撑 | 88 |
| height_cache.rs | 89 | 真实行为 | — | 0 | gable 前置 profile 单独处理（`:391-392`） | 85 |
| height_rules.rs | 66 | 真实行为 | — | 0 | special flag 原字段名未知（`:265-267`） | 90 |
| in_progress_wall.rs | 23 | 数据合同（元数据投影） | 1/1（name_only） | 0 | 曲线与 wall id 归 `InProgressCurve`（`:1-2,23`），本文件仅后置字段 | 50 |
| inner_wall_state.rs | 340 | 混合（行为，字段 U） | 7+3泛型/10 | 4 | `StructureType` 字节布局 U（`:28-35`）；source +0x65 名未解（`:216-220`）；+0x69 U（`:154`）；曲线点不可由 0x70B 重建（`:155-178`） | 86 |
| inner_walls.rs | 386 | 真实行为 | 11+1va/12 | 6 | 记录内曲线点同样丢失（转调 `from_record_bytes`） | 90 |
| material_stats.rs | 24 | 占位空壳 | 4+5name+9no/18 | 0 | `Gap raw` `:12`；4 个空体 `:17,19,21,23`；9 个 no_trace + 5 个 name_only 未实现 | 10 |
| mod.rs | 114 | 混合（声明+行为） | 3+2moved+2泛型/7 | 0 | `HighlightColor::to_rgb`、`split_height_into_skirts` 为真实行为 | 88 |
| plaster.rs | 29 | 声明（占位） | moved+name/2 | 0 | `Plaster`/`AnimatedPlasterPeel` 均 `Gap`（`:13,17`）；命令仅名字 | 20 |
| public_wall_state.rs | 292 | 真实行为 | 7/7 | 0 | 高度缓存缺失时按原版 panic（`:32-33,197-207`） | 88 |
| public_walls.rs | 363 | 真实行为（含跨域接缝） | 4/4 | 3 | 密集表 `+0x1a8` 等未建模，`roof_component` 用 Query 回退（`:4-15,113-135`） | 85 |
| publication_fields.rs | 137 | 数据合同（字节投影） | — | 0 | `0x2a0` 记录、shape `+0x258`、style `+0x299`；`+0x284/+0x288/+0x294/+0x296/+0x29a/+0x29e` 语义 U（`:109-133`） | 55 |
| publication_source.rs | 238 | 真实行为 | — | 0 | terrain sampling-fix 全局默认未推断（`:13-15`） | 88 |
| saved_wall_state.rs | 337 | 混合（serde+行为） | — | 3 | 语义 JSON 投影，非原生存档格式（`:1-2,126-127`） | 90 |
| shape_alignment.rs | 99 | 数据合同（宿主容器） | — | 2 | 填充所需 `InnerWalls+8/+0x28/+0x30/+0x40` 未建模（`:26-34`） | 65 |
| spatial_index.rs | 157 | 真实行为 | — | 0 | 无独立测试；网格/分段算法按机器码重写 | 85 |
| wall_holes.rs | 70 | 混合（包装+容器） | 4+2name+moved+6no/13 | 0 | `as_finalized/new_finalized/holes_no_duplicates` 未落名；行为下沉到 geometry.rs | 75 |
| wall_holes/geometry.rs | 238 | 真实行为 | — | 0 | 哈希用 std DefaultHasher 复刻原 SipHash 字段序 | 88 |
| wall_holes/opening_rules.rs | 103 | 真实行为 | — | 0 | Gothic 三矩形常量表完整；无测试 | 85 |
| wall_path_segment.rs | 27 | 占位空壳 | 3/3 | 0 | `gap: Gap` `:11`；三个访问器恒返回 0/0.0（`:15-27`） | 5 |
| wall_stone_columns.rs | 41 | 混合（数据+行为） | 1/1 | 0 | `StoneColumn` 仅 handle u64，布局 I（`:18-22`） | 75 |
| wall_trigger_volume.rs | 246 | 真实行为（部分缺） | 3+va+moved+name+2no/8 | 0 | `get_collision_geo`、`get_tris` 未落名；墙版 `get_position_from_mesh_uv` 缺失 | 78 |
| wooden_gates.rs | 27 | 数据合同 | 1/1 | 0 | 值元素仅 u64 实体引用 | 85 |

### 1.2 roofs/

| 文件 | 行 | 性质 | 骨架 fn | 测试 | 空壳 / 缺口 | 估计% |
|---|---:|---|---:|---:|---|---:|
| associations.rs | 107 | 真实行为 | — | 0 | Query 顺序/过滤归 Bevy（`:67-68`） | 85 |
| commands.rs | 102 | 声明（事件合同） | 1/1（name_only） | 0 | 高亮事件首 qword 语义 U（`:79-94`）；事件处理不在本文件 | 70 |
| components.rs | 176 | 混合（类型+动画行为） | — | 0 | `EmptyRoofTriggerVolume`（`:32`）无几何；创建路径只产出空体积（`:333-348`） | 75 |
| creation.rs | 79 | 真实行为 | — | 0 | 外观/样式字段直接拷贝，不猜默认（`:39-42`） | 85 |
| customization.rs | 146 | 真实行为 | — | 0 | History/Arc 容器边界外置 | 88 |
| deletion.rs | 38 | 真实行为 | — | 0 | 无测试 | 88 |
| direction.rs | 44 | 真实行为 | — | 0 | 无测试 | 88 |
| dormer_tiles.rs | 13 | 声明（占位） | — | 0 | `_marker: ()` `:12`；原值语义待接线（`:11`） | 5 |
| geometry/edit_rules.rs | 330 | 真实行为 | — | 0 | M1 通知条件按 reviewer 裁决，非直接单解（`:251-258`） | 88 |
| geometry/mod.rs | 18 | 声明 | — | 0 | 模块导引；说明从 system-roof 归位（`:1-8`） | — |
| geometry/parameters.rs | 74 | 数据合同（适配） | — | 0 | 仅替换已确认序列化字段；rim/padding 保留（`:23-24`） | 80 |
| geometry/pipeline.rs | 69 | 真实行为（CPU 组合） | — | 0 | 明确无 ECS/回放/GPU（`:1`） | 85 |
| geometry/rectangular_caps.rs | 173 | 真实行为 | — | 0 | 无测试；常量/分支按机器码注释 | 85 |
| geometry/rectangular_edges.rs | 142 | 真实行为 | — | 0 | 无测试 | 85 |
| geometry/rectangular_faces.rs | 287 | 真实行为 | — | 0 | 无测试；行插值/seam 平面完整 | 85 |
| geometry/replay.rs | 299 | 真实行为 | — | 0 | 回放游标/ECS 调度外置（`:1-2`） | 85 |
| geometry/ridge.rs | 191 | 真实行为 | — | 0 | 无测试；`get_inner` 地址错标已更正（见 roof_ridge） | 88 |
| geometry/surface.rs | 296 | 真实行为 | — | 0 | Curve2 不保留原分配器身份（`:193-195`） | 90 |
| geometry/tiles.rs | 473 | 混合（行为+记录字节 U） | — | 0 | 64B 记录中 `0x10..0x18` 硬编码 `0x7bff00003c000000`（`:144`）；`0x38..` 未观测；矩形 tip 中心用 `*0.0`（`:442`，需人工确认是否原式） | 82 |
| invalid_notifications.rs | 46 | 真实行为 | — | 0 | History 提交边界外置 | 85 |
| lifecycle.rs | 55 | 真实行为（编排） | — | 0 | 命令冲刷/调度显式外置（`:1-2`） | 85 |
| mod.rs | 74 | 声明（注册） | — | 0 | `register_roof_events` 只是事件资源注册 | 90 |
| roof_animation.rs | 42 | 混合（类型+行为） | 2+moved+name/4 | 0 | 行为委托 components::RoofAnimation | 85 |
| roof_buffers.rs | 215 | 混合（数据合同+opaque 资源） | — | 1 | `RoofBuffers { _opaque: () }`（`:11-15`）；SupportBeam tail 8B 不透明（`:83-93`） | 65 |
| roof_noise.rs | 169 | 真实行为 | 2/2 | 4 | 机器数值对照测试（`:119-134`） | 92 |
| roof_ridge.rs | 135 | 真实行为 | 3/3 | 4 | 数值实现在 geometry/ridge；`get_inner` 原地址错标已更正（`:43-45`） | 90 |
| roof_shape.rs | 1419 | 混合（行为+serde） | 15+2name+3泛型/20 | 21 | 两级 GlobalId UV 索引仅有 trait，实际回退全槽射线（`:693-746`）；cap 常量经 reviewer 裁决（`:235-250`）；inactive payload/padding 无 wire 表示（`:526-528`） | 85 |
| roof_tile_sim.rs | 39 | 真实行为（数据） | 1/1 | 0 | 行为薄，状态在 components | 85 |
| roof_tile_ssbo.rs | 2 | 声明（alias） | — | 0 | 仅 `pub use` 兼容路径 | 100（按设计） |
| roof_tiles.rs | 53 | 混合（数据合同+占位） | 1+name/2 | 0 | `RoofTile { _marker: () }`（`:22`）；`RoofTiles.allocation` 默认 None，装配/U仍缺 | 55 |
| roof_trigger_volume.rs | 62 | 真实行为 | 4+moved+name/6 | 0 | 仅几何容器；生产者在外部 | 85 |
| roof_visual_shape.rs | 223 | 真实行为 | 4/4 | 4 | 圆高度基 8.0 已纠正（`:73-78`） | 88 |
| spatial.rs | 90 | 真实行为 | — | 0 | 关联 48B 组件仅字段投影（`:1054-1055`） | 85 |

## 2. 空壳与水分（文件:行）

### 2.1 `todo!` / `unimplemented!`
- **0 处**（72 个文件全文扫描，`todo=0, unimplemented=0`）。

### 2.2 `Gap` 空类型（`lib.rs:23` 的 `pub struct Gap;`）
- `walls/arches.rs:12,16,24,28`：`ArchSegment/ArchSegments/LintelSegment/LintelSegments` 全部无布局。
- `walls/curved_halftimber.rs:12`：`CurvedHalfTimberDrawables`。
- `walls/elevation_support_stats.rs:12,16`：`ElevationSupportStats/ElevationSupports`。
- `walls/material_stats.rs:12`：`MaterialStats.raw`。
- `walls/plaster.rs:13,17`：`Plaster`、`AnimatedPlasterPeel`。
- `walls/wall_path_segment.rs:11`：`WallPathSegment.gap`。

### 2.3 空体 / 零返回 / 只默认值
- `walls/material_stats.rs:17,19,21,23`：4 个方法体为 `{}`，参数被忽略。
- `walls/wall_path_segment.rs:15-17,19-21,23-25`：`pt_count()` 恒 0、`length_ws()`/`length_ws_2d()` 恒 0.0。
- `roofs/dormer_tiles.rs:11-12`：`_marker: ()`，类型占位。
- `roofs/roof_tiles.rs:22`：`RoofTile { _marker: () }`，无业务状态。
- `roofs/roof_buffers.rs:14`：`RoofBuffers { _opaque: () }`，原字段未恢复。
- `roofs/components.rs:32,116,134`：`EmptyRoofTriggerVolume` 单元类型；创建路径只放入空值。
- `walls/decorator_storage.rs:1319`：`verify_integrity` 空体——注释明确是原作 `ret` 空函数，**不计缺口**。
- `roofs/commands.rs` 全部为事件类型声明（合法数据合同，不是空壳）。

### 2.4 注释明确标 占位/待取证/待接线/未实现
- `walls/decorator.rs:125,139,167,196,212,444,504,523,560-561,716,780`（U/I：Transient 生成器所有权、DecoratorLocator 字段、ClutterVersion latest、Dst tag0/1、DecoratorInfo 逐字段、disc 2/3）。
- `walls/decorator_storage.rs:30`（wire 格式 U）、`:574`（removed 注册表取/留语义 I）、`:947-948`（尾部字段 U 近似）。
- `walls/decorator_subtype.rs:27,102,111,131,149,161,271`（迭代器、DerivedDecoratorInfo、alt、碰撞/交互网格、From 目标类型）。
- `walls/inner_wall_state.rs:28,35`（StructureType 布局/判别字节）、`:154`（+0x69 U）、`:216-220`（+0x65 原名未解）。
- `walls/publication_fields.rs:109,130`（+0x284/+0x29e 语义待取证）。
- `walls/cached_decorator_transforms.rs:75`（读/写册归属 I/U）。
- `roofs/commands.rs:79,85,94`（高亮事件首 qword U、便捷构造 I）。
- `walls/decorator/anchors.rs:1`（整文件自标 UNVERIFIED）。
- `walls/public_walls.rs:15`（密集表未落，登记 GAP）。
- `roofs/components.rs:29-32`（三角形/法线/UV 元素类型“deliberately not fabricated”）。
- `roofs/roof_buffers.rs:11`（其余字段仍待原生产者/消费者）。

## 3. 对照骨架：no_trace 清单与分类

`no_trace` 定义：`fn_audit.json` 中全 crate 既无函数名、也无原作地址引用。范围合计 **46**。

### 3.1 单态化/派生噪声（24，非业务缺口）
- `roofs/roof_shape.rs`（3）：`Value>`、`Error>`、`Global>`（serde 投影 impl）。
- `walls/decorator.rs`（5）：`Error>`、`Value>`、`StrRead>`、`Serializer>`、`DefaultHasher>`（serde/hash）。
- `walls/decorator_storage.rs`（4）：`Value>`、`Error>`、`Global>`、`Serializer>`。
- `walls/inner_wall_state.rs`（3）：`Value>`、`Serializer>`、`Global>`。
- `walls/mod.rs`（2）：`Error>`、`Global>`。
- `walls/wall_attachment.rs`（4）：`Error>`、`Global>`、`Serializer>`、`Value>`。
- `walls/wall_holes.rs`（3）：`HoleType>`、`Global>`、`closure_env$0>`。
> 这些是泛型实例化/派生符号，不应计为未恢复业务函数。

### 3.2 真实未落名（22）与判定

| 骨架文件 | 函数（地址） | 判定 | 证据 |
|---|---|---|---|
| material_stats.rs | `set_pillar_placements` 0x140a88320、`set_lintel_pillars_count` 0x140a88420、`set_arch_pillars_count` 0x140a88430、`add_lintels_length` 0x140a88450、`add_sewer_length` 0x140a88540、`set_lintels_segments_count` 0x140a885e0、`set_sewer_segments_count` 0x140a88610、`set_fences_segments_count` 0x140a88620、`set_fences_segments_seed` 0x140a88630 | **真缺失（该文件面）** | 全 crate 无同名定义；`material_stats.rs` 只有 4 个空体。相关投影另写在 `systems/wall/construction/segmentation.rs:413-448`，但未接入此类型 |
| wall_attachment.rs | `is_attached` 0x140b256b0、`from_rectangle_corner_id` 0x140b256c0、`to_rectangle_corner_id` 0x140b256f0 | **真缺失（整文件未复现）** | 还原树无 `wall_attachment.rs`；`decorator/anchors.rs` 只覆盖锚点高度/坐标，未提供 corner-id 互转 |
| decorator_outline.rs | `closure$0` 0x140b34640、`closure$1` 0x140b348e0、`closure$2` 0x140b34c40 | **真缺失（整文件未复现）** | 还原树无 `decorator_outline.rs`；`decorator_subtype.rs:197-198` 只注明“专项在文件外” |
| decorator_archivist.rs | `decorator_colliders` 0x140005c60、`decorator_interactions` 0x1403c03c0 | **改名/未落名（疑似吸收）** | 结构体已有 `collision_a/collision_b`（changes.rs:1318-1319），但无对应访问器；collision 语义域在 native-physics 下 |
| wall_holes.rs | `holes_no_duplicates` 0x140f86cd0、`new_finalized` 0x140f86e50、`as_finalized` 0x140f86eb0 | **疑似改名/吸收（需人工）** | 等价行为分散在 `geometry.rs:91-101,169-237`（`new_in_progress`/`finalize`/`finalized_holes`），签名不同 |
| wall_trigger_volume.rs | `get_tris` 0x1408e80f0、`get_collision_geo` 0x1408e8d70 | **`get_tris` 疑似=`default_tris` 改名；`get_collision_geo` 真缺失** | `default_tris` 0x1408e80e0 已实现（wall_trigger_volume.rs:40-42）；无碰撞几何构造入口 |

### 3.3 name_only（24）中的“真名但异处/胶水”
- 已搬移/异处实现（非缺口）：
  - `material_stats::add_stairs_length` → `systems/wall/construction/segmentation.rs:427`。
  - `walls/dormers::{cottage,dummy,gothic}` → `systems/decorator/src/utils/mod.rs:31,37,58`（注意：resources/walls/dormers.rs 当前是 DormerHole SSBO，不是这些构造器）。
  - `curved_halftimber::remove_wall_id` → `systems/wall/wall_state/wall_spatial_hash.rs:86`。
- 派生/Bevy 胶水（不计业务）：`fmt`、`eq`、`register_required_components`、`default`（多处）。
- 仍真实缺失/未接：`material_stats::{init_this_update,max_pillar_movement,which_material_changed_the_most,set_path_segmentation_count}`、`wall_trigger_volume::get_position_from_mesh_uv`（墙版）、`wall_holes::{clone_and_reopen,wall_length}`（有等价但未同名）。

## 4. 抽读最大 12 文件的判定（结论）

抽读顺序按行数：`decorator_storage(1869)`、`roof_shape(1419)`、`decorator(1335)`、`decorator_archivist(667)`、`color_selection(610)`、`geometry/tiles(473)`、`inner_walls(386)`、`decorator/anchors(384)`、`public_walls(363)`、`dormers(347)`、`inner_wall_state(340)`、`decorator_subtype(339)`；另读了 `freeform_height_cache(338)`、`saved_wall_state(337)` 等。

1. **WallState/InnerWallState 字段**：`WallState` 是 `WallAuthoritySnapshot` 包装。已证偏移：`+0x54` height、`+0x58` terrain_floor、`+0x5c` arch scalar、`+0x60` plaster bits、`+0x64` arch_type、`+0x65` source flag、`+0x66` freehand、`+0x67` supports、`+0x68` style（inner_wall_state.rs:148-158）。`+0x65` 原字段名未解；`+0x69` 语义 U。`from_record_bytes` **明确丢失 Curve2 点列**（`:155-178`），只保留 length，这是存档/加载链的真实数据缺口，不是实现细节。
2. **publication_fields**：`PublicWallBuildingState` 是 0x2a0 字节投影（`:70-73`），shape 在 `0x258..0x274`（tag 首 u32，`:103`），style 在 `+0x299`（`:107`）。`+0x284/+0x288/+0x294/+0x296/+0x29a/+0x29e` 全标 U/待取证（`:109-133`）。它**不是**原 Rust 类型 ABI，只是被读字段投影。
3. **publication_fields 与 color_selection 偏移冲突**：`color_selection.rs:47,50,82-92` 依据机器码写 stride `0x2a8`、shape tag `+0x260`、style `+0x2a1`；`publication_fields.rs:70-73,103,107` 写 `0x2a0/0x258/0x299`。两套值不可能同时正确；若两者指向同一记录，则至少一侧记录起点或字段偏移错 **0x08**。**需人工用调用点复核**。
4. **decorator/DecoratorInfo**：谓词位运算、id/rank/type 访问、事件队列均为真实实现（decorator.rs:24-64,821-918）。但 72B 记录中 `u18,u24,u28,u30,u38,u3c,u40,u45` 及 state 的完整语义仍 U（`:514-562`）。`extra_disc` **不是原生 +0x11**，而是把判别移到 `dst_payload` 最高字节（`:669-678,755-757`），文档明说是为避免载荷往返损坏；这属于已登记偏离，不能当作等价 ABI。`DecoratorDst` tag0/1 差异 U（`:444`）。
5. **decorator_subtype**：`can_be_on_corner` 已按机器码改为 `0x91008` 位测试（`:32-39`），`DecoratorMeshLibrary` 四表/插入/查询为真实行为（`:211-268`）。但 `DerivedDecoratorInfo` 的 bbx/alt_subtype 语义 I/U（`:100-113`），`collision_mesh`/`interaction_mesh` 只是 bbox→Rectangle3d 近似，原网格数据未取证（`:149-164`）。`decorator_outline` 专项完全缺席。
6. **wall_trigger_volume**：四角顺序、freehand 双面反向绕序、special 字节、`0.315` 偏移、三角重算均为真实行为（:1-5,124-234）。但原 `get_collision_geo`/`get_tris` 未落名；墙版 `get_position_from_mesh_uv` 缺失；碰撞消费受 `native-physics` 门控。
7. **roof geometry/tiles**：圆形/矩形面、边、脊帽、随机分割、记录打包均是真实算法，保留 SSE/NaN 运算序（tiles.rs:22-109,201-310; rectangular_faces.rs:169-287; caps/edges）。但 64B 记录中 `0x10..0x18` 是硬编码 `0x7bff00003c000000`（tiles.rs:144），`0x38..0x40` 等从未解释；`rectangular_context` 的 `local_tip_center` 用 `*0.0`（`:442`），需人工确认是原式还是退化占位。`pipeline.rs` 只做 CPU 组合，明确无 ECS/GPU。
8. **structural graph**：本范围没有结构图实现；墙体侧只有 attachment anchor 数学（decorator/anchors.rs:147-248）和子类型/洞口规则。结构图的 `register_edge_index` 在范围外的 `resources/stairs/structural_graph.rs:220`，其精确类型/来源仍“待取证”；`wall_attachment.rs` 的矩形角 id 互转缺失，使墙体附件到结构图的键链未闭。
9. **Roof shape 的两级 UV 查询**：`RoofUvColliderIndex` 只固定两级表语义（roof_shape.rs:658-675）；实际 `roof_uv_and_point_indexed` 在查表后仍用 `RaycastWorld::raycast(ws_pos,-Y,u32::MAX)` 全槽回退（`:693-722`），没有 ColliderSet 句柄选择。这是明确的跨域未接线，不是优化。
10. **color_selection**：`get_building_and_roof_colors` 的控制流、panic 点、NaN 语义均按 fresh 反编译重建（`:271-324`），并有 9 个测试。但 `RoofColorComponentTables` 的生产者未实现（`:70-73,225-232`），意味着该函数在真实运行路径上拿不到屋顶颜色/外观表。
11. **decorator_archivist**：`extract_diff`/`extract_and_restore_to_storage`/`archive_*` 真实存在，且会 drain/take 并断言（`:301-457`）。但 collision collider/mesh 重建未实现（`:400-403`），`extract_before_after_pair` 自标“登记 U，非判等”（`:240-243`），before/after 配对是推断。
12. **saved_wall_state**：字段名/默认值/别名/重复字段拒绝与 3 个 fixture 测试一致（`:83-233`）；但它是**语义 JSON**，不是原生存档字节格式（`:1-2,126-127`），`structure_ty` 复用 `clutter_version` 字段的做法注释自认“字段名称错误，待协同重命名”（`:219-221`）。

## 5. 字段级未知（U/待取证）

| 位置 | 未知内容 |
|---|---|
| `walls/decorator.rs:535-562` | `DecoratorInfo` 的 `u18/u24/u28/u30/u38/u3c/u40`、`u45`、`state` 完整语义；+0x48.. 包裹记录边界 |
| `walls/decorator.rs:444,504` | `DecoratorDst` tag 0 与 tag 1 的原子/复合差异；复合墙载荷语义 |
| `walls/decorator.rs:669-678,715-719` | `DecoratorExtraInfo` 判别 2/3 未识别；判别字节存储位置已偏离原生 +0x11 |
| `walls/decorator.rs:95-99,125,139,212` | `DecoratorIdGen` 结构/`TransientDecoratorIdGen` base 所有权、构造初值、`ClutterVersion::latest()` |
| `walls/decorator_storage.rs:30` | `DecoratorStorageAddr` 的 wire 格式 |
| `walls/decorator_storage.rs:574,947-948` | removed 注册表取/留语义；change 尾部字段（op blob +0x60..）与 Rust `u40/u45` 的对应 |
| `walls/decorator_subtype.rs:100-113` | `DerivedDecoratorInfo` 的 bbx 语义、`alt_subtype` 精确字段 |
| `walls/decorator_subtype.rs:149-164` | collision/interaction 网格数据（当前仅 bbox 近似） |
| `walls/anchors.rs:130-131` (decorator/anchors.rs) | raw curve length 与 published length 的双长度合同；bottom-door 垂直 profile 未解 |
| `walls/inner_wall_state.rs:28-35` | `StructureType` 字节 0 的字段布局/判别语义 |
| `walls/inner_wall_state.rs:154,216-220` | WallState +0x65 原名、+0x69 metadata 字节 |
| `walls/inner_wall_state.rs:155-178` | 0x70B 记录内 Curve2 点列不可重建（存档缺口） |
| `walls/publication_fields.rs:109-133` | +0x284/+0x288/+0x294/+0x296/+0x29a/+0x29e |
| `walls/publication_fields.rs:70-73 vs color_selection.rs:82-92` | 记录大小及 shape/style 偏移冲突（0x2a0/0x258/0x299 vs 0x2a8/0x260/0x2a1） |
| `walls/material_stats.rs:11` | 原 64B 统计块字段布局 |
| `walls/height_rules.rs:265-267` | special flag 的原字段名/来源 |
| `walls/wall_stone_columns.rs:18-22` | `StoneColumn` 除 handle 外的布局 |
| `roofs/roof_shape.rs:526-528,617` | RoofShapeParams/FlatRoofStyle 的 inactive union payload、padding（无 wire 表示，宿主初始化 0 非原作合同） |
| `roofs/roof_shape.rs:310-336` | `FlatRoofStyle::pattern_id()` 读 material 字节的遗留观测 vs 原生 pattern_id u32@+8 |
| `roofs/geometry/tiles.rs:125-157` | 64B RoofTileRecord 的 `0x10..0x18`（硬编码）、`0x38..0x40` 等字节 |
| `roofs/geometry/tiles.rs:442` | 矩形 `local_tip_center` 的 `*0.0` 是原式还是占位（需人工） |
| `roofs/roof_buffers.rs:11-15,83-93` | `RoofBuffers` 其余字段；`RoofSupportBeamSsbo` tail 8B |
| `roofs/commands.rs:79-94` | Roof 高亮事件首 qword |
| `roofs/dormer_tiles.rs:11-12` | DormerTiles 原始值语义 |
| `roofs/dormers.rs`(walls/dormers.rs):8-11 | DormerHoleSsbo 12×u32 洞记录 |

## 6. 数据格式未定

- `DecoratorStorageAddr` 的序列化/wire 格式（decorator_storage.rs:30）。
- `DecoratorInfo` 归档是 72B 原始字节 + `ptr::read_unaligned` 强转（decorator_archivist.rs:158-177，anchors.rs 自标 UNVERIFIED）；字段级布局未经完整机器码验证。
- WallState 原生 0x70B 记录只恢复 length，曲线点列无法从记录重建；原生存档二进制格式未恢复，当前是语义 JSON（saved_wall_state.rs:126-127）。
- `RoofShapeParams` 28B 有两套适配（roof_shape.rs 的 `RoofShapeParams` 与 geometry/parameters.rs 的 `RoofParameterBlock`），inactive 字节/rim/padding 无 wire 合同。
- 64B RoofTileRecord：`0x10..0x18` 硬编码、`0x38..0x40` 等未解释；圆形/矩形共用布局但字段语义未全表化。
- DormerHoleSsbo 的 12 个 u32 语义未定。
- `RoofTriggerVolume` 的三角形/法线/UV 元素类型与库容器 ABI 明确“不编造”（components.rs:29-32）。
- `PublicWallBuildingState` 是字节投影而非原 Rust ABI；0x2a0/0x2a8 冲突未裁。
- `DecoratorOpeningInput` 的 `size_variant` 只接受 1/2/3，Gothic 常量表按单一精度字面量复刻，但整体资产 schema 未恢复。

## 7. GPU / 渲染消费缺失

1. **屋顶瓦片无上传/ECS 装配**：`geometry/pipeline.rs:1` 明确“No ECS, replay or GPU execution”；`RoofTiles.allocation` 默认 `None`（roof_tiles.rs:937-942），`draw_ranges` 有查询面但范围内没有把 `RoofTileRecord` 写入 `SparseInstanceBuffer` 的装配调用。
2. **RoofBuffers 资源不可用**：`RoofBuffers { _opaque: () }`（roof_buffers.rs:14）；原字段、生产者、消费者均未恢复。`WoodenPlankSsbo`/`RoofSupportBeamSsbo` 只是记录合同，没有从门/半木构到记录的生成链。
3. **屋顶触发体积为空**：`prepare_resolved_roof_components` 只返回 `EmptyRoofTriggerVolume`（components.rs:333-348）；没有三角形/法线/UV 生产路径，故创建后的屋顶没有可用触发体积。
4. **墙体碰撞消费受门控**：`wall_trigger_volume` 的 quads/tris 在 CPU 侧重算，但 collision/GPU 消费依赖 `native-physics`；decorator 碰撞体重建未实现（decorator_archivist.rs:400-403），archivist 的 collision_a/b 仅在 native-physics 下存在（:40-50）。
5. **color/roof 密集表生产者缺失**：`PublicWalls` 未建模 `InnerWalls+0x1a8` 密集表（public_walls.rs:4-15）；`RoofColorComponentTables` 无生产者（color_selection.rs:225-232）。屋顶颜色/外观在真实路径上无法解析。
6. **屋顶 UV 命中链回退**：两级 GlobalId/collider 表未实现，退化为全槽射线（roof_shape.rs:693-746）；`ColliderSet` 句柄选择未接入。
7. **Dormer SSBO**：DormerHole/Count 缓冲类型与构造已恢复并有 GPU 假设备测试（dormers.rs:109-149），但**天窗洞生成/写回/绘制**不在本范围，未见生产者。
8. **壁洞/灰泥/拱的渲染**：`MaterialStats`、`Plaster`、`ArchSegments` 等资源是 Gap/空壳，渲染系统读不到真实字段；`system_wall_constructor` 的写回链未接到这些类型。
9. **高亮**：`HighlightRoofCmd`/`HighlightColor` 有数据面，但渲染/高亮系统消费未在范围内验证。

## 8. 仍未复现清单（≥10，带证据）

1. `walls/material_stats.rs` 的 18 个骨架函数中：9 个 no_trace + 5 个 name_only 无同名实现，4 个为 `{}` 空体（`:17-23`）。`systems/wall/construction/segmentation.rs:413-448` 只有另一套投影，未接入本类型。
2. `walls/wall_attachment.rs` 整文件缺失（骨架 1925 行 / 9 个具名函数）：`is_attached`、`from_rectangle_corner_id`、`to_rectangle_corner_id` 全 crate 无定义。
3. `walls/decorator_outline.rs` 整文件缺失（骨架 1304 行 / 4 个具名函数）：3 个 closure + `display` 未复现；`decorator_subtype.rs:197-198` 只留“专项在文件外”。
4. `walls/wall_holes.rs` 的三个函数面 `as_finalized`/`new_finalized`/`holes_no_duplicates` 未按原名实现；等价逻辑在 `wall_holes/geometry.rs:91-101,169-237`，签名/生命周期不同，需人工确认是否可接受。
5. `walls/wall_trigger_volume.rs`：`get_collision_geo` 0x1408e8d70 无实现；`get_tris` 只有 `default_tris` 近似（`:40-42`）；墙版 `get_position_from_mesh_uv` 缺失。
6. `walls/plaster.rs`：`Plaster`/`AnimatedPlasterPeel` 均为 `Gap`（`:13,17`），无字段、无动画/剥离行为；仅有事件名。
7. `walls/arches.rs`：拱/过梁四类型全部 `Gap`（`:12,16,24,28`），`ArchSegment`/`LintelSegment` 布局未复现。
8. `walls/wall_path_segment.rs`：`WallPathSegment` 为 `Gap`，三个长度/计数访问器恒零（`:11,15-27`）。
9. `walls/elevation_support_stats.rs`：两个资源均 `Gap`（`:12,16`），audio 消费面拿不到统计。
10. `walls/curved_halftimber.rs`：`CurvedHalfTimberDrawables` 为 `Gap`（`:12`），半木构绘制表未恢复。
11. `roofs/roof_buffers.rs`：`RoofBuffers` 仅 `_opaque`（`:14`）；`RoofSupportBeamSsbo` 尾部 8B 不透明（`:83-93`）。
12. `roofs/roof_tiles.rs`：`RoofTile` 仅 `_marker`（`:22`）；`RoofTiles.allocation` 默认 None，无装配/上传链。
13. `roofs/dormer_tiles.rs`：`DormerTiles` 仅 `_marker`（`:12`）。
14. `roofs/components.rs`：`EmptyRoofTriggerVolume`（`:32`）使创建路径无触发体积几何（`:333-348`）。
15. `roof_uv_and_point_indexed`：两级 GlobalId/collider 表只有 trait，实际全槽射线回退（roof_shape.rs:658-722）。
16. `PublicWalls` 密集屋顶/墙记录表未建模，`roof_component` 用 `Query<&Roof>` 回退（public_walls.rs:113-135）。
17. `RoofColorComponentTables` 无生产者（color_selection.rs:225-232），屋顶颜色选择在真实运行路径缺输入。
18. DecoratorArchivist 的 collider/mesh 重建未实现（decorator_archivist.rs:400-403）；`decorator_colliders`/`decorator_interactions` 无访问器。
19. `DerivedDecoratorInfo` 的真实碰撞/交互网格未恢复，当前 `collision_mesh`/`interaction_mesh` 是 bbox 近似（decorator_subtype.rs:149-164）。
20. `publication_fields` 与 `color_selection` 的公共墙记录偏移冲突（0x2a0/0x258/0x299 vs 0x2a8/0x260/0x2a1），字段级身份未裁决。
21. WallState 0x70B 记录无法重建曲线点列（inner_wall_state.rs:155-178），影响存档→运行时的墙几何恢复。
22. 结构图/墙体附件键链未闭：`wall_attachment` corner-id 互转缺失；结构图 `register_edge_index` 在范围外仍待取证（resources/stairs/structural_graph.rs:220）。

## 9. 完成度估计与依据

| 维度 | 估计 | 依据 | 置信度 |
|---|---:|---|---|
| 骨架函数可追溯 | 83.8% | 238/284 = in_file 198 + name_only 24 + moved 13 + va_only 3；no_trace 46（其中 24 为泛型/闭包噪声） | 高 |
| 同文件命中 | 69.7% | 198/284 `in_file`；其余多为拆分/搬移/胶水 | 高 |
| CPU 行为面（范围内算法） | ~75% | 大文件（roof_shape、decorator_storage、decorator、tiles、trigger volume、inner_walls）均有真实算法、O 注释与部分测试；缺口集中在 material_stats/attachment/outline 与几条 seam | 中 |
| 字段/序列化面 | ~55% | 大量 U 字段、0x2a0/0x2a8 冲突、DecoratorInfo 判别位置偏离、WallState 曲线丢失、tile/dormer 记录未解释 | 中 |
| 集成/GPU 消费面 | ~35% | RoofBuffers opaque、RoofTile/EmptyTriggerVolume 占位、密集表无生产者、两级 UV 回退、collision 门控 | 中 |
| **综合** | **约 66%** | 加权：结构 35% + 行为 25% + 字段 20% + 集成 20% | **中** |

**上界/下界判断**：
- 若只按“文件有主体、能编译”的宽松口径，可达 **75–80%**（范围内 90% 以上行数处于真实行为/混合文件，纯声明/占位类文件合计约 2.4% 行数）。
- 若按“与原版逐行为等价、字段合同完整、GPU 能消费”的严格口径，约 **50–55%**：多个关键生产者/消费者缺失，且语义验收（`inventory.json` 的 `semantic_acceptance:false`）尚未做。
- 本次未编译、未跑测试；基线 STUBS 声称 `cargo test -p country-core --lib` 2452 通过/5 失败（失败均为本机缺游戏资源），但**不能替代本范围的原版等价验证**。

## 10. 需人工裁决 / 不要自动“修”的项

1. `publication_fields` vs `color_selection` 的公共墙记录偏移冲突（0x2a0/0x258/0x299 vs 0x2a8/0x260/0x2a1）。
2. `tiles.rs:442` 的 `local_tip_center = [dim * tip * 0.0, ...]` 是否为原式（若为 0 则矩形天窗尖心恒零）。
3. `DecoratorInfo` 判别从原生 +0x11 移到 `dst_payload` 高位，是否允许作为长期表示（会影响原始 72B 归档互换）。
4. `wall_holes::as_finalized/new_finalized/holes_no_duplicates` 与 `wall_trigger_volume::get_tris` 是否接受“语义等价但改名”的判定。
5. `walls/dormers.rs` 的归属：骨架的 `cottage/dummy/gothic` 已在 `systems/decorator`，而 resources 侧改为 DormerHole SSBO；两处是否应合并/重命名。
6. `MaterialStats` 的双表示：`resources/walls/material_stats.rs`（空壳）与 `systems/wall/construction/segmentation.rs:413-448`（投影）谁是权威。
7. `saved_wall_state` 的 `structure_ty` 复用 `clutter_version` 字段（`:219-221`）——注释自认字段名错误，需协同重命名后再冻结格式。
8. `DecoratorInfo` 归档使用 `ptr::read_unaligned` 强转 72B（decorator_archivist.rs:158-177），仅在 size 断言下成立；需逐字段 offset 机器码复核。
9. `roof_shape` 两级 UV 索引的 `RoofUvColliderIndex` trait 是否保留为碰撞域正式接口。
10. `RoofBuffers`/`RoofTile`/`DormerTiles`/`EmptyRoofTriggerVolume` 的字段与生产者来源（本次未取证，不编造）。

## 11. 附：范围机械统计

- 72 文件、16,099 行、943 个 `fn` 文本匹配、114 个 `#[test]`。
- 骨架范围内 31 文件 / 284 具名函数：`in_file` 198、`name_only` 24、`moved` 13、`va_only` 3、`no_trace` 46。
- 整文件缺失：`walls/decorator_outline.rs`、`walls/wall_attachment.rs`。
- 还原树新增/拆分文件 38 个（主要是 `roofs/geometry/*`、`walls/*_cache`、`publication_*` 等；部分是从 skeleton 大文件或 systems 域拆出，不能直接按行数比较）。
- 本范围 `todo!/unimplemented!`：0。
