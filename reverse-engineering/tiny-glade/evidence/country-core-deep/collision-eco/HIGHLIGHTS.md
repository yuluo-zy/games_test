# collision-eco 域 HIGHLIGHTS（10 条最重要发现）

1. **碰撞世界 = 17 个 rapier 子世界槽 + 全局 QueryPipeline + DirtyTiles 脏瓦片生命周期**。
   每槽 0x390B（`Rapier+0x20..+0x3cb0`）：ColliderSet@+0、查询形状@+0x30、**0x200B 脏瓦片缓冲@+0x188 + 脏标志字节@+0x388**。`flush_colliders`（0x1408cd5c0）逐槽 drain 脏瓦片 → `tabloid publish<DirtyTiles>` → `log::debug!("Flushing colliders of {..}")` → QueryPipeline::update。两个 QueryPipeline 是**全局静态**（0x142aa3858 带法线 / 0x142aa37f8 simple）。发布方：stairs_assemble、wall 悬挑检测、autoclutter（bush/birdnest/regions）。

2. **`SubworldColliderHandle` 不是 {subworld,index}，而是 24B 带按变体负载的枚举**：Debug 元数据给出 `Roof{flag,gable}`、`DecoratorCollision{decorator_id,addr}`、`AutoClutter{entity}`、`StairSegment{graph_edge}`、`UnsupportedStairStructural{point_idx}` 等变体；Empty 判别=0x11(17)；eq 跳表 17 个负载变体（负载形状按判别分组）。重建代码的扁平结构建模错误。

3. **Terrain 槽号=5（三证锚定）**：raycast_w_terrain 检查 mask 位 0x20=bit5；地形命中写 handle byte0=0xb；判别→槽映射表（0x142aef03c / 0x142aa4418，两处同值）`disc 11→slot 5`。重建代码 `SubworldIndex` 枚举序（Terrain=13）错误；`RaycastGeoSource` 实为 24B {disc,flag 8B}（重建 8B 模型错），其 `subworld()`=查该映射表。

4. **`RaycastWorldResult2` = 扁平 0x40B**：{handle 24B, point@+0x18, normal@+0x24, toi@+0x30, marker u32@+0x34(0=命中/2=无), uv@+0x38}。`rapier_hit_to_raycast_world_result`（0x1408ce4a0）按三角形 barycentric 插值法线/UV 并 quat 旋转法线。重建的 {Option<RaycastHit>,source,handle} 模型错误。

5. **`raycast_simple_exclude_wall_id`（0x1408ccc50）的 exclude 参数在本构建被忽略**（纯委托 `Rapier::raycast_simple`）；`raycast_simple` 语义=**后命中覆盖**（不比较距离）；`legacy_spherecast`（0x1408ccc90）用**固定 10 槽顺序 {0,1,3,7,6,4,11,2,12,13}**，被 critters（avoid_collisions/move_critters）消费。

6. **v1 符号清单（sysres-procedures.json）系统性错位，v2（S_PUB32）为准**——本域已用机器码证实 10+ 处：0x1408ccc90=legacy_spherecast（v1 称 raycast_simple）、0x1408cd5c0=flush_colliders（v1 称 intersects_any_w_terrain）、0x1408ffef0=remove_node（v1 称 disconnect_edge）、0x1408ffd70=add_edge（v1 称 add_node）、0x14094c800=closest_segment_index、0x1408d9590=dispose_collider_mesh_instance、0x140a879b0=Debug impl（v1 称 contains_edge_or_stub_impl）等。wp21 合同地址大多正确。

7. **StairsState 布局自洽性确认**：endpoint 24B @+0x10..+0x27 → **GraphMap 起点 @+0x28**（graph getter=`&self+0x28`）；**脏边/桩集合 @+0x148**（add_node/remove_node 写入；round2 更正初版 +0x29 笔误——系 Ghidra 指针算术）；脏字节 +0x2f（变更置 0、restore 置 1）；backed_up @+0x178（add_node 清零）；边 map @+0x78/+0x80、节点 map @+0x60/+0x68。重建代码的字段布局与 restore_backup 脏标志极性（代码 false vs 机器 1）需修。

8. **structural_graph 断言串确认字段名与不变量**：`assertion failed: self.span_to_collider_handle.remove(&span).is_some()`、`self.graph.remove_node(prev)`、`self.graph.remove_node(collider)`——重建语义模型正确，但 8 张 hashbrown 表实际偏移（+0x90/+0xb0/+0xc0/+0xf0/+0x110/+0x150）与 std HashMap 建模不符。

9. **ClutterMeshTy 真实为 56 个资产路径键控变体**（shovel/bucket/fallen_leaves/bench/pumpkin2/anvil/door_bell…，From<&str> 68KB + data 表 switch 0..0x37 + enum_iterator::Sequence），重建代码的 11 变体粗分类（Plants/Watering/…）是错误模型。`ClutterMarkedForDeletion` 等组件面 U。

10. **stairs/critters/ivy 小函数手解补全**：`StairNodeAnchor::try_as_wall` 判别=2 才返 payload（重建 Wall=0 错）；`AlignKind::position_offset` 真实计算 {flag,Vec2,Vec2}+缩放（重建 ZERO stub 错）；`Sheep::being_petted`=+0x58>0.0；`BirdSpotManager::gen_id`=+8 自增返旧值；`xz_flatten_aabb` 输出 {min.x,min.y,max.y}；`mesh_intersect_collider_diff_publisher`=&RaycastWorld+0x3cb0；`prev_requests`=&self+0x138。ivy grower 主系统= 10 相跳表（0x142ab3be0，索引 world+0x118）分派，**主闭包=0x14094ac20**（在 0x14094b06a 调 closest_segment_index；28B 墙段投影+max_ivy_grow_height 过滤）；add_node 64B 全手解（+0x00/+0x178 清零→GraphMap::add_node→+0x148 自环桩）。
