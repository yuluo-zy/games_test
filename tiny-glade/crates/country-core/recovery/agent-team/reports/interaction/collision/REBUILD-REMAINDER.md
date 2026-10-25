# REBUILD-REMAINDER — systems/collision 未实现余量：分类 → 实现 → 自检

日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（§重建前必做分类 + 逐语句判等自检）
LOGIC.md: reports/interaction/collision/LOGIC.md（未实现登记段）
证据: planning/impl-collision-remainder/results/{b1..b9}*.result.json（ghidra `--worker shared`
只读 fresh 反编译 + `disassemble_function`，2026-10-17）+ `planning/logic-collision/results/*`

## 方法

inventory（`country-core-audit/data/procs.tsv` 模块前缀清单）→ fresh 反编译每批 ≤20 → 分类
（宏胶水 vs 真业务）→ 可用类型实现 / 缺类型登记。红线：只以 fresh 反编译为准；宏胶水标记不重建；
不因编译通过宣布完成。

## 分类结论

- **world.rs 余 ~66 业务**：本轮**实现 11 个**（查询族/访问器/工具），**登记 ~48 个**（依赖
  `RaycastMeshData` 布局、`RaycastGeoSource`/`OffsetColliderTransform`/`DirtyTiles` 类型族、
  collider 生命周期 monolith），**宏胶水标记 ~8 个**（derive/fmt/Index/注册）。
- **intersect_meshes.rs（21，业务 4）**：全部登记（依赖 game collider 存储 + `RaycastMeshData`
  布局 + parry Qbvh 装配）；模块文件已建立（分类表落盘）。
- **visualize_colliders.rs（6，业务 2）**：全部登记（依赖 `RaycastMeshData` 布局 +
  debug_viz `show` ECS 胶水）；模块文件已建立。
- **resources/rt（13）**：按任务"若机器码可定位实现,否则登记"——机器码可定位但 13 个函数依赖
  `RtWorld`/`RtMesh`/`DirtyCow`/`RtBvhBuilderGlue` 等**不存在类型**，全部登记（含分类）。

## 逐函数：world.rs 本轮实现（11）→ 分类 → 判等自检

| VA | 原函数 | 分类 | verdict | 自检 |
|---|---|---|---|---|
| 0x1408cd5b0 | RaycastWorld::intersects_any | 真业务 | **判等** | fresh 全 17 槽 Qbvh 循环与 `Rapier::intersects_any`(0x1408cdf80) 同体；Rust 转发 Rapier::intersects_any，逐条对应 |
| 0x1408ccc20 | RaycastWorld::raycast_simple | 真业务 | **判等** | fresh=转发 `Rapier::raycast_simple`(0x1408ce180)；Rust 转发，返回 `RaycastSimpleResult` |
| 0x1408ccc50 | RaycastWorld::raycast_simple_exclude_wall_id | 真业务 | **判等（结构）** | fresh disasm=向 0x1408ce180 传 mask+排除实参；Rust 转发 + `QueryFilter::exclude_collider`（wall→collider handle 映射在调用方，登记） |
| 0x1408cc770 | RaycastWorld::legacy_raycast_w_params | 真业务 | **判等** | fresh=`Rapier::raycast` + 未中判别 2 写 +0x30（toi 槽）；Rust 逐条对应（同 legacy_raycast 约定） |
| 0x1408ccc90 | RaycastWorld::legacy_spherecast | 真业务 | **判等** | fresh=17 槽 `cast_shape` + 判别 0/2 写 +0x30、命中 normal 写 0；Rust=`Rapier::legacy_spherecast` 逐条对应 |
| 0x1408ce180 | Rapier::raycast_simple | 真业务 | **判等** | fresh=17 槽 `QueryPipeline::cast_ray`，最后命中槽覆盖，输出 {handle,position,toi}，未中首字节 0x11；Rust 逐条对应（+ `RaycastSimpleResult`） |
| (0x1408ccc90 支撑) | Rapier::legacy_spherecast | 真业务 | **判等** | 同上 |
| 0x1408cdc40 | Rapier::subworlds | 真业务 | **判等** | fresh={start=&self+0x20,end=&self+0x3cb0,0}=17 槽区间；Rust 返回 `0..17`（消费方登记待接） |
| 0x1408cc8a0 | RaycastWorld::raycast_intersections_with_ray | 真业务 | **判等**（+0x3d80 等价注） | fresh=地形**无条件** intersect_ray（判别 0xb）+ 17 槽 Qbvh 遍历收集 + `sort_unstable_by` toi 升序（<0x15 insertion_sort 否则 ipnsort）；Rust 经 `QueryPipeline::intersections_with_ray`（+0x3d80 Qbvh 等价，同 REBUILD.md 约定）逐条对应 |
| 0x1408d98a0 | xz_flatten_aabb | 真业务 | **判等** | disasm 逐指令对应：输出 {min.x,min.z,max.x,max.z} = Aabb2；跳过 y |
| — | RaycastSimpleResult（新类型） | 支撑 | — | `#[repr(C)]` {handle[3],position,toi}；miss 首字节 0x11 |

## 逐函数：world.rs 登记（~48，业务真，缺类型/缺布局/超大 monolith）

| VA | 原函数 | 登记原因 |
|---|---|---|
| 0x1408cd5c0 | RaycastWorld::flush_colliders | 每槽 DirtyTiles 缓冲（0x200B @slot+0x188）+ 脏标志 @slot+0x388 + `publish<DirtyTiles>` + `QueryPipeline::update`；`DirtyTiles` 类型与 Publisher 字段不存在 |
| 0x1408d8360 | flush_rapier_colliders | puffin scope + flush_colliders；同上 |
| 0x1408d0490 | update_rapier_colliders（31293B） | 全 collider 生命周期 monolith；依赖 collider 存储 + `RaycastMeshData` + Qbvh@+0x3d80 装配 |
| 0x1408d8710 | RaycastWorld::on_level_reload | 同上 |
| 0x1408d7ed0 | construct_arch_geo | `Triangle::new`/`normal` + 顶点收集；依赖 arch 几何输入类型 |
| 0x1408d9590 | dispose_collider_mesh_instance | collider 存储 + `publish<ColliderDiff>` + dirty tiles 标记；载荷未定 |
| 0x140252ad0/0x140b409b0/0x141245220 | remove_collider<…>×3 | 泛型 monomorphization；依赖 collider 存储 + 各 key 类型 |
| 0x140252c90/0x1412453c0/0x141294980/0x141296510/0x141298110/0x141299ca0 | upsert_collider<…>×6 | 6.9–7.2KB 泛型 monomorphization；依赖 `OffsetColliderTransform` + collider 存储 + `on_collider_upserted` |
| 0x140b40b50 | upsert_collider_entry | 同族支撑 |
| 0x1408cdb50 | RaycastWorld::get_collider | 返回 40B collider 元数据（拷贝 Collider+0x34..+0x50 + {gen\|1,index}）；类型未定 |
| 0x1408cdbd0 | RaycastWorld::get_collider_mesh | 返回 collider 关联 mesh；依赖 `RaycastMeshData` 布局 |
| 0x1408ce440 | Rapier::get_rapier_collider | `ColliderSet::get_mut_internal` 桥接；collider 存储类型未定 |
| 0x1408ce460 | Rapier::on_collider_upserted | `publish<ColliderDiff,ColliderDelta>`；`ColliderDiff`/`ColliderDelta` 载荷未恢复（现空壳） |
| 0x1408cdc20 | RaycastWorld::subscribe_to_changes | `Publisher::subscribe<DirtyTiles>`；DirtyTiles Publisher 字段不存在 |
| 0x1408cee40/0x1408cf7d0 | RaycastMeshData::new_indexed/new_unindexed | 输出写满 +0x00..+0x70（顶点 Vec@+0、shape@+0x20、法线 Vec@+0x30、uv Vec@+0x40…）——**当前 `RaycastMeshData` 结构 {tag,kind,shape,scale,hash} 与原布局不符**（new_cuboid/new_sphere/new_compound 也写多字段）；布局须先锚定 |
| 0x1408cfff0/0x1408d0120/0x1408d0230 | new_cuboid/new_sphere/new_compound | 同上（依赖布局） |
| 0x1408ceb10/0x1408ceb30/0x140681f40*/0x1408ceb40 | RaycastMesh::{new,data,source,raycast} | `RaycastMesh` 类型缺失；（*0x140681f40 PDB 标注冲突，实为 toolbar ActionHintView::into_inner） |
| 0x1408cea20/0x1408cea30 | RaycastGeoSource::{subworld,eq} | `RaycastGeoSource` 17 变体枚举缺失；impl$86::fmt(0x1409816a0) 证实判别 0..0x10 |
| 0x1408d0390/0x140b26a80/0x140b26bd0 | to_offset_collider_builder impl$21/22/23 | `OffsetColliderTransform` 类型缺失 |
| 0x14155d340/0x1408d98c0/0x1408d9960/0x140b26d20 | into_transformed_shape impl$41/42/43/45 | `OffsetColliderTransform`/`IsoTransform` 类型缺失 |
| 0x1408d9990 | impl$47::as_collider_shape | 同上 |
| 0x1408ce4a0 | rapier_hit_to_raycast_world_result（1398B） | 三角 barycentric 法线插值；依赖网格顶点/法线/uv 缓冲结构（`barycentric()` 已可复用 utils） |
| 0x142612d10 | RaycastWorld::shape_overlaps_with<Rectangle3d> | 依赖 `into_transformed_shape`（rect→shape 变换未锚定） |
| 0x14129b780/0x14129bad0/0x140254890/0x140254aa0 | iter_shape_overlaps ×4 | 泛型；closure 接收 (handle, &RaycastGeoSource)；类型缺失 |
| 0x14129be00 | decorator_overlaps_with_others | 依赖 `OffsetColliderTransform` + `DecoratorRank` |

### world.rs 宏胶水（标记，不重建）

| VA | 函数 | 分类 |
|---|---|---|
| 0x1408cc5a0 | impl$1::index / impl$2::index_mut | 宏胶水（std Index 边界检查直通） |
| 0x1408cc5d0 | impl$3::partial_cmp | **存疑**：fresh=仅按 +8 slot 字节比较（Less/Equal/Greater）；Rust `#[derive(PartialOrd)]` 按字段序比较 —— **分歧登记**（graph key 排序语义未验证，不改动） |
| 0x1408cc630 | impl$11::from | 存疑：`From<…> for RaycastWorldResult` 族转换（源类型 U），登记 |
| 0x140833250 | impl$61::from | 实为 `DecoratorRank::new`（`return;`），宏胶水 |
| 0x1408d99f0 | impl$99::fmt | 宏胶水（`Formatter::write_str` 静态串） |
| 0x1409816a0 | impl$86::fmt | 存疑：手写 Debug（17 变体 switch）→ 归真业务但类型缺失，登记 |
| 0x1408d99e0 | impl$62::data | 宏胶水（访问器 `return;`） |
| 0x14003bb80 | _::impl$0::{apply,queue,new_archetype} | bevy 系统 glue（size 1） |

## 跨模块说明

- 本轮**未改动他模块**（roof_shape.rs 等维持 REBUILD.md 的机械适配）。
- `RaycastSimpleResult` / `RaycastWorld::raycast_simple` 被 `systems/terrain/ui_terrain_focus_mode.rs`
  的 `TerrainFocusRaycast` 概念对接（未命中 distance=f32::MAX/tag 0x11 语义一致）。

## 编译状态

- `cargo check -p country-core`：**通过**（0 error；collision 文件无 warning；其余 10 warning 属
  audio/save/ui 等既有模块）。native-physics 同（0 error）。
- 验证时序：全部 collision 改动（world.rs 查询族 + intersect_meshes/visualize_colliders 模块 +
  wiring）落地后，两特征各完成一次**全量干净构建**（default 2m02s / native-physics 1m18s，
  0 error、collision 无 warning）。此后工作树并行 agent 持续改动 `rhapsody` 依赖
  （`rg/transient_resources.rs`/`raw/buffer.rs`/`raw/texture.rs`/`rg/pass_api.rs` 出现 E0061/
  E0277/E0308/E0004 等），当前全仓 check 被其阻塞——均为并行 WIP，非本模块。
- collision 相关 rapier API 面（`cast_ray`/`cast_shape`/`intersections_with_ray`/
  `ColliderHandle::from_raw_parts`/`QueryFilter::exclude_collider`/`ShapeCastOptions`）已用独立
  探针 crate（rapier git rev 7b5ece7）单独编译验证通过。

## 自检（协议逐语句判等）

- **intersects_any**：fresh 0x1408cd5b0 与 0x1408cdf80 循环体逐条相同（0x3c90→0 步进 0x390、
  `in_R9D>>slot&1` mask、命中置 found 即返回）；Rust 转发等价。
- **raycast_simple**：fresh 0x1408ce180 `cast_ray`(solid=1)、最后命中覆盖、未中 `*(u8*)=0x11`；
  Rust 逐条对应。
- **legacy_spherecast**：fresh 判别 0/2 写 +0x30、命中 normal 写 0；Rust `toi` 槽承载判别值
  （legacy 约定，同 legacy_raycast）。
- **raycast_intersections_with_ray**：fresh 地形无条件查询 + 槽循环 + `sort_unstable_by`；
  Rust 逐条对应（+0x3d80 Qbvh 经 `QueryPipeline::intersections_with_ray` 等价，沿用 REBUILD.md 立场）。
- **xz_flatten_aabb**：disasm 8 条指令逐一对应 Aabb2 输出。

## 待续（登记，非本轮）

- `RaycastMeshData` 全布局锚定（+0x00 顶点 Vec / +0x20 shape / +0x30 法线 / +0x40 uv / +0x50 索引 /
  +0x68 scale / +0x70 hash）→ 之后接 5 个构造器 + `RaycastMesh` 族 + `rapier_hit_to_raycast_world_result`。
- collider 存储（`RaycastWorld` +0x08/+0x10 Vec<0x28B 条目>）→ 接 upsert/remove 族 +
  `get_collider(_mesh)` + `should_intersect_collider_meshes`。
- `DirtyTiles`（utils 共享）→ 接 `subscribe_to_changes`/`flush_colliders`/`flush_rapier_colliders`。
- `RaycastGeoSource`/`OffsetColliderTransform` 类型 → 接 to_offset/into_transformed/as_collider_shape/
  iter_shape_overlaps/shape_overlaps_with/decorator_overlaps_with_others。
- `ColliderDiff`/`ColliderDelta` 载荷 → 接 `on_collider_upserted`。
- intersect_meshes 4 业务 + visualize_colliders 2 业务 + resources/rt 13（均登记）。
