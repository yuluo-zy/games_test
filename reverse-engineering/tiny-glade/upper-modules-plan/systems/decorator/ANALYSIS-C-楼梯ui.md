# 楼梯视觉 + ui_systems + utils 子范围分析（B）

> 由批次 3 分析 agent 产出，父级代为落盘。

全部结论基于 Ghidra 共享库逐函数反编译（65 函数）、骨架 enriched/pdb 元数据（RVA/泛型接口候选/静态调用边）、原指令字节（playermade_stairs_assemble 开头 48B）。标注 O=反编译观察到 / I=推断 / U=未知。地址为 B VA（0x140000000+RVA）。既有候选 reconstruction/country-core/src/resources/stairs/*（WP21）已复用（StairsState/StairsStructuralGraph/PlayermadeStairsAssemblyParams/ColliderHandle 等），未重推导。

## 1. decorator_visual/stairs.rs
- generate_playermade_stair_decorators @0x140803dd0 — O。签名 void(*)(Res<DecoratorStorage>,Res<CachedDecoratorTransforms>,Res<PlayermadeStairsAssemblyRequests>,ResMut<DecoratorBlueprints>,Res<PublicWalls>,Query<&Roof>,Res<UiSignifierStream>)。demo 模式返回；遍历 modified decorators 的 as_stair；查 (id,id)→AlignKind IndexMap 得判别→switch 定 cuboid 尺寸(0.65/0.95/1.0,1.0,stair_h+0.1)；四元数→基向量写 blueprint 变换；new_cuboid×2-3+new_compound+set_interaction_mesh；墙对齐时 WallStyle/Roof::is_flat/max_y→0.76(半木)/0.66(石)，向 0x14 步长墙饰回写列表追加(color_id,h,h0,0x41200000,4)。算法：复合交互网格+贴墙墙饰回写。
- generate_playermade_stair_requests<false> @0x140276e80 / <true> @0x14027e470 — O。签名 void(*)(Res<DecoratorStorage>,Res<CachedDecoratorTransforms>,Res<PublicWalls>,Res<TerrainHeightsData<VisualTerrainHeightSsbo>>,Res<StairsState>,ResMut<PlayermadeStairsAssemblyRequests>,Res<UiSignifierStream>)。清 requests→阶段一逐边 find_best_neighbor_alignments+IndexMap 对齐偏好→阶段二 closure_1<true>+AlignKind::outgoing_dir 建 GraphMap<DecoratorId,AdjustedStairEdge,Directed>+derive_stair_node_params+walk_shape+Curve::try_new_from_points→阶段三 VecDeque 连通分量闭合。U：<true/false> 泛型语义、阶段三细节。
- maintain_stair_graph @0x140806110 — O。遍历 StairsState 图；storage 缺失→删除列；邻居度==1 且 is_decorator_cull_allowed→级联裁剪；remove_node+endpoint/unset_endpoint；桩边簿记；HashMap<(StairGraphEdge,u32),ColliderMeshInstance>::retain 按存活裁剪；apply_decorator_op 回写。算法：图维护+级联裁剪。
- walk_shape @0x140809cc0 — O。wall_space/subtract；段数 bit0 与 in_R8 早退→ShapeWalkOutput::invalid(哨兵0x8000000000000000)；ceil(长度) 定采样数；Vec<f32>高+Vec<(u8,u32)> 逐点 project_clamp/lerp；单位切线；TupleWindows+Cyle 每点±0.625 切线偏移成踏步边；叉积 1e-08 凹点剔除(vec::remove 平行数组)；闭合墙走 Rectangle2d::circular_slice_segment_points。算法：墙空间密集采样+凹顶点剔除+环形切片。
- compute_stair_assembly_params @0x140807090 — O。两端高线性插值 lerp；切线归一化；每样本 slope=|Δh|/√(Δx²+Δy²) 分类 3(>2.5)/2(0.25~2.5)/1(≤0.25)；StairSubSegment smallvec u32=bVar22|0x10100 合并；bitvec 逐点；Curve::crashy_new_from_points；两端墙锚前插/后接端点。输出 ~0x29×8B PlayermadeStairsAssemblyParams。算法：坡度分类+子段合并+样条。
- densely_sample_spline_segment @0x140809ad0 — O。样本数 ceil(2*Δx)；SplineSegment2D::tangent_at_local_t×2；from_iter<Vec2>。样条密集采样(2/单位)。
- find_alignment_preferences @0x14080beb0 — O。节点+2 bit0 已对齐→空；decorator_info 子类型(+0x22,≤1)；方向归一化+垂直基；1 个(墙空间 subtract 修正)/2 个镜像候选(0x2aB)。find_best_neighbor_alignments @0x14080c360 — O。0x18 步长遍历邻候选；评分 helper(误标本名)；kind&掩码==0→-10 惩罚；保留最高分。U：评分 helper 真名。
- derive_stair_node_params @0x14080c640 — O/U。StairsState::graph+IndexMap 取邻接；按 param+0x22 判别经 jumptable DAT_142a8e448 分发。determine_segment_type @0x14080c810 — O/U。只剩边界检查骨架，业务内联。内联：ShapeWalkOutput::invalid、is_ccw_quad_convex。

## 2. stairs_assemble.rs
- playermade_stairs_assemble<true> @0x140008c90 / <false> @0x140018040 — O(部分)/U。PDB 实例 RVA 0x8c90/0x18040(module=2)；两函数开头 48B 逐字节相同(0x948 栈帧大函数)；Ghidra 空反编译(坏控制流)。调用边：calculate_adjusted_ladder_positions/construct_ladder/construct_stair_steps/create_stairs_collider/determine_ladder_support/gen_stair_floors/gen_stairs_collider_and_rt_geo。I：两大装配泛型，按 params 分派。U：函数体、泛型参数语义。
- construct_stair_steps @0x1407da0b0 — O。步数 floor(0.56/水平run)；|Δh| 钳≥0.6+总高差；TerrainHeightFloatGrid::bilinear_sample 贴地；fastrand×3；砖数 ceil(宽/((3-2r)r²(1.3t+0.7(1-t))+0.46))；random_splits；push<BrickTransformSsbo>。常量 0.56/0.6/0.46。
- gen_stair_floors @0x1407dc170 — O。侧墙悬挑 max(0,0.15-深度)；段法线叉积；每 quad 半悬挑偏移+6 三角索引；Trimesh builder+spawn<NaniBundle<StairsFloorInstanceData>,StairsFloorMeshChunk>；失败 log "Error spawning stair mesh"。内联 floor_tri_count_to_atlas_idx。
- construct_ladder @0x1407db3e0 — O。两端 2 根 WoodenPlankSsbo 侧轨(fastrand 序列 0xa0761d6478bd642f/0x40ec3ac8f17ac85e)；bVar6<5&&(0x16>>bVar6&1)→中轨；档数 ceil(长/0.65)、高 max(0.4,高-0.593)。
- calculate_adjusted_ladder_positions @0x1407d8fe0 — O。锚+0x18/+0x38==1→fVar11=0.28；closure_0→fVar2=0.3/0；按高向加减偏移两端。
- determine_ladder_support @0x1407de800 — O。两槽支撑记录(0x0/0x20)：+0x18 类型==1 且计数/高(≤0.13)短路判定。
- create_stairs_collider @0x1407deb60 — O。==0x11(ColliderHandle::Empty)→remove_collider<StairGraphEdge>+drop Arc；否则 upsert_collider<StairGraphEdge,IsoTransform>。
- gen_stairs_collider_and_rt_geo @0x1407de8b0 — O。20 值三角索引常量表(2,3,6,3,7,6,0,4,1,1,4,5,2,3,6,3,7,6,0,2,4,2,6,4,1,5,3,3,5,7…)；FilterMap<TupleWindows<StepBy<Zip<Zip<Vec3,Vec2>,f32>>>>→Vec<RaycastMeshData>。U：表映射。
- gen_stairs_structural_collider @0x1407dec90 — O。x0y×2+new_convex；0 点→new_sphere。
- create_structural_colliders_and_initial_graph @0x1407d20c0 — O。遍历 params(0x150,+0x148 标志)；HashMap<StairGraphEdge,u32> 查 id；span 表(+0x130)=='3' 时读结构点；try_as_wall(+0xe8/0x108)→0.28；closure_0→0.3；沿向偏移两端；gen_stairs_structural_collider；位集去重；RaycastWorld::upsert_collider<(StairGraphEdge,u32),IsoTransform>；StairsStructuralGraph::upsert_span/add_edge_by_span；遍历 StairsState 图节点(0x28)连边/自环。
- calculate_distortion_from_structural_graph @0x1407d49d0 — O。Subscriber::take<ColliderDiff>；三趟 bitset：remove_node_by_collider/disconnect_node_by_collider(kind∈0x1a203)；kind==0x10 或 0xa203→iter_shape_overlaps 找重叠；畸变 fVar103=clamp(0.046875-overlap,..,1)；PriorityQueue<SubworldColliderHandle,OrderedFloat<f32>>；Dijkstra/BFS 洪泛 0.046875 衰减写回碰撞体+c0。U：kind 掩码枚举语义。
- split_segment_into_supported_and_unsupported_subsegments @0x1407d94a0 — O。wall_normal+方向；y-1.17 净空；raycast_simple 向下；Empty(0x11)→无支撑；fVar18=高*0.5+横向距；切 (start,end) 区间+Vec<f32> 支撑距。受力支撑分割。
- playermade_stairs_interpolate_and_compare @0x1407cf500 — O。time_continuous_lerp_factor 帧间插值 pos/高/tangent；bVar39|=5e-05 位移检测；未变且 USE_STAIRS_LAZINESS==1→惰性跳过；否则重建 SampledStairsPoints+insert HashMap<StairGraphEdgeIdentity,SampledStairsPoints>，+0x148 脏。
- playermade_stairs_assembly_propagate_dirty_flags @0x1407d16b0 — O。fold 建 HashMap<StairGraphEdgeIdentity,SmallVec<StairSubSegment>>；合并两子世界脏瓦(0x90)；'?'→try_as_wall(+0x284<=3.0) 记受触墙；DirtyTiles::new_clear+take+extend+iter_tiles。
- playermade_stairs_add_wall_holes @0x1407d1f80 — O。脏洞组 bitset(0x110)，条目(0x18)非空→WallHoles::add。
- stairs_subscribe_lazy @0x1407cf210 — O。3×RaycastWorld::subscribe_to_changes+2×Publisher::subscribe<DirtyTiles>+insert_resource<StairsLazySubscriber>。structural_graph_subscribe @0x1407d4990 — O。subscribe<ColliderDiff>+insert_resource<StructuralGraphSubscription>。
- 内联：resample_curve。

## 3. stairs_railings_assemble.rs
- assemble_railings @0x1407f98e0 — O。断言 len；按类型(in_stack_58 映射)+标志(in_stack_50/68) jumptable DAT_142a8cc40 分发。
- railings_tips @0x1407fcc80 — O。closure_0×2 判端；±1；0.0975 法向偏移；类型>2→try_new_from_points<Vec3>+resample+construct_wooden_railing；否则 stone 端帽 random_splits+push<BrickTransformSsbo>(0.46)。
- construct_wooden_railing @0x1407fdf90 — O。顶轨 WoodenPlankSsbo+3×fastrand；竖栏 in_XMM3<1.6 分支。
- add_corner_stone_railing_piece @0x1407febb0 — O。点积选角；push<BrickTransformSsbo>×1-2。

## 4. ui_systems/*
- convert_raycast_hit_to_decorator_dst @0x14127c990 — O。判别 0=Free/2=Wall(can_place_on_roof 门)/0xb=Terrain(can_place_on_terrain)/0xd=StairTop(can_place_on_stair_top)/默认=Invalid(4)。
- try_get_grab_offset_ss_if_valid @0x141277bb0 — O/U。子类型 jumptable DAT_142c23990。raycast_with_ss_offset @0x141278050 — O。from_mouse_ss+raycast；Terrain intersect_ray；无命中 y=0 平面投影(t=-dir.y/dir.z)；输出判别 0xb。
- calculate_decorator_interaction_intent @0x14127cf70 — O。系统。Wall: roof_entity+rotation_yaw_radians+snap_to_roof_ridge+derive_subtype+DecoratorLibKey 碰撞+intersect_roof_decorators+merge_proposal；Terrain: rank2；StairTop: 遍历 PlayermadeStairsAssemblyParams 找端点命中段+Curve::get_coord_at_u 黄金段搜索(1/7×4 迭代)+from_xz_forward_vector；Free/墙: wall_space+get_approx_u_from_pos+intersect_wall_decorators+find_best_wall_decorator_merge_position+should_offer_snap_to_corner(半木0.46/石0.2)+WallCornerAttachment+snap_balcony_door。输出 DecoratorInteractionIntent(~0x60B)。
- snap_balcony_door @0x14127f0e0 — O。'8'→门4、'9'→门9；向下 raycast_simple 判 0xd；平台净空 +0x4c-0.79、边界 0.4；Circle/Rectangle 重叠；输出 (stair_id,height,snap_kind 1..4)。
- display_decorator_wall_parts_and_get_hovered_part_id @0x1412782b0 — O。四元数旋入墙局部；info+0x44 部件数；每部件碰撞取长宽；(i/(n-1)-0.5)*宽*(1-1/n) 布点；signifier 绘制；距离²<100 记悬停 id。
- handle_moving_out_wall_decorator_part @0x141279410 — O。is_attached 角落特例；碰撞；逐部件沿墙新位置；apply_decorator_op。
- try_display_roof_decorator_parts_and_get_hovered_part_id @0x14127a790 — O。roof_entity+get_roof_shape_from_pos_ws_clamped+ShapeParameters::perimeter/as_rectangle+Rectangle2d::get_closest_edge_id/edge_length+RoofTriggerVolume::get_position_from_mesh_uv 布点+signifier。handle_moving_out_roof_decorator_part @0x14127bab0 — O。FracturedPart+TransientDecoratorIdGen::generate+apply_decorator_op×6。
- calculate_move_dst @0x1412801f0 — O/U。按 dst 判别 jumptable DAT_142c248b0。show_decorator_remove_signifier @0x14127fe10 — O。dist 投影+signifier。
- ui_change_chimney_height @0x1407ae2e0 — O。状态4；slider3d；proportional screen space 光标→高射线；fVar33=(cursor_y-原点)-偏移；apply_decorator_op；IndexMap<(id,id),AlignKind> 写 0x12 判别记录 |Δh|/scale；show_signifiers。show_signifiers @0x1407ae910 — O。chimney_height/arrow+height_gizmo 两个 signifier。
- ui_change_stair_height @0x1407adea0 — O。状态4+0x5c=='?'；子类型 jumptable DAT_142a832b0。
- DecoratorAffordanceDispatcher::clear @0x1407af330 — O。*out=6。ui_dispatch_decorator_affordance @0x1407af340 — O。EscMenu 早退；状态 0/3/4/6/0xf；WidgetParams+sounds+occlude_by_world+UiState::add；合并脏 widget 列表(0x70)非空→复制悬停数据；否则 jumptable DAT_142a83790。create_decorator_widget<DecoratorHandleName> @0x141282300 / <&str> @0x1412823c0 — O。WidgetParams::new+sounds+UiState::add<T>+NodeContext::id。
- ui_focus_stairs @0x14079c6d0 — O(头部/调用面)。get_all_connected_nodes(BFS)+WidgetParams+sounds+boop+ui_edit_rectangle_orientation+signifier+DebugLine/DebugSphere+apply_decorator_op。U：函数体。
- ui_decorator_inactive @0x140791400 — O。非交互→DecoratorBackup::clear。
- ui_move_decorator @0x1407a4850 — O(头部/调用面)。签名长(见骨架)。关键：slider3d/is_fence/decorator_info/as_stair/try_get_grab_offset/raycast_with_ss_offset/calculate_move_dst/find_stair_placement_candidate/apply_decorator_op/show_decorator_remove_signifier/RequestAppModeChange/MainCamera/PlayDecoratorSoundCmd/DecoratorOutline::display/can_change_seed。
- find_stair_placement_candidate @0x1407a7e80 — O。flat_roof_y 钳高；Plane::intersect_ray；terrain+FloatGrid 钳；按 snap 类型 roof_uv_at_ws_pos/Raycasting::raycast；fVar11=clamp(Δh,0,100)；输出 StairPlacementCandidate。
- ui_place_decorator @0x1407a9c10 — O(头部/调用面)。TransientDecoratorIdGen+StairsState::endpoint/add_edge/unset_endpoint+StairSegment::new(as_stair 楼梯连图)+apply_decorator_op×5+is_fence+raycast+show_decorator_icon。
- show_decorator_icon @0x1407adb20 — O。Vec2+文本→signifier。decorator_cursor_icon @0x1407adcb0 — O。bVar4=min(6,sub-0x38)；图标查表 DAT_142a830b0；SignifierEntityBuilder+insert<(Ui2dExtent,Ui2dImage,UiInactive)>+Ui2DFollowMouse。
- ui_rotate_decorator @0x1407a8580 — O。状态3；slider3d；子类型 jumptable DAT_142a82be8。
- stairs_reset_for_new_frame @0x14079c350 — O。非绘制态→unset_endpoint(+0x10=3)。continue_stairs @0x14079c3a0 — O。绘制态；TweakerView+共享锁；在建楼梯(0x88)：类型3&&'?'→add_node+endpoint+add_edge+try_set_endpoint。
- visualize_ui_stairs @0x140791510 — O。合并脏瓦计数；drop+重建 StairGizmosCache(5 组 id 分配器)；jumptable DAT_142a81b90 分态绘制。insert_point_ui @0x14079bc20 — O(头部)。查在建楼梯(0x78)+最近段(MAX 初值)。内联 find_rightmost_position_for_gizmo。

## 5. utils/visual_entity_builder.rs
WindowVisualEntityBuilder::build<DecoratorPersistentVisualState> @0x1412a9b30 / <DecoratorTransientVisualState> @0x1412ab270 — O。读 pos+quat 旋转；rank==1 用预览 pos；按网格名 switch：9="nani_ube"→UberInstanceData；10="nani_cloth"→ClothInstanceData / "nani_smoke"→SmokeInstanceData；0x11="nani_balcony_door"→BalconyDoorInstanceData；0x15="nani_lantern_emissive"→LanternEmissiveInstanceData；0x17="nani_solid_vertex_color"→spawn_empty+insert<NaniBundle<SolidVertexColorInstanceData>>(0x88B 命令)；0x19="nani_gothic_window_bricks"→GothicWindowBricksInstanceData；0x1b 砖族。NameHash::compute 各分支前。U：完整清单(>0x1b)。

## 6. lib.rs
impl Plugin::build 无独立符号（enriched 仅内联 try_merge_decorators/snapping_direction 等）。I：被内联进 tiny-glade App::add_plugins 单态化。U：注册顺序。

## 7. 总览
流水线：requests(true/false)→assemble(true/false 分派 8 子函数)→interpolate_and_compare→propagate_dirty_flags+add_wall_holes→generate_playermade_stair_decorators；maintain_stair_graph 维护图。UI：raycast→dst→interaction_intent→place/move/rotate/height→split→affordance→stairs 绘制(focus/viz/insert/continue)。
算法清单(10)：①图维护/级联裁剪 ②受力支撑分割(-1.17/0.5) ③结构图 Dijkstra 畸变洪泛(3/64) ④样条密集采样+坡度分类(0.25/2.5) ⑤墙空间行走(凹点剔除/环形切片) ⑥踏步砖(0.56)/梯(0.65)/扶手(0.0975) ⑦射线落位门控+高钳 ⑧UI 状态机+slider3d ⑨屋脊/墙角/楼梯吸附+合并提案 ⑩帧间插值差分(USE_STAIRS_LAZINESS)。
未知清单：assemble 函数体(空反编译)；<true/false> 泛型语义；各 jumptable 分支(derive_stair_node_params/calculate_move_dst/grab_offset/determine_segment_type)；畸变 kind 掩码枚举；rt_geo 索引表映射；build 完整网格名单；ui_move/place/focus 系统体细节；requests 阶段三；lib.rs 注册顺序；各 closure 规则。
