# wall 域第二轮：逐文件审查表（round2/wall-REVIEW.md）

审查基准：round1 DIGEST 的 O 条目（机器码/反编译/原字节）对照
`reconstruction/country-core/src/{systems/wall,resources/walls}/**` 的现有实现。
状态：**ok**=逻辑与 O 证据一致（typed gap 属预期）；**需修**=存在与证据冲突的错误；
**未知**=O 证据不足无法裁决（多为纯 Gap 门面）。地址均 B patched EXE VA。
本表不改源码，只列差异。

## A. resources/walls/（25 文件）

| file | 状态 | 具体错误 / 缺失项（"代码写 X，证据为 Y"） |
|---|---|---|
| mod.rs | 需修 | `split_height_into_skirts` 循环条件实现 `height > f+1.4` 与机器码一致，ok；但缺 **WallId::is_valid 真身地址注记**（0x140b34f20，代码注释把它挂在 get_inner 0x140885d0 名下，见 DIGEST §7——两个函数都存在，注释措辞易误导，建议分列）。Floor/WallHighlight/HighlightColor 其余为 Gap/占位，ok。 |
| inner_wall_state.rs | 需修 | ① `StructureType::is_fence` 写 `discriminant() != Wall`，证据 0x1408c5680 仅"判别读取、比较在调用方内联"——Fence 值未证实（I），且 `!= Wall` 把 Other 也算 fence，**错误**（应为单变体判别，值 U）。② `is_default_arch_height` 注释地址 0x1408c5690 与 PDB 索引挂名 0x1408c56b0 冲突：v2 索引确认 0x1408c56b0 = `WallState::get_enclosure_params`（机器码 `tag@+0x38<2 ? &self+0x38 : null`），is_default_arch_height 真身在 0x1408c5690（待字节复核）——注释/实现需分开登记。③ WallState 大小未标：证据 **0x70B**（InnerWalls 条目步长 0x78），字段偏移见 DIGEST §1.2（enclosure@+0x38/height@+0x54/freehand_ty@+0x60/arch_type@+0x64/removed_supports@+0x67），当前 `{raw: Gap, height_ty, curve, shape}` 命名布局与真实偏移不符。④ serde 13 字段名表（DIGEST §1.2）未写入文档。 |
| public_wall_state.rs | 需修 | ① `point_count()`/`source_curve_length()` 注释称"偏移歧义"——已解：calc_wall_bend 读的是 0x2a8 步长条目内偏移（map 值区 +0x18 起算），与 raw 曲线 +0x10 是两个不同数组（+0x10 为 raw Curve2 点计数 O）。歧义注释应删除。② `wall_space()` 返回 `WallSpace(Gap)` 空占位——证据 = `{tag!=2, f32@+0x28c}`（Option<f32>=Some(published 长度)）；`wall_space_length()` = 纯 +0x28c；`u_space()` = `{tag!=2, 1.0f@0x142925904}`。三方法可 100% 落地。③ `is_enclosed()` 无证据直连（行 118 内联，语义仍 U，注释可保留）。④ `max_ivy_grow_height`/`corner_normal_xz` 为 Gap，ok（U）。⑤ `max_y()` 返回 Result 包装 panic 语义可接受（注明即可）。 |
| inner_walls.rs | 需修 | ① 结构 `{walls, next_id, gap}` 与证据不符：+0xc0 是**每调用递增的脏代计数器**（wall_ids 0x14085a200），不是 ID 分配游标；+0x30 是**工厂子结构**（wall_ids/new_wall_with_id 尾调 `FUN_1402ca2f0(self+0x30, id)`）。② `new_wall` 用 `next_id++` 分配 id——原作用 max_id/工厂分配（`max_id` 0x14085a730、`reset_max_id` 0x14085a550、`new_wall_with_id` 0x14085a4a0 见 v2 索引）。③ 方法面缺：`update_arch_height`（v2 新证 0x14085ad20 → `_try_update_state<update_arch_height>` 0x140aaa710）、`get_unwrapped` 0x14085a650、`remove` 0x14085aa60（v2 均已有反编译）。④ `update_wall_state_curve/enclosure/height` 等全空占位——`_try_update_state` 六实例（0x140aa9800..0x140aaaca0）已给出精确写偏移（DIGEST §1.2），可落地。 |
| public_walls.rs | 需修 | ① 布局错：证据 PublicWalls.wall_entities_map 在 **+0x30**（delete_wall 0x140aac1e0 从 `*self+0x30` remove_entry），代码写 `{walls, wall_entities_map, roofs, gap}` 按 Rust 字段序摆放与偏移无关但注释未标注；② `simplified_curve_2d` 字段/方法缺失（`assertion failed: w.simplified_curve_2d.is_valid()` public_walls.rs:50 被 publish/spatial_hash_update/clamp_wall_height 三处内联，O）——应登记。③ WallEntities/WallPlasters map 基址在 +0x0（delete_wall 用 `*param_3` 直接作 map），与代码一致，ok。 |
| wall_attachment.rs | ok | `to_rectangle_corner_id`/`from_rectangle_corner_id` 编码（0=None, 1..=4=id+1）与手解机器码一致；`is_attached`=字节!=0 O。注意：to_rectangle_corner_id 证据（0x140b256f0）返回 {present: self!=0, id: self-1}——重建代码 None 分支返回 0 而非 (false, 0)，语义等价但无 present 通道，记 ok-remark。 |
| decorator.rs | 需修 | ① DecoratorType 全表待补（DIGEST §1.5：is_window={0x38,0x39,0x3a}、can_place_on_roof=t≠0x3a、can_place_on_stair_top={0x3c,0x3d,0x3e}∨非窗、can_place_on_terrain=t∉{0x38..0x3b}、should_offer_snap_to_corner=t≠0x3f、can_change_seed={0x38,0x39,0x3a,0x3d}、spawns_clutter={0x38,0x39}）——代码仍 Unknown=0。② `try_dst_as_wall` 手解 = `[rcx+0x22]<2 ? &self : null` ✓；dst_as_wall（v2：0x140833260）= tag≤1 → &self 否则 unreachable(line 40)——代码一致。③ v2 名表：0x140833570 = DecoratorOpEventKind::id（u32），DecoratorDst::as_wall_mut 真身 0x140833540——代码注释无冲突但注意不要引用旧名。④ TransientDecoratorIdGen {base:*mut u32, +0x8 u64, +0x10 bool} 两模式 generate 手解未写进代码（当前 Gap）。⑤ DecoratorRank::new 0x140833250（v2 名）、DecoratorLocator::new 0x140833240 等小方法缺。 |
| decorator_storage.rs | 需修 | ① 五集合映射证据（round2 闭合）：+0x0 wall（iter 0x140a1e5b0 读 [self+0]/[self+8]）、+0x90 terrain（iter_terrain_decorators 0x140a1e280）、+0x120 stair（iter_stair_decorators 0x140a1e2c0）、+0x78 为 0x18B {DecoratorId u32, ptr} 查找索引（lookup_storage 0x140a1c3e0）、+0x18/+0x48 为其余集合（take_snapshot 0x140a20d70 五处判空：+0x18/+0x48/+0x78/+0x90/+0x120）、+0x1b0 可写标志（mark_mutable 0x140a1c2b0 清零）；wall 条目 = 0x98 步长 {WallId u64, Decorators 0x90B}，Decorators 内嵌 DecoratorId→Info map @+0x30、modified@+0x60、removed_ids@+0x0。当前代码全 Gap——可直接落地骨架。② `!self.immutable` 断言（get_or_insert 族）即 +0x1b0 标志语义。 |
| decorator_archivist.rs | 需修 | ① `decorator_interactions` 0x1403c03c0 = 返回 &self+0x90（手解）——代码无此方法。② `decorator_cull_allow_any` 0x140b2bc00 遍历 0x98 步长 map + 命中回调 FUN_140b76260——重建代码恒返 true，**错误**（语义未证实，至少非恒真）。③ validate_param 0x140b2bb00 构建校验描述符（+0x20/+0x28/+0x30/+0x38 u16 掩码/+0x40/+0xc0）——缺失。 |
| decorator_subtype.rs | ok | EMPTY=0x1a、DecoratorLibKey 2 字节键、4 表、insert 克隆语义与证据一致；`can_be_on_corner` 写 `matches!(0, 3|6)`——机器码 0xb25b50 极短（表查询或位移），I 推断已注明，ok。queue 0x14003bb80 = 空函数 ret（代码无 queue，ok）。 |
| wall_holes.rs | 需修 | ① `finalize` 断言文案：原作 panic = `finalize called on a wall hole storage that has not been cleared yet.`（0x140f87300 域）——要求"前一状态已 clear"的前置条件，重建代码断言 `phase == InProgress` + 文案 "finalize"，**文案与前置条件均不符**。② FinalizedWallHoles 布局已证实（holes Vec@0..0x18、wall_length f32@+0x18、closed@+0x1c——holes_no_duplicates 闭包 `movss [rcx+0x18]`），代码注释可升级为 O。③ InProgressHoleStorage::holes = {ptr@+8, len@+0x10} 切片（0x140c37760 手解），代码未表达。 |
| material_stats.rs | 需修 | 全 Gap，但 round1 已解出完整布局（DIGEST §1.6：prev 7×f32@+0x28..、cur 7×f32@+0x90..、8 计数 u32@+0xac..+0xc8、seed u64@+0x88、pillar placements Vec@+0x70{cap,ptr,len}、自旋锁@+0x68、CAS 原子 add、arches_length_delta argmax+bias -0.01、init_this_update 0x140a88280）——可直接落地。v2 名修正：0x140a88420 = set_lintel_pillar_count（非 pillar_count）。 |
| wall_trigger_volume.rs | 需修 | 全 Gap；证据：quad 数组 {ptr@+8, len@+0x10}、元素 0x60B、tri 输出 0x48B（get_tris 0x1408e80f0）、选项 {+0xc bit, +0x10 f32, +0x14/+0x18 u32}、panic `!triangles.is_empty()`/`!mesh_uvs.is_empty()`/`!normals.is_empty()`。`is_valid` 恒 true 无依据（U）。 |
| cached_decorator_transforms.rs | 需修 | `transform` 0x140a1ada0 = 缓存矩阵 9 分量有限性校验 + panic（NaN/Inf），非取位——代码 `try_position/try_transform` 返 None 空占位；v2 新增 transform_at/with_* 族（0x140a1a9f0/aa20/ab80/ac80/af40 已反编译）缺失。 |
| in_progress_wall.rs | ok | `new`/fmt、Option 语义与合同一致（布局 U 标注清晰）。 |
| wall_path_segment.rs | ok | pt_count/length_ws/length_ws_2d 地址 O；语义 I 已标注。 |
| decorator_collision.rs | ok | 无独立函数，Gap 合理。 |
| decorator_outline.rs | ok | display 族（0x140b33920/0x140b33b00/0x140b348e0）语义与代码一致（Lines/Sphere 判别、+0x18 偏移）。 |
| elevation_support_stats.rs | ok | 无独立函数。 |
| arches.rs / curved_halftimber.rs / plaster.rs / wooden_gates.rs / wall_stone_columns.rs | 未知 | 纯 Gap 门面 + 命名证据；wall_stone_columns 有 v2 新反编译 0x140a78880（11KB StoneColumnCollisionStorage）未对照，建议下轮。 |
| dormers.rs | 需修 | 代码写 cottage `e[2]=1.0, e[3]=-50.0`（注释称 0xc24800003f800000）；手解 0x1412866e0：+0x28 处写 **f64**（movsd 全局常量），非两个 f32——gothic 4×f32 vs cottage 2×f32+f64 尾区差异。代码的 f32 数组建模**错误**。 |

## B. systems/wall/（根 13 + 4 子目录）

| file | 状态 | 具体错误 / 缺失项 |
|---|---|---|
| calc_wall_bend.rs | ok | 逻辑与反编译一致（>5 点才算 bend、sum/(n-1)、acos clamp≤1.0）；`compute_bend` 用 Vec2 方向归一（原作用 `d.y*d.y + d.x*d.x` 2D ✓）。事件 16B 行、ResMut 判别（EventWriter param_3 回写）均 O。remark：重建补了 `dirs.len().saturating_sub(1)` 与原文 n-1 一致。 |
| delete_wall.rs | ok | 三 map remove + Commands::despawn 双类型 id 与反编译一致（PublicWalls+0x30 wall_entities_map ✓）。remark：注释 "keep inner wall record removal out of the public cascade" 属推断（原 inner 记录删除域外，U 注明即可）。 |
| draw_wall.rs | 需修 | ① 签名证据 4 ResMut（param_2/7/8/9）+ 合同（InnerWalls/InProgressWallResource/CurveNursery/InProgressCurve）✓，但重建自造 `current_wall_id`（`*local += 1; Some(WallId(*local))`）**编造 id**——原作用 InProgressWall 携带 WallId（typed gap），不应伪造。② CurveNursery 结构（重建 {curve: Option<Curve>}）无 O 依据（U）。③ `update_freehand_ty(wall, 0)` 硬编码 0——证据只证明 draw_wall 调 update_freehand_ty（0x140aa99e0 ×3），参数语义 U。④ 吸附位置 `_snapped` 未用（占位 ok）。 |
| finish_wall.rs | 需修 | ① 签名缺：反编译 12+ 参、5 ResMut（param_4/6/9/10/11），重建只有 5 参 3 ResMut（缺 2 个 ResMut，U 名称）。② panic 语义缺失：`Was asked to finish a wall, but wall_manager has no temp curve`、`Finish wall marked a wall as non-enclosed, but it doesn't exist`（原 panic 路径未表达）。③ `wall_id_of` 返 None 使整段逻辑空转——InProgressWall 的 WallId 字段为 typed gap（证据：原 panic 文案证明 in-progress wall 与 inner 墙 id 关联）。④ enclosure snapshot 构造 `[0;28]` 占位 ok（U）。 |
| merge_walls.rs | 需修 | ① 签名证据含 `DecoratorArchivalResources + ResMut<DecoratorArchivist>`（调用 decorator_cull_allow_any 0x140b2bc00 ×2）✓ 重建一致；② 合并判据实现（"end point touches start within terrain tolerance"）无 O 依据——反编译只见曲线/地形采样调用，判据数值 U，重建的遍历骨架可能误导；③ `InnerWalls::remove`（v2 0x14085aa60）在合并路径应被调用（重建无）。 |
| set_wall_style.rs | 需修 | ① 事件行大小：style 0x18B/freehand 0x20B/plaster 0x18B（证据）——重建读三 reader 但未表达行布局；② 音频事件 {tag=0x10, style, pos.xy, pos.z}（0x20B RequestWallMaterialAudio）——重建写 {wall, pos, raw:Gap} 结构**不符**；③ history：证据 = CompoundEditBuilder::next + from_iter + History::add（0x140a82b40 ×4、0x141696f50 ×3、0x140a84ec0 ×3）——重建 `history.mark_changed()` 占位（可接受 typed gap，但应注明真实序列）；④ `_try_update_state<update_style/update_plaster_peel>` 实例 0x140aaa530/aaa350 未在索引。 |
| wall_splitter.rs | 需修 | ① `cursor_u_for_wall` 硬编码 0.5——证据 = get_curve_segment_from_u/get_u_at_coord（FUN_141467ce0 ×3）真实光标投影；② ChildWall::new(0, 0.0, 0.0) 全零参数占位（U，参数面未知）；③ 样式 `convert_to_stone_version` 命中处转换未表达（合同 O：split 命中处转石）；④ 签名：证据含 Res<AppMode>/Res<PublicWalls>/DecoratorArchivalResources/Res<UiState> ✓ 一致。 |
| wall_highlight.rs | ok | WallEntities 查表 + WallHighlight{entity, wall, ty} 与证据一致；ty→rgba 4 常量（0x3d8f5c29 等）未在重建落地（注释已列，ok-remark）。 |
| upload_instanced_wall_ssbo.rs | ok | get_mut<BrickTransformSsbo> unwrap、+0x108 compact 双路径、gpu_flush 与合同一致；typed gap 明确。 |
| deferred_wall_history.rs | ok | try_new 投影规则（分母下限 1e-8、严格更小、并列取首段、clamp01、严格 low<high→i64::MIN 哨兵）与机器码 0x1408dc9f0 一致；wall_deleted 3 分支组（0x140a876a0）注明。remark：`approximate_curve_coord` 用 `dist < best` 严格比较 ✓。 |
| structure_outline.rs | ok | Esc 关闭才画、ShapeParameters 消费、signifier 名串（outline_wall_top_ 等）与证据一致；描边几何为 typed gap（U 已注明）。 |
| ui_continue_wall.rs | 需修 | ① 证据：ui_continue_wall 消费 `tuple<(Res<InnerWalls>, Res<PublicWalls>)>` + GladeBorder::is_pos_inside + decorator cull + WallContinueState::display_signifier/finish——重建缺 GladeBorder 参数（重建签名无 glade_border）与 decorator cull；② suppress_continue_wall：证据含 Res<GameTime>（"Resource does not exist…GameTime…" panic 串），重建签名缺 GameTime；③ 续画判定阈值 U 已注明。 |
| ui_draw_wall.rs | 需修 | ① 证据 **5 ResMut**（param_5/7/9/10/14），重建签名只有 ResMut<WallDrawState>（且无 CurveNursery！合同列 ResMut<CurveNursery>）——CurveNursery 参数缺失；② RecordMouseInputIntoWall 转发进 CurveNursery 的路径缺失（重建只写事件，无 nursery 写入）；③ 5 个 ResMut 中 3 个未登记（U）。 |
| wall_state/publish_public_wall_state.rs | 需修 | ① 签名：反编译 **14 系统参数**；重建 6 参。证据面：ResMut<InnerWalls>（param_2 回写）+ ResMut<PublicWalls> + Res<WallSpatialHash> + Res<TerrainHeightsData> + EventWriter×2 + 未登记参数（SparseInstanceBuffer 存取 0x140969720、AnimateWallBulldozeBuffer::contains 0x140b772b0、material_stats 0x1409e9640、调试 signifier 等）。② `gather_lazy_update_sources` 重建自造 kind=known?1:0——语义 U（反编译见 lazy 源收集但判据未逐行）。③ 重建循环对每个 source 无条件发 OnWallChanged——证据有 `assertion failed: !is_adding`（添加中重复发布防护）与删除事件分支，缺失。 |
| wall_state/spatial_hash_update.rs | 需修 | ① 证据用 **`w.simplified_curve_2d`** 建段（断言 public_walls.rs:50 三处内联），重建用 `published_curve_points()`——**错字段**；② add_segment 参数面：证据 `add_segment(hash, 3 floats?, &event payload, …)`（0x140aa5dd0 调用点），重建 `(wall: u64, low, high, segment)` 不符（0x38B 成员 = {WallId, 段数据 0x30}）；③ `assertion failed: (0.0..=1.0).contains(&t)` 前置缺失；④ 缺失墙 → panic（SpatialQuery received OnWallChanged with wall index …），重建 continue 静默跳过——**行为不符**。 |
| wall_state/wall_spatial_hash.rs | 需修 | ① `add_segment` 实现为端点包围盒填格——证据：逐格闭包填充（get_cells_from_* 栅格化）+ 0x38B 成员 push（FUN_140562870/1406319d0 扩容）；② `get_cells_from_aabb/circle/rectangle2d/segment` 全空——原断言族（RESAMPLING < cell_size、bbx_min/max 越界）与真实栅格化缺失；③ `cell_center/world_space_to_grid_coord` 未接 bounds.min（代码自注 typed gap，ok 但布局无 O 常量——bounds 常量来源 U）；④ 64×64 为合同先验（walls-audit），未复测。 |
| wall_state/wall_roof_hashmap.rs | 需修 | ① `wall_id_for_roof` 反向遍历 roofs map 找 entity——原逻辑为"预置形状无 roof → 清除；有 roof → 写入映射"（合同 O），重建无清除分支；② 证据 Query<(Entity, &Roof, Option<…>)>，重建 Query<(Entity, &Roof)> 缺 Option 项；③ 1 个 ResMut 回写（param_3）与合同 2 ResMut 的差异未注明。 |
| wall_state/spatial_hash_debug.rs | 需修 | 合同/证据为 DebugRectangle2d，重建用 DebugRectangle3d——**类型不符**（低危）。 |
| wall_state/debug_viz_threshold_curve.rs | ok | 逐墙 DebugSphere 沿发布曲线 ✓。 |
| snapping/mod.rs | 需修 | `position_to_snap_to` 返 Vec3::ZERO 占位（U）；mode 1/2 语义注释 ✓。 |
| snapping/snapping_init.rs | 需修 | ① 门控方向**反了**：证据/合同"DebugGui 禁用或进行中墙空 → 返回"，重建 `if debug_gui.enabled() { return; }`；② 1.65/1.5 阈值条件未实现（重建无条件 `snapping.mode = 1`）。 |
| snapping/snapping_maintain.rs | 需修 | ① `check_for_disable_snapping`：证据 0x140a6a560 = mode∈{2,3} → **写事件**（0x20B 行，间接禁用），重建直接清零 mode——**行为不符**；② `snapping_maintain`：证据 = mod key 切换 + 跳表分发（0x140a6ad00 手解）+ NotifyHintSystemSnapping，重建复制了 check_for_disable 逻辑（错）；③ snapping_signifiers 双查询签名 ✓。 |
| snapping/snapping_to_ends.rs | 需修 | ① 重建构造空 Curve 后 find_closest_end——原作用每墙真实曲线投影（`first_raw_input_point` 投影 + start_pos/get_pos_at_u）；② `find_closest_end_to_snap_to` 无半径限制实现（原有限制 U 值）；③ 证据入口有 mode 前置（`*pcVar4 != 1 → return`，I），重建无。 |
| snapping/snapping_to_segments.rs | ok-remark | 半径 1.5 与 SQRT 距离路径 ✓；closest_segment_to_snap_to 投影实现为占位（typed gap 注明）；mode==1 前置未表达（同 to_ends）。 |
| snapping/snapping_audio.rs | 需修 | 证据：mode==2 且 `[SnappingMode+0x20]==2` → 跳过；mode==3 且 `[+0x20]>1` → 跳过；否则写 PlayWallUiSound。重建：`mode==2 && !snapped → write; mode==3 → write`——**两个分支条件均错**（应读 +0x20 声音状态字节，非 snapped）。 |
| snapping/telegraph_walls_are_merged.rs | ok | PlayLineAcrossWall{a,b} → 连线 signifier + 动画时长 ✓（细节 U 已注）。 |
| adjust_wall_height/adjust_wall_height_impl.rs | 需修 | ① clamp 回退语义：证据"clamp_wall_height 返回 1/0（bool）"（0x140ac7f30 全读：PublicWalls 查墙 → roof_entity → 返回 1；缺失返回 0），重建把它当 `Option<f32>` 数值钳制——**函数语义错**（数值策略在 S02 height_rules/调用方内联，不在该函数）；② 命令 0x30B 行布局注释 ✓；③ `_try_update_state<update_wall_state_height>` ×2（0x140aaaca0）证据未在重建体现（重建直接改 state.height()）。 |
| adjust_wall_height/ui_adjust_wall_height_mode.rs | 需修 | ① `clamp_wall_height` 重建实现数值策略（1.6/0.7/0.3/0.5、2.5/14.0 magic bits）——机器码 0x140ac7f30 无任何浮点比较，只有查表+roof_entity+bool 返回。**函数名与行为不符**：要么 0x140ac7f30 的 PDB 名错位（v1/v2 均挂此名），要么数值钳制被内联进调用方而该函数实为"墙存在性/roof 查询"。需下轮定点裁决；重建实现应标 U 而非 O。② 其余 UI 面 typed gap ok。 |
| adjust_wall_height/ui_adjust_wall_height.rs | ok-remark | 预置形状 + roof flat/no-rim 分支 ✓（调用链）；widget 细节 U 注明。 |
| adjust_wall_height/adjust_wall_ui_mode.rs | ok | widget/signifier 占位 + 地址注记 ✓（U 布局）。 |
| adjust_wall_height/signifier_animation.rs | ok | is_flat_w_no_rim → 重算曲线；否则 SignifierAnimation::t 驱动 ✓；实现占位（typed gap）。 |
| shapes/mod.rs | ok | 4 资源注册 ✓。 |
| shapes/calc_shape_height.rs | ok-remark | 数值合同（base 6.0/8.0、-0.46、clamp[0.7,14.0]、+0x40 写入、time_continuous_lerp）注释完整；实现前半占位（写 `mut cache: Res<CalcShapeHeight>` **Res 却要写入**——编译级 bug，应为 ResMut），后半 typed gap。 |
| shapes/shape_placement.rs | ok-remark | new_unsnapped/closest_segment_to_snap_to/snap_to_roof_ridge 占位 + 证据调用链（WallSpatialHash 查询 0x140b04f70、Roof::pivot_3d/RoofRidgeDims）注明；实现为 stub（typed gap）。 |
| shapes/circle/*, rectangle/*, shape_placement_signifiers.rs, utils/* | 未知 | round1 已反编译全族（ui_edit_circle 94KB 等），但本轮未逐文件对照实现（时间预算）；下一轮建议按 create/edit/ui/signifier 四组逐对照。 |
| valid_enclosure/mod.rs | 需修 | ① 判别编码：证据 get_normal_at_u/get_pos_at_u/perimeter = `test byte[rax],1` **位 0 判别**（tag@+0，载荷@+4），重建用 Rust enum（Curve/Circle/Rectangle）自动判别——位 0 判别与 enum 变体序需核对（serde 名 `IsEnclosedNoYesPremadeShapeIsEnclosedV2` 存在）；② contains_shape/expand/perimeter 算法 U（parry3d 0.17 路径串证据）；③ `center()` 对 Curve 取首末中点——证据 0x1411ff2a0 为 15B 极短函数（不可能含遍历），**实现与机器码规模矛盾**（可能为直接字段读取）。 |
| eraser/animate_wall_bulldoze.rs | ok | 命令行 152B、phase 推进/CubicInOut、增长/完成事件（128 场景 768 全 RET 先验验证）——与合同一致；曲线切片细节 typed gap。 |
| eraser/wall_eraser.rs | ok-remark | 调用链（PublicWalls::roof_entity ×4、WallEraserArchivist::affected_walls、ChildWall::new）✓；determine_freehand_height_ty/wall_height_based_on_length 返 0 占位（U 注明）；eraser 段重连细节 U。 |
| eraser/archivist.rs | ok-remark | ChildWalls/WallHistoryEdits 骨架 + finish→CompoundEditBuilder::build ✓；缓存字段 U。v2 新反编译 new/record_rewiring_event/cache_if_original_wall/shrink_wall（0x14092e6e0..92eff0）已落盘，可下轮对照。 |
| eraser/ui_bulldozer.rs | ok-remark | bulldoze_condition = AppMode{tag==9, +4==3}（round1 手解补正）与 RoofDelete FIRST + WallCompound StateChange+Delete 双组次序 ✓；命中/选区上游 U。 |

## C. 汇总统计

- ok / ok-remark：16 文件
- 需修：22 文件（A 域 10、B 域 12）
- 未知（下轮）：6 文件（arches/curved_halftimber/plaster/wooden_gates/wall_stone_columns/shapes 子族）

最高优先级修复（与 O 证据直接冲突，按严重度）：
1. `snapping_init` 门控方向反（DebugGui 分支）+ `snapping_audio` 两分支条件（应读 +0x20 状态字节）+ `check_for_disable_snapping` 应写事件非直接清零。
2. `spatial_hash_update` 用错字段（应为 simplified_curve_2d）+ 缺墙 panic 行为不符。
3. `clamp_wall_height`（0x140ac7f30）重建实现与机器码完全不符（实为查表+roof_entity+bool）。
4. `ui_draw_wall` 缺 CurveNursery ResMut + `publish_public_wall_state` 签名 6/14 参数缺口。
5. `dormers.rs` cottage 尾区 f64 建模错；`wall_holes.rs` finalize 断言文案/前置错。
6. `draw_wall.rs` 自造 id 分配、`finish_wall.rs` 缺 2 ResMut 与 panic 语义。
7. `valid_enclosure::center()` 实现规模与 15B 机器码矛盾。
