# system_wall_constructor 子范围分析（B 版）：门/栅栏/地板/楼梯/半木/抹灰/utils

> 由批次 3 分析 agent 产出，父级代为落盘。

证据源：B patched EXE（sha256 cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323），Ghidra 共享只读库 get_functions 反编译（job：tool-usage/jobs/wall-constructor-agent/batch1|2|3-*.json，结果已落盘）。地址为文件内 VA（0x140000000 基址）。标注：O=可复用；I=有反编译分析无完整实现；U=骨架/内联待定位。

## 1. 门 construct_gates
### 1.1 construct_wooden_gate_doors — O @0x141210110
逻辑：输入=具体化墙段+prepared 曲线3 positions/U+caller Public288 标量+观察 Public29a 字节+WallId/命令 ordinal+地形高度网格；输出 GateCpuOutput{bricks,leaves,nanis}。先跑负Y比例选择并发出内部柱（ratio>.5），宽度>2.7(f32 严格)选两门叶（中心间距 min(.05/总长,.1)），=2.7 单叶。算法：曲线切片（from_curve/门底曲线 offset）+砖支撑行+door_arch_heights+两叶网格。常量：seed=WallId::get_inner()*cmd.ordinal+leaf.ordinal→u32；header inset bits 3f547ae2；footer mesh offset 后重载端点；shader 键 solid_vertex_color_wooden_gate；spawn 载荷 136B（transform16..56、mesh@64、shader@72、top@80..104、WallId@104、两端点 XZ@112..128、seed@128、Public29a@132）。未知：整体 envelope 验收、GPU/SSBO 后置。可复用：reconstruction/building/gates.rs + evidence/building-gates（B-gate-mesh-draft-verification.json 28 返回/1886 砖/46 网格字节证明）。
### 1.2 DoorMeshInProgress::add_quad — O @0x141214ed0
6 顶点（0,1,2,0,2,3）并行写入 positions Vec3 / colors (DecoratorId,DecoratorId,u8) / uvs [f32;2]（stride 0xc/0xc/8），纯装配无几何。可复用同上。
### 1.3 construct_gates（systems.rs）— O @0x140756370
Bevy system：EventReader<ConstructGateCmd(120B)>，9 参数序（含 UiSignifierStream 为真实参数）；热 CALL 140756da0→141210110；发 NaniBundle<SolidVertexColorInstanceData>+DrawableMeshBundle+TransientMesh+WoodenGate。is_sewer_gate/construct_sewer_bars（行143/152）分支。update_gates_hashmap @0x140755690 消费 WallChanged/CreateRoof/SwitchRoofType/WallDeleted/DeleteRoof 六组合 despawn 旧实体。可复用：B-gate-system-fallback-verification.json（64 selector/189 内部柱）。

## 2. 栅栏 construct_fences（全 O）
construct_fences @0x140765830：system(EventReader<OnWallChanged>,PublicWalls,AssetSsboLibrary)，对每墙传 Public+c8/d0 段值+Public+90/98 prepared XYZ→emit_fences；累计 MaterialStats set_fences_segments_count/seed；RNG seed=WallId inner+begin+end wrap64。emit_fences @0x1412388f0：找最大非正[begin,end)，取墙高(Public284)+惰性 Aabb2(Public+8/+10)→process_segment；首/尾桩跳过规则。process_segment @0x141238d80：perchance 由 seed 派生；spacing=smoothstep(clamp((范围-1)/5))，.7→bits 3fce147a；crashy_resample(1411f8ed0) 保留角点+累计归一化长度 vs 规则增量；64B plank+post(-0.1Y)+lean/rail 随机+affine。split_curve_based_on_angle @0x141fdf9e0：40°阈值位 3f32b8c2+水平 dot+重复点断言。可复用：reconstruction/building/fences.rs（FenceConstructionInput→construct_fences_cpu→FenceCpuOutput）+ evidence/building-fences。

## 3. 地板 construct_floors — I @0x1407661e0
墙→Floor 实体 HashMap（stride 0x2a8）diff 维护：remove_entry+Entities::contains+despawn 失效；存续墙 get_wall_shape 后按 max(h,1.0)*3.0 做 log2f 缩放，spawn_empty 插入 NaniBundle<StoneFloorInstanceData>+Floor（命令槽 stride 0xb8，WallId@0x58/0x98）。算法：去旧→重建；幂次缩放 log2f(h*3)。未知：StoneFloorInstanceData/Floor ABI、缩放语义。与屋顶地板路径独立（findings 明确）。

## 4. 平台楼梯 — O @0x1407744d0
80B 装饰源（raw[0x4e]==0x3f 且 raw[0x2a]∉{2,3,4}）→blockers（coordinate=[f(8),f(12)]，terrain 差<3.0）→process_loop_range_into_wallpath_segments（行456）→cells/碰撞（update_phys 行535、add_colision_geo 行564）→96B 砖/holes/stats。可复用：reconstruction/building/wall_platform_stairs.rs / stair_rows.rs / segmentation.rs / collision.rs + evidence/building-platform-stairs（collision hash/policy 已验证）。

## 5. 玩家自制楼梯 construct_playermade_stairs（全 I）
5.1 construct_playermade_stairs_arches_n_walls<true> @0x140496800：ArchWalker::new 行走+WallConstructor::from_curve+TransientStairEdgeIdAllocator::get_or_allocate+BrickSourceId::with_transient_stair_edge_id/from_wall_id+RaycastWorld::raycast_simple+RaycastMeshData::new_indexed/RaycastMesh::new+StairsPillarProposals::add。closure$6<true> @0x14048b320。未知：<true> 开关语义、closure 编号绑定。
5.2 preprocess_curve @0x140780e20（closure$0 @0x140782f50）：Curve::slice_into<Vec2>→try_new_from_points→密度 min(2.0/len,0.45) clamp[0,1]→try_resample_approx_pt_count_remap。未知：0.45/0.9 语义、closure$1-3。
5.3 construct_playermade_stair_brackets @0x1407834f0：fastrand Rng::f32×18+DecoratorExtraInfo::try_as_stair+random_orthogonal_quat。随机托架放置。
5.4 playermade_stair_floating_bricks @0x14077a2f0：PublicWalls::get+MagicSupports::clear+Rng::f32×15+EasingFunction::apply 动画+BrickSourceId::from_wall_id+random_orthogonal_quat；3 处 Mutex。未知：easing 曲线、生命周期。
5.5 construct_playermade_stair_pillars_visuals @0x14077d520：StairsState::graph+StairSegment::new+Rectangle2d::new/get_point/get_edge_normal/as_points2+WallConstructor::from_curve+random_splits+Rng::f32+inverse_lerp/xz_rotation_bad；construct_pillar_stub(inlinee 24718)。未知：柱高/数量。
5.6 process_playermade_stair_pillars_proposals（pillars_process_proposals.rs，内联）— I：宿主=bevy impl$3::call_mut 系统包装 **0x140178a70**（host_rva 1542320，module 73 inlinee 10287）；系统参数=(ResMut<StairsPillarProposals>,Res<UiSignifierStream>,ResMut<RaycastWorld>,ResMut<StairColliderRegistry>,ResMut<RtWorld>,EventWriter<UpdateRtWorldEvent>)。逻辑：提案→raycast→碰撞注册→rt 事件。未知：宿主内层业务块未定点。

## 6. 半木 curved_halftimber（全 I）
6.1 curved_halftimber_maintain_entities @0x1407706d0：遍历 CurvedHalfTimberDrawables（stride 0x10）+每墙砖表（stride 0x28：4 个 Vec2 对+索引字节+WallId）；HashMap<WallId,Vec<Entity>> despawn 旧→逐砖 spawn NaniBundle<CurvedHalfTimberInstanceData>+CurvedHalfTimber（命令槽 0xd8；@0xa0 字节 2/1 区分来源）；每墙联动 wooden_floor_borders。未知：实例字段语义、@0xa0 枚举。
6.2 wooden_floor_borders @0x141244310：PublicWalls::roof_entity→屋顶 hole 定位墙（WallHoleStorage::as_finalized/holes_no_duplicates）→WallStyle::is_halftimbered 过滤→SIMD 随机选墙→wall_space_length(CircularF32)→CurvedHalfTimberInstanceData::new_curved_xz（弯曲）或 new_straight（矩形 as_curve2/circular_slice）→spawn；矩形周长 fmodf(方向/周期,1.0)。未知：边框宽高常量。

## 7. 抹灰 plaster_systems（全 I）
7.1 create_plaster @0x14075d460：遍历待抹灰墙集（定位 map，条目 stride 0x2a8）；查抹灰记录表（stride 0x68）判重→WallHoleStorage::as_finalized→holes_no_duplicates→查 mesh 实体表（stride 0x20）→generate_plaster_mesh_and_entities @0x1412269b0。
7.2 maintain_plaster @0x14075b810：Decorators::iter_removed_ids+StructureType::is_fence+WallHoleStorage+EntityCommands::despawn+utils::time_continuous_lerp 淡出+Mutex；maintain_deferred_plaster_buffer_disposal(inlinee 24528) 延迟 buffer 释放。未知：淡出曲线/时长。
7.3 split_wall_into_sides @0x14075dc60：把墙切成 SmallVec<WallSide,4>（每侧 24B=0x18：u_begin+1e-7、u_end-1e-7、高度、y 偏移、u32 格数、u8 屋顶标志）。分支：in_RDX+600 墙样式(&1)；无屋顶/非 gable 单高；gable 按 Roof::height 叠 max_y 双高（uVar9/10 标记）；4×Rectangle2d::get_point_u+divps(…,0.1)(3ecccccd) round+psubusw 00020002 下限→格数。算法：矩形四边 u 采样+0.1m 网格取整+屋顶高约束。未知：WallSide::new/sd_box(inlinee 24542/24547) 细节、roof_flag 枚举。
7.4 generate_plaster_mesh_and_entities @0x1412269b0：Roof::is_flat_w_no_rim/is_flat 分支；WallSpaceCoord::with_y_offset/to_world_space；Rectangle2d/Circle2d::get_normal_at_u；Aabb2::center/half_dims；BufferDesc::new_gpu_only+Buffer::device_address+spawn 实体；mirror_holes_as_needed(inlinee 70970)。未知：mesh 拓扑/UV、buffer 布局。

## 8. UI 控制（全 I）
ui_arch_controls @0x140766e70：UI 控件表遍历（stride 0x20 SIMD 掩码）→PublicWalls::get→Rectangle2d::get_point_u+get_approx_coord_from_pos<Thresholded3D>→Triangle::new 剖面链→format_inner "arch_height"(PTR_s_arch_height__142a7e0a8)→WidgetParams::new+UiState::add<String>→UiWidgetContext::is_active→事件入队。
do_ui_arch_controls @0x140767b70（closure$1 @0x1407690d0）：PublicWalls×5+is_active/intersecting+UiSignifierStream::write+TweakerView::as_inner_mut+InnerWalls::update_arch_type/height+CompoundEditBuilder::next/build+History::add+SignifierFactory::new。
ui_lintel_controls @0x140771a60：同 arch，加 ProportionalScreenSpace::from_window_size 定位；closure$1 与 arch 共用 0x1407690d0。

## 9. utils
9.1 half_timbering — U @0x141218e30：Ghidra 空反编译（void() 占位）+ReVa 失败；骨架显示 17 个 closure+依赖 utils/roof/Curve/Circle2d/WoodenPlankSsbo，推断为半木梁布局主函数。须修签名后定点重取。
9.2 roof_outline — I @0x141217be0：PublicWallState::try_get_wall_shape（非矩形返回）→锁 SSBO BufferState→get_roof_expanded_top_and_bottom+RoofBoundsDims::as_rectangle+Roof::profile_curve_normalized+calc_roof_tip_center_ls→4 轮矩形边点采样 profile_curve_ws→roundf 木板数(≥2)→Curve::get_coord_at_u 双点+rotation_from_normal→push WoodenPlankSsbo（0.4 档/0.25 档系数）。未知：两档木板类型。
9.3 overlapping_windows — I @0x1412218a0：遍历 DecoratorStorage（SIMD 掩码）；过滤 subtype 字节（byte[10]<2 且 byte[0xe]-0x38<3）；get_window_aabb 后 ±0.038（dormer 'g' 且 leaf[0xc]==1 再 ±0.14）；intersects→收集 ResourceAccessInfo（stride 0x14：两 f32x2+u8+u8+标记 0/1）。
9.4 get_window_aabb — I @0x141221da0：门类（dst_as_wall 且 in_RDX+0x46=='9'）：gothic_door_profile_curve（subtype '\t'）或 gothic_half_dormer_profile_curve，取 profile 末点算 [x-x0,y0+y,y0,..+0.093] Aabb2；普通装饰 bbx_ls+Aabb2::from_center_dims。常量 0.093 窗台偏移。
9.5 check_for_overlap_with_all_windows — I @0x141221f70：get_window_aabb+Aabb2::contains 中心/侧向/扩展三测试。
9.6 add_underwindow_pattern — I @0x141fd1c30：样式检查 (rcx+0x11)&0x1e!=0x16；窗高差∈(0.5,2.0)；7 定位点（含 0xbe23d70b≈-0.115）；fmodf(u/周期,1.0)+WallSpaceCoord 转世界+Circle2d::get_pos_at_u/ShapeParameters::get_normal_at_u 圆角≤0.078→WoodenPlankSsbo 0.15 宽。
9.7 add_wall_corners — I @0x141215540（closure$1 @0x141217a20）：墙高/矩形/曲线输入；相邻点 get_point_u+Curve 采样 Triangle::new 链→find_corners/check_angle(inlinee 70916/70917) 夹角检测→角砖插值→BrickTransformSsbo（stride 0x60）。未知：转角阈值。
9.8 add_skirts — I @0x1412221f0：max_y/is_rectangle 分支（矩形 get_point_u×8，非矩形 [0,1]）；split_height_into_skirts；每裙 ceilf((u1-u0)*h/0.69)+random_splits_into；Curve::get_coord_at_u+abs 累计距离 inverse_lerp；Aabb2::from_center_dims+holes_no_duplicates→resolve_hole_overlap；WallSpaceCoord::to_world_space→BrickTransformSsbo（@0x54/58=(中点±半宽)*h）。裙阈值 wall_y+3.5；行距 0.69。
9.9 resolve_hole_overlap — I @0x1412411c0：输入墙 Aabb2+VecDeque<MyBrick>（stride 0x28：2 Vec2+u32 行号+2 字节）；Aabb2::contains 四角计数 switch(0/1/2/4)：case0 中心分裂±100 副砖；case1 inverse_lerp 求交点切主+残；case2/4 残砖+VecDeque 段 swap。check_for_hole_overlap(inlinee 71192)。副砖行号偏移 100；%10 周期编号。
9.10 trim_rows — I @0x14123eef0：高度图 f32 数组+RowTrimmerSink→split_heightmap_by_threshold<RowTrimmerSink> 切行→sink 回调 above/below_to_*；输出 Vec len=0/cap=8。
9.11 above_to_below @0x14123fab0 / below_to_above @0x14123fb70 — I：断言 t∈[0,1]+行号界；utils::lerp(t)；above 写第 idx 行尾值 f32；below 插入 Vec<(DecoratorId,Vec<PreferredNodeAlignment>)>（stride 0x20）+0x3f800000。
9.12 split_into_random_rows — I @0x14123e110：对相邻 u 点对 floorf((u1-u0)/0.46) 行数(下限3→≥2段)+random_splits 切点 t+divps/lerp 映射+ABS<0.046/高 去重→push f32。常量：砖高 0.46、最小间距 0.046。
9.13 perturb_row_ends — I @0x141240d30：前向端点=min(1.0,1.0-(prev+cur)*0.5)×1.99；后向=min(中点,1.0-中点)×1.99；输出 clamp×幅度+原值。
9.14 split_horizontally — I @0x141240ec0：t=in_XMM0*0.4+0.3；Aabb2::height/width/center 求中点 lerp；VecDeque<MyBrick> 推下块(idx)/上块(idx+1)。
9.15 sine_wave.rs — U：BrickRowSine::new(inlinee 24627)/sine_wave_deform(24641)/random_f(24629)，module 401 纯内联，宿主待定位（候选 construct_walls/arches 砖行块）。
9.16 stairs_signifiers_new — I @0x141223d10：非矩形墙占位[0,end-1]；矩形墙 get_point×2+get_approx_coord_from_pos<Thresholded3D>→tuple_windows(u0,u1)；TerrainHeightsData::sample_at_world_xz（y-terrain<0.5 且 max_y-y>=3.0 判楼梯段）；SubtractiveBoolean1D::new_range/subtract 减 hole（holes_no_duplicates，HoleType 0/1/3/4 过滤）；剩余范围 ceil(段长*h/0.69)+random_splits_into 踏板；每阶 Curve::get_coord_at_u 双端+xz_rotation_bad→BrickTransformSsbo（@0x54 两行 u、@0x5c=4），2 阶循环。常量 0.69；max_y-0.79-0.1-0.3 顶高。

## 10. 本子范围逻辑总览
门族(O 已恢复)：ConstructGateCmd→CPU 木门/支撑/post fallback→add_quad 六顶点 mesh→Nani/TransientMesh/WoodenGate；update_gates_hashmap 生命周期。栅栏族(O)：construct_fences→emit_fences→process_segment(40°切分+crashy_resample+perchance)→64B plank/post/统计。地板(I)：construct_floors=Floor 实体 HashMap diff+StoneFloorInstanceData。平台楼梯(O 已恢复)：80B 装饰→blockers/cells/碰撞/96B 砖。玩家楼梯(I)：preprocess_curve→arches_n_walls(ArchWalker+raycast+柱提案)→brackets/magic_support/pillars_visual→proposals 宿主 0x140178a70。半木(I)：curved_halftimber_maintain_entities+wooden_floor_borders。抹灰(I)：create_plaster→split_wall_into_sides(WallSide 0.1m 格+屋顶高)→generate_plaster_mesh(世界顶点/法线/GPU buffer)；maintain_plaster 淡出。UI(I)：剖面三角+UiState+InnerWalls::update_arch_type/height+复合编辑。utils：gates/fences 链已恢复；add_wall_corners/add_skirts/resolve_hole_overlap/trim_rows/split_into_random_rows/perturb_row_ends/split_horizontally/stairs_signifiers_new/窗工具族(I)；half_timbering/sine_wave(U)。

## 11. 算法清单
曲线分割/角度(split_curve_based_on_angle 40°)；曲线重采样(crashy_resample 角点保留)；砖行切分/抖动(perturb_row_ends×1.99、split_horizontally 0.4t+0.3、split_into_random_rows 0.46/0.046、add_skirts 0.69+累计距离、stairs_signifiers 0.69+SubtractiveBoolean1D)；随机切段(random_splits/random_splits_into/perchance/fastrand Rng)；几何偏移(WallSpaceCoord/xz_rotation_bad/rotation_from_normal/random_orthogonal_quat/inverse_lerp/EasingFunction)；半木梁布局(roof_outline 0.4/0.1875/0.7、add_underwindow_pattern 0.15+圆角0.078、overlapping_windows ±0.038/±0.14)；抹灰网格(split_wall_into_sides 0.1m 格+gable、generate_plaster 世界顶点/法线/GPU buffer、sd_box 有向距离场)；AABB 四角分类裁砖(resolve_hole_overlap)；高度图阈值切行(trim_rows)；地形 raycast(RaycastWorld/RaycastMeshData::new_indexed)；UI(剖面三角链+CompoundEditBuilder 历史)。

## 12. 未知清单
1. half_timbering(0x141218e30) 空反编译/ReVa 失败，控制流未取。2. sine_wave.rs 内联宿主未定位。3. pillars proposals 宿主 0x140178a70 内层业务块未定点；碰撞体形状未知。4. arches_n_walls<true> 泛型开关语义与 closure 绑定。5. maintain_plaster 淡出曲线/时长；generate_plaster mesh 拓扑/UV。6. curved_halftimber 实例字段语义与 @0xa0 枚举。7. add_wall_corners 转角阈值/check_angle 判据。8. MyBrick 2 字节标记位与 VecDeque 布局。9. WallSide roof_flag 枚举与 split_height_into_skirts 规则。10. add_skirts 3.5 裙阈值与 0.093 窗台来源。11. 门/栅栏族 World 验收与 GPU/SSBO 配额（实现已有待验收）。12. construct_floors 组件 ABI 与 log2f 语义。

## 13. 依赖与工具
依赖(glam/fastrand/perchance/smallvec/utils/lerp/itertools/hashbrown/parking_lot/frozen/puffin)直接复用。反编译结果：tool-usage/jobs/wall-constructor-agent/batch1|2|3-{result}.json；逐函数文本 /tmp/wallagent/dc/*.txt。
