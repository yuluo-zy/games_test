# collision-eco 域 round2 逐文件审查（对照 DIGEST O 条目）

> 审查对象：reconstruction/country-core/src 下本域文件。依据：DIGEST.md（O=机器码证实）、
> v2 权威符号表、手解字节。状态：**OK**=与 O 证据一致 / **PARTIAL**=部分一致有缺漏 /
> **WRONG**=与 O 证据冲突。仅记录证据性结论，不改源码。

## systems/collision/

| file | 状态 | 具体错误 | 缺失项 |
|---|---|---|---|
| world.rs | WRONG | ① `SubworldIndex` 16 变体顺序错（Terrain 真实槽=5，重建=13；真实 17 槽）② `SubworldColliderHandle` 建模 `{subworld,index}` 错（真实 24B 带负载枚举，Empty=0x11，`new` 写 {u32@+0,u32@+4,u8@+8}=index/gen/slot）③ `RaycastWorldResult2` 建模 {Option<RaycastHit>,source,handle} 错（真实扁平 0x40B：handle24B/point@0x18/normal@0x24/toi@0x30/marker@0x34/uv@0x38）④ `RaycastGeoSource` 8B 错（真实 24B {disc,flag}）⑤ `subscribe_to_changes` 注释称 Subscriber<ColliderDiff> 错（真实 DirtyTiles）⑥ 函数地址注释多处随 v1 错位：raycast_simple=0x1408ccc20（注释 0x1408cc870 对但 0x1408ccc90 错标）、flush_colliders=0x1408cd5c0、xz_flatten_aabb=0x1408d98a0、dispose_collider_mesh_instance=0x1408d9590（注释称 remove_collider_entry 不存在）、on_collider_upserted=0x1408ce460 ⑦ `RaycastMeshData` 字段注释（SharedShape@+0x20/+0x28）与机器码不符（见 DIGEST §2.5）⑧ `RapierSubWorld` 只建模 ColliderSet（缺 shape@+0x30、脏瓦片@+0x188、脏标志@+0x388）⑨ `Raycasting` 建模 {world: RaycastWorld} 错（真实含 rapier 指针+max_toi 配置） | `flush_colliders` 脏瓦片 drain/publish 逻辑、`update_rapier_colliders` 重建管线、`iter_shape_overlaps` 3 实例、`shape_overlaps_with<Rectangle3d>`、`decorator_overlaps_with_others`、`to_offset_collider_builder`、upsert 泛型族 6 实例、命中视图结构（DIGEST §2.5 裁决） |
| intersect_meshes.rs | PARTIAL | 注释名/址部分随 v1 错（v2：0x1408f2850=polyline::merge_segments_into_polylines、0x140a97b70=find_mesh_mesh_intersections、0x140a9afa0=intersect_mesh_mesh、0x140a9b3c0=should_intersect_collider_meshes；文件头注释把 0x1408f2850 标为 merge 主函数但对齐到 remove_invalid_polylines 的 v1 名） | merge_segments_into_polylines 44KB 实现（重建为 stub 返空）、find_mesh_mesh_intersections 系统体、should_intersect_collider_meshes 源过滤规则、MeshIntersectionGraph 真实容器（GraphMap<SubworldColliderHandle,Artifacts> 待核） |
| visualize_colliders.rs | PARTIAL | 注释名/址对（0x141326370/0x141326930 ✓ v2） | 两函数体（线框图元队列）均为 stub |
| mod.rs | OK | 无 | — |

## resources/stairs/

| file | 状态 | 具体错误 | 缺失项 |
|---|---|---|---|
| stairs_state.rs | WRONG | ① `restore_backup` 置 `dirty=false` 与机器码相反（+0x2f=1）；且把 backed_up（+0x178，add_node 清零/restore 置 1）与脏标志混用 ② stubs/脏边集合实际在 **+0x148**（重建 `edges/stubs: HashMap` 独立字段无此偏移；DIGEST round2 已更正 +0x29 笔误）③ `contains_edge_or_stub` 快路径（+0x38 节点数==1、+0x80 边数==1）未体现 ④ `drain_dirty_edges`（stubs.drain）无机器码对应——真实脏边集合在 +0x148，迭代经 iter_edges_and_stubs 3 实例 ⑤ GraphMap 实际起点 +0x28（重建 `graph` 字段注释 @+0x30/+0x38 系竞技场内指针）⑥ `endpoint` 判别是 **qword**（重建 4 变体枚举语义对，但 Unset 置 qword=3） | +0x148 脏边集合的 drain 面、备份区 +0xb0 起 0x90B blob 结构、+0x00 qword（变更清零）语义、apply_diff/restore_snapshot 完整算法（DIGEST O 部分已述） |
| structural_graph.rs | PARTIAL | 字段名 `span_to_collider` 应作 `span_to_collider_handle`（断言串 O）；8 张 hashbrown 表实际偏移 +0x90/+0xb0/+0xc0/+0xe0/+0xf0/+0x110/+0x150（std HashMap 建模偏移不符）；`ColliderHandle` 局部枚举（Empty=0x11 ✓ 但 Handle(u64) 与 24B 真实句柄不符） | upsert_span 旧关联解绑断言（remove_node(prev)）、connections IndexMap 真实类型 |
| stairs_assembly_params.rs | WRONG | ① `StairNodeAnchor` Wall 判别=**2**（重建 enum{Wall,Free} Wall=0 错）② `AlignKind::position_offset` 实际计算 {flag u8@+1,Vec2@+4,Vec2@+0xc}+常量缩放（重建返 ZERO stub 错）③ `PlayermadeStairsAssemblyRequests::prev_requests` = &self+**0x138**（重建 `params` 字段偏移不符）④ `AlignKind::outgoing_dir` 判别/常量未核（重建 Wall→(1,0)/Free→(0,1) 为猜测） | AlignKind 变体 2 语义、StairSubSegment/AdjustedStairEdge/SampledStairsPoints 布局 |
| invalidate.rs | PARTIAL | 骨架签名（Gap 参数）与真实系统参数面不符（真实：World + StairsState + …，`remove_collider<StairGraphEdge>`×3 + Arc drop + contains_edge_or_stub + StairsVisualState::remove_edge） | 系统参数面（ECS Query/ResMut 判别）、RtWorld RtMesh 删除面、StairSubSegment/StairArchWallSupportType map 清理 |
| visual.rs | PARTIAL | `StairsVisualState::remove_edge` 0x140aa8620 ✓；`force_rebuild` 名错（v2=reset_edge 0x140aa8a60）；`MagicSupports::clear` 0x140aa8210 缺失 | StairsFloorMeshChunk 布局、StairsEdgeVisualState/StairsFloatingBrickParams |
| stairs_collider.rs | PARTIAL | 与 wp21 合同一致（标记资源、无独立函数、U 布局） | StairColliderRegistry 布局 |
| stairs_removed_supports.rs | OK | 与合同一致（serde 存档类型，字段 U） | 判别/字段 |
| mod.rs | PARTIAL | `ColliderHandle` 空判别 0x11 ✓；Handle(u64) 建模与 24B 句柄不符（跨域 boundary 注明即可） | — |

## critters / paths / ivy / clutter

| file | 状态 | 具体错误 | 缺失项 |
|---|---|---|---|
| resources/critters.rs | PARTIAL | 只有 `Wren` 占位；`Sheep::being_petted`=+0x58>0.0（O）、BirdSpotManager gen_id=+8 自增返旧值（O）、ButterflyAudio::set_position=thunk→0x140a57da0（O）均缺失 | CritterControl/Critter/Firefly/Butterfly/Duck/DuckState/CritterGoal/CritterLocomotion/CritterRng/CritterVelocityTarget/Multiplier/BirdSpotManager/BirdSpots 等 18 实体布局（wp10 合同 typed gap 清单仍有效） |
| systems/paths/draw_path.rs | WRONG | ① `draw_path` 是 0x140ae4d00（注释 ✓），但函数体 stub；模式串匹配 "eraser"/"path"/"cobblestone"（O 串）未体现 ② `DrawState::update` 0x140ae4bd0 状态机缺失（手解：raw=ui_mode-2 夹取 0..3；raw∈{1,2}→mode=min(mode-1,raw)；返回 0/3）③ `PathRastersResources::as_float_grid` 0x140ae4c20 stub ④ 注释称 `erase_other_path_variants` 为内联但实际为 draw_path 内 9 个 draw_raster 单态分派组合 | DrawState 转移表、4×WorldRaster<512> 布局（+0x298 GPU 标志）、draw_raster 9 单态接线 |
| systems/paths/path_brush_preview_size.rs | PARTIAL | 名/址 ✓（0x140b1c010；SetSizeBasedOnAppModePath） | 系统体（按模式设笔刷尺寸；brush size 1.5 分支见 misc-systems 合同） |
| systems/paths/record_path_history.rs | PARTIAL | 名/址 ✓（0x140b1c270）；4 命令实体 ✓ | 系统体（4 事件游标差→历史写入） |
| systems/paths/mod.rs | OK | — | — |
| resources/ivy/ivy_storage.rs | PARTIAL | Ivy{points: VecDeque,next_point_id} 语义 ✓；IvyStorage{HashMap<usize,Ivy>…} ✓；`last_segment_point_ids` 重建返回 (Option,Option) 而真实 expect 失败语义（合同） | Ivy 0x20..0x58 区间字段（U）、spawn_ivy 参数面（真实含 RNG 采样根点，重建 signature (pos,dir) 为 I） |
| resources/ivy/ivy_leaf_storage.rs | PARTIAL | IvyLeaf 104B 字段名全 I/U；容器 `HashMap<usize,Vec<IvyLeaf>>` vs 重建 `Vec<IvyLeaf>`（合同说 HashMap 键=ivy id，重建用 Vec 不符） | IvyLeafRng、delete_affected_leaves 按 ivy id 过滤逻辑 |
| resources/ivy/ivy_direction_proposer.rs | PARTIAL | θ 范围 O（[-40°,+40°]）；重建 `noise_a/noise_b: FastNoise` 建模对；get_direction 体 stub | 噪声采样坐标映射（U）、两个 FastNoise 参数（seeded/fractal 细节）、1.3 常量用途 |
| resources/ivy/wall_coord.rs | PARTIAL | WallCoord{u,v} 与 into_world_space 语义 ✓；重建 PublicWallCurve 为独立承载（真实读 PublicWallState 曲线） | 曲线采样精确几何（Curve::get_coord_at_u）、0.63*0.5 偏移方向细节 |
| resources/ivy/mod.rs | OK | — | — |
| systems/ivy/*（9 文件） | PARTIAL | mod.rs 注释地址 ✓；`closest_segment_index` 0x14094c800 ✓ 但体 stub（真实 28B 步长墙段投影+高度过滤）；`intersect_ivy_growth_w_wall_segment`=0x14094cc60 ✓（v1 三名错位已随 v2 更正）；`get_wall_double_sided_normal`=0x14094caa0（重建注释把 0x14094cc60 标为 get_wall_double_sided_normal 错） | round2：grower 主闭包=0x14094ac20（10 相跳表 @0x142ab3be0，见 CLOSURE）；pruner/leaf_spawner/spawner 体（部分 O 已述 DIGEST §6） |
| resources/clutter_meshes.rs | WRONG | ① `ClutterMeshTy` 11 变体粗分类错——真实 **56 变体按资产路径**（shovel/bucket/fallen_leaves/bench/pumpkin2/anvil/door_bell…）② `clutter_mesh_ty_data` 是静态表 switch(0..0x37)→8B 条目（重建 (name,kind) 对不符）③ From<&str> 匹配语义（前缀 vs 全名）待核 ④ ClutterMesh/ClutterMeshes 集合类型 U | 56 变体完整名序、ClutterMarkedForDeletion 等 3 组件布局 |
| resources/preplaced_autoclutter.rs | OK | 与合同一致（default 内联，无独立过程） | 布局 U |

## 结论
- 必改（WRONG 级）：world.rs、stairs_state.rs、stairs_assembly_params.rs、draw_path.rs、clutter_meshes.rs。
- 以 v2 符号表 + DIGEST §2/§3 布局表为修复依据；本审查不改源码。
