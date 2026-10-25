# -*- coding: utf-8 -*-
"""Round-3 chapter filler: parse domain chapter files, fill conclusions from
knowledge base + decomp evidence. Run: python fill_chapters.py [--dry]"""
import re, os, sys, json

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
BASE = os.path.normpath(os.path.join(ROOT, 'chapters'))
DECOMP = os.path.normpath(os.path.join(ROOT, 'collision-eco', 'decomp'))
DRY = '--dry' in sys.argv

DOMAIN_PREFIXES = ['systems/collision', 'systems/paths', 'systems/ivy',
                   'resources/stairs', 'resources/ivy', 'resources/critters']
DOMAIN_FILES = {'resources/clutter_meshes.rs.md', 'resources/preplaced_autoclutter.rs.md'}

# --------------------------------------------------------------------------
# Knowledge base: VA -> (作用, 结论)  (结论: O=机器码证实 / I=推断 / U=未知)
# --------------------------------------------------------------------------
KB = {}

def add(va, role, concl):
    KB.setdefault(va, []).append((role, concl))

# --- world.rs (v2 权威名址) ---
W = {
 '0x1408cc2e0': ('几何/碰撞', 'O：Raycasting::raycast——转发 raycast_w_terrain（mask 来自调用者；251B 包装）'),
 '0x1408cc320': ('几何/碰撞', 'O：Raycasting::intersects_any——旧包装（掩码=0/语义待核）'),
 '0x1408cc390': ('几何/碰撞', 'O：Raycasting::spherecast——旧包装（转发 RaycastWorld::spherecast）'),
 '0x1408cc620': ('构造', 'O：SubworldColliderHandle::new=写 {u32@+0(index),u32@+4(gen),u8@+8(slot)}；24B 枚举'),
 '0x1408cc670': ('查询/计算', 'O：返回 &RaycastWorld+0x3cb0（tabloid Publisher 字段）'),
 '0x1408cc680': ('几何/碰撞', 'O：legacy_raycast——mask=0 的 rapier raycast（无命中语义，遗留）'),
 '0x1408cc770': ('几何/碰撞', 'O：legacy_raycast_w_params——同上带参数'),
 '0x1408cc870': ('几何/碰撞', 'O：RaycastWorld::raycast→Rapier::raycast（17 槽 mask 遍历+最近命中）'),
 '0x1408cc8a0': ('几何/碰撞', 'O：raycast_intersections_with_ray——全交点收集'),
 '0x1408ccc20': ('几何/碰撞', 'O：raycast_simple——纯委托 Rapier::raycast_simple（39B）'),
 '0x1408ccc50': ('几何/碰撞', 'O：raycast_simple_exclude_wall_id——同样纯委托；exclude 参数本构建被忽略'),
 '0x1408ccc90': ('几何/碰撞', 'O：legacy_spherecast——固定 10 槽顺序 {0,1,3,7,6,4,11,2,12,13}；critters 消费（avoid/move）'),
 '0x1408ccfc0': ('几何/碰撞', 'O：spherecast——17 槽 mask 驱动 cast_shape 最近命中'),
 '0x1408cd430': ('几何/碰撞', 'O：raycast_w_terrain——mask 位0x20(槽5=Terrain)时先碰撞后地形（地形以碰撞 toi 为上限）；地形命中=byte0 0xb+法线(0,1,0)'),
 '0x1408cd5b0': ('几何/碰撞', 'O：intersects_any——5B thunk→0x1408cdf80'),
 '0x1408cd5c0': ('系统:刷新', 'O：flush_colliders——逐槽脏标志(+0x388)→drain 0x200B 脏瓦片(+0x188)→tabloid publish<DirtyTiles>→log "Flushing colliders of {..}"→QueryPipeline::update'),
 '0x1408cdb50': ('查询/计算', 'O：get_collider（Rapier/RaycastWorld 折叠）——rapier 竞技场回查 {index,gen,slot}→0x28B 游戏侧视图'),
 '0x1408cdbd0': ('查询/计算', 'O：get_collider_mesh（折叠）——rapier 竞技场回查（entry 0x28B，gen 标签@+0x20）'),
 '0x1408cdc20': ('系统:订阅', 'O：subscribe_to_changes=tabloid Publisher<DirtyTiles>::subscribe（25B）——碰撞世界订阅脏瓦片'),
 '0x1408cdc40': ('查询/计算', 'O：Rapier::subworlds——返回 [self+0x20, self+0x3cb0)（17 槽切片）'),
 '0x1408cdc60': ('几何/碰撞', 'O：Rapier::raycast——17 槽 mask 遍历 cast_ray_and_get_normal（pipeline 全局 0x142aa3858）→rapier_hit_to_raycast_world_result'),
 '0x1408cdf80': ('几何/碰撞', 'O：Rapier::intersects_any——mask 遍历任一命中'),
 '0x1408ce180': ('几何/碰撞', 'O：Rapier::raycast_simple——cast_ray（pipeline 0x142aa37f8）；后命中覆盖；无命中 byte0=0x11'),
 '0x1408ce440': ('查询/计算', 'O：Rapier::get_rapier_collider——槽号+索引→rapier ColliderHandle（手解）'),
 '0x1408ce460': ('系统:事件响应', 'O：Rapier::on_collider_upserted（upsert 后通知，v2 名址）'),
 '0x1408ce4a0': ('几何/碰撞', 'O：rapier_hit_to_raycast_world_result——三角形 feature==2 时 barycentric 插值法线/uv+quat 旋转；无索引法线取原始命中；组装 0x40B 结果'),
 '0x14067dd80': ('转换', 'O：SubworldIndex::from_usize_unchecked=恒等（mov rax,rcx；判别 0..16 连续）'),
 '0x1408cea20': ('查询/计算', 'O：RaycastGeoSource::subworld()=查 disc→槽表（0x142aa4418）'),
 '0x1408ceb10': ('构造', 'O：RaycastMesh::new（key/data/source 包装）'),
 '0x1408ceb30': ('查询/计算', 'O：RaycastMesh::data getter'),
 '0x1408ceb40': ('几何/碰撞', 'O：RaycastMesh::raycast——parry shape vtable 虚调 cast_ray_and_get_normal（单网格）'),
 '0x140681f40': ('查询/计算', 'O：RaycastMesh::source——拷贝 24B（RaycastGeoSource={disc,flag 8B}）'),
 '0x1408cec10': ('构造', 'O：RaycastMeshData::new_convex——parry convex_hull；Some 写 tag=4、Box 0xd0@+0x20、vtable@+0x28、hash@+0x70'),
 '0x1408cee40': ('构造', 'O：RaycastMeshData::new_indexed——顶点/法线/索引→parry trimesh+去重哈希'),
 '0x1408cf7d0': ('构造', 'O：RaycastMeshData::new_unindexed——顺序索引化（36B 步长顶点）；写入面 +0x10/+0x18=indices{ptr,len}'),
 '0x1408cfff0': ('构造', 'O：RaycastMeshData::new_cuboid'),
 '0x1408d0120': ('构造', 'O：RaycastMeshData::new_sphere'),
 '0x1408d0230': ('构造', 'O：RaycastMeshData::new_compound'),
 '0x1408d0490': ('系统:刷新', 'O/I：update_rapier_colliders（0x1408d0490..0x1408d5e50）——puffin 系统；变更网格→parry 重建→upsert_collider_entry→dispose_collider_mesh_instance（Ghidra varnode 失败，管线经 callgraph 证实）'),
 '0x1408d8360': ('系统:刷新', 'O：flush_rapier_colliders——puffin+ResMut<RaycastWorld>→flush_colliders'),
 '0x1408d8710': ('系统:生命周期', 'O：RaycastWorld::on_level_reload——清空 rapier 竞技场+注册表+网格 drop'),
 '0x1408d9590': ('释放', 'O：dispose_collider_mesh_instance——回收 collider 网格实例（v2 名）'),
 '0x1408d98a0': ('几何/碰撞', 'O：xz_flatten_aabb——输出 {min.x,min.y,max.y}（AABB xz 展平）'),
 '0x1408d99b0': ('Bevy 参数胶水', 'O：SystemParam impl（get_param 包装）'),
 '0x1408d7ed0': ('构造', 'O：update_rapier_colliders 闭包——arch/floor 网格构造 helper（7482B）'),
 '0x140b40b50': ('系统:写入', 'O：upsert_collider_entry——同 handle+同几何→未变(status2)；几何变→dispose+新建 rapier collider（user_data 编码 handle）+pipeline update；panic 一致性断言'),
 '0x140252ad0': ('系统:写入', 'O：remove_collider<StairGraphEdge>——invalidate ×3 调用'),
 '0x140252c90': ('系统:写入', 'O：upsert_collider<StairGraphEdge,(Vec3,Quat)>'),
 '0x141245220': ('系统:写入', 'O：remove_collider<WallId>（v2；v1 标 get_rapier_collider 错）'),
 '0x1412453c0': ('系统:写入', 'O：upsert_collider<WallId,(Vec3,Quat)>'),
 '0x141294980': ('系统:写入', 'O：upsert_collider<DecoratorId,(Vec3,Quat)>'),
 '0x141296510': ('系统:写入', 'O：upsert_collider<StairGraphEdge,IsoTransform>'),
 '0x141298110': ('系统:写入', 'O：upsert_collider<DecoratorId,OffsetColliderTransform>'),
 '0x141299ca0': ('系统:写入', 'O：upsert_collider<(StairGraphEdge,u32),IsoTransform>'),
 '0x140b409b0': ('系统:写入', 'O：remove_collider<DecoratorId>'),
 '0x14129b780': ('几何/碰撞', 'O：iter_shape_overlaps（calculate_distortion_from_structural_graph 闭包实例1）——QBVH 相交遍历'),
 '0x14129bad0': ('几何/碰撞', 'O：iter_shape_overlaps（同上实例2）'),
 '0x140254890': ('几何/碰撞', 'O：iter_shape_overlaps（playermade_stairs_assemble closure_env$21<true> 实例）'),
 '0x140254aa0': ('几何/碰撞', 'O：iter_shape_overlaps（closure_env$21<false> 实例）'),
 '0x14129be00': ('几何/碰撞', 'O：decorator_overlaps_with_others——装饰物与他物重叠检测（QBVH）'),
 '0x14129c3d0': ('几何/碰撞', 'O：Rapier::decorator_overlaps_with_others 闭包'),
 '0x142612d10': ('几何/碰撞', 'O：shape_overlaps_with<Rectangle3d>——矩形形状重叠查询'),
 '0x14155d340': ('几何/碰撞', 'U：v1 清单误标 xz_flatten_aabb；真实函数名待定（v2 无此符号）'),
 '0x1408d0390': ('转换', 'O：impl$23::to_offset_collider_builder——转偏移 collider 构建器'),
 '0x1408d98c0': ('转换', 'O：into_transformed_shape 族之一（parry 形状变换）'),
 '0x1408d9960': ('转换', 'O：as_collider_shape——0x30B 拷贝（parry 形状转换）'),
 '0x1408d9990': ('构造', 'O：Vec 容量对齐内部（std 胶水，DEPENDENCY_FILTERING 跳过）'),
 '0x140b26bd0': ('构造', 'O：new_compound 闭包'),
 '0x140b26d20': ('转换', 'O：into_transformed_shape 族'),
}
for va, entry in W.items():
    if entry is not None:
        role, concl = entry
        add(va, role, concl)

# --- stairs_state.rs ---
S = {
 '0x1408fe5e0': ('构造', 'O：TransientStairEdgeIdAllocator::get_or_allocate——已有返 id，否则插 next+1'),
 '0x1408fe7b0': ('系统:写入', 'O：try_set_endpoint——候选邻边≥8→Unset，否则 Single'),
 '0x1408fe880': ('系统:写入', 'O：unset_endpoint=写 qword[+0x10]=3'),
 '0x1408fe890': ('查询/计算', 'O：endpoint getter——拷贝 24B（判别 qword@+0x10+数据@+0x18/+0x20）'),
 '0x1408fe8b0': ('查询/计算', 'O：graph getter=&self+0x28（GraphMap 起点）'),
 '0x1408fe8c0': ('查询/计算', 'O：get_all_connected_nodes——BFS 连通集+排序去重'),
 '0x1408fefe0': ('系统:备份', 'O：make_backup——0x90B 图状态备份'),
 '0x1408ff210': ('系统:备份', 'O：restore_backup——0x90B 回拷+置脏 +0x2f=1'),
 '0x1408ff590': ('系统:备份', 'O：restore_snapshot——checksum→diff→apply'),
 '0x1408ffd30': ('系统:写入', 'O：add_node（64B 手解）——+0x00/+0x178 清零→GraphMap::add_node(self+0x28)→+0x148 插 (n,n) 自环桩'),
 '0x1408ffd70': ('系统:写入', 'O：add_edge——自环 panic；归一化(min,max)；端点命中短路；3 处边/桩表插'),
 '0x1408ffe80': ('系统:写入', 'O：disconnect_edge——同端点→remove_node；否则 remove_edge+插桩'),
 '0x1408ffef0': ('系统:写入', 'O：remove_node——邻边插桩 (min,max)+自环 (n,n)→GraphMap::remove_node'),
 '0x140900010': ('系统:备份', 'O：extract_diff_and_make_backup——两快照差集 {removed,added}+备份'),
 '0x140900ba0': ('系统:备份', 'O：apply_diff——移除 removed/添加 added'),
 '0x140900e80': ('查询/计算', 'O：contains_edge_or_stub——a==b 节点存在性；否则边/桩存在性（+0x38/+0x80 单元素快路径）'),
 '0x140900fd0': ('查询/计算', 'O：contains_edge_or_stub_impl（非内联面）'),
 '0x140901120': ('构造', 'O：StairsSnapshot::new——边排序→checksum'),
 '0x140901260': ('构造', 'O：StairGraphEdge::new 与 StairSegment::new ICF 折叠——(min,max) 归一化 helper'),
 '0x140a879b0': ('派生胶水', 'O：Debug for StairsSnapshot/StairHistoryDiff（v1 名 contains_edge_or_stub_impl 错）'),
 '0x14157cec0': ('查询/计算', 'O：iter_edges_and_stubs 实例1（restore_backup 闭包）'),
 '0x14157d090': ('查询/计算', 'O：iter_edges_and_stubs 实例2（reduce_diff 闭包）'),
 '0x14157d200': ('查询/计算', 'O：iter_edges_and_stubs 实例3（take_snapshot 闭包）'),
}
for va, (role, concl) in S.items():
    add(va, role, concl)

# --- structural_graph.rs ---
SG = {
 '0x140869a30': ('系统:写入', 'O：clear——清 8 张表'),
 '0x140869d20': ('系统:写入', 'O：upsert_span——旧 span 有 collider 先解绑（断言 remove_node(prev)）→加节点/边/高/visited'),
 '0x140869f90': ('系统:写入', 'O：upsert_support_source_collider——只加节点/高/visited（Ghidra varnode 失败，合同语义）'),
 '0x140869ff0': ('系统:写入', 'O：add_edge_by_span——两 span collider 间加边（已存在重写 0）'),
 '0x14086a260': ('系统:写入', 'O：add_edge_by_collider——直接加边+IndexMap 写 0'),
 '0x14086a320': ('系统:写入', 'O：remove_node_by_collider——删邻边+remove_node+逆映射清理（断言 remove_node(collider)/span 移除）'),
 '0x14086a4c0': ('系统:写入', 'O：disconnect_node_by_collider——枚举边移除（保留节点）'),
}
for va, (role, concl) in SG.items():
    add(va, role, concl)

# --- stairs_assembly_params.rs ---
SA = {
 '0x140959ca0': ('查询/计算', 'O：PlayermadeStairsAssemblyParams::length_ws——点/高 zip→Vec3 链墙空间总长'),
 '0x140959e80': ('构造', 'O：StairWallAnchor::new——16B 拷贝 {Vec2 anchor, f32 angle, pad}'),
 '0x140959e90': ('查询/计算', 'O：wall_normal=(sin(angle),cos(angle))'),
 '0x140959ef0': ('查询/计算', 'O：StairNodeAnchor::try_as_wall——判别==2 才返 &payload(+8)（Wall 判别=2）'),
 '0x140959f00': ('查询/计算', 'O：AlignKind::position_offset——变体1={flag u8@+1,Vec2@+4,Vec2@+0xc} 组合+常量缩放'),
 '0x140959f50': ('查询/计算', 'O：AlignKind::outgoing_dir——判别 0/1 否则 panic'),
 '0x140274480': ('查询/计算', 'O：PlayermadeStairsAssemblyRequests::prev_requests=&self+0x138'),
}
for va, (role, concl) in SA.items():
    add(va, role, concl)

# --- stairs invalidate/visual ---
IV = {
 '0x140b37180': ('系统:失效', 'O：invalidate_dirty_stair_artifacts（3455B）——脏边 drain；每边 remove_collider<StairGraphEdge>×3+Arc drop+RtMesh 删+StairsVisualState::remove_edge；contains_edge_or_stub 决定撤视觉'),
 '0x140aa8210': ('系统:写入', 'O：MagicSupports::clear'),
 '0x140aa8620': ('系统:写入', 'O：StairsVisualState::remove_edge'),
 '0x140aa8a60': ('系统:写入', 'O：StairsVisualState::reset_edge（v1 force_rebuild 名错）'),
}
for va, (role, concl) in IV.items():
    add(va, role, concl)

# --- critters.rs ---
CR = {
 '0x140f85ba0': ('音频', 'O：ButterflyAudio::set_position——5B thunk→0x140a57da0'),
 '0x140f85bb0': ('构造', 'O：Duck::new——FMOD 事件 event:/SFX/Ducks/{Quack,TakeOff,Land}（wp10 合同）'),
 '0x140f86330': ('查询/计算', 'O：BirdSpotManager::gen_id——+8 自增返回旧值'),
 '0x140f86340': ('系统:写入', 'O：BirdSpotManager::create_spots——池复用/分配（wp10 合同）'),
 '0x1422f86e0': ('系统:写入', 'O：BirdSpotManager::recycle——+0x10 原子借用守卫后池回收'),
 '0x14089d4c0': ('查询/计算', 'O：Sheep::being_petted=f32@+0x58>0.0'),
 '0x140002370': ('折叠 stub', 'O：ICF 折叠（Duck::being_petted 等多符号共享）'),
 '0x140f8a840': ('折叠 stub', 'O：clone_behavior 折叠 stub（返回 {静态ptr,2}）'),
}
for va, (role, concl) in CR.items():
    add(va, role, concl)

# --- paths ---
PA = {
 '0x140ae4bd0': ('系统:状态机', 'O：DrawState::update——raw=ui_mode-2 夹取 0..3；raw∈{1,2}→mode=min(mode-1,raw)；返回 0/3'),
 '0x140ae4c20': ('查询/计算', 'O：PathRastersResources::as_float_grid——4 路 f32 采样视图（CPU+8/+0x10 vs GPU 暂存 +0x20/+0x28，按 +0x298 标志）'),
 '0x140ae4d00': ('系统:绘制', 'O：draw_path——UI 模式串 "eraser"/"path"/"cobblestone" 分派 9 个 world_raster::draw_raster 单态（每变体绘制+擦除组合）'),
 '0x140ae5b20': ('Bevy 参数胶水', 'O：SystemParam impl'),
 '0x140b1c010': ('系统:预览', 'O：path_brush_preview_size——按 UI 模式设置笔刷预览尺寸'),
 '0x140b1c270': ('系统:历史', 'O：record_path_history——4 命令事件游标差→历史写入'),
 '0x140b1dd70': ('Bevy 参数胶水', 'O：register_required_components 族'),
 '0x140b68e40': ('系统:注册', 'O：paths::impl$0::build——插件注册'),
}
for va, (role, concl) in PA.items():
    add(va, role, concl)

# --- ivy resources ---
IY = {
 '0x140869460': ('系统:写入', 'O：Ivy::add_point——push_back 40B 点（id=next+1，reserved=0）'),
 '0x1408694f0': ('查询/计算', 'O：Ivy::last_point——末点（空 unwrap 失败）'),
 '0x140869540': ('查询/计算', 'O：Ivy::last_segment_point_ids——末两段端点 id（len<2 expect 失败）'),
 '0x1408696f0': ('系统:写入', 'O：IvyStorage::spawn_ivy——RNG 采样根点建新株，next_ivy_id+1'),
 '0x1408fe280': ('系统:写入', 'O：IvyLeafStorage::add——push 104B IvyLeaf 到对应 ivy'),
 '0x141234590': ('查询/计算', 'O：IvyDirectionProposer::get_direction——3×noise3d+3×uniform 混合→θ∈[-40°,+40°]→(cosθ,sinθ)'),
 '0x140b199e0': ('转换', 'O：WallCoord::into_world_space——PublicWallState 曲线采样+法向偏移 0.63*0.5（按侧±）'),
}
for va, (role, concl) in IY.items():
    add(va, role, concl)

# --- ivy systems ---
IS = {
 '0x140944070': ('系统:注册', 'O：add_ivy_systems——插件系统注册'),
 '0x1409440c0': ('系统:注册', 'O：insert_ivy_resources——IvyStorage/IvyLeafStorage/IvySpawnNoise/IvyDirectionProposer'),
 '0x140944280': ('系统:注册', 'O：add_ivy_events'),
 '0x14094a910': ('系统:生长', 'O：ivy_grower（0x14094a910..0x14094c7fb，7915B 单函数）——10 相状态机（跳表 0x142ab3be0，索引 world+0x118）；主闭包 0x14094ac20：last_point→get_direction→closest_segment_index→墙跳→add_point'),
 '0x14094c800': ('几何/碰撞', 'O：closest_segment_index（666B）——28B 步长墙段 {xy0,xy1,h0@+0xc,h1@+0x1c} 投影+高度≤max_ivy_grow_height 过滤'),
 '0x14094caa0': ('几何/碰撞', 'O：get_wall_double_sided_normal——双面法线（v2 名址）'),
 '0x14094cc60': ('几何/碰撞', 'O：intersect_ivy_growth_w_wall_segment——生长线是否穿墙段（v2 名址）'),
 '0x14094cf90': ('系统:生成', 'O：ivy_spawner——try_random_ivy_root/try_ivy_root_next_to_cursor/is_point_inside_wall_segment'),
 '0x140995a80': ('系统:动画', 'O：ivy_branch_thickness_animation——分支粗细动画'),
 '0x140996080': ('系统:生成', 'O：ivy_leaf_spawner——消费 IvySegmentDoneGrowingMsg（"Received IvySegmentDoneGrowingMsg with ivy id "）沿末段放叶'),
 '0x140997410': ('系统:修剪', 'O：ivy_pruner（70KB）——墙空间哈希校验（"Spatial Hash returned a wall id "）+逐点有效性'),
 '0x140998dc0': ('查询/计算', 'O：is_point_still_valid——点仍有效检查（含 closest_segment_index 调用）'),
 '0x140999330': ('系统:命令', 'O：ivy_pruner_cmd——RemoveIvyCmd 消费'),
 '0x140999860': ('系统:命令', 'O：remove_ivy_at_location——按位置移除'),
 '0x1408c1580': ('系统:上传', 'O：ivy_send_to_gpu_leaf——叶 SSBO 上传'),
 '0x140a9b4e0': ('系统:上传', 'O：ivy_send_to_gpu_segments——段 SSBO 上传'),
 '0x140b0e380': ('系统:动画', 'O：ivy_leaf_animation——叶动画'),
}
for va, (role, concl) in IS.items():
    add(va, role, concl)

# --- clutter ---
CL = {
 '0x1408b83d0': ('转换', 'O：ClutterMeshTy::from(&str)（68KB）——56 变体按资产路径匹配（shovel/bucket/fallen_leaves/bench/pumpkin2/anvil/door_bell…），未识别 panic'),
 '0x1408b8e90': ('查询/计算', 'O：clutter_mesh_ty_data——静态表 switch(0..0x37)→8B 条目（名+kind）'),
 '0x1408b9170': ('Bevy 注册胶水', 'O：register_required_components（Bevy 组件注册）'),
 '0x1408b99e0': ('派生胶水', 'O：enum_iterator::Sequence impl（56 变体迭代）'),
}
for va, (role, concl) in CL.items():
    add(va, role, concl)

# --- intersect_meshes ---
IM = {
 '0x1408f2850': ('几何/碰撞', 'O：polyline::merge_segments_into_polylines（44KB）——交点段按端点连通性合并为折线'),
 '0x1408f4670': ('几何/碰撞', 'O：merge 闭包（线段匹配）'),
 '0x140a97b70': ('系统:相交', 'O：find_mesh_mesh_intersections——新 collider 与相关网格相交→伪影图更新+折线化+发布'),
 '0x140a9afa0': ('几何/碰撞', 'O：intersect_mesh_mesh——parry traverse_bvtt 两网格三角两两相交'),
 '0x140a9b3c0': ('查询/计算', 'O：should_intersect_collider_meshes——按几何源过滤'),
}
for va, (role, concl) in IM.items():
    add(va, role, concl)

# --- visualize_colliders ---
VC = {
 '0x141326370': ('系统:调试', 'O：visualize_colliders——debug 线框遍历'),
 '0x141326930': ('系统:调试', 'O：visualize_shape——parry 形状→线框图元'),
}
for va, (role, concl) in VC.items():
    add(va, role, concl)

# --- critters draw shaders (render glue) ---
DS = {
 '0x1415547e0': ('渲染胶水', 'I：draw_instanced_butterfly_shader——蝴蝶实例绘制'),
 '0x1415549b0': ('渲染胶水', 'I：draw_instanced_firefly_shader——萤火虫实例绘制'),
 '0x141554cf0': ('渲染胶水', 'I：draw_instanced_firefly_glow_shader——萤火虫辉光绘制'),
 '0x141555080': ('渲染胶水', 'I：draw_duck_shader——鸭子实例绘制'),
 '0x141555ec0': ('渲染胶水', 'I：draw_wren_shader——鹪鹩实例绘制'),
}
for va, (role, concl) in DS.items():
    add(va, role, concl)

# --- targeted round-3 additions ---
add('0x1408ff430', '系统:备份', 'O：compute_checksum——iter_edges_and_stubs→StairsSnapshot::new→utils::calculate_hash（wp21 合同；decomp 1450B）')
add('0x1409007b0', '系统:备份', 'O：reduce_diff——差异化简（与 apply_diff 0x900ba0 成对；移除/添加边对预处理，wp21 合同；decomp 8550B）')
add('0x140b26a80', '转换', 'O：impl$21::to_offset_collider_builder——转偏移 collider 构建器（round3 反编译 2569B 落盘；同族 impl$23=0x1408d0390）')
add('0x140b685e0', '系统:注册', 'O：collision::impl$0::build——碰撞插件注册（round3 反编译 25KB 落盘；内部注册函数 0x140b660c0/0x140b66ff0/0x140b679c0 + bevy 注册胶水）')
add('0x140901270', '系统:写入', 'O：StairsPillarProposals::add——按键找 Vec push 44B StairsPillarProposal（wp21 合同）')

# 内联函数名称推断（无独立符号；名称为 O，语义按名+上下文）
INLINE_KB = {
    'force_make_backup_no_cleanup': 'I：强制备份（不清临时状态；make_backup 变体）',
    'take_snapshot': 'I：取快照（iter_edges_and_stubs→StairsSnapshot）',
    'take_snapshot_impl': 'I：take_snapshot 实现面（3 实例之一 0x14157d200）',
    'drain_dirty_edges': 'I：排空脏边集合（+0x148 集合，供 invalidate 消费）',
    'edges': 'O：StairsSnapshot::edges getter（I 内联）',
    'empty': 'O：ColliderDiff::empty——空差异（delta=None）',
    'apply_to': 'O：ColliderDiff::apply_to——置 delta=Some(Added/Removed)',
    'from_rapier_collider': 'O：RaycastMeshKey::from_rapier_collider——rapier ColliderHandle 索引转键',
    'from_rapier_user_data': 'O：RaycastMeshKey::from_rapier_user_data——user_data u128 低 64 位转键',
    'keys': 'O：ColliderRegistry::keys——遍历句柄键',
    'retain_colliders': 'I：retain_colliders——按谓词保留 collider（stairs/autoclutter 内联）',
    'legacy_spherecast': 'I：Rapier 层 legacy_spherecast（固定 10 槽序，见 0x1408ccc90）',
    'spherecast': 'I：Rapier 层 spherecast（mask 驱动，见 0x1408ccfc0）',
    'legacy_raycast': 'I：Rapier 层 legacy_raycast（mask=0，见 0x1408cc680）',
    'capture_sheep_snapshot': 'I：绵羊快照捕获（serde 保存/回放路径）',
    'all_leaves': 'O：全量叶迭代（内联）',
    'all_leaves_mut': 'O：全量叶可变迭代（内联）',
    'point_count': 'O：点数 getter（grower/pruner 内联）',
    'roots': 'O：全部根位置收集（spawner 内联）',
    'total_segment_count': 'O：全株段数求和（闭包内联）',
    'from_wall_u': 'O：WallCoord::from_wall_u——u 参数构造（v=0）',
    'force_rebuild': 'I：强制重建视觉（v2 名 reset_edge=0x140aa8a60）',
    'max_ivy_grow_height': 'O：墙最大藤蔓生长高度 getter（grower 引用，跨域）',
    'from_parry_triangle': 'I：parry 三角→游戏三角转换（intersect_mesh_mesh 内联）',
    'as_usize': 'O：VertexId::as_usize——u32→usize',
    'subscribe_mesh_mesh_intersections': 'I：网格相交订阅（tabloid 消费者）',
    'decorator_overlaps_with_others': 'I：Rapier 层装饰物重叠检测（见 0x14129be00）',
    'shape_overlaps_with': 'I：Rapier 层形状重叠查询（见 0x142612d10）',
    'to_bitflag': 'O：SubworldIndex::to_bitflag=1<<槽号（Terrain=槽5→0x20；stairs_assemble 内联）',
    'floor_mesh': 'I：地板网格构造（new_cuboid 包装；update_rapier_colliders 内联）',
    'to_rapier': 'I：glam→rapier 转换族（Isometry/Vector/Point 各实例）',
    'to_glam': 'I：rapier→glam 转换族',
    'upsert_collider_entry': 'O：upsert_collider 内部实现（0x140b40b50；同几何去重/重建）',
    'try_ivy_root_next_to_cursor': 'I：光标旁根点尝试（ivy_spawner 内联）',
    'try_random_ivy_root': 'I：随机根点尝试（RNG 采样）',
    'bilinear_sample_at_world_xz': 'I：世界 xz 双线性采样（路径栅格；draw_path 内联）',
    'impl$15::from': 'I：From 转换（RaycastGeoSourceFlag 族）',
}

def name_classify(name):
    n = name
    if 'serde' in n or 'Serialize' in n or 'Deserialize' in n:
        return '派生胶水', 'O：serde derive（复用依赖，不计业务）'
    if 'register_required_components' in n:
        return 'Bevy 注册胶水', 'O：Bevy 组件注册（复用 derive，不计业务）'
    if 'clone_behavior' in n:
        return '派生胶水', 'O：Clone derive 折叠'
    if '::apply' in n or '::queue' in n or 'validate_param' in n or 'get_param' in n:
        return 'Bevy 参数胶水', 'O：SystemParam 胶水（apply/queue/validate）'
    if 'init_state' in n or 'new_archetype' in n:
        return 'Bevy 参数胶水', 'O：SystemParam 生命周期胶水'
    if '::fmt' in n or '::Debug' in n:
        return '派生胶水', 'O：Debug derive/fmt'
    if '::drop' in n:
        return '释放', 'O：Drop（含 Arc/Vec 清理）'
    if '::eq' in n or '::hash' in n:
        return '派生胶水', 'O：Eq/Hash derive'
    if '::clone' in n:
        return '派生胶水', 'O：Clone derive'
    if '::default' in n:
        return '构造', 'I：Default（字段值 U 或见合同）'
    if '::from' in n or '$LT$str' in n:
        return '转换', 'I：From 转换'
    if n.endswith('::new'):
        return '构造', 'I：构造（见合同/反编译）'
    if 'Panic' in n or 'panic' in n:
        return '派生胶水', 'O：panic 处理胶水'
    return None

# --------------------------------------------------------------------------
# Fill chapters
# --------------------------------------------------------------------------
def decomp_exists(va):
    return os.path.exists(os.path.join(DECOMP, f'decomp-{va}.txt'))

def fill_file(rel):
    path = os.path.join(BASE, rel)
    txt = open(path, encoding='utf-8').read()
    rows = re.findall(r'(\| `([^`]+)` \| (0x[0-9a-f]+) \| \d+ \| ([^|]*) \| ([^|]*) \| ([^|]*)\|)', txt)
    changed = 0
    for full, name, va, role, concl, ev in rows:
        new_role, new_concl, new_ev = None, None, ev.strip()
        for kva, lst in KB.items():
            if kva.lower().lstrip('0x').rstrip('l') == va.lstrip('0x').lower():
                new_role, new_concl = lst[0]
                break
        if new_concl is None:
            nc = name_classify(name)
            if nc: new_role, new_concl = nc
        if new_concl is None:
            # unknown business function — leave blank but mark 待反编译 if no evidence
            new_role, new_concl = '待推断', 'U：无证据（待定点反编译）'
        if not new_ev:
            if decomp_exists(va):
                new_ev = f'collision-eco\\decomp\\decomp-{va}.txt'
        # only write when changed
        new_role = new_role.strip() or '待推断'
        new_concl = new_concl.strip() or '待证'
        new_full = f'| `{name}` | {va} | {{line}} | {new_role} | {new_concl} | {new_ev} |'
        # rebuild with original line number
        m = re.match(r'\| `([^`]+)` \| (0x[0-9a-f]+) \| (\d+) \|', full)
        line_no = m.group(3)
        new_full = f'| `{name}` | {va} | {line_no} | {new_role} | {new_concl} | {new_ev} |'
        if new_full != full:
            txt = txt.replace(full, new_full, 1)
            changed += 1
    # file purpose line
    purposes = {
        'systems/collision.rs.md': '碰撞插件（systems/collision 模块入口）：impl$0::build 注册碰撞资源/系统（round3 O）',
        'systems/collision/world.rs.md': 'rapier3d 碰撞世界胶水：17 子世界槽、ray/sphere/shape 查询、dirty 瓦片生命周期（flush/publish/QueryPipeline::update）、collider upsert/remove 泛型族、parry 网格数据构造',
        'systems/collision/intersect_meshes.rs.md': '网格-网格相交检测：parry QBVH 两两三角相交→交点段合并为折线→伪影图+tabloid 发布',
        'systems/collision/visualize_colliders.rs.md': 'debug 碰撞体线框可视化',
        'resources/stairs/stairs_state.rs.md': '权威楼梯结构图：petgraph GraphMap（@+0x28）+端点/脏边集合(@+0x148)/备份快照 diff（历史撤销用）',
        'resources/stairs/structural_graph.rs.md': '楼梯支撑碰撞体图：span↔collider 双向映射+支撑高+GraphMap（拱/托架装配的支撑结构）',
        'resources/stairs/stairs_assembly_params.rs.md': '手工楼梯装配参数族：锚点/对齐/边标识（供 stairs_assemble 系统消费）',
        'resources/stairs/invalidate.rs.md': '脏楼梯伪影失效系统：脏边→撤碰撞体/RT 网格/视觉/装配缓存',
        'resources/stairs/visual.rs.md': '楼梯可视化资源：MagicSupports/StairsVisualState/地板 mesh 块',
        'resources/stairs/stairs_collider.rs.md': '楼梯碰撞体注册标记资源（rapier 边界，无独立函数）',
        'resources/stairs/stairs_removed_supports.rs.md': '已移除楼梯支撑存档集（serde）',
        'resources/critters.rs.md': '小动物组件/资源族：Critter/Firefly/Butterfly/Duck/Wren/BirdSpotManager 等（CPU 状态+FMOD+池）',
        'resources/critters/sheep.rs.md': 'Sheep 组件族（serde 存档+快照；业务体在 systems/critters）',
        'resources/critters/sheep/control.rs.md': 'SheepControlSnapshot/SheepSnapshot（serde 快照）',
        'systems/paths/draw_path.rs.md': '路径绘制：UI 拖拽→DrawState 状态机→WorldRaster<512> 四变体绘制+互擦',
        'systems/paths/path_brush_preview_size.rs.md': '路径笔刷预览尺寸（按 UI 模式）',
        'systems/paths/record_path_history.rs.md': '路径/石板编辑历史记录（4 命令→撤销历史）',
        'systems/paths/mod.rs.md': 'paths 插件',
        'resources/ivy/ivy_storage.rs.md': '常春藤折线存储：Ivy(VecDeque<IvyPoint 40B>)+IvyStorage(HashMap+RNG)',
        'resources/ivy/ivy_leaf_storage.rs.md': '常春藤叶存储：IvyLeaf 104B GPU 实例参数集',
        'resources/ivy/ivy_direction_proposer.rs.md': '生长方向噪声提议器：双 FastNoise 混合→θ∈[-40°,+40°]',
        'resources/ivy/wall_coord.rs.md': '墙坐标→世界空间（PublicWallState 曲线采样+法向偏移）',
        'resources/clutter_meshes.rs.md': '杂饰网格类型表：ClutterMeshTy 56 资产路径变体+删除标记组件',
        'resources/preplaced_autoclutter.rs.md': '预放置自动杂饰资源（default 内联，无独立过程）',
        'systems/ivy/mod.rs.md': 'ivy 插件（资源/系统/事件注册）',
        'systems/ivy/ivy_grower.rs.md': '常春藤生长：10 相状态机（grower 7.9KB 单函数）沿墙推进折线',
        'systems/ivy/ivy_spawner.rs.md': '常春藤生成：随机根点/光标旁根点',
        'systems/ivy/ivy_pruner.rs.md': '常春藤修剪：墙变更后点有效性校验+按位置移除',
        'systems/ivy/ivy_leaf_spawner.rs.md': '叶生成：消费段完成消息沿末段放叶',
        'systems/ivy/ivy_branch_thickness_animation.rs.md': '分支粗细动画',
        'systems/ivy/ivy_leaf_animation.rs.md': '叶动画',
        'systems/ivy/ivy_send_to_gpu_leaf.rs.md': '叶 GPU 上传（SSBO）',
        'systems/ivy/ivy_send_to_gpu_segments.rs.md': '段 GPU 上传（SSBO）',
    }
    purpose = purposes.get(rel.replace('\\', '/'))
    if purpose:
        txt = re.sub(r'> 文件作用：\*\*待推断\*\*[^\n]*', f'> 文件作用：**{purpose}**', txt)
    # 内联函数行（名称推断）
    inline_changed = 0
    for key, infer in INLINE_KB.items():
        pat = re.compile(r'(`[^`]*' + re.escape(key) + r'[^`]*`（行 \d+）— )待推断')
        txt, n = pat.subn(r'\1' + infer, txt)
        inline_changed += n
    open(path, 'w', encoding='utf-8').write(txt)
    return len(rows), changed, len([r for r in rows if not r[4].strip()]), inline_changed

files = []
for root, dirs, fs in os.walk(BASE):
    for f in fs:
        if not f.endswith('.md'): continue
        rel = os.path.relpath(os.path.join(root, f), BASE).replace('\\', '/')
        if any(rel.startswith(p) for p in DOMAIN_PREFIXES) or rel in DOMAIN_FILES:
            files.append(rel)
files.sort()
tot_rows = tot_changed = tot_blank = 0
for rel in files:
    n, c, b, ic = fill_file(rel)
    tot_rows += n; tot_changed += c; tot_blank += b
    print(f'{rel}: {n} 行, 更新 {c}, 仍空白 {b}, 内联 {ic}')
print(f'TOTAL: {len(files)} 章, {tot_rows} 行, 更新 {tot_changed}, 仍空白 {tot_blank}')
