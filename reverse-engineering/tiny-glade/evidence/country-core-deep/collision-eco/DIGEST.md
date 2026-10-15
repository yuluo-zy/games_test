# collision-eco 域 DIGEST（systems/collision、stairs、critters、paths、ivy、clutter）

> owner: tg_a7。反编译：`run-decomp-batch.py collision-eco tg_a7`（154 VA，134 ok / 20 err）+ v2 权威符号表补跑（tg_a7b，46 VA，32 ok / 14 NotFound 小函数手解）。
> 符号表：**以 `sysres-procedures-v2.json`（S_PUB32，rustc-demangle）为准**；旧 v1 清单存在系统性名/址错位（见 §7）。
> 标注：**O**=反编译/机器码直接支持；**I**=推断（注明依据）；**U**=未知（不臆造）。

## 1. 模块清单与业务过程数

| 文件（PDB 路径） | v2 过程数 | 说明 |
|---|---|---|
| systems/collision/world.rs | ~60 | rapier3d 胶水 + 17 子世界/查询/dirty 生命周期（最大单文件） |
| systems/collision/intersect_meshes.rs | 4 | 网格-网格相交 + 折线合并（parry QBVH） |
| systems/collision/visualize_colliders.rs | 2 | debug 线框可视化 |
| resources/stairs/stairs_state.rs | 17+ | 权威楼梯图（备份/快照/diff/端点/桩） |
| resources/stairs/structural_graph.rs | 7 | 支撑碰撞体图（8 张表） |
| resources/stairs/stairs_assembly_params.rs | 8 | 装配参数族（锚点/对齐） |
| resources/stairs/invalidate.rs | 1 | 脏楼梯伪影失效系统（3455B） |
| resources/stairs/visual.rs | 4 | MagicSupports/StairsVisualState |
| resources/stairs/stairs_removed_supports.rs | 1* | serde 存档类型（derive 胶水） |
| resources/critters.rs | ~8 业务 + 12 注册胶水 | 组件/资源族 |
| resources/critters/sheep.rs / control.rs | serde derive 为主 | 复用 derive（不计业务） |
| systems/paths/{draw_path,path_brush_preview_size,record_path_history}.rs | 6 | 路径绘制状态机/栅格/历史命令 |
| resources/ivy/{ivy_storage,ivy_leaf_storage,ivy_direction_proposer,wall_coord}.rs | 6 | 折线存储/叶存储/方向噪声/墙坐标 |
| systems/ivy/*（7 文件） | 12 | grower/spawner/pruner/leaf/GPU 上传 |
| resources/clutter_meshes.rs | 4 业务 + 5 serde | ClutterMeshTy 56 变体 |
| resources/preplaced_autoclutter.rs | 0* | default 内联，无独立过程 |
| systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs | 5 | critter 绘制 shader 调用（render 域胶水） |

反编译产物在 `decomp/`（不删除）。`proc-list.txt`（v1）、`v2-proc-list.txt`（权威）为完整清单。

## 2. 碰撞世界（systems/collision/world.rs）——本域核心

### 2.1 总体架构（O）

- **Rapier**（资源）＝ 17 个 rapier 子世界槽，每槽 **0x390B** 步长，从 `Rapier+0x20` 到 `+0x3cb0`（O：`Rapier::subworlds` 0x1408cdc40 返回 `[+0x20, +0x3cb0)`；`flush_colliders` 循环 `lVar11 += 0x390` 至 0x3c90；raycast/raycast_simple/upsert 同布局）。
- **每槽布局**：`+0x0` = rapier ColliderSet；`+0x30` = 查询形状（spherecast 的 cast_shape 输入在 `slot+0x30`）；**`+0x188` = 0x200B 脏瓦片缓冲**；**`+0x388` = 脏标志字节**（O：flush_colliders `memcpy(lVar2+lVar11-0x200, 0x200)` + 清标志）。
- **QueryPipeline 是全局静态**（O）：raycast 用 `0x142aa3858`，raycast_simple 用 `0x142aa37f8` —— 两个不同 pipeline 全局（raycast 带法线，simple 不带）。
- `Rapier` 头部：`+0x8` = collider 竞技场 ptr、`+0x10` = len（rapier ColliderSet 内部，条目 0x28=40B，slot 有效性标签 `+0x20 == gen|1`）（O：raycast 闭包 0x1408cab80/0x1408ca9a0）。
- 依赖版本（O 串）：rapier3d（内嵌 slotmap-1.0.6）、puffin-0.14.1、bevy_ecs-0.16.0。rapier/parry 内部不逆向（DEPENDENCY_FILTERING）。

### 2.2 查询（O）

| 函数 | VA（v2 权威） | 行为 |
|---|---|---|
| `RaycastWorld::raycast` | 0x1408cc870 | 委托 `Rapier::raycast`（mask） |
| `Rapier::raycast` | 0x1408cdc60（784B） | 按 mask 位序（位 k=槽 k）遍历 17 槽，`cast_ray_and_get_normal`，最近命中→`rapier_hit_to_raycast_world_result`；无命中置 marker=2 |
| `RaycastWorld::raycast_simple` | 0x1408ccc20（39B） | 纯委托 `Rapier::raycast_simple` |
| `RaycastWorld::raycast_simple_exclude_wall_id` | 0x1408ccc50（57B） | 同样纯委托——**exclude_wall_id 参数被忽略**（本构建） |
| `Rapier::raycast_simple` | 0x1408ce180（696B） | `cast_ray`（无法线）；**后命中覆盖先命中**（不比较距离）；无命中写 byte0=0x11（Empty） |
| `RaycastWorld::raycast_w_terrain` | 0x1408cd430 | mask 含位 0x20（槽 5=Terrain）时先碰撞后地形：地形 raycast（FUN_1409e9a20，terrain 域）以碰撞 toi 为上限过滤，地形命中→合成命中（handle byte0=**0xb**、法线(0,1,0)、marker=0） |
| `RaycastWorld::spherecast` | 0x1408ccfc0 | 全 17 槽 mask 驱动 cast_shape，最近命中 |
| `RaycastWorld::legacy_spherecast` | 0x1408ccc90 | **固定 10 槽顺序 {0,1,3,7,6,4,11,2,12,13}** 的 spherecast；**专用小结果 {handle 24B@+0, point@+0x18, 零 12B@+0x24, marker@+0x30}（无 normal/toi，marker 在 +0x30 非 +0x34）**；critters_avoid_collisions（0x1423179a0）/move_critters（0x142318fa0）/0x1423246b0 调用（O：callgraph + 反编译全解，ALGORITHMS §2.6） |
| `RaycastWorld::intersects_any` | 0x1408cd5b0 | 5B jmp thunk → 0x1408cdf80 |
| `RaycastWorld::flush_colliders` | 0x1408cd5c0（1149B） | 见 §2.4 |
| `RaycastWorld::raycast_intersections_with_ray` | 0x1408cc8a0 | 全交点收集（RaycastHit Vec） |
| `Raycasting::{raycast,intersects_any,spherecast}` | 0x1408cc2e0/0x1408cc320/0x1408cc390 | 旧包装族（Raycasting::raycast = 251B 转发 raycast_w_terrain；9 个调用者含 critters/wall/terrain 系统，O：callgraph） |

### 2.3 结果结构（O，机器码）

**RaycastWorldResult2 = 0x40B**：
`{ handle: 24B @+0x00（byte0=判别，0x11=Empty），point: Vec3 @+0x18，normal: Vec3 @+0x24，
   toi: f32 @+0x30，marker: u32 @+0x34（0=命中/2=无命中），uv: Vec2 @+0x38（mesh 命中时） }`
证据：`rapier_hit_to_raycast_world_result` 0x1408ce4a0（法线/UV 按 barycentric 插值 + quat 旋转）与 raycast_w_terrain/legacy_spherecast 写入面三方一致。
legacy_spherecast 变体把状态写在 +0x30（不写 toi）——**round3 裁决：legacy 为专用小结果 {handle 24B, point, 零 12B, marker@+0x30}，非 0x40B 族的 4B 偏移差**（ALGORITHMS §2.6）。

**SubworldColliderHandle = 24B 带按变体负载的枚举（不是 {subworld,index}！）**（O）：
- `SubworldColliderHandle::new`（0x1408cc620）= 写 `{u32@+0, u32@+4, u8@+8}`（9B/24B）。
- 判别 byte0：**0x11=17=Empty**（raycast_simple 无命中写）；**0xb=11=Terrain**（raycast_w_terrain 地形命中写）。
- **RaycastGeoSource = 24B** `{disc u8 @+0, flag 8B @+0x10}`（`RaycastMesh::source` 0x140681f40 拷 24B）；`RaycastGeoSource::subworld()`（0x1408cea20）= `table[disc]`，表 @0x142aa4418 与 0x142aef03c 同值（disc→槽号）。重建的 8B {subworld,flag} 模型错。
- eq（0x1408cea30，17 入口跳表）：disc 0..=16 = 17 个负载变体，负载形状分组：{0,1,6,7,8,9,12}=qword@+8；{2}=qword@+8+2B；{3,4,5}=u32@+4+qword@+8+Option<qword@+0x10>；{10,11}=2B；{13,14}=qword@+4；{15,16}=qword@+8。
- Debug 元数据表（0x142ab71e0 起）：变体名 `Roof{flag,gable}`、`DecoratorCollision{decorator_id,addr}`、`DecoratorInteraction`、`StairInteraction`、`Column`、`Stairs`、`Fence`、`Arches`、`Tree`、`Terrain`、`AutoClutter{entity}`、`StairSegment{graph_edge}`、`StairSupport`、`SupportedStairStructural{point_idx}`、`UnsupportedStairStructural`（+Floor/Roof 全名序 U）。

**判别→槽号映射表**（O：`upsert_collider_entry` 读 `(&UNK_142aef03c)[disc]`）：
`disc 0..16 → 槽 {0,9,1,2,8,14,3,7,6,10,4,5,11,12,13,15,16}`。**disc 0xb → 槽 5 = Terrain**（与 mask 位 0x20、地形命中 byte0=0xb 三证一致）。查询掩码位 k = 槽 k；`to_bitflag = 1 << 槽号`（I）。

### 2.4 dirty 生命周期（O）

- `RaycastWorld::subscribe_to_changes`（0x1408cdc20，25B）= **`tabloid::Publisher<DirtyTiles>::subscribe()`**——碰撞世界订阅 utils::dirty_tiles（64×64 瓦片表，0x200B=512B=4096 位）。
- 发布者（O：global-references）：`playermade_stairs_assembly_propagate_dirty_flags`、`detect_overhang_support_segments`（wall）、`read_bush_positions_from_gpu`、`add_birdnests`、`populate_autoclutter_regions`（clutter）→ 墙/楼梯/自动杂饰改瓦片 → 碰撞世界。
- `RaycastWorld::flush_colliders`（系统方法，0x1408cd5c0）：逐槽查脏字节（+0x388）→ memcpy 0x200B 脏瓦片缓冲（+0x188）→ 清零 → **`publish<DirtyTiles,DirtyTiles>`**（下游转播）→ `log::debug!("Flushing colliders of {..}")`（串 @0x142aa3900，模块 `country_core::systems::collision::world`）→ `QueryPipeline::update()`。
- `flush_rapier_colliders`（系统，0x1408d8360）= puffin scope + `ResMut<RaycastWorld>`（PLAYBOOK 事实 6：`param[2]=param+0x1c` tick 回写）+ 调上面方法。
- `update_rapier_colliders`（0x1408d0490）：反编译失败（varnode），入口见 §7；其闭包 0x1408d7ed0（7482B）= arch/floor 网格构造 helper（`construct_arch_geo`/`floor_mesh` 族，I）。
- `RaycastWorld::on_level_reload`（0x1408d8710，28KB）：清空世界（rapier arena + registry + 网格 drop 多条路径）。
- `upsert_collider_entry`（0x140b40b50）：同 handle+同几何 → 返回"未变"（status=2）；几何变 → `dispose_collider_mesh_instance`（0x1408d9590，v2 名；v1 误标 remove_collider_entry）+ 新建 collider（user_data 编码 handle）+ QueryPipeline::update。一致性 panic（O 串）：
  - `"collider exists in the physics world but not its collider registry"`（0x142aa3d50）
  - `"collider was removed from physics world but not its collider registry"`（0x142aa4250）
- `Rapier::on_collider_upserted` = 0x1408ce460（v2；v1 误标 0x1408ce4a0）。
- upsert/remove 泛型族（v2）：`upsert_collider<StairGraphEdge>` 0x140252c90、`<WallId>` 0x1412453c0、`<DecoratorId>` 0x141294980、`<StairGraphEdge,IsoTransform>` 0x141296510、`<DecoratorId,OffsetColliderTransform>` 0x141298110、`<(StairGraphEdge,u32),IsoTransform>` 0x141299ca0；`remove_collider<StairGraphEdge>` 0x140252ad0（invalidate ×3 调用）、`<DecoratorId>` 0x140b409b0/0x141245220。

### 2.5 网格数据（O）

- `RaycastMeshData` 0xd0=208B 堆块 + Option tag（new_convex 0x1408cec10：`Some` 写 tag=4、堆指针@+0x20、vtable 类静态@+0x28、哈希@+0x70）。new_unindexed（0x1408cf7d0）写入面：+0x00/+0x08 = parry shape、+0x10 = indices ptr、+0x18 = indices len、+0x20/+0x28 = 哈希与 ptr、+0x30/+0x38 = 法线、+0x40/+0x48 = uv、+0x50..+0x68 = 4 个类型静态 qword、+0x70 = 哈希。
- 输入顶点步长 **0x24=36B**（pos+normal+uv 各 Vec3，I）；`new_unindexed` 生成顺序索引 i,i+1,i+2。
- 读取面（rapier_hit_to_raycast_world_result 的 lVar7）：indices {ptr@+0x20,len@+0x28}、法线 {ptr@+0x40(0=无),len@+0x48}、uv {ptr@+0x50(0=无),len@+0x58}。
- **round2 裁决**：写入面（RaycastMeshData）与读取面（命中视图）为**两个不同结构**——写入面 +0x10/+0x18 = indices {ptr,len}，读取面 +0x20/+0x28 = indices {ptr,len}，偏移不可平移对齐；读取面是 Rapier::raycast 从 collider user_data 解出的**命中视图**（0x60B：indices/法线/uv 三对 {ptr,len}），非 RaycastMeshData。重建代码单结构建模无法同时满足两面，需新增命中视图类型（名 U）。
- `RaycastMesh::raycast`（0x1408ceb40，round2 落盘）：从 `RaycastMesh` 取 Arc 内 shape（vtable @[arc+0x38]），经 parry shape 虚函数 `cast_ray_and_get_normal` 执行单网格 raycast（无命中 → marker=2）。
- `new_{convex,indexed,unindexed,cuboid,sphere,compound}` = 0x1408cec10/0x1408cee40/0x1408cf7d0/0x1408cfff0/0x1408d0120/0x1408d0230（v2）；`RaycastMesh::{new,data,raycast,source}` = 0x1408ceb10/0x1408ceb30/0x1408ceb40/0x140681f40。
- `xz_flatten_aabb` = 0x1408d98a0（v2；v1 标 0x14155d340 错）——手解：输出 `Vec3{min.x, min.y, max.y}`（AABB→xz 展平三元组，输入 0x18B AABB 布局）。
- `mesh_intersect_collider_diff_publisher`（0x1408cc670）= 返回 `&RaycastWorld+0x3cb0`（17 槽之后的 tabloid Publisher 字段；upsert 亦在 +0x3cb0 调 `QueryPipeline::update`——该区域兼作 pipeline 通知面，U 裁决）。

## 3. 楼梯（resources/stairs/*）

### 3.1 stairs_state.rs（O）

**StairsState 布局（机器码观察，部分）**：
| 偏移 | 含义 | 证据 |
|---|---|---|
| +0x00 | qword，变更时清零 | remove_node 0x1408ffef0 `*param_1=0` |
| +0x10 | endpoint 判别 qword（0/1/2/3）+ 数据 @+0x18/+0x20（24B 区间）| unset_endpoint 写 3；endpoint getter 拷贝 24B |
| +0x28 | **GraphMap 起点**（`graph` getter 0x1408fe8b0 = `&self+0x28`，O 手解） | 与 endpoint 24B 结束于 +0x28 自洽 |
| **+0x148** | **脏边/桩 hashbrown 集合**（add_node 插 (n,n)；remove_node 插 (min,max)+(n,n)） | O 手解：add_node `add rdi,0x148` 后 tail-call 插桩；remove_node 的 `param_1+0x29`（undefined8* 指针算术）= 字节 +0x148。**round2 更正：初版误记 +0x29** |
| +0x178 | **backed_up 字节**（add_node/remove_node 清零；make_backup/restore_backup 置 1） | O 手解：add_node `mov byte[rcx+0x178],0`；restore_backup llvm 全解：开头 `mov byte[rdi+0x178],0`、结尾 `mov byte[rdi+0x178],1`（0x1408ff36e）—— **round3 更正：原"+0x2f 脏字节"无机器码依据，实为 +0x178；备份区起点 +0xb8（非 +0xb0）** |
| +0x30/+0x38 | 图竞技场（ptr/节点数；单节点快路径检查 entry+0x20） | contains_edge_or_stub |
| +0x60/+0x68 | 节点查找 map（hashbrown） | remove_node/contains |
| +0x78/+0x80 | 边 map（ptr/len；单边快路径 entry+0xc） | contains_edge_or_stub |
| +0xb0.. | 备份区（0x90B blob） | restore_backup memcpy 0x90 |

方法（v2 名/址）：
- `add_node` 0x1408ffd30 / `add_edge` 0x1408ffd70（自环→panic；归一化(min,max)；端点命中短路）/ `disconnect_edge` 0x1408ffe80（同端点→remove_node）/ `remove_node` 0x1408ffef0（邻边插桩+删）/ `get_all_connected_nodes` 0x1408fe8c0（BFS+排序去重，15KB 尾段 dedup）/ `graph` 0x1408fe8b0 / `endpoint` 0x1408fe890 / `unset_endpoint` 0x1408fe880 / `try_set_endpoint` 0x1408fe7b0 / `make_backup` 0x1408fefe0 / `restore_backup` 0x1408ff210（0x90B 回拷 + **置脏 +0x2f=1**）/ `restore_snapshot` 0x1408ff590 / `extract_diff_and_make_backup` 0x140900010 / `apply_diff` 0x140900ba0 / `contains_edge_or_stub(_impl)` 0x140900e80/0x140900fd0 / `iter_edges_and_stubs` 0x14157cec0/0x14157d090/0x14157d200（3 实例）。
- `TransientStairEdgeIdAllocator::get_or_allocate` 0x1408fe5e0；`StairsSnapshot::new` 0x140901120（先排序再 checksum）；`StairGraphEdge::new` 与 `StairSegment::new` **ICF 折叠到同一 min/max helper** 0x140901260。
- `add_node` 0x1408ffd30（**round2 全 64B 手解**）：`*self=0`（+0x00 清零）→ `byte[+0x178]=0` → `GraphMap::add_node(self+0x28, node)`（调 petgraph 内部 0x140cb10c0）→ tail-call 插桩 helper（参数 `self+0x148, node, node`——自环桩进脏边集合）。`make_backup` 0x1408fefe0、`try_set_endpoint` 0x1408fe7b0 已落盘 decomp/。
- **纠错**：0x140a879b0（v1 称 contains_edge_or_stub_impl）= 实际 `Debug for StairsSnapshot/StairHistoryDiff`（debug_struct 字段 before/after/checksum，串 @0x142ad7b78/0x142ad7b7e/0x142ad8601）。

### 3.2 structural_graph.rs（O）

- 断言串（O）：`"assertion failed: self.graph.remove_node(prev)"`（upsert_span）、`"assertion failed: self.graph.remove_node(collider)"`、`"assertion failed: self.span_to_collider_handle.remove(&span).is_some()"`——确认字段名 `span_to_collider_handle`。
- 8 张表实际 hashbrown 偏移：+0x90（collider→span）、+0xb0（span 查找）、+0xc0（span→collider_handle）、+0xe0（seed）、+0xf0/+0x110/+0x150。重建代码的 std HashMap 字段序与偏移不符（语义等价，布局待改）。
- `clear` 0x140869a30 / `upsert_span` 0x140869d20（旧 span 有 collider 时先解绑；0x11=空判别）/ `upsert_support_source_collider` 0x140869f90 / `add_edge_by_span` 0x140869ff0 / `add_edge_by_collider` 0x14086a260 / `remove_node_by_collider` 0x14086a320 / `disconnect_node_by_collider` 0x14086a4c0。

### 3.3 stairs_assembly_params.rs（O）

- `StairNodeAnchor::try_as_wall` 0x140959ef0：**判别==2 才返回 &payload(+8)**——Wall 判别=2（重建的 `enum{Wall,Free}` Wall=0 错）。
- `AlignKind::position_offset` 0x140959f00：判别 byte0∈{0,1,..}；变体1={flag u8@+1, Vec2@+4, Vec2@+0xc} 组合+常量缩放——重建 stub 返回 ZERO 且枚举形状错。
- `AlignKind::outgoing_dir` 0x140959f50；`StairWallAnchor::new` 0x140959e80（**16B 拷贝** {Vec2 anchor, f32 angle, pad} ✓ 重建布局）/ `wall_normal` 0x140959e90（(sin,cos)✓）；`PlayermadeStairsAssemblyParams::length_ws` 0x140959ca0；`PlayermadeStairsAssemblyRequests::prev_requests` 0x140274480 = **`&self+0x138`**（重建 `params` 字段偏移不符）。

### 3.4 invalidate.rs / visual.rs（O）

- `invalidate_dirty_stair_artifacts` 0x140b37180（3455B）：**`remove_collider<StairGraphEdge>`（0x140252ad0）×3** + Arc drop + `contains_edge_or_stub` 决定 + `StairsVisualState::remove_edge`（0x140aa8620）——与 wp21 合同一致 ✓。
- `MagicSupports::clear` 0x140aa8210；`StairsVisualState::{remove_edge,reset_edge}` 0x140aa8620/0x140aa8a60（v2；v1 的 force_rebuild 名错）。

## 4. critters（resources/critters.rs）

- `ButterflyAudio::set_position` 0x140f85ba0 = 5B thunk → 0x140a57da0（共享实现，U）。
- `Sheep::being_petted` 0x14089d4c0 = `f32@+0x58 > 0.0`（ucomiss 常量 0.0）。
- `BirdSpotManager::gen_id` 0x140f86330（手解）= `+8` 后自增、**返回旧值**（wp10 "+8 += 1" ✓）。
- `Duck::new` 0x140f85bb0 / `BirdSpotManager::{gen_id,create_spots}` 0x140f86330/0x140f86340（wp10 合同已述）；`BirdSpotManager::recycle` 0x1422f86e0：+0x10 原子借用守卫（LOCK/CAS）后池回收。
- `Duck::being_petted` 0x140002370 = ICF 折叠 stub（多类型共享）。
- 12×`register_required_components` = Bevy 注册胶水（复用 derive，不计业务）；sheep/control serde ×14 = serde derive 胶水。
- 上限 96/24（Firefly/Butterfly）见 wp10 合同（GPU 侧，本域资源面不重复验证）。

## 5. paths（systems/paths）

- `draw_path` 0x140ae4d00（15KB）：读 UI 模式串（`"eraser"`/`"path"`/`"cobblestone"`，O 串 @0x142ae2e80/0x142ae2ef0/0x142ae2f10）分派 9 个 `world_raster::draw_raster` 单态（0x140ab35f0/0x140ab6de0/0x140ab8090/0x140abba80/0x140ab2340/0x140ab0f10/0x140ab9340/0x140ab4750/0x140aba770——每变体绘制+擦除组合）。共享断言 `"assertion failed: brush_size.is_finite()"` 来自 draw_garden.rs（跨域共享路径）。v1 名 validate_param 错。
- `DrawState::update` 0x140ae4bd0（手解）：`raw = ui_mode - 2` 夹取到 0..3；`raw∈{1,2}` 时 `mode = min(mode-1, raw)（mode-1≥2 时取 raw）`；返回 `raw==0 ? 0 : (mode-1<2 ? 3 : 0)`。状态机模式字节 @+0（重建 `DrawState{mode:u8}` 形状对，转移表待完整恢复）。
- `PathRastersResources::as_float_grid` 0x140ae4c20 已落盘 decomp/。
- `record_path_history` 0x140b1c270：4 事件游标差（Events<T> cursor 运算）→ 历史写入。
- `path_brush_preview_size` 0x140b1c010；`impl$0::build` 0x140b68e40（插件注册）。

## 6. ivy（resources/ivy + systems/ivy）

- `Ivy::add_point` 0x140869460 / `last_point` 0x1408694f0 / `last_segment_point_ids` 0x140869540 / `IvyStorage::spawn_ivy` 0x1408696f0 / `IvyLeafStorage::add` 0x1408fe280——与 wp16/wp21 合同一致（40B 点、VecDeque、HashMap<usize,Ivy>）。
- `IvyDirectionProposer::get_direction` 0x141234590（θ∈[-40°,+40°] 混合，(cosθ,sinθ)）。
- `WallCoord::into_world_space` 0x140b199e0（墙曲线采样 + 法向偏移 0.63*0.5）。
- **grower**：`ivy_grower` 0x14094a910 = puffin + Bevy 系统跳表分派（`world+0x118` 索引 @0x142ab3be0 表——**10 相跳表字节级验证**，ALGORITHMS §4.1）；`closest_segment_index` 0x14094c800（**56B 步长墙段** {xy0,xy1,f32@+0x10/+0x14（打包进结果）,h0@+0x18,h1@+0x1c}，投影+高度过滤 `max_ivy_grow_height`，距离度量 = |(P0−P)·单位方向|；ALGORITHMS §4.2）；`get_wall_double_sided_normal` 0x14094caa0；`intersect_ivy_growth_w_wall_segment` 0x14094cc60（v2 名；v1 三名全错）。
- **spawner** 0x14094cf90（38KB）、**pruner** 0x140997410（70KB）/ `is_point_still_valid` 0x140998dc0 / `ivy_pruner_cmd` 0x140999330 / `remove_ivy_at_location` 0x140999860、**leaf_spawner** 0x140996080、branch_thickness 0x140995a80、leaf_animation 0x140b0e380、send_to_gpu_{leaf,segments} 0x1408c1580/0x140a9b4e0。
- panic/日志串（O）：`"Received IvySegmentDoneGrowingWithMsg with ivy id "`（leaf_spawner）、`"Spatial Hash returned a wall id "`（pruner 墙空间哈希）；引用 `"Tried to init CurveU from an invalid u: "` + `"assertion failed: !u.is_infinite()"`（CurveU 跨域）、`"assertion failed: ev_count < 2"`（merge_walls.rs）。

## 7. clutter（clutter_meshes.rs / preplaced_autoclutter.rs）

- **ClutterMeshTy = 56 变体，按具体资产路径**（O：`ClutterMeshTy::from` 0x1408b83d0 68KB + `clutter_mesh_ty_data` 0x1408b8e90 switch(0..0x37) + enum_iterator::Sequence impl 0x1408b99e0）。名字表（0x142a9fa00 区，部分）：shovel/bucket/fallen_leaves/fallen_petals/golden_glade_rock/bench/bench_w_basket/bench_short/barrel/haystack/birdhouse/basket/potato_sack/road_sign/sunflower/cart/cart2/crates_w_tools/pumpkin/pumpkin2/pumpkin_w_face/anvil/stall_bread/rack_brooms/flowerbox/crates_w_pumpkins_regular/door_bell/…+ window_f_flowers/pitchfork/firewood/box_w_stuff/watering/… 未识别 panic。
- **重建代码的 11 变体粗分类（Plants/Watering/Windows/Firewood/…）是错误模型**——真实为 56 个路径键控变体。
- `ClutterMarkedForDeletion`/`DecoratorAutoClutterChildren`/`DisplayChildAutoClutterInfo` 组件面 U（PDB 名 O）。
- `PreplacedAutoClutter`：default 内联，无独立过程（wp21 标记 ✓）。

## 8. panic/日志串清单（本域新增）

| 串 | 地址 | 出处 |
|---|---|---|
| `Flushing colliders of ` | 0x142aa3900 | flush_colliders debug 日志格式 |
| `collider exists in the physics world but not its collider registry` | 0x142aa3d50 | upsert 一致性断言 |
| `collider was removed from physics world but not its collider registry` | 0x142aa4250 | remove 一致性断言 |
| `assertion failed: self.graph.remove_node(prev)` | 0x142a96b48 | structural_graph upsert_span |
| `assertion failed: self.graph.remove_node(collider)` | 0x142a96c00 | structural_graph remove_node_by_collider |
| `assertion failed: self.span_to_collider_handle.remove(&span).is_some()` | 0x142a96c50 | structural_graph |
| `Received IvySegmentDoneGrowingMsg with ivy id ` | 0x142abaeb0 | ivy_leaf_spawner |
| `Spatial Hash returned a wall id ` | 0x142abb028 | ivy_pruner |
| `assertion failed: brush_size.is_finite()` | 0x142ae2e06 | draw_path（共享 draw_garden.rs） |
| `Tried to init CurveU from an invalid u: ` / `assertion failed: !u.is_infinite()` | 0x142aa53c0/0x142aa5498 | CurveU（ivy 引用，跨域） |
| `clutter mesh type not found: {name}` | （From<&str> 尾 panic，地址见 decomp-0x1408b83d0.txt） | ClutterMeshTy |
| scope 串族 | 0x142ab3680/0x142ab3830/0x142aa3610… | puffin scope（各系统） |

## 9. 与现有合同/代码的差异（逐条）

**world.rs（重建代码 `src/systems/collision/world.rs`）：**
1. `SubworldIndex` 枚举顺序错：重建 Terrain=13；真实 **Terrain 槽=5**（mask 0x20 + 地形命中 0xb + 映射表三方锚定）。重建 16 常量序与真实 17 槽不符（缺 1 槽；完整名字映射 U）。
2. `SubworldColliderHandle` 建模错：重建 `{subworld, index}`（16B）→ 真实 **24B 带负载枚举**（§2.3），Empty=0x11。
3. `RaycastWorldResult2` 建模错：重建 `{hit: Option<RaycastHit>, source, handle}` → 真实扁平 0x40B（§2.3）。
4. `subscribe_to_changes` 注释错：真实订阅 **DirtyTiles**（非 ColliderDiff）；`mesh_intersect_collider_diff_publisher`（0x1408cc670）存在但语义待核。
5. `flush_colliders`/`update_rapier_colliders` 注释为 stub：真实逻辑 §2.4（脏瓦片 drain + publish + QueryPipeline::update）。
6. 函数名/址注释多处随 v1 错位：`rapier_hit_to_raycast_world_result` 0x1408ce4a0 ✓（v1 标 on_collider_upserted ✗）；`raycast_simple` 0x1408ccc20 ✓（v1 标 0x1408ccc90 ✗，后者=legacy_spherecast）；`flush_colliders` 0x1408cd5c0 ✓（v1 标 intersects_any_w_terrain ✗）；`xz_flatten_aabb` 0x1408d98a0（v1 标 0x14155d340 ✗）；`dispose_collider_mesh_instance` 0x1408d9590（v1 标 remove_collider_entry ✗）；`on_collider_upserted` 0x1408ce460（v1 标 0x1408ce4a0 ✗）。
7. `RaycastMeshData` 布局注释（"法线、索引、哈希键+0x70、SharedShape+0x20/+0x28"）与机器码写入面不符（§2.5）；vertices/normals/indices 三 Vec 建模待改。

**stairs_state.rs（重建代码）：**
8. `restore_backup` 置 `dirty=false` 与机器码相反（置 +0x2f=**1**）；且重建把 +0x178 backed_up 与脏标志混用（机器码 restore 写 +0x2f）。
9. stubs 实际在 **+0x29 字节偏移** 的 hashbrown 表；重建 `edges/stubs: HashMap` 独立字段布局与偏移不符。
10. `contains_edge_or_stub` 快路径语义（+0x38 节点数==1 单节点检查、+0x80 边数==1 单边检查）未体现在重建实现。
11. 重建 `drain_dirty_edges`（stubs.drain）无机器码对应（实际脏边集合/迭代在 iter_edges_and_stubs 3 实例，U）。

**stairs_assembly_params.rs：** `StairNodeAnchor` Wall 判别=2（重建 0）；`AlignKind::position_offset` 实际计算（重建 ZERO stub）。
**stairs/structural_graph.rs：** hashbrown 偏移表（§3.2）；字段名 span_to_collider_handle（重建 span_to_collider 近义）。
**stairs/mod.rs ColliderHandle：** 空判别 0x11 ✓（O：raycast_simple/upsert 多处）；但 "Handle(u64)" 建模与 24B 枚举不符。
**clutter_meshes.rs：** 56 变体 vs 重建 11 粗分类（§7）。
**critters.rs：** `Sheep::being_petted` = +0x58 > 0.0（wp10 说 Duck 行 136——地址 0x14089d4c0 是 Sheep，注意 v2）。
**invalidate.rs：** 与 wp21 合同一致 ✓（×3 remove_collider<StairGraphEdge>、contains_edge_or_stub、remove_edge）。

## 10. 跨领域线索（供中央索引）

- **DirtyTiles pub/sub**：发布者 playermade_stairs_assembly_propagate_dirty_flags（0x1407d16b0）、detect_overhang_support_segments（0x1407867d0）、read_bush_positions_from_gpu（0x140eebaa0）、add_birdnests（0x1422c9c70）、populate_autoclutter_regions（0x1422d6ed0）；订阅者=碰撞世界（本域）。
- 地形 raycast `FUN_1409e9a20`（terrain-water 域）——raycast_w_terrain 的地形半场。
- `utils::geometry::barycentric`（共享）、`utils::dirty_tiles::DirtyTiles`（共享 utils）、`CurveU`（shared-foundation）。
- critters 系统（0x1423179a0/0x142318fa0/0x1423246b0）消费 legacy_spherecast/legacy_raycast。
- stairs_assemble（system_decorator）闭包 0x140254890/0x140254aa0（wall 域）实例化 iter_shape_overlaps；calculate_distortion_from_structural_graph 0x14129b780/0x14129bad0。
- wall 域：`merge_walls` 断言 ev_count<2、`public_walls` simplified_curve_2d 断言（ivy pruner 引用）。
- 依赖确认：puffin-0.14.1、bevy_ecs-0.16.0、slotmap-1.0.6（rapier 内部）、petgraph/indexmap/foldhash（stairs）。

## 11. 待办/未知（U）

- SubworldIndex 17 槽完整名字↔槽号映射（Terrain=5 已锚定，其余待各 mask 使用点交叉验证）。
- update_rapier_colliders（0x1408d0490）反编译失败，需 asm/邻接函数恢复（管线已闭合，见 ALGORITHMS §2.9）。
- StairsState 完整 0x180B 布局（+0xb8 备份区结构已解：0x90B 镜像图区；+0x178 语义已裁决 = backed_up；残余字段 U）。
- Ivy grower 相 5-9 宿主定位（跳表目标在 grower 体尾部区间，二级跳表/panic 入口，语义面已闭合）。
- ClutterMeshTy 56 变体完整名序（名字表碎片化，需按 len-prefix 全量解析）。
- closest_segment_index 双高度过滤互斥疑点（高度坐标方向语义，待调用方数值裁决）。
