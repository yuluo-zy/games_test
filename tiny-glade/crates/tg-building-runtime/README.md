# tg-building-runtime

`tg-building-runtime` 将建筑编辑和几何算法接入 Bevy 0.16 的 `App` / `World`，提供可在无窗口环境运行的事件、资源、组件和调度。它负责墙体权威状态、屋顶生命周期、附着结构、洞口、CPU 几何、碰撞及撤销重做之间的联接。具体几何和业务算法复用 `country-core 的 resources 和 systems/wall/construction 模块`。

本包使用真实 ECS 和 Rapier 平台碰撞后端。Nani 输出是 CPU 描述与后端请求，实际 GPU 分配、绘制、声音播放、交互 UI 和场景持久化由调用方承担。当前调度是本工程的明确实现，不声明与原引擎分叉的系统注册图相同。

## 接入与数据所有权

`building_app()` 创建已安装 `BuildingPlugin` 的空应用；已有应用可调用 `app.add_plugins(BuildingPlugin)`。生产者在更新前写入事件，调用 `app.update()` 后读取结果；嵌入应用的系统可用公开的 `BuildingStage` 约束执行顺序。

| 对象 | 职责 |
| --- | --- |
| `WallCollectionState` | 保存原始 `Curve2` 墙源、显式地形与空间索引 |
| `PublicWalls` / `PublishedWall` | 保存发布后的墙轮廓、业务状态、版本及基础/结构/最终高度缓存 |
| `WallRoofMap` | 从稳定 `WallId` 查询当前屋顶 ECS 实体 |
| `RoofBundle` | 屋顶数据、瓦片模拟、动画、触发体及 Nani CPU 描述组件 |
| `BuildingFrame` | 调用方设置 `delta_time`、`phase`；运行时每次更新递增 `tick` |
| `BuildingReport` | 当前更新的诊断、创建/删除、几何变化及音效请求 |
| 各 `*Entities` 资源 | 查询墙体对应的砖块、拱门、台阶、门、栅栏和洞口实体 |
| 各 `*BuildState` / `pending` | 查询派生结果重建情况及尚未满足的输入条件 |

`WallId` 是业务标识，`Entity` 是 ECS 生命周期标识，两者不能互换。资源中用于外部检查的映射不等同于修改入口；常规编辑应发送事件，让版本、索引、历史及缓存同步更新。

通过 `CreateBuildingFromPlacement` 创建时，调用方必须提供 ID 计数器、创建设置及必要元数据。计数器必须能够继续递增；ID 空间耗尽或选中的 ID 已被原始墙、已发布墙、同帧新墙占用时，整次创建在提交前拒绝。创建和自由墙编辑均先验证候选源，再更新 ID、元数据及历史，避免部分提交。创建快照仅在录制期间进入历史待处理队列；录制缺少必需元数据时拒绝创建，不消耗 ID。

`BuildingReport`、`DeferredNaniRequests` 和 `SolidNaniCpuChanges` 在每帧 `Begin` 清空。需要留存诊断或投递后端的调用方应在下一次更新前消费或复制结果。事件的保留与各 reader 的游标由 Bevy 管理。

## 墙体输入与发布

墙体有两类入口：

- `UpsertRawWall` 提交 `RawWallPublicationSource`。先用 `SetTerrainGrid` 提供 512 × 512 的 `u16` 高度网格及解码高度范围；采集阶段将原始曲线映射到地形，再生成 `PublishWall`。
- `PublishWall` 接受已经解析的 `WallPublicationInput`，可用于独立验证轮廓、屋顶及缓存。需要原始曲线、路径栅格或源元数据的后续消费者仍会报告 `pending`。

原始曲线需要至少两个点，有限的正长度，以及与点数和累计弧长一致的归一化参数。参数必须非递减，首尾精确为 `0` 和 `1`；内部参数允许计算舍入误差，零长边可以产生重复参数；建议通过 `Curve2::try_new()` 构造。地形网格长度必须准确，高度范围有限且有序。发布轮廓至少两个点，地形采样与轮廓等长，坐标、长度及相关高度参数必须有限。无效事件记录诊断并保留上一次有效权威状态。

`SetWallPathRasters` 提供路径准备使用的显式栅格。发布完成后，路径记录按墙发布版本和栅格版本刷新，后续分段、平台、洞口放置及砖块生产只使用当前有效路径。`TerrainDirtyTiles` 通过空间索引筛选候选，`TerrainDirtyCandidates` 可显式指定候选；需要重采样的墙才重新发布。

`SetWallSourceHeight` 设置权威高度；`AdjustWallHeight` 执行带地形和屋顶约束的编辑，并产生反馈。`RemoveWall` 删除原始与已发布墙体，同一采集批次中的移除优先于其新增和更新。相关派生实体及缓存随后按阶段清理。

## 更新阶段与业务流程

| 阶段 | 工作内容 |
| --- | --- |
| `Begin` → `HistoryPrepare` → `Authoring` | 清理帧输出，准备历史回放，处理创建、矩形/自由墙编辑、归档、拆除和源变更 |
| `CollectRaw` → `Publish` | 收集地形和墙源、处理高度请求、重采样并发布有效墙状态 |
| `PathPreparation` → `Segmentation` → `Cache` | 更新路径和初始分段，建立墙基础高度缓存 |
| `Replay` → `Create` → `Associate` → `Spatial` → `InvalidRoofs` | 路由屋顶回放、创建实体、建立墙屋顶关联、同步空间状态并移除失效关联 |
| `Edit` → `GableCache` → `Visual` | 执行方向/参数/类型/样式编辑，记录历史，更新山墙高度和 CPU 瓦片 |
| `Mark` → `Animate` → `NaniUpdate` → `Delete` | 标记删除、推进动画，满足条件后销毁实体；`NaniUpdate` 保留为公开顺序点 |
| `Reassociate` → `FinalCache` | 根据最终实体集合重新关联并更新墙高度 |
| `PlatformSegmentation` → `PlatformStairs` → `PlatformCollision` | 刷新平台处理结果、台阶几何与洞口贡献、Rapier 碰撞及脏瓦片 |
| `WallStructures` → `WallArches` → `Gates` → `Fences` | 构造过梁、拱门、门和栅栏，更新结构高度 |
| `Placement` → `Openings` → `WallGeometry` → `NaniCleanup` | 投影装饰洞口、汇总洞口、生产最终砖块并暂存最终 Nani 变化和释放请求 |

创建、视觉、标记和删除等阶段显式应用 deferred commands，使后续系统看到对应实体变化。阶段名的枚举声明顺序不代表调度顺序，应以上表及 `BuildingPlugin` 中的约束为准。

## 屋顶、洞口与历史

`CreateRoof` 需要已有有效墙体。一个墙同时只能拥有一个活屋顶；重复创建，包括同帧重复请求，拒绝并记入诊断。`SwitchRoofDirection`、`roof_edits::EditRoof`、`SetFlatRoofStyle` 和 `SetFlatRoofRimStyle` 修改不同参数。一个实体同帧多次跨越平顶阈值时，最终派生类型与该批次结束后的屋顶状态一致。`DeleteRoof` 通过标记和动画进入删除生命周期；删除帧的门构造请求不会重新生成门。

屋顶创建和编辑同时验证输入与派生几何。编辑先作用于候选副本，通过校验后整体提交；坏参数不应只修改其中一部分字段。CPU 瓦片生成失败时撤回旧组件，避免新屋顶继续挂着旧几何。无变化通知不会删除仍然有效的门。门消费者保留每墙最新完整构造批次，缺地形等输入时等待重试；补齐输入后可恢复，无需上游重发。新批次（包括空结果）替换旧批次，墙或屋顶删除时清理；地形范围、采样策略与缓冲身份均参与缓存失效。

洞口支持三种来源：`UpsertOpening` 直接提供权威输入；`PlaceWallOpening` 结合显式碰撞资产、原始曲线及当前墙/屋顶派生放置结果；native storage 绑定将观测到的装饰记录映射为 `OpeningId`。多个 native identity 不能竞争同一个洞口身份。`RemoveOpening` 移除权威输入，`WithdrawOpening` 撤回当前派生贡献。墙体消失、路径缺失或放置失败时撤销旧贡献，不能继续使用过时洞口。

同帧修改装饰 storage 会在放置阶段前重新发布 legacy 投影。入口拒绝重复身份；直接修改公开映射造成冲突时，解析器撤销冲突的派生贡献并保留 `pending`，直到调用方纠正绑定。

`WallOpenings` 每帧汇总显式洞口与台阶贡献，砖块生产在汇总之后运行。装饰资产库显式读取给定目录中的 `Vertex_Position` / `float3` 碰撞 JSON；缺失资产或尚不支持的生产分支通过错误或 `pending` 表达。

历史使用显式的 `BuildingHistory.transactions` 与 `HistoryAction`，不会猜测启动账本或编辑默认值。`Start` / `TryStart` / `Finish` 控制事务，`Undo` / `Redo` 选择回放桶。源操作通过 `RecordRoofVisualChange`、`RecordRoofDirectionChange`、`RecordRoofOwner` 或墙历史 owner 记录；回放事件不会再次隐式记录历史。同帧连续墙变更和多个屋顶视觉记录按顺序保存，撤销可以回到最初状态。回放按桶推进，每次更新最多完成一个外层桶；未满足的账本、元数据或 owner 条件会保留队列并给出诊断。

回放未结束时拒绝开启新录制、重置或移动历史游标，并继续推进原回放。桶中任一命令失败时，保留原进度和命令组，不提交该次局部效应；修正输入后可以重试。墙回放仅保存时间及完成标记的检查点，避免复制整份曲线历史。录制中的墙删除必须先取得完整快照，缺少元数据或颜色时保留墙。Wall 历史当前接入创建、删除、状态变化、封闭状态变化和高度调整；其他操作在录入及回放边界检查，不把不可回放的操作当作成功事务提交。

## 模块索引

| 模块 | 功能 |
| --- | --- |
| `wall_publication`、`roof_runtime`（私有） | 墙输入/发布/高度缓存与屋顶生命周期实现；根模块保留公开模型及 Plugin 装配 |
| `roof_record`、`wall_projection`、`history_replay`（私有） | 固定记录的已知字段访问，以及失败不提交进度的墙历史回放 |
| `wall_authoring`、`wall_rectangle_edit`、`wall_pending_changed`、`wall_bulldoze` | 创建、矩形编辑、自由墙变更、拆除及对应历史源 |
| `wall_height`、`wall_paths`、`wall_segments` | 高度编辑、路径准备、墙/平台分段与材质统计 |
| `wall_platform_stairs`、`wall_platform_collision` | 台阶 CPU 几何、装饰行、Rapier 更新、脏瓦片发布和射线查询 |
| `wall_structures`、`wall_arches`、`gates`、`fences` | 过梁、拱门、木门、支撑和栅栏生命周期 |
| `decorator_assets`、`decorator_placement` | 碰撞资产解析、子类型选择及世界位置投影 |
| `wall_decorator_archivist`、`decorator_info_binding` | 装饰存储/归档恢复、身份绑定和洞口投影 |
| `openings`、`wall_bricks` | 洞口汇总、墙砖实例生产和失效结果撤回 |
| `roof_edits`、`roof_invalid`、`history` | 屋顶编辑/类型切换、失效删除和事务回放 |
| `nani` | CPU 实例描述、atlas 解析、投影、增量变化与后端确认 |

## 最小示例

以下示例使用解析后的自由墙轮廓，只演示发布和读取；完整原始墙链见 `tests/raw_building_chain.rs`。

```rust
use std::sync::Arc;
use bevy_ecs::event::Events;
use country_core::resources::walls::publication_fields::{WallId, WallShapeSnapshot};
use tg_building_runtime::{building_app, PublishWall, PublicWalls, WallPublicationInput};

let mut app = building_app();
let mut shape = [0_u8; 28];
shape[..4].copy_from_slice(&2_u32.to_le_bytes());
app.world_mut().resource_mut::<Events<PublishWall>>().send(PublishWall {
    wall: WallId(1),
    input: WallPublicationInput {
        outline: Arc::new(vec![[0.0, 0.0, 0.0], [4.0, 0.0, 0.0]]),
        terrain_y: vec![0.0, 0.0],
        source_curve_length: 4.0,
        source_point_count: 2,
        shape: WallShapeSnapshot::from_observed(shape),
        style: 0,
        extra: 2.0,
        special: false,
    },
});
app.update();
assert!(app.world().resource::<PublicWalls>().walls.contains_key(&WallId(1)));
```

## 性能、诊断与边界

路径、台阶、拱门、砖块和放置结果使用发布版本、资源版本或输入指纹避免无变化重算；砖块指纹复用私有工作缓冲。装饰投影失败撤回实体后保留脏标记，即使输入恢复为上次成功值也会重新生成；派生高度即使命中缓存也需恢复到本帧的公开状态。地形身份缓存持有 `Arc` 快照，保证写时复制或缓冲替换能使依赖失效。公开装饰行的内容变化也参与台阶缓存判定。

拆除系统在空闲帧不复制墙源，拆除和历史回放只复制目标墙的快照。砖块构造在缓存失配后才准备工作数组；相同 native 洞口不重复发布。系统内先建立 native 身份集合，避免对每个 legacy 洞口重复扫描全部绑定。高度缓存仅在内容不同时回填；旧历史请求队列一次移除成功前缀，避免逐项删除首元素的二次方移动成本。`WallBulldozeState::authority_snapshots_this_frame` 与 `WallBrickBuildState::prepared_arrays_this_frame` 可用于检查这些工作量。

`BuildingFrame.delta_time` 必须有限且非负；非法值记录 fault 并按零时间推进，后续有效时间可以恢复。历史回放另外检查播放速度及乘积，拒绝非法推进。瓦片模拟的高度差溢出时保留上次有效模拟值并诊断，仍允许删除动画完成。

独立坏请求写入 `BuildingReport.faults` 后继续处理后续请求；历史账本和回放队列具有串行依赖，不能越过失败步骤推进游标。缺少必要资源与暂不支持的算法分支通过各模块的 `pending` 资源报告，调用方应同时检查诊断与 pending。任意修改公开底层字节投影或资源中的内部一致性仍由调用方负责。

`NaniMeshAtlas` 解析实际网格片段；atlas 映射变化会产生 mesh change。`DeferredNaniRequests` / `SolidNaniCpuChanges` 是提交意图，实际后端成功后才能更新 `NaniBackendConfirmations`。空的 `RoofTiles::allocation` 不代表已分配 GPU 资源。 CPU 实例投影仅在 `NaniCleanup` 对本帧最终实体状态运行一次；同帧创建并完成删除的屋顶不会产生中间的后端创建与释放申请。

本包不提供完整的任意形状/装饰生产分支、原作启动初始化或最终渲染验收；这些限制以 `pending` 和显式输入呈现。平台射线接口只覆盖该平台碰撞后端，不等同于所有原作子世界的查询过滤规则。

## 开发与验证

在工作区根目录执行：

```sh
cargo test -p tg-building-runtime --all-targets
cargo test -p tg-building-runtime --doc
cargo clippy -p tg-building-runtime --all-targets --no-deps
cargo run -p tg-building-runtime --bin building-run -- scenario.json
cargo run -p tg-building-runtime --bin nani-probe < roof-lines.txt
```

`building-run` 读取操作对象组成的 JSON 数组，按 `kind` 投递输入并输出快照；墙 ID 使用十进制字符串，字节投影使用固定长度 ASCII 十六进制串，数值必须可表示为有限 `f32`。输入解析、操作派发、快照和构造 trace 分别组织；完整操作字段以 `src/bin/building-run.rs` 及其 `building-run/input.rs` 为准。快照在类型化 ID 上排序，记录编码避免逐字节格式化分配；缺少诊断输入时输出 pending，而不索引不存在的权威记录。`nani-probe` 每行读取 176 位屋顶 hex 和 8 位动画浮点 bits，输出 CPU 记录和 bounds 的 hex；非法行给出行号并以非零状态退出。

集成测试覆盖墙发布与地形更新、屋顶关联/编辑/删除、历史回放、装饰洞口、结构/砖块、台阶/Rapier 及 Nani 注册。碰撞 JSON 默认来自仓库内 `tests/assets`；`TG_B_ASSETS` 可覆盖目录。测试不会自动读取原作安装位置。

历史来源见 [building-audit.json](../../provenance/building-audit.json)。当前源码审查、修复和验证记录见 [源码审查报告](../../reports/building-foundation-review-2026-10-18/README.md)。

支持能力、必需输入与后端边界的逐项说明见 [功能完整性矩阵](../../reports/building-foundation-review-2026-10-18/FEATURE_MATRIX.md)。
