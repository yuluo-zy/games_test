# systems/collision/world.rs

> 文件作用：**rapier3d 碰撞世界胶水：17 子世界槽、ray/sphere/shape 查询、dirty 瓦片生命周期（flush/publish/QueryPipeline::update）、collider upsert/remove 泛型族、parry 网格数据构造**

函数总计 170：业务 75 / 内联无独立符号 36 / 派生胶水 54

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::collision::world::RaycastMeshData::new_convex` | 0x1408cec10 | 0 | 构造 | O：RaycastMeshData::new_convex——parry convex_hull；Some 写 tag=4、Box 0xd0@+0x20、vtable@+0x28、hash@+0x70 | collision-eco\decomp\decomp-0x1408cec10.txt |
| `country_core::systems::collision::world::Raycasting::spherecast` | 0x1408cc390 | 0 | 几何/碰撞 | O：Raycasting::spherecast——旧包装（转发 RaycastWorld::spherecast） | collision-eco\decomp\decomp-0x1408cc390.txt |
| `country_core::systems::collision::world::_::impl$0::apply` | 0x14003bb80 | 53 | Bevy 参数胶水 | O：SystemParam 胶水（apply/queue/validate） | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::systems::collision::world::_::impl$0::init_state` | 0x1408d99b0 | 53 | Bevy 参数胶水 | O：SystemParam impl（get_param 包装） | collision-eco\decomp\decomp-0x1408d99b0.txt |
| `country_core::systems::collision::world::_::impl$0::new_archetype` | 0x14003bb80 | 53 | Bevy 参数胶水 | O：SystemParam 生命周期胶水 | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::systems::collision::world::_::impl$0::queue` | 0x14003bb80 | 53 | Bevy 参数胶水 | O：SystemParam 胶水（apply/queue/validate） | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::systems::collision::world::Raycasting::raycast` | 0x1408cc2e0 | 60 | 几何/碰撞 | O：Raycasting::raycast——转发 raycast_w_terrain（mask 来自调用者；251B 包装） | collision-eco\decomp\decomp-0x1408cc2e0.txt |
| `country_core::systems::collision::world::Raycasting::intersects_any` | 0x1408cc320 | 70 | 几何/碰撞 | O：Raycasting::intersects_any——旧包装（掩码=0/语义待核） | collision-eco\decomp\decomp-0x1408cc320.txt |
| `country_core::systems::collision::world::SubworldColliderHandle::new` | 0x1408cc620 | 210 | 构造 | O：SubworldColliderHandle::new=写 {u32@+0(index),u32@+4(gen),u8@+8(slot)}；24B 枚举 | collision-eco\decomp\decomp-0x1408cc620.txt |
| `country_core::systems::collision::world::impl$11::from` | 0x1408cc630 | 335 | 转换 | I：From 转换 |  |
| `country_core::systems::collision::world::RaycastWorld::mesh_intersect_collider_diff_publisher` | 0x1408cc670 | 363 | 查询/计算 | O：返回 &RaycastWorld+0x3cb0（tabloid Publisher 字段） | collision-eco\decomp\decomp-0x1408cc670.txt |
| `country_core::systems::collision::world::RaycastWorld::legacy_raycast` | 0x1408cc680 | 371 | 几何/碰撞 | O：legacy_raycast——mask=0 的 rapier raycast（无命中语义，遗留） | collision-eco\decomp\decomp-0x1408cc680.txt |
| `country_core::systems::collision::world::RaycastWorld::legacy_raycast_w_params` | 0x1408cc770 | 378 | 几何/碰撞 | O：legacy_raycast_w_params——同上带参数 | collision-eco\decomp\decomp-0x1408cc770.txt |
| `country_core::systems::collision::world::RaycastWorld::raycast` | 0x1408cc870 | 401 | 几何/碰撞 | O：RaycastWorld::raycast→Rapier::raycast（17 槽 mask 遍历+最近命中） | collision-eco\decomp\decomp-0x1408cc870.txt |
| `country_core::systems::collision::world::RaycastWorld::raycast_intersections_with_ray` | 0x1408cc8a0 | 411 | 几何/碰撞 | O：raycast_intersections_with_ray——全交点收集 | collision-eco\decomp\decomp-0x1408cc8a0.txt |
| `country_core::systems::collision::world::RaycastWorld::raycast_simple` | 0x1408ccc20 | 423 | 几何/碰撞 | O：raycast_simple——纯委托 Rapier::raycast_simple（39B） | collision-eco\decomp\decomp-0x1408ccc20.txt |
| `country_core::systems::collision::world::RaycastWorld::raycast_simple_exclude_wall_id` | 0x1408ccc50 | 433 | 几何/碰撞 | O：raycast_simple_exclude_wall_id——同样纯委托；exclude 参数本构建被忽略 | collision-eco\decomp\decomp-0x1408ccc50.txt |
| `country_core::systems::collision::world::RaycastWorld::legacy_spherecast` | 0x1408ccc90 | 446 | 几何/碰撞 | O：legacy_spherecast——固定 10 槽顺序 {0,1,3,7,6,4,11,2,12,13}；critters 消费（avoid/move） | collision-eco\decomp\decomp-0x1408ccc90.txt |
| `country_core::systems::collision::world::RaycastWorld::spherecast` | 0x1408ccfc0 | 454 | 几何/碰撞 | O：spherecast——17 槽 mask 驱动 cast_shape 最近命中 | collision-eco\decomp\decomp-0x1408ccfc0.txt |
| `country_core::systems::collision::world::RaycastWorld::shape_overlaps_with<utils::geometry::rectangle::Rectangle3d>` | 0x142612d10 | 464 | 几何/碰撞 | O：shape_overlaps_with<Rectangle3d>——矩形形状重叠查询 | collision-eco\decomp\decomp-0x142612d10.txt |
| `country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps<country_core::systems::collision::world::SubworldColliderHandle,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env$0,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env$1>` | 0x14129b780 | 472 | 几何/碰撞 | O：iter_shape_overlaps（calculate_distortion_from_structural_graph 闭包实例1）——QBVH 相交遍历 | collision-eco\decomp\decomp-0x14129b780.txt |
| `country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps<country_core::systems::collision::world::SubworldColliderHandle,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env$2,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env$3>` | 0x14129bad0 | 472 | 几何/碰撞 | O：iter_shape_overlaps（同上实例2） | collision-eco\decomp\decomp-0x14129bad0.txt |
| `country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> >` | 0x140254aa0 | 472 | 几何/碰撞 | O：iter_shape_overlaps（closure_env$21<false> 实例） | collision-eco\decomp\decomp-0x140254aa0.txt |
| `country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> >` | 0x140254890 | 472 | 几何/碰撞 | O：iter_shape_overlaps（playermade_stairs_assemble closure_env$21<true> 实例） | collision-eco\decomp\decomp-0x140254890.txt |
| `country_core::systems::collision::world::RaycastWorld::decorator_overlaps_with_others<tuple$<country_core::systems::collision::world::OffsetColliderTransform,alloc::sync::Arc<dyn$<parry3d::shape::shape::Shape>,alloc::alloc::Global> > >` | 0x14129be00 | 482 | 几何/碰撞 | O：decorator_overlaps_with_others——装饰物与他物重叠检测（QBVH） | collision-eco\decomp\decomp-0x14129be00.txt |
| `country_core::systems::collision::world::RaycastWorld::raycast_w_terrain` | 0x1408cd430 | 498 | 几何/碰撞 | O：raycast_w_terrain——mask 位0x20(槽5=Terrain)时先碰撞后地形（地形以碰撞 toi 为上限）；地形命中=byte0 0xb+法线(0,1,0) | collision-eco\decomp\decomp-0x1408cd430.txt |
| `country_core::systems::collision::world::RaycastWorld::intersects_any` | 0x1408cd5b0 | 545 | 几何/碰撞 | O：intersects_any——5B thunk→0x1408cdf80 | collision-eco\decomp\decomp-0x1408cd5b0.txt |
| `country_core::systems::collision::world::RaycastWorld::flush_colliders` | 0x1408cd5c0 | 565 | 系统:刷新 | O：flush_colliders——逐槽脏标志(+0x388)→drain 0x200B 脏瓦片(+0x188)→tabloid publish<DirtyTiles>→log "Flushing colliders of {..}"→QueryPipeline::update | collision-eco\decomp\decomp-0x1408cd5c0.txt |
| `country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> >` | 0x140252c90 | 610 | 系统:写入 | O：upsert_collider<StairGraphEdge,(Vec3,Quat)> | collision-eco\decomp\decomp-0x140252c90.txt |
| `country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,utils::IsoTransform>` | 0x141296510 | 610 | 系统:写入 | O：upsert_collider<StairGraphEdge,IsoTransform> | collision-eco\decomp\decomp-0x141296510.txt |
| `country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::WallId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> >` | 0x1412453c0 | 610 | 系统:写入 | O：upsert_collider<WallId,(Vec3,Quat)> | collision-eco\decomp\decomp-0x1412453c0.txt |
| `country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,country_core::systems::collision::world::OffsetColliderTransform>` | 0x141298110 | 610 | 系统:写入 | O：upsert_collider<DecoratorId,OffsetColliderTransform> | collision-eco\decomp\decomp-0x141298110.txt |
| `country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> >` | 0x141294980 | 610 | 系统:写入 | O：upsert_collider<DecoratorId,(Vec3,Quat)> | collision-eco\decomp\decomp-0x141294980.txt |
| `country_core::systems::collision::world::RaycastWorld::upsert_collider<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,utils::IsoTransform>` | 0x141299ca0 | 610 | 系统:写入 | O：upsert_collider<(StairGraphEdge,u32),IsoTransform> | collision-eco\decomp\decomp-0x141299ca0.txt |
| `country_core::systems::collision::world::Rapier::get_collider` | 0x1408cdb50 | 631 | 查询/计算 | O：get_collider（Rapier/RaycastWorld 折叠）——rapier 竞技场回查 {index,gen,slot}→0x28B 游戏侧视图 | collision-eco\decomp\decomp-0x1408cdb50.txt |
| `country_core::systems::collision::world::RaycastWorld::get_collider` | 0x1408cdb50 | 631 | 查询/计算 | O：get_collider（Rapier/RaycastWorld 折叠）——rapier 竞技场回查 {index,gen,slot}→0x28B 游戏侧视图 | collision-eco\decomp\decomp-0x1408cdb50.txt |
| `country_core::systems::collision::world::Rapier::get_collider_mesh` | 0x1408cdbd0 | 638 | 查询/计算 | O：get_collider_mesh（折叠）——rapier 竞技场回查（entry 0x28B，gen 标签@+0x20） | collision-eco\decomp\decomp-0x1408cdbd0.txt |
| `country_core::systems::collision::world::RaycastWorld::get_collider_mesh` | 0x1408cdbd0 | 638 | 查询/计算 | O：get_collider_mesh（折叠）——rapier 竞技场回查（entry 0x28B，gen 标签@+0x20） | collision-eco\decomp\decomp-0x1408cdbd0.txt |
| `country_core::systems::collision::world::RaycastWorld::remove_collider<country_core::resources::stairs::stairs_state::StairGraphEdge>` | 0x140252ad0 | 643 | 系统:写入 | O：remove_collider<StairGraphEdge>——invalidate ×3 调用 | collision-eco\decomp\decomp-0x140252ad0.txt |
| `country_core::systems::collision::world::RaycastWorld::remove_collider<country_core::resources::walls::WallId>` | 0x141245220 | 643 | 系统:写入 | O：remove_collider<WallId>（v2；v1 标 get_rapier_collider 错） | collision-eco\decomp\decomp-0x141245220.txt |
| `country_core::systems::collision::world::RaycastWorld::remove_collider<country_core::resources::walls::decorator::DecoratorId>` | 0x140b409b0 | 643 | 系统:写入 | O：remove_collider<DecoratorId> | collision-eco\decomp\decomp-0x140b409b0.txt |
| `country_core::systems::collision::world::RaycastWorld::subscribe_to_changes` | 0x1408cdc20 | 665 | 系统:订阅 | O：subscribe_to_changes=tabloid Publisher<DirtyTiles>::subscribe（25B）——碰撞世界订阅脏瓦片 | collision-eco\decomp\decomp-0x1408cdc20.txt |
| `country_core::systems::collision::world::Rapier::subworlds` | 0x1408cdc40 | 687 | 查询/计算 | O：Rapier::subworlds——返回 [self+0x20, self+0x3cb0)（17 槽切片） | collision-eco\decomp\decomp-0x1408cdc40.txt |
| `country_core::systems::collision::world::Rapier::raycast` | 0x1408cdc60 | 895 | 几何/碰撞 | O：Rapier::raycast——17 槽 mask 遍历 cast_ray_and_get_normal（pipeline 全局 0x142aa3858）→rapier_hit_to_raycast_world_result | collision-eco\decomp\decomp-0x1408cdc60.txt |
| `country_core::systems::collision::world::Rapier::intersects_any` | 0x1408cdf80 | 947 | 几何/碰撞 | O：Rapier::intersects_any——mask 遍历任一命中 | collision-eco\decomp\decomp-0x1408cdf80.txt |
| `country_core::systems::collision::world::Rapier::raycast_simple` | 0x1408ce180 | 996 | 几何/碰撞 | O：Rapier::raycast_simple——cast_ray（pipeline 0x142aa37f8）；后命中覆盖；无命中 byte0=0x11 | collision-eco\decomp\decomp-0x1408ce180.txt |
| `country_core::systems::collision::world::Rapier::get_rapier_collider` | 0x1408ce440 | 1221 | 查询/计算 | O：Rapier::get_rapier_collider——槽号+索引→rapier ColliderHandle（手解） | collision-eco\decomp\decomp-0x1408ce440.txt |
| `country_core::systems::collision::world::Rapier::on_collider_upserted` | 0x1408ce460 | 1230 | 系统:事件响应 | O：Rapier::on_collider_upserted（upsert 后通知，v2 名址） | collision-eco\decomp\decomp-0x1408ce460.txt |
| `country_core::systems::collision::world::rapier_hit_to_raycast_world_result` | 0x1408ce4a0 | 1262 | 几何/碰撞 | O：rapier_hit_to_raycast_world_result——三角形 feature==2 时 barycentric 插值法线/uv+quat 旋转；无索引法线取原始命中；组装 0x40B 结果 | collision-eco\decomp\decomp-0x1408ce4a0.txt |
| `country_core::systems::collision::world::SubworldIndex::from_usize_unchecked` | 0x14067dd80 | 1456 | 转换 | O：SubworldIndex::from_usize_unchecked=恒等（mov rax,rcx；判别 0..16 连续） | audio-misc\decomp\decomp-0x14067dd80.txt, collision-eco\decomp\decomp-0x14067dd80.txt, terrain-water\decomp\decomp-0x14067dd80.txt |
| `country_core::systems::collision::world::RaycastGeoSource::subworld` | 0x1408cea20 | 1471 | 查询/计算 | O：RaycastGeoSource::subworld()=查 disc→槽表（0x142aa4415） | collision-eco\decomp\decomp-0x1408cea20.txt |
| `country_core::systems::collision::world::RaycastMesh::new` | 0x1408ceb10 | 1561 | 构造 | O：RaycastMesh::new（key/data/source 包装） | collision-eco\decomp\decomp-0x1408ceb10.txt |
| `country_core::systems::collision::world::RaycastMesh::data` | 0x1408ceb30 | 1566 | 查询/计算 | O：RaycastMesh::data getter | collision-eco\decomp\decomp-0x1408ceb30.txt |
| `country_core::systems::collision::world::RaycastMesh::source` | 0x140681f40 | 1569 | 查询/计算 | O：RaycastMesh::source——拷贝 24B（RaycastGeoSource={disc,flag 8B}） | collision-eco\decomp\decomp-0x140681f40.txt |
| `country_core::systems::collision::world::RaycastMesh::raycast` | 0x1408ceb40 | 1573 | 几何/碰撞 | O：RaycastMesh::raycast——parry shape vtable 虚调 cast_ray_and_get_normal（单网格） | collision-eco\decomp\decomp-0x1408ceb40.txt |
| `country_core::systems::collision::world::RaycastMeshData::new_indexed` | 0x1408cee40 | 1629 | 构造 | O：RaycastMeshData::new_indexed——顶点/法线/索引→parry trimesh+去重哈希 | collision-eco\decomp\decomp-0x1408cee40.txt |
| `country_core::systems::collision::world::RaycastMeshData::new_unindexed` | 0x1408cf7d0 | 1665 | 构造 | O：RaycastMeshData::new_unindexed——顺序索引化（36B 步长顶点）；写入面 +0x10/+0x18=indices{ptr,len} | collision-eco\decomp\decomp-0x1408cf7d0.txt |
| `country_core::systems::collision::world::RaycastMeshData::new_cuboid` | 0x1408cfff0 | 1710 | 构造 | O：RaycastMeshData::new_cuboid | collision-eco\decomp\decomp-0x1408cfff0.txt |
| `country_core::systems::collision::world::RaycastMeshData::new_sphere` | 0x1408d0120 | 1755 | 构造 | O：RaycastMeshData::new_sphere | collision-eco\decomp\decomp-0x1408d0120.txt |
| `country_core::systems::collision::world::RaycastMeshData::new_compound` | 0x1408d0230 | 1775 | 构造 | O：RaycastMeshData::new_compound | collision-eco\decomp\decomp-0x1408d0230.txt |
| `country_core::systems::collision::world::impl$21::to_offset_collider_builder` | 0x140b26a80 | 1818 | 转换 | O：impl$21::to_offset_collider_builder——转偏移 collider 构建器（round3 反编译 2569B 落盘；同族 impl$23=0x1408d0390） | collision-eco\decomp\decomp-0x140b26a80.txt |
| `country_core::systems::collision::world::impl$22::to_offset_collider_builder` | 0x140b26bd0 | 1849 | 构造 | O：new_compound 闭包 | collision-eco\decomp\decomp-0x140b26bd0.txt |
| `country_core::systems::collision::world::impl$23::to_offset_collider_builder` | 0x1408d0390 | 1859 | 转换 | O：impl$23::to_offset_collider_builder——转偏移 collider 构建器 | collision-eco\decomp\decomp-0x1408d0390.txt |
| `country_core::systems::collision::world::update_rapier_colliders` | 0x1408d0490 | 1875 | 系统:刷新 | O/I：update_rapier_colliders（0x1408d0490..0x1408d5e50）——puffin 系统；变更网格→parry 重建→upsert_collider_entry→dispose_collider_mesh_instance（Ghidra varnode 失败，管线经 callgraph 证实） | collision-eco\decomp\decomp-0x1408d0490.txt |
| `country_core::systems::collision::world::construct_arch_geo` | 0x1408d7ed0 | 2128 | 构造 | O：update_rapier_colliders 闭包——arch/floor 网格构造 helper（7482B） | collision-eco\decomp\decomp-0x1408d7ed0.txt |
| `country_core::systems::collision::world::flush_rapier_colliders` | 0x1408d8360 | 2151 | 系统:刷新 | O：flush_rapier_colliders——puffin+ResMut<RaycastWorld>→flush_colliders | collision-eco\decomp\decomp-0x1408d8360.txt |
| `country_core::systems::collision::world::RaycastWorld::on_level_reload` | 0x1408d8710 | 2161 | 系统:生命周期 | O：RaycastWorld::on_level_reload——清空 rapier 竞技场+注册表+网格 drop | collision-eco\decomp\decomp-0x1408d8710.txt |
| `country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> >` | 0x140b40b50 | 2169 | 系统:写入 | O：upsert_collider_entry——同 handle+同几何→未变(status2)；几何变→dispose+新建 rapier collider（user_data 编码 handle）+pipeline update；panic 一致性断言 | collision-eco\decomp\decomp-0x140b40b50.txt |
| `country_core::systems::collision::world::dispose_collider_mesh_instance` | 0x1408d9590 | 2299 | 释放 | O：dispose_collider_mesh_instance——回收 collider 网格实例（v2 名） | collision-eco\decomp\decomp-0x1408d9590.txt |
| `country_core::systems::collision::world::xz_flatten_aabb` | 0x1408d98a0 | 2335 | 几何/碰撞 | O：xz_flatten_aabb——输出 **{min.x,min.z,max.x,max.z}**（4×f32：in[0]/in[8]/in[0xc]/in[0x14]；review-audit 纠错：非 {min.x,min.y,max.y}） | collision-eco\decomp\decomp-0x1408d98a0.txt |
| `country_core::systems::collision::world::impl$41::into_transformed_shape` | 0x14155d340 | 2486 | 几何/碰撞 | U：v1 清单误标 xz_flatten_aabb；真实函数名待定（v2 无此符号） | collision-eco\decomp\decomp-0x14155d340.txt |
| `country_core::systems::collision::world::impl$42::into_transformed_shape` | 0x1408d98c0 | 2505 | 转换 | O：into_transformed_shape 族之一（parry 形状变换） | collision-eco\decomp\decomp-0x1408d98c0.txt |
| `country_core::systems::collision::world::impl$43::into_transformed_shape` | 0x1408d9960 | 2520 | 转换 | O：as_collider_shape——0x30B 拷贝（parry 形状转换） | collision-eco\decomp\decomp-0x1408d9960.txt |
| `country_core::systems::collision::world::impl$45::into_transformed_shape` | 0x140b26d20 | 2550 | 转换 | O：into_transformed_shape 族 | collision-eco\decomp\decomp-0x140b26d20.txt |
| `country_core::systems::collision::world::impl$47::as_collider_shape` | 0x1408d9990 | 2582 | 构造 | O：Vec 容量对齐内部（std 胶水，DEPENDENCY_FILTERING 跳过） | collision-eco\decomp\decomp-0x1408d9990.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::collision::world::_::impl$0::get_param`（行 53）— 查询/计算
- `country_core::systems::collision::world::_::impl$0::validate_param`（行 53）— 谓词/校验
- `country_core::systems::collision::world::impl$6::empty`（行 222）— O：ColliderDiff::empty——空差异（delta=None）
- `country_core::systems::collision::world::impl$7::apply_to`（行 233）— O：ColliderDiff::apply_to——置 delta=Some(Added/Removed)
- `country_core::systems::collision::world::RaycastMeshKey::from_rapier_collider`（行 256）— O：RaycastMeshKey::from_rapier_collider——rapier ColliderHandle 索引转键
- `country_core::systems::collision::world::RaycastMeshKey::from_rapier_user_data`（行 261）— O：RaycastMeshKey::from_rapier_user_data——user_data u128 低 64 位转键
- `country_core::systems::collision::world::ColliderRegistry::keys`（行 271）— O：ColliderRegistry::keys——遍历句柄键
- `country_core::systems::collision::world::ColliderRegistry::get`（行 275）— 查询/计算
- `country_core::systems::collision::world::RaycastWorld::intersects_any_w_terrain`（行 549）— 几何/碰撞
- `country_core::systems::collision::world::RaycastWorld::get_rapier_collider`（行 621）— 查询/计算
- `country_core::systems::collision::world::RaycastWorld::retain_colliders`（行 651）— I：retain_colliders——按谓词保留 collider（stairs/autoclutter 内联）
- `country_core::systems::collision::world::Rapier::legacy_spherecast`（行 695）— I：Rapier 层 legacy_spherecast（固定 10 槽序，见 0x1408ccc90）
- `country_core::systems::collision::world::Rapier::spherecast`（行 751）— I：Rapier 层 spherecast（mask 驱动，见 0x1408ccfc0）
- `country_core::systems::collision::world::Rapier::legacy_raycast`（行 805）— I：Rapier 层 legacy_raycast（mask=0，见 0x1408cc680）
- `country_core::systems::collision::world::Rapier::raycast_intersections_with_ray`（行 826）— 几何/碰撞
- `country_core::systems::collision::world::Rapier::shape_overlaps_with`（行 1052）— I：Rapier 层形状重叠查询（见 0x142612d10）
- `country_core::systems::collision::world::Rapier::iter_shape_overlaps`（行 1082）— 迭代/变换
- `country_core::systems::collision::world::Rapier::decorator_overlaps_with_others`（行 1148）— I：Rapier 层装饰物重叠检测（见 0x14129be00）
- `country_core::systems::collision::world::Rapier::on_collider_removed`（行 1235）— 系统:事件响应
- `country_core::systems::collision::world::Rapier::clear`（行 1240）— 删除/清理
- `country_core::systems::collision::world::impl$15::from`（行 1363）— I：From 转换（RaycastGeoSourceFlag 族）
- `country_core::systems::collision::world::SubworldIndex::to_bitflag`（行 1462）— O：SubworldIndex::to_bitflag=1<<槽号（Terrain=槽5→0x20；stairs_assemble 内联）
- `country_core::systems::collision::world::update_arch_collisions`（行 2066）— 系统:周期性处理
- `country_core::systems::collision::world::upsert_collider_entry`（行 2169）— O：upsert_collider 内部实现（0x140b40b50；同几何去重/重建）
- `country_core::systems::collision::world::remove_collider_entry`（行 2285）— 删除/清理
- `country_core::systems::collision::world::impl$25::to_rapier`（行 2351）— I：glam→rapier 转换族（Isometry/Vector/Point 各实例）
- `country_core::systems::collision::world::impl$27::to_rapier`（行 2363）— I：glam→rapier 转换族（Isometry/Vector/Point 各实例）
- `country_core::systems::collision::world::impl$29::to_rapier`（行 2375）— I：glam→rapier 转换族（Isometry/Vector/Point 各实例）
- `country_core::systems::collision::world::impl$31::to_rapier`（行 2391）— I：glam→rapier 转换族（Isometry/Vector/Point 各实例）
- `country_core::systems::collision::world::impl$32::to_rapier`（行 2397）— I：glam→rapier 转换族（Isometry/Vector/Point 各实例）
- `country_core::systems::collision::world::impl$33::to_rapier`（行 2403）— I：glam→rapier 转换族（Isometry/Vector/Point 各实例）
- `country_core::systems::collision::world::impl$34::to_glam`（行 2412）— I：rapier→glam 转换族
- `country_core::systems::collision::world::impl$35::to_glam`（行 2418）— I：rapier→glam 转换族
- `country_core::systems::collision::world::impl$36::to_glam`（行 2424）— I：rapier→glam 转换族
- `country_core::systems::collision::world::impl$38::to_glam`（行 2436）— I：rapier→glam 转换族
- `country_core::systems::collision::world::floor_mesh`（行 2585）— I：地板网格构造（new_cuboid 包装；update_rapier_colliders 内联）

## 派生胶水（derive/宏生成；不计独立业务）

`Global> > >`、`clone`、`closure`、`closure_env`、`cmp`、`default`、`eq`、`fmt`、`hash`、`index`、`index_mut`、`partial_cmp` 等 54 项
