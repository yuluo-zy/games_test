"""Round-3 chapter filler for the wall domain (tg_a2)."""
import glob
import json
import os
import re

ROOT = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep"
CHAPTERS = os.path.join(ROOT, "chapters")
DECOMP = os.path.join(ROOT, "wall", "decomp")
NL = chr(10)
CRNL = chr(13) + chr(10)

FILE_PURPOSES = {
 "resources/walls/mod.rs": "墙资源族根模块：全局墙 ID（WallId u64 newtype）、Floor 楼板组件、WallHighlight 高亮资源、HighlightColor 高亮色枚举，以及模块级 split_height_into_skirts 墙裙边高度带计算（height>=5.9 起 4.5 每 +3.5）。",
 "resources/walls/arches.rs": "拱段/过梁段的跨帧状态容器（ArchSegment/ArchSegments/LintelSegment/LintelSegments/BuildingHadArchesLastFrame）：由 system_wall_constructor::construct_arches 写回，供跨帧差分，无独立业务函数。",
 "resources/walls/cached_decorator_transforms.rs": "装饰物缓存变换资源：墙高/几何变化后装饰重定位的缓存；transform 入口对缓存 3x3 矩阵做有限性（NaN/Inf）校验并 panic（0x140a1ada0）。",
 "resources/walls/curved_halftimber.rs": "弯曲半木墙组件（CurvedHalfTimber，含 16 段端帽掩码）与 drawable 集合资源；渲染域消费，无独立业务函数。",
 "resources/walls/decorator.rs": "装饰附着/信息类型族：DecoratorInfo 72B 记录、DecoratorId(u32)/DecoratorIdGen、DecoratorDst 目的地联合（tag@+0x22）、DecoratorExtraInfo 附加信息联合、DecoratorType 判别族（0x38..0x3f 位运算谓词）、DecoratorRank、DecoratorRotationXZ、TransientDecoratorIdGen、装饰操作事件族。",
 "resources/walls/decorator_archivist.rs": "装饰归档/剪枝资源：墙合并/分裂/擦除前对墙属装饰做快照归档，提供 cull 放行策略（任意/地形/楼梯）与 before/after 差异提取；interactions 集合在 +0x90。",
 "resources/walls/decorator_collision.rs": "装饰物理碰撞体集合（DecoratorPhysicsColliders）：parry3d 0.17.0 复用边界，无独立业务函数。",
 "resources/walls/decorator_outline.rs": "装饰描边数据（DecoratorOutline{尺寸+0x20..,位置+0x30..,判别}/DecoratorCollisionOutline{Lines(Vec<Vec3>)/Sphere}）与调试显示（Mat3+平移逐顶点变换）。",
 "resources/walls/decorator_storage.rs": "装饰存储：按目的地分桶（+0x0 墙 HashMap<WallId,Decorators 0x90B>、+0x90 地形、+0x120 楼梯、+0x18/+0x48 其余、+0x78 为 0x18B{DecoratorId,ptr} 查找索引、+0x1b0 可写标志）；支持快照(take_snapshot，16B XOR 掩码)、修改追踪、墙合并/分裂时装饰迁移（d2s 同步）。",
 "resources/walls/decorator_subtype.rs": "装饰子类型网格库：DecoratorSubtype(u8, 空=0x1a)、DecoratorLibKey{subtype,variant} 复合键、DecoratorMeshLibrary 4 表（collision/interaction/outline/collision_outline）、DerivedDecoratorSubtypes(HashMap<u32,Arc<DerivedDecoratorInfo>>)。",
 "resources/walls/dormers.rs": "老虎窗洞 SSBO 记录：DormerHoleSsbo 48B（gothic 4×f32 尾区 / cottage 2×f32+f64 尾区）与 DormerHoleCountPerCellSsbo 计数。",
 "resources/walls/elevation_support_stats.rs": "升降支撑统计（ElevationSupportStats/ElevationSupports）：墙高调整的支撑统计，供 audio elevation_supports_audio 消费；无独立业务函数。",
 "resources/walls/in_progress_wall.rs": "编辑中墙：InProgressWall 组件与 InProgressWallResource（Option 语义）；徒手墙绘制输入到预备轮廓到提交的前置状态（finish_wall 取 .wall.take()）。",
 "resources/walls/inner_wall_state.rs": "内部墙状态权威：WallState 0x70B（enclosure 记录@+0x38、height f32@+0x54、terrain floor@+0x58、freehand_ty u8@+0x60、arch_type u8@+0x64、removed_supports u8@+0x67、双 Vec 曲线）+ WallStyle/StructureType/ArchType/FreehandHeightType/PlasterPeelAmount 枚举族与兼容 serde（13 字段名表）。",
 "resources/walls/inner_walls.rs": "内部墙集合资源：HashMap<WallId, WallState 0x70>（条目步长 0x78）+ +0xc0 脏代计数器 + +0x30 墙工厂子结构；new_wall/new_wall_with_id/get/get_unwrapped/remove/max_id/reset_max_id 与 _try_update_state 闭包族（6+3 实例，写偏移即 WallState 布局证据）。",
 "resources/walls/material_stats.rs": "墙材质统计：7 特征长度 prev@+0x28..+0x40/cur@+0x90..+0xa8（plain/lintels/arches/栅栏/gates/stairs/sewers）、8 段/柱计数 u32@+0xac..+0xc8、seed u64@+0x88、pillar placements Vec@+0x70{cap,ptr,len}（自旋锁@+0x68 保护）；add_*_length 为 CAS 原子累加（加本特征扣 plain 预算）；which_material_changed_the_most = 7 对 |cur-prev| argmax（plain 带 -0.01 bias）。",
 "resources/walls/plaster.rs": "石膏墙组件（Plaster）与剥离动画资源（AnimatedPlasterPeel）+ Recalculate* 命令族；渲染域消费（WP6），无独立业务函数。",
 "resources/walls/public_wall_state.rs": "公共墙状态 PublicWallState 0x2a0B：raw Curve2{ptr@+8,len@+0x10,长@+0x30}、3D 曲线@+0x88、高度数组{ptr@+0xe0,len@+0xe8}、max_y/gable 缓存{+0x108/+0x10c, +0x110/+0x114}、形状记录 28B@+0x258（0=Circle/1=Rectangle/2=Freehand）、发布长度@+0x28c、style@+0x299；方法面含 wall_space/u_space（Option<f32>，u_space=1.0f 常量）与锚高投影。",
 "resources/walls/public_walls.rs": "公共墙集合：HashMap<WallId, PublicWallState>（0x2a8 步长）+ wall_entities_map@+0x30 + roofs map + simplified_curve_2d 简化 2D 曲线（publish/spatial_hash/clamp 三处断言 is_valid）；WallEntities/WallPlasters 资源。",
 "resources/walls/wall_attachment.rs": "墙锚类型：WallAttachmentAnchor（platform WallId/wall_height_relative_y/anchor_ws[3]/corner/is_bottom_door/side）、WallAttachmentSide 0/1、WallCornerAttachment（0=None, 1..=4=矩形角 id+1）；serde 兼容层。",
 "resources/walls/wall_holes.rs": "墙门窗洞存储：WallHoleStorage 0x68B 两阶段（raw Vec 20B {center,dims,kind} 到 finalized 20B {Aabb2,kind,origin}）+ sum/xor 指纹 + wall_length/closed；finalize 按 maxX 排序、闭合时跨 seam 平移副本（origin 0/1）；clear_for_next_frame 迁入 PrevWallHoles。",
 "resources/walls/wall_path_segment.rs": "墙路径段（WallPathSegment）：墙切分/轮廓预备中间态，含预备点数组与 2D/3D 长度（pt_count/length_ws/length_ws_2d）。",
 "resources/walls/wall_stone_columns.rs": "石柱碰撞存储（StoneColumnCollisionStorage）：墙体石柱的碰撞数据存储（v2 新反编译 0x140a78880）。",
 "resources/walls/wall_trigger_volume.rs": "墙触发体积：quad 数组 {ptr@+8,len@+0x10}（元素 0x60B）到 get_tris 输出 0x48B 三角形；recompute_quads 重算网格（panic !triangles/!mesh_uvs/!normals.is_empty）；形状编辑拖拽触发体。",
 "resources/walls/wooden_gates.rs": "木门资源：木门/栅栏门相关状态（无独立业务函数，渲染/构造域消费）。",
 "resources/stairs_walls.rs": "楼梯-墙支柱提议（StairsPillarProposals）：楼梯与墙交界处的支柱放置提议集合（add 0x140901270）。",
 "systems/wall/calc_wall_bend.rs": "墙弯折角重算系统：消费 OnWallChanged，读发布曲线前 4 段法向 delta 相邻点积平均到 acos 到 clamp<=1.0（点>5 才算，否则 0），写 WallBendChangedEvent（16B 行）。",
 "systems/wall/deferred_wall_history.rs": "延迟墙历史记录：PendingWallChanged::try_new 把前一墙首/末端点投影到下一曲线（approximate_curve_u：分母下限 1e-8、严格更小、并列取首段、clamp01、严格 low<high，否则 i64::MIN 哨兵）到 12B StateChange range；history_record_wall_changed/merge 记 ConcurrentEdits 组；CompoundEditBuilder::wall_deleted 记 Delete 组。",
 "systems/wall/delete_wall.rs": "墙删除系统：消费 OnWallDeleted（16B 行），从 PublicWalls.wall_entities_map(+0x30)/WallEntities/WallPlasters 三 map remove_entry，Commands::despawn 墙实体与逐石膏实体（两个类型 id）。",
 "systems/wall/draw_wall.rs": "徒手墙绘制系统（最大生命周期系统）：平滑输入空早退；SnappingMode 吸附光标位置；InProgressWall::new 起笔；CurveNursery 累积笔画；InProgressCurve::add_weight_to_the_last_point；Curve::crashy_new_from_points 到 InnerWalls::update_wall_state_curve + update_freehand_ty。4 ResMut（InnerWalls/InProgressWallResource/CurveNursery/InProgressCurve）。",
 "systems/wall/finish_wall.rs": "墙收尾系统：消费 FinishWall，取进行中墙（无则 panic 'wall_manager has no temp curve'），mark_as_non_enclosed + update_wall_state_enclosure，CompoundEditBuilder 记历史（Create(seed) 到 StateChange）；5 ResMut（含 History）。",
 "systems/wall/merge_walls.rs": "墙合并系统：检测共线/相邻墙（地形采样），Curve::crashy_new_from_points 合并曲线，InnerWalls::new_wall；经 DecoratorArchivist::decorator_cull_allow_any 放行装饰归档重挂。",
 "systems/wall/set_wall_style.rs": "墙样式命令系统：三事件读器（SetWallStyleCmd 0x18B/SetFreehandHeightTypeCmd 0x20B/SetPlasterPeelAmountCmd 0x18B）；style 到 update_style + CompoundEditBuilder 历史；freehand ty 到 update_freehand_ty+update_arch_type；plaster peel 到 update_plaster_peel；样式改变后写 0x20B RequestWallMaterialAudio {tag=0x10, style, pos}。",
 "systems/wall/structure_outline.rs": "结构描边系统：消费 OutlineWallCmd/OutlineStairsCmd，Esc 菜单关闭才画；逐墙 ShapeParameters（as_rectangle/as_circle/as_curve2/center/expand/perimeter）+ 曲线采样到 UiSignifierStream 描边 signifier（outline_wall_top_/bottom_/roof_ridge_/roof_side_/circle_*/rect_corner_ 名前缀）。",
 "systems/wall/ui_continue_wall.rs": "续画墙 UI：光标接近已结束墙端点到 InProgressWall::new 续画（GladeBorder::is_pos_inside + 装饰 cull + WallContinueState::display_signifier/finish）；suppress_continue_wall 在修饰键/光标移动时抑制（Res<GameTime>）。",
 "systems/wall/ui_draw_wall.rs": "墙绘制 UI 入口：glade 内拖动且非清画布到 RecordMouseInputIntoWall 转发进 CurveNursery；释放到 WallDrawState::finish + FinishWall。5 ResMut（含 WallDrawState/CurveNursery）。",
 "systems/wall/upload_instanced_wall_ssbo.rs": "实例化墙砖 SSBO 上载：AssetSsboLibrary::get_mut<BrickTransformSsbo>（缺失 unwrap panic）；逐 InstancedWall 脏区间（+0x108 compact 标志双路径）upload_to_storage_buffer；SparseInstanceBuffer::gpu_flush。GPU 前段。",
 "systems/wall/wall_highlight.rs": "墙高亮系统：消费 HighlightWallCmd，WallEntities 查实体，写 WallHighlight{entity, wall, ty}；EditingHighlight rgba 按 ty 四常量选择。",
 "systems/wall/wall_splitter.rs": "墙分裂系统：光标拖过墙到 get_curve_segment_from_u/get_u_at_coord 求 split u 到 ChildWall::new 两子墙到 InnerWalls::new_wall；命中处 WallStyle::convert_to_stone_version；装饰经归档资源重挂（split_wall/split_wall_scissors signifier）。",
 "systems/wall/snapping/mod.rs": "吸附模块根：SnappingMode{mode u8@0, snapped bool@1}（mode 1=段吸附 2=端吸附）+ +0x20 声音状态字节；is_active/position_to_snap_to。",
 "systems/wall/snapping/snapping_audio.rs": "吸附音效系统：mode==2 且声音状态字节(+0x20)!=2，或 mode==3 且 <=1 到写 PlayWallUiSound。",
 "systems/wall/snapping/snapping_init.rs": "吸附初始化：重置 mode/snapped；DebugGui 禁用或进行中墙空返回；首段长<=1.65 返回；后续点投影首段 <=1.5 到 mode=1+snapped。",
 "systems/wall/snapping/snapping_maintain.rs": "吸附维护：snapping_maintain = mod key 切换 + NotifyHintSystemSnapping（跳表分发）；check_for_disable_snapping = mode 在 {2,3} 到写事件；snapping_signifiers 双查询画段/端预览线。",
 "systems/wall/snapping/snapping_to_ends.rs": "端吸附系统：非 enclosure 公共墙；InProgressCurve::first_raw_input_point 投影墙曲线（get_pos_at_coord/get_pos_at_u/start_pos）到 find_closest_end_to_snap_to（u=0/1）到 mode=2。",
 "systems/wall/snapping/snapping_to_segments.rs": "段吸附系统：raw_input_curve_length_ws；closest_segment_to_snap_to（get_curve_segment_from_u/get_tangent_at_u + distance_segment_point）最近段到 mode=1。",
 "systems/wall/snapping/telegraph_walls_are_merged.rs": "合并预告线系统：PlayLineAcrossWall{a,b} 到 Curve::crashy_new_from_points+crashy_slice 连线两墙端点到 LineAcrossWallSignifier（animation_duration/custom_shader）。",
 "systems/wall/wall_state/mod.rs": "wall_state 子模块根：发布/空间哈希系统。",
 "systems/wall/wall_state/publish_public_wall_state.rs": "公共墙状态发布（权威流）：14 系统参数；收集 lazy 更新源到逐墙重建 public 记录（源曲线点/形状/style halftimber/material_stats/SparseInstanceBuffer::allocate/哈希段维护）；AnimateWallBulldozeBuffer::contains 保留动画墙；发 OnWallChanged/OnWallDeleted（16B 行）；断言 !is_adding 与 w.simplified_curve_2d.is_valid()。",
 "systems/wall/wall_state/spatial_hash_update.rs": "空间哈希同步系统：OnWallChanged 到按 simplified_curve_2d 逐点 get_pos_at_coord/sample_at_coord 建段到 add_segment（缺墙 panic 'SpatialQuery received OnWallChanged with wall index'）；OnWallDeleted 到 remove_wall_id。断言 (0.0..=1.0).contains(&t)。",
 "systems/wall/wall_state/wall_spatial_hash.rs": "墙空间哈希结构：固定 64×64 格（walls-audit）；add_segment 逐格填 0x38B 成员 {WallId, 段数据}（格条目 0x30B 链表）；remove_wall_id；get_cells_from_aabb/circle/rectangle2d/segment 栅格化查询（断言 RESAMPLING<cell_size 与 bbx 越界）。",
 "systems/wall/wall_state/wall_roof_hashmap.rs": "墙到屋顶映射维护：Query<(Entity,&Roof,Option<...>)> 遍历，维护 PublicWalls.roof_entity 与 WallEntities.wall_entity；预置形状无 roof 到清除。",
 "systems/wall/wall_state/spatial_hash_debug.rs": "空间哈希调试可视化：DebugGui 开时逐格 DebugRectangle2d + 成员线（MouseDepthValue 平面）。",
 "systems/wall/wall_state/debug_viz_threshold_curve.rs": "墙高阈值曲线调试：逐公共墙沿发布曲线画 DebugSphere。",
 "systems/wall/adjust_wall_height/mod.rs": "墙高调整子模块根。",
 "systems/wall/adjust_wall_height/adjust_wall_height_impl.rs": "墙高命令执行：AdjustWallHeightCmd 0x30B{ty u32@+0x10, height f32@+0x14, WallId@+0x18, follow-terrain}；clamp 回退命令高度；follow-terrain 采样 TerrainHeightsData；预置形状到 _try_update_state<update_wall_state_height>（×2）+ 0x28B 事件{WallId,pos+height,u32,t,3}，否则 0x20B RequestWallMaterialAudio。",
 "systems/wall/adjust_wall_height/ui_adjust_wall_height_mode.rs": "墙高调整模式 UI：clamp_wall_height（机器码 = PublicWalls 查墙到 roof_entity 到 bool；数值钳制策略见 S02 height_rules，名实冲突待裁决）、history_record_adjust_wall_height（ev_height_changed）、get_closest_point_u_to_cursor（points_with_u/segments_with_u 投影）、模式进出 + 光线拾取 + TestCube + 装饰地形 cull。",
 "systems/wall/adjust_wall_height/ui_adjust_wall_height.rs": "墙高 UI 入口：未拖动时选中预置墙到 UiWallHeight::new 开高度控件；roof flat/no-rim/动画分支；写 AdjustWallHeightCmd。",
 "systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs": "墙高 widget/曲线 UI：UiWallHeight{widget,signifier,new} + adjust_wall_height_curve 插值曲线（widget 层）。",
 "systems/wall/adjust_wall_height/signifier_animation.rs": "墙高 signifier 动画：平屋顶无 rim（Roof::is_flat_w_no_rim）到 adjust_wall_height_curve 重算 + crashy_new_from_points；否则 SignifierAnimation::t 驱动。",
 "systems/wall/eraser/mod.rs": "擦除子模块根。",
 "systems/wall/eraser/animate_wall_bulldoze.rs": "墙推土动画：AnimateWallBulldozeBuffer 152B 命令行{Source112, WallId+0x70, low/high+0x78/+0x7c, anchor+0x80, phase+0x8c, finished+0x90}；每帧 phase=clamp01(phase+dt/0.2)（先推进后应用 CubicInOut）；增长写 Source 切片+enclosure tag2+style；完成移除 authority（128 场景/768 全 RET 先验验证）。",
 "systems/wall/eraser/ui_bulldozer.rs": "推土机 UI 入口：wall_eraser_bulldoze_condition = AppMode{tag==9, +4==3}；handle_decorators 排空墙属装饰 + terrain/stair cull；can_reconnect_stairs_if_midpoint_deleted；StairsState::disconnect_edge；命中到 AnimateWallBulldozeCmd 入 buffer（RoofDelete FIRST + WallCompound StateChange+Delete 双组）。",
 "systems/wall/eraser/wall_eraser.rs": "徒手墙擦除系统：拖笔选段到 WallEraserArchivist::affected_walls/has_anything_been_affected 到 InnerWalls::get_unwrapped/update_wall_state(_curve/_enclosure) + new_wall；determine_freehand_height_ty、wall_height_based_on_length、adjust_wall_height_if_erased_flat_roof_w_no_rims（PublicWalls::roof_entity ×4）；ChildWall::new 子墙。",
 "systems/wall/eraser/archivist.rs": "擦除归档器：WallEraserArchivist 累积 original/split/rewire 事件（cache_if_original_wall/shrink_wall/add_split_event/add_fully_erased_event），finish 经 CompoundEditBuilder::build 编译 WallHistoryCompoundEdit；断言 original_wall_state.curve.is_valid()/!before_wall.curve.is_empty()；ChildWalls/WallHistoryEdits。",
 "systems/wall/shapes/mod.rs": "形状子模块根：add_wall_shape_ux 注册 CalcShapeHeight/PreviewCircle/PreviewRectangle/ShapePlacement 四资源。",
 "systems/wall/shapes/calc_shape_height.rs": "形状预览高度计算：shape{tag=6,ty} ty1=circle/2=rectangle 否则高度 0；as_curve2 到 crashy_resample_remap_per_segment 地形采样；base rect 6.0/circle 8.0；height-ty 2 到 +地形否则 avg+base-0.46；final-(max-0.46) clamp[0.7,14.0] 写预览+0x40 + time_continuous_lerp。",
 "systems/wall/shapes/shape_placement.rs": "形状放置参数：光标地形位到 ShapePlacement::new_unsnapped；closest_segment_to_snap_to（WallSpatialHash 查询 + distance_segment_point）；snap_to_roof_ridge（Roof::pivot_3d/RoofRidgeDims::calc）。",
 "systems/wall/shapes/shape_placement_signifiers.rs": "形状放置 signifier：吸附时 preview_snapping_line（ShapeParameters::expand + Curve::get_tangent_at_u/try_slice）+ 形状 outline；断言 !u.is_nan()/!u.is_infinite()。",
 "systems/wall/shapes/utils/shape_signifier_lerp.rs": "形状 signifier 插值：ShapeVal{interpolate,get_radius,get_rectangle_dims}、ShapeSignifierLerp::lerp_radius——circle/rectangle 值间插值（glam 复用）。",
 "systems/wall/shapes/circle/mod.rs": "circle 形状子模块根。",
 "systems/wall/shapes/circle/create_circle.rs": "圆形墙创建：CreateCircleCmd 到 Circle2d::as_curve2 到 crashy_new_from_points、enclosure=circle、style/elevation、RoofShapeParams::from_wall_shape + roof_height_01_from_wall_shape 到 CreateRoofCmd、历史。",
 "systems/wall/shapes/circle/ui_create_circle.rs": "圆形绘制 UI：CameraCursorRay::intersect_with_xz_plane 拖拽半径到 PreviewCircle；circle_signifier（custom shader）。",
 "systems/wall/shapes/circle/ui_edit_circle.rs": "圆形编辑 UI：update_circle_radius（拖触发体 WallTriggerVolume::is_valid/edit_shape_default）到 update_wall_state_curve/update_wall_state_enclosure；finish_circle_edit 历史。",
 "systems/wall/shapes/circle/ui_rotate_circle.rs": "圆形旋转 UI：world_space_to_screen_space + DebugLine 旋转到更新曲线/enclosure；history 复合编辑。",
 "systems/wall/shapes/circle/signifiers/edit_arrow.rs": "圆形编辑箭头 signifier：Query 驱动箭头位姿（edit_circle_ 前缀）。",
 "systems/wall/shapes/circle/signifiers/edit_line.rs": "圆形编辑半径线 signifier：Query 驱动半径线位姿。",
 "systems/wall/shapes/rectangle/mod.rs": "rectangle 形状子模块根。",
 "systems/wall/shapes/rectangle/create_rectangle.rs": "矩形墙创建：CreateRectangleCmd 到 Rectangle2d::as_curve2 + smoothstep 角 + ridge_length_01_from_rectangle_ratio + roof_height_01_from_wall_shape 到 CreateRoofCmd、历史。",
 "systems/wall/shapes/rectangle/edit_rectangle.rs": "矩形编辑命令：EditRectangleCmd 到 update_wall_state_curve + update_wall_state_enclosure。",
 "systems/wall/shapes/rectangle/finish_edit_rectangle.rs": "矩形编辑收尾：UiState80 快照先发；维度 state1/朝向 state2；改变判据 abs(acos(prev·cur))*57.29578>0.01 或维度差>0.01；记 ConcurrentEdits + niche8000000000000004；无变化到 DecoratorArchivist 恢复（192 全原调用验证）。",
 "systems/wall/shapes/rectangle/ui_create_rectangle.rs": "矩形绘制 UI：rectangle_dims_from_cursor_pos + get_rectangle_orientation 到 PreviewRectangle。",
 "systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs": "矩形尺寸编辑 UI：get_distance_to_edge 拖边到新 dims 到 EditRectangleCmd；decorator_cull_allow_any。",
 "systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs": "矩形朝向编辑 UI：box_trigger_volume/skewed_box_trigger_vol 旋转到 EditRectangleCmd/FinishEditRectangleCmd；StairsPillarProposals 变体。",
 "systems/wall/shapes/rectangle/signifier_edit_dims_line.rs": "矩形尺寸编辑线 signifier（rectangle_edit_dims_line_ 前缀）。",
 "systems/wall/shapes/rectangle/ui_modes/edit_dims.rs": "矩形尺寸编辑 widget/signifier/trigger_volume（UiRectangleDimsEdit，widget 层）。",
 "systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs": "矩形朝向编辑 widget/signifier/trigger_volume（UiRectangleOrientationEdit，widget 层）。",
 "systems/wall/valid_enclosure/mod.rs": "有效围合 ShapeParameters：Curve/Circle/Rectangle 枚举（tag@+0 位 0 判别、载荷@+4）；center/as_curve2/as_curve3/as_circle/as_rectangle/contains_point/contains_shape/perimeter/get_normal_at_u/get_pos_at_u/get_closest_u_from_pos/expand；serde 名 IsEnclosedNoYesPremadeShapeIsEnclosedV2。",
}

ROLE = json.loads(open(os.path.join(ROOT, "wall", "role-map.json"), encoding="utf-8").read()) if os.path.exists(os.path.join(ROOT, "wall", "role-map.json")) else {}

def name_role(name):
    if "::is_" in name or "::has_" in name or "::can_" in name or "::should_" in name or "::contains" in name:
        return "谓词/校验"
    if "::set_" in name:
        return "设置/写入"
    if "::add_" in name or "::push" in name or "::insert" in name or "::extend" in name:
        return "累加/追加"
    if "::update_" in name:
        return "更新"
    if "::new" in name or "::from_" in name:
        return "构造/初始化"
    if "::get" in name or "::try_get" in name or "::as_" in name or "::to_" in name or "::iter" in name or "::into_" in name:
        return "访问器/转换"
    if "::generate" in name or "::reserve" in name:
        return "生成/分配"
    if "::finish" in name or "::finalize" in name:
        return "收尾/提交"
    if "::ui_" in name:
        return "UI 交互"
    if "::record" in name or "::history" in name:
        return "记录/历史"
    if "::display" in name or "::draw" in name or "::render" in name:
        return "显示/绘制"
    if "::serialize" in name or "::deserialize" in name or "::visit_" in name:
        return "序列化"
    if "::remove" in name or "::delete" in name or "::clear" in name or "::retain" in name:
        return "删除/清理"
    if "::clone" in name or "::eq" in name or "::fmt" in name or "::hash" in name:
        return "derive 胶水"
    if "::send" in name or "::write" in name or "::flush" in name:
        return "事件发送"
    if "::recompute" in name or "::rebuild" in name or "::resample" in name:
        return "重算"
    if "::maintain" in name or "::init" in name or "::tick" in name or "::advance" in name:
        return "维护/推进"
    if "::snap" in name or "::clamp" in name or "::lerp" in name or "::interpolate" in name:
        return "吸附/数值"
    return None

def main():
    files = []
    files += glob.glob(os.path.join(CHAPTERS, "resources", "walls", "*.md"))
    files += glob.glob(os.path.join(CHAPTERS, "systems", "wall", "**", "*.md"), recursive=True)
    sp = os.path.join(CHAPTERS, "resources", "stairs_walls.rs.md")
    if os.path.exists(sp):
        files.append(sp)
    files = sorted(set(files))
    done = 0
    for path in files:
        rel = os.path.relpath(path, CHAPTERS).replace("\\", "/")
        raw = open(path, "rb").read()
        crlf = raw.find(CRNL.encode()) >= 0
        text = raw.decode("utf-8").replace(CRNL, NL)
        if not text.startswith("# "):
            print("SKIP (no header)", rel)
            continue
        purpose = FILE_PURPOSES.get(rel[:-3] if rel.endswith(".md") else rel)
        if purpose:
            pat = "(# .*?" + NL + NL + "> 文件作用：).*?(?=" + NL + ")"
            text = re.sub(pat, lambda m: m.group(1) + purpose, text, count=1)

        def fill_row(m):
            fname, addr, line, old_role, concl, evid = m.group(1), m.group(2), m.group(3), m.group(4), m.group(5), m.group(6).strip()
            role, status = ROLE.get(addr, (None, None))
            if role is None:
                role = name_role(fname) or old_role
                if role == "待推断":
                    role = "待推断"
            good = os.path.exists(os.path.join(DECOMP, "decomp-" + addr + ".txt")) and \
                   '"error"' not in open(os.path.join(DECOMP, "decomp-" + addr + ".txt"), encoding="utf-8").read(200)
            if status in ("O-d", "O-h"):
                concl = "O"
            elif status == "I":
                concl = "I"
            elif status == "U":
                concl = "U"
            elif concl == "待证":
                concl = "I" if good else "U"
            return "| `" + fname + "` | " + addr + " | " + line + " | " + role + " | " + concl + " | " + evid + " |"

        text = re.sub(r"\| `([^`]+)` \| (0x[0-9a-f]+) \| (\d+) \| ([^|]*) \| ([^|]*) \| ([^|]*)\|", fill_row, text)

        def fill_inline(m):
            nm = m.group(1)
            role = name_role(nm) or "待推断"
            return "- `" + nm + "`（行 " + m.group(2) + "）— " + role

        text = re.sub(r"- `([^`]+)`（行 (\d+)）— (待推断|谓词/校验)", fill_inline, text)
        open(path, "w", encoding="utf-8", newline="").write(text.replace(NL, CRNL) if crlf else text)
        done += 1
    print("chapters updated:", done)

if __name__ == "__main__":
    main()
