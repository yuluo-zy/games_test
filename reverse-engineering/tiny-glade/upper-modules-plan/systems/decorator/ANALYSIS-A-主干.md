# systems/decorator 主干子范围分析（B 版）

> 由批次 3 分析 agent 产出，父级代为落盘。证据：6 批 ghidra 反编译（`evidence/toolchain/decor-trunk/decompiled/*.c`，job JSON 在 `tool-usage/jobs/decor-trunk/`），全部 `--worker shared` 只读。骨架 PDB 均 void() 占位，字段偏移为反编译观察级。

- 子范围：蓝图/加载/子类型、操作/命令、状态/资源、越界/合并/跟随、墙/形状变化响应、历史/回放/音频。visual 族与 ui_systems、utils/visual_entity_builder 由并行 agent 覆盖，本文件只登记其被调用事实。
- 全部签名 PDB 为 void() 占位；O=逻辑恢复，I=实现明确，U=未知。墙面四族/subtype 复用 `reconstruction/building/decorator_subtype.rs`（标注「可复用」）。

## 1. blueprint.rs
- **DecoratorBlueprint::set_colliders 0x1408252b0 O/I**：输入两个碰撞数据；先 `Vec4::is_finite_mask` 有限性检查（不过→panic_fmt，用 Vec3/Quat::fmt）；对 `+0x50` 计数非 0 时释放 `+0x60` 处 `Arc<RaycastMeshData>`（归零 drop_slow）；写 `+0x50`(u32=0)/`+0x54`(ptr)/`+0x5c`(len)/`+0x60`(Arc)/`+0x70+0x78`；第二块同构 `+0x80..+0x90`。算法：替换式安装两个蓝图碰撞网格。U：碰撞类型精确拆分。
- **set_interaction_mesh 0x1408255e0 O/I**：同构但只写第二块（`+0x80`）。
- **add_door_stairs 0x140825830 O/I**：__rust_alloc 32B，复制 `*in_RDX` 32 字节；释放 `+0x120` 旧 Box 再写新指针。U：32B 结构字段（door_stairs params）。
- **DecoratorBlueprints::insert 0x1408258b0 O/I**：`HashMap<DecoratorId, Arc<DecoratorBlueprint>>`（hashbrown，key u32）。找到既有→Arc 唯一所有权则就地更新（drop 旧 RaycastMesh Arc、清 Vec 字段），否则用 0x140 字节模板 memcpy 重建替换；未找到→raw table 探测插入新 Arc<DecoratorBlueprint>。收尾写 `蓝图+0x50/+0x58/+0x138`（pos/rot/标志）。算法：`Arc::make_mut` 式增量 + hashbrown 探测。
- **get 0x140825ff0 O/I**：标准 hashbrown get（len!=0 时探测，命中返回，未命中 None）。
- **clear_blueprints 0x1408260d0 O/I**：puffin；`DecoratorStorage::iter_mut`；对 modified/removed id 与 removed storages：SipHash13 后 `RawTable::remove_entry<DecoratorId, Arc<DecoratorBlueprint>>`，Arc 归零 drop。
- **copy_blueprints_to_prev 0x140826d70 O/I**：`hashbrown clone_from<HashMap<DecoratorId,Arc<DecoratorBlueprint>>>` 当前→prev + 拷贝 `+0x20/+0x28`。
- **validate_blueprints 0x140827130 O/I(部分)**：`clear_modification_tracking`；对 `HashSet<DecoratorLocator>` try_fold(all) 逐 locator 校验；通过且 `+0x281` 置位时按 `DAT_142a91c00` 8 分支跳转表分派；错误累积写 `+0x282=2` 并 push 事件（Ghidra 误标 `WriteSettingsFileCmd`）。U：错误枚举、事件类型。
- **show_max_wall_y 0x140827350 O/I**：from_iter zip(Vec3, f32) 合成墙顶折线；`self+0x258!=2` 再 push 一点；`DebugLine dashed+line_width+show`。U：+0x258 语义。
- **instantiate_blueprint_colliders 0x140827510 O/I**：puffin + 写计数器后直接转发 `instantiate_decorator_colliders`。
- **instantiate_blueprints 0x140827910 O/I(主体)**：先 instantiate_decorator_colliders；对墙表 `remove_entry<WallId,HashSet<usize>>` 与 `RawTable<DecoratorId,DecoDiff<Chimney/DoorStairsAssemblyParams>>` 清差分；对装饰 id 删 `PersistentVisualEntities`、despawn 旧实体；`WindowVisualEntityBuilder::equal`+`Transform::eq`+`Option<Box<ClothInfo/Chimney/DoorStairs>>::eq` **未变跳过重建**；变则拷贝蓝图 `+0x11` 处 0x50 字节条目（tag 0/1/2）门/楼梯请求、`WindowVisualEntityBuilder::build<DecoratorPersistentVisualState>`、`seal_persistent_entities`；`+0x128/+0x130` 触发 `WallAttachedDecoTracker::remove/add<Chimney/DoorStairs>`。日志串 `crates/systems/decorator/src/blueprint.rs`、"Decorator blueprint did not specify …" panic。算法：差分重建+碰撞+seal。
- **instantiate_decorator_colliders 0x140829850 O/I(主体)**：removed 集逐个 `RaycastWorld::remove_collider<DecoratorId>`+Arc 减引用+日志 "Decorator collider REMOVED"；主循环 `蓝图+0x60==0` 时用 `+0x50`(Vec3)/`+0x58`(Quat)/`+0x64`/`+0x6c`/`+0x70` 构造 `RaycastMesh::new`→`upsert_collider<DecoratorId,(Vec3,Quat)>`；`+0x90==0` 时用 `+0x94/+0x9c/+0xa0/+0xb0/+0xb8/+0x139` 第二碰撞 upsert；`+0x90!=1` panic。算法：增量 upsert/remove 蓝图碰撞+引用计数。U：ptr/len 切片语义。

## 2. load_decorators.rs
- **load_decorators 0x1407b66e0 O/I**：readdir 扫 `.json`（字节序交换 0x2e6a736f+末字节 'n'）；`load_json_as_mesh<PathBuf>`；开头 8 字节魔数 "outline"（0x6c74756f+0x656e696c）则 `Mesh::add_color`(白 1.0f)；`load_mesh_into_library`。U：目录来源。
- **closure0 0x1407baad0 I**：Mesh→`RaycastMeshData::new_indexed`（arrayed_indices/pos/normals/uvs/tri/bbx）。
- **closure1 0x1407baf00 I**：`load_json_as_mesh<String>`+unwrap+pos/normals。
- **closure3 0x1407bb160 I**：`AssetMeshLibrary::get_by_name`→`RaycastMeshData::new_convex`。
- **load_collision_as_bbx 0x1407bb4b0 O/I**：get_by_name→`Mesh::collision_bbx` 断言 `Vec2==(1.0,0.0)`；`box_trigger_volume`+`RaycastMeshData::new_cuboid` 构造 0x90B 对象；再 get_by_name→bbx 组合写输出 `[0..7]`。U：名字串、断言含义。

## 3. derive_decorator_subtype.rs（可复用 reconstruction/building/decorator_subtype.rs）
- **derive_decorator_subtypes 0x140812b20 O/I(可复用)**：puffin；`DecoratorStorage::iter`+`iter_removed_ids`；`PublicWalls::get`/`PublicWallState::max_y`/`DecoratorExtraInfo::as_chimney`/`wall_space`/`project_unbounded`/`to_world_space`/`distance` 后调 derive_decorator_subtype；日志 "position"。
- **derive_decorator_subtype 0x140813d00 O/I(可复用)**：读 `Info+0x46`(DecoratorType-0x38)→`DAT_142a8f990` 跳转表；`Info+0x22`(attachment) 二次分派（UNK_142a8f9c0/9b0/9d0）；0x3a 分支经 `roof_entity`+屋顶实体表(0x14 条目)+`PublicWalls` 位图+`+0x1a8` 屋顶形状表→is_half_dormer；`WallCornerAttachment::is_attached` 命中返回（角窗）；否则 is_half_dormer；输出 u8 经 `*local_50` 含 `*2` 折叠。与 reconstruction 的 cottage 0/1/3/4、gothic 6/7/9/6 一致。U：跳转表精确映射。
- **is_half_dormer 0x140814470 O/I(可复用)**：`墙形tag(+600)>1` 返 false；有屋顶：`Roof::is_flat` 返 false、`is_gable`→`as_rectangle`+`gable_sides`+`to_world_space`+`get_closest_edge_id` 边命中两端返 false；`Curve::get_approx_coord_from_pos<Vec3,Thresholded3D>` 求近邻；`0x160600>>((subtype-0x38&3)<<3)` 族常量；`DecoratorLibKey::new`+`HashMap<DecoratorLibKey,DecoratorCollision>`(0x48 步长) 查 bbx，未命中 panic；命中 xvy 返回 bbx，`top>wall_height` 在调用方完成。U：末尾比较段被反编译截断（以 reconstruction 为准）。

## 4. apply_decorator_op.rs
- **apply_decorator_op 0x141276e30 O/I**：顶层 tag→`(x-3)<2→x-2` 分支：lVar1==0(tag 0/1/2/5..) 读 `in_RCX[2]` 子 tag→`archive_current_decorators_in_storage`(tag!=3 二次)+push 历史 tag=3+`DecoratorLocator::new`+`HashMap<DecoratorLocator,()>`+`apply_decorator_change`；lVar1==1(tag3)→push 4+`apply_decorator_place`；else(tag4)→push 5+`apply_decorator_remove`。末尾 `DeferredDecoratorOpEvents::send`。内联 write_place/change/remove_op_history 对应 tag 4/3/5。I。U：op 字段、edit tag 与 DecoratorHistoryEdit 变体映射。

## 5. handle_external_decorator_cmds.rs
- **handle_external_decorator_cmds 0x14081a380 O/I(部分)**：三组缓冲(0x18/0x28/0x30 步长)：①0x18 组 `StairSegment::new`+`DecoratorStorage::get`+楼梯图(0x78 表,tag4)查边→`Curve::try_new_from_points<Vec3,3D>`+`inverse_lerp`+`CachedDecoratorTransforms::try_position`+`apply_decorator_op`；②0x28 组 op+`History::add(5)`；③0x30 组 op+`History::add(3)`。U：三组来源/结构。

## 6. backup.rs
- **DecoratorBackup::clear 0x140823660 O/I**：default 模板 + clone_from：DecoratorStorage、`HashMap<DecoratorId,Arc<DecoratorBlueprint>>`(+0x1f0/1f8)、`HashMap<DecoratorId,Transform>`(+0x220/228)、`HashMap<(Id,Id),()>`(+0x250/258)、`Vec<DecoratorHistoryEdit>`(switch tag 3/4/5 归一化)；写 +0x280/281/282。算法：重置备份。
- **create_or_restore_decorator_backup 0x1408238a0 O/I(主体)**：三态：`+0x282==1` 创建（all_decorators+get_or_insert_default+tracked_clone_from、clone_from 各表、置 +0x281=1）；`+0x281!=0` 恢复（all_storages 清、`StairsState::restore_backup`、各表 clone_from 回写、`DecoMoveToStorage::new`+tracked_clone_from、`Info+0x47==1` 清 0）；`+0x280!=0` 处理完 clear。U：+0x47/280..282 完整语义。

## 7. restore_archived_decorators.rs
- **restore_archived_decorators 0x140817a40 O/I(主体)**：`mark_mutable`；all_storages 清；`StairsState::restore_backup`；`extract_diff`（drop diff 表）；all_decorators 逐 `InnerWalls::get`+`get_or_insert_default`+`DecoMoveToStorage::new`；restored id `remove_entry`+`impl$8::update`+`upsert_collider<DecoratorId,OffsetColliderTransform>`。算法：归档恢复。

## 8. cache_decorator_transforms.rs
- **cache_decorator_transforms 0x1408153d0 O/I(主体)**：`swap_buffers_and_clear`+`iter_removed_ids`；逐装饰：墙→`get_tangent`+`to_world_space`+`xz_rotation_bad`+`xvy`；屋顶→`roof_entity`+`get_position_from_mesh_uv`+`rotation_yaw_radians`+`DecoratorRotationXZ::to_quat`+`is_flat`+`get_roof_shape_from_pos_ws_clamped`+`as_rectangle`+`closest_edge_normal`；楼梯→`as_stair`；`from_xz_forward_vector`/`offset`/`to_quat`；`CachedDecoratorTransforms::insert`。算法：按附着类型算世界 Transform 缓存。U：旋转细节。

## 9. decorator_visual_state.rs
- **PersistentVisualEntities::entities 0x1408119d0 I**：访问器，退化 return。值= {seal 状态字节, Vec<Entity>}（0x28 步长）。
- **DecoratorPersistentVisualState::entities 0x14067dd80 I**：访问器；Ghidra 命名被 `TweakerView::as_inner_mut` 覆盖（同址别名，骨架确认符号）。
- **impl$4::add_decorator_visual_entity 0x1408119e0 O/I**：`HashMap<DecoratorId,PersistentVisualEntities>`；命中且状态 bit0==1→panic（已 seal）；push `in_R8`(Entity) 进 `Vec<Entity>`(grow_one<usize>)。
- **impl$5::add_decorator_visual_entity 0x140811c30 O/I**：目标 `HashMap<DecoratorId,Vec<DisplayChildAutoClutterInfo>>`，push autoclutter 子项（瞬态视觉）。
- **DecoratorVisualState::seal_persistent_entities 0x140811e80 O/I**：查 id；状态 `!=1`→写 1 并 return；`==1`→panic（重复 seal）。

## 10. decorator_resources.rs / collision_detection.rs（内联宿主）
- **decorator_resources**：`DecoratorHandleName`（IPI 3）；唯一符号 `impl$0::to_widget_name_hash`(inline,module 1499) 宿主= `UiState::add<DecoratorHandleName>`/`create_decorator_widget<…>`（UI 族范围）。
- **collision_detection**：`intersect_wall_decorators_impl`(inline,8 实例) 通用求交内核；宿主 `intersect_wall_decorators<…merge…closure_env$1>` 0x141fe5900（`in_RDX` bit0 选 `ZeroBoundedComparativeSpace`/`CircularComparativeSpace`，命中 push 槽索引）、`<…interaction_intent closure_env$0>` 0x141fe5c20；`DecoratorWorldSpace::project_to_wall_space`(inline,module 1522)。

## 11. dormer_holes / chimney_smoke / window_lights
- **upload_dormer_holes_to_gpu 0x1407b1d40 O(GPU 后置)**：puffin；`DormerHoleSsbo::dummy`+`Rectangle2d::aabb`+`GraphicsDevice::update_buffer_bytes`（rhapsody）。内联 world_space_to_grid_coord_unclamped/get_cells_from_bbx。U：SSBO 布局。
- **chimney_smoke_anim_thickness 0x140818df0 O/I**：`HashMap<usize,f32>` retain 清过期；`init_rng`+`fastrand::f32` 随机演化+帧计时；初始 1.0f。U：消费方。
- **window_lights 0x1407b5980 O/I**：`SessionLoadReadiness::deny_load`；`HashMap<usize,LightInfo>` retain；`fastrand::f32` 抖动；`time_continuous_lerp` 亮度插值；写 1.0f。U：LightInfo/消费方。

## 12. cull_oob_decorators.rs
- **auto_allow_decorator_culling 0x14080d3d0 O/I**：`History::is_recording` 假即返回；遍历墙 `wall_decorator_ids` 扩展放行集；收尾 `decorator_cull_allow_stairs`。
- **cull_oob_decorators 0x14080d540 O/I(主体)**：断言串 `"assertion failed: decorator_archivist.is_decorator_cull_allowed(locator.decorator)"`；`iter_removed_ids`；多遍墙表(0x30/0x10/0x40)；主循环 `is_decorator_cull_allowed` 断言、subtype 窗口族(`Info+0x46-0x38: !=6 && <8`) 时 `is_decorator_location_valid`；结果 0xb 且蓝图 `+0x60==0` 时 `to_offset_collider_builder`→`decorator_overlaps_with_others<OffsetColliderTransform,Arc<dyn Shape>>`；**重叠保留**、不重叠裁剪（archive+remove_entry+apply_decorator_op）。算法：越界裁剪+碰撞重叠保护+历史化删除。U：DecoratorRemoveInfo、LocationCheckResult 变体。
- **is_decorator_location_valid 0x14080f470 O(分派壳)**：`Info+0x22`→`DAT_142a8eab0` 跳转表分派。
- **should_snap_to_corner 0x140811900 O/I**：`should_offer_snap_to_corner`+`is_halftimbered`+`wall_space`+`subtract`。
- **check_if_dormer_ridge_is_within_roof_bounds 0x140811440 O/I**：gothic tag `(x-7)>1` 返 0xb；`roof_entity`+实体表+位图；`+0x1a8` 屋顶形状表取坡面曲线，段 `+0x10*0x58`；有脊→`gothic_(half_)dormer_profile_curve`+世界点；`roof_shape_at_intersection_y`(0=circle/1=rect/2=other)→`closest_point_on_perimeter`+`xvy`；`+0x4c` 偏移+`Roof::height`：`fY<=h+off-0.1` 且 `0.993<=dot` 返 0xb，否则返 1+脊世界点。算法：脊点→屋顶面最近边+高度/方向容差。
- **cull_stair_attached_decorators 0x14080ec10 / on_invalid_stairs 0x14080f040 O/I**：puffin+断言串→转发 impl。
- **cull_stair_attached_decorators_impl<closure_env$0> 0x1412b6c30 O/I**：`iter_stair_decorators`+`is_decorator_cull_allowed`+`dst_as_stair`+`StairsState::contains_edge_or_stub`(0x150 表,`+0x128` 边 id, movmskpd)；边不在了→`try_position`+`insert<DecoratorLocator,DecoratorRemoveInfo>`。
- **impl<on_invalid closure_env$0> 0x1412b6420 O/I**：同构（搜索耗尽）。

## 13. merge_proposal_wall_decorators.rs
- **propose_wall_decorator_merge_position_inner 0x142001d40 O/I(主体)**：查被合并装饰(0x50 表,首 float==目标 X)；`subtype`+`DecoratorRank::new`+`DecoratorLibKey::new`+get 碰撞；类型兼容 `(new-0x38==6? found==new: …) && found-0x38==new-0x38`、附着 tag≤1；rank 合并≤`max_rank`；偏移 `distance/subtract/project`，类型 0x3a 权重互换、`local_f1=='\t'` 时 `(w_m-w_f)*0.5` 居中；`intersect_wall_decorators` 无重叠→输出提案（位置、`+0x5d`=type*0x100+族、`+0x5f`=标志、`+0x18`=距离 SQRT）；重叠→+0x5f=2。算法：墙装饰合并提案（同族同型+rank 累加+沿墙偏移+无重叠）。U：提案字段布局。
- **find_best_wall_decorator_merge_position 0x142001880 O/I**：多候选逐 propose，按距离取最小；无→+0x5f=2。

## 14. merge_proposal_roof_decorators.rs
- **find_best_roof_decorator_merge_position 0x142000e60 O/I(主体)**：候选逐 `call_mut<closure_env_0>`；有效时逐屋顶装饰 subtype/rank/get；类型兼容+rank≤max_rank；偏移 `fVar36=(w_m-w_f)*0.5`，非角 `/perimeter`、角（矩形）`as_rectangle+get_closest_edge_id+edge_length` 后 `*0.25`；新位置 `found_pos±offset`；`dst_as_roof`；距离 SQRT；取最近写入 `in_RCX(+0xd..+0x1a)`。算法：屋顶合并提案（沿边偏移+角/边权重+最近者）。U：offset 符号/权重常数。
- **intersect_roof_decorators<…closure_env$1> 0x141fe7280 O/I**：`Rectangle3d::new` 合并碰撞框；逐屋顶装饰求交 push 槽索引。

## 15. move_decorators_following_anchors.rs
- **move_decorators_following_anchors 0x14081c790 O/I(主体)**：断言串 "internal error: entered unreachable code"（`crates/systems/decorator/src/move_decorators_following_anchors.rs`）；阶段1 `on_wall_removed`+多组墙表 `wall_decorators_mut`+`mark_all_modified`，`Info+0x22==2` 预标记；阶段2 `iter_walls_mut`+`iter_modified_mut`：`as_wall_mut`+`get_wall_attachment_height`+`get_approx_coord_from_pos<Vec2,2D>`+`lerp`+`wall_space`+`project_clamp`；写 `Info[0]=X,[1]=Y+高,+0x21`；`+0x21==1` 且 try_get_subtype 命中：`extend_right_until`+`DecoratorRank/LibKey::new`+get 碰撞；`fVar41=lerp(arch)`、`fVar42=fVar41+(max.y-min.y)*0.5+0.05`、取 max；`max_y`；`Info[4]=fVar41-max_y`（相对）、`[1]=fVar41`。算法：墙变→按坐标/高度/拱高重挂（0.05 余量）。U：+0x21、extend_right_until 分支。

## 16. remove_ivy_under_windows.rs
- **remove_ivy_under_windows 0x140814860 O/I(主体)**：墙表(0x90 步长)逐装饰条目：tag3（窗）条件 `+0x80==0` 且 `+0x3a` 附着侧合法且存储命中；`bilinear_sample_at_world_xz<TerrainHeightFloatGrid>` 地形高；`local_128=窗口pos-地形高`；`into_decorator_subtype`+`DecoratorLibKey::new`+get 碰撞（未命中 panic）；`try_as_wall`+`remove_ivy_at_location`；其余（门）`+0x42>1` 跳过。算法：窗/门下地形 ivy 移除。U：+0x3a/+0x42、半径来源。

## 17. 墙/形状变化响应
- **decorator_on_wall_height_changed 0x14081b5e0 + closure0 0x14081b920 O/I**：4 组墙表遍历；逐装饰（`Info+0x22`→lVar1==0）：`get_mut`+`dst_as_wall_mut`+`get_wall_attachment_height`；`Info+0x10 = Info+4 - 墙附着高`（重算 wall_height_relative_y）；lVar1==1 跳过，其它 panic。
- **decorator_on_move_shape 0x140811fc0 O/I**：墙表(0x30)；`HashMap::insert` 取 Info；附着字节 `-0x26`；lVar1==0：`新位置=旧位置+墙偏移(+0x20)`，xvy 写 `Info-0x2c/-0x34`；非 0/1→panic。
- **decorator_on_rectangle_edited 0x140819c00 O/I(主体)**：矩形墙(0x58)；墙变化→`archive_current_decorators_in_storages`+`decorator_cull_allow_wall`+`get_enclosure_params`；逐装饰（lVar1==0）：`as_wall_mut`+`dst_as_wall` 读锚点(+0x14/+0x1c)；`to_rectangle_corner_id`：角→`get_point`+`get_closest_u_from_pos`+`perimeter`；非角→`orientation_mat2` 投影/裁剪 [0,1]（divps/maxps/minps ±1、NaN 掩码）再求新 u；写新位置+u。算法：矩形改尺寸→装饰沿周长重映射（含角吸附）。
- **decorator_on_freehand_wall_edited 0x14081bac0 O/I(主体)**：自由墙(0x28)；lVar19==0：`PublicWalls::get`（outline 边界检查）；xvy 写新位置；`WallSpaceCoord::from_curve_internal_coord`（piVar23[2]）写新坐标(-0x48/-0x44)。
- **decorator_on_circle_edited 0x14081c060 O/I**：圆墙(0x10)；lVar5==0：`to_world_space`+`get_mut`+`dst_as_wall_mut` 写 `Info+0x14/+0x1c`（anchor_ws）。

## 18. 历史/回放
- **decorator_record_history 0x140817850 O/I**：`DeferredDecoratorEditHistory` 脏标志；逐条目(0x20 步长, tag==6 终止) 分配 48B 对象 `{1,1,tag,载荷}` 调 `History::add`(kind 10)；清 Vec+HashSet(memset 控制区)。U：tag6 终止符。
- **replay_decorator 0x14080c8d0 O/I(主体)**：`sync_point_count`+`ReplayBuffer::front_mut_bucket`；`Arc<[DecoratorId]>` 受影响 id 插入标记；`Vec<0x30B 记录>`（tag+0）：`tag==0x7` 分隔符结束；`+8`bit0 未置记索引、置则 `fVar22=(+0xc)/sync≥0.05`、`<+0x18` 记索引并 `+0x18+=当前时间`；倒序 remove(memmove) 并按 tag 释放：0/1/2=RasterEdit、3=TerrainEdit、4=WallReplayCmd、5=RoofEdit、6/7=DecoratorHistoryEdit、8=TreeEdit、9=CrashMiscInfo、默认=Mutex<PhysicalSize>。算法：回放缓冲清理（移除已消费命令）。U：时间戳语义。

## 19. audio/*（FMOD 胶水）
- **op_confirm_audio 0x140822530 O(映射主体)**：迭代墙装饰+op 事件；非 change 类调 place_sound 生成 `PlayDecoratorSoundCmd`→`HashMap<AudioEventKey,PlayDecoratorSoundCmd>::insert`；subtype/op kind 多级 dispatch（0x38..0x40 族、tag 0x11/0xb/0x23/0x10/0x17/0x1e/0x20、8/0x19）；附带 `Bucket<(Id,Id),AlignKind>` 楼梯对齐。U：tag→FMOD 事件映射。
- **place_sound 0x140822cf0 O/I**：按 DecoratorType(-0x38) 构造 `{tag,variant}`：0/1/2→tag1+variant+标志；3→0x11+`(0x100>>((subtype-0x13&3)<<3))`；4→0xb+标志；5/默认→UNK_142a910c0 跳转表；6→0x23+subtype；9→计数。
- **op_preview_audio 0x140822e00 O**：`roof_component`+`is_flat`+`FlatRoofStyle::is_wooden` 选预览音（木质平屋顶软/闷 vs 清脆）。
- **op_stairs_audio 0x14081d9b0 O**：`DecoratorOpEventKind::id`+`lookup_decorator_info`+`try_as_stair`+`StairsState::graph`+`WidgetId::from_name`+`DragState::is_some`；closure9 0x1408221a0 为按 id 事件 helper（DecoratorOpEventKind::id 分派）。

## 总览
`system_decorator` 主干 = 装饰生命周期与状态机：加载/装配（load）→蓝图缓存（HashMap<DecoratorId,Arc<DecoratorBlueprint>>，增量重建碰撞+视觉）→subtype 派生（复用 reconstruction）→操作统一入口（apply_decorator_op/handle_external，写历史+归档备份/恢复）→状态/资源（持久/瞬态实体、灯/烟/洞）→越界/合并/跟随净化（越界裁剪带碰撞重叠保护、同族同型+rank 合并提案、跟随锚点重挂、窗下 ivy 移除）→墙/形状变化响应（相对高度/位置/锚点重算）→历史/回放/音频。

## 算法清单
蓝图实例化/碰撞（Arc::make_mut 增量 + RaycastWorld upsert/remove_collider + Arc 引用计数）；差分重建（equal 判断未变跳过+despawn/build+seal）；subtype 派生（窗口族跳转表+半坡 flat/gable 边+碰撞高度 top>wall_height）；越界判定（attachment 分派+dormer 脊 perimeter+高度容差 -0.1/方向 0.993）；合并提案（同族同型+rank≤max_rank+沿墙/屋顶边偏移+intersect 无重叠+最近距离）；历史（History::add kind10/tag3-5，回放缓冲倒序清理释放）。

## 未知清单
DecoratorBlueprint 全字段 TPI 布局（+0x50/58 Vec3/Quat、+0x60/90 RaycastMeshData Arc、+0x120 door stairs Box、+0x128/130 tracker、+0x138/139 标志）；DecoratorLocationCheckResult 变体与 DecoratorRemoveInfo；DecoratorHistoryEdit 非 3/4/5 tag；音频 tag→FMOD 事件映射与 AudioEventKey；+0x47/+0x280..282 备份标志三态；is_half_dormer 末尾比较段（反编译截断，以 reconstruction 为准待原指令差分复验）；GPU 后置的 dormer SSBO 布局与灯/烟视觉消费方。
