# systems/decorator 模块内部逻辑分析（B 版）

> 由批次 3 三个子范围 agent 产出合并。本文件 = A（主干）+ B（visual 族非楼梯）+ C（楼梯视觉/ui_systems/utils）。

## 目录
- [A 主干](#a-主干)
- [B visual 族（非楼梯）](#b-decorator_visual-视觉族非楼梯)
- [C 楼梯视觉 / ui_systems / utils](#c-楼梯视觉--ui_systems--utils)

---

# A 主干

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

---

# B decorator_visual 视觉族（非楼梯）

# systems/decorator · decorator_visual 视觉族（非楼梯）B 版分析

> 由批次 3 分析 agent 产出，父级代为落盘。

范围：主干/stairs/ui_systems 由并行 agent 覆盖。证据 = Ghidra 持久 B 库只读反编译（`--worker shared`，无写入）；地址 = 骨架「编译实例 RVAs」+0x140000000；下述为 RVA。状态标注 O/I/U。

## 通用结构（跨函数复用）
- **WindowVisualEntityBuilder**（0x160 字节/条，全族批量 push）：+0x00 实体区（+0x18 排序键 subtype*1000+deco_id，+0x28 全局实例 hash）、+0x44 稀疏分配、+0x84/+0x94 子类型、**+0xa0 mesh 名 ptr / +0xa8 mesh 名 len**、**+0xb0 shader 名 ptr / +0xb8 shader 名 len**（len 即字符数，如 0x19/0x1e/0x24/0x2a/0x35）、+0xc0 pos、+0xd0 quat、+0xdc scale、+0xf0=1、+0xf4 DecoratorRank、+0x108..0x110 垂直范围。O
- **rhapsody ResourceAccessInfo** 向量：+0x10 len/+0x1c 计数，元素 0x14 字节 `(u32,u32,f32,f32,u8)` 碰撞/实例。O
- **DecoratorCollision 表**：`HashMap<DecoratorLibKey,DecoratorCollision>`，+0x30 为碰撞半尺寸相关值。O
- **DormerHoleSsbo**：0x30 字节/条；`::cottage()`/`::gothic()` 追加屋顶洞。O
- 多数生成系统先查 `DEV_OVERRIDE_IS_DEMO_MODE`，非零直接返回。O

## 一、状态/维护
1. **WallAttachedDecoTracker add/remove（4 单态）** 0x12ad540/0x12acd70/0x12ae1c0/0x12add20。类型（O，PDB）：`HashMap<WallId, WallDecoDiff<T>>`，`WallDecoDiff<T>=HashMap<DecoratorId, enum DecoDiff<T>>`。add：按 WallId（`DecoratorStorageAddr::as_wall`）hashbrown 开放寻址找/建外层桶，再按 DecoratorId 向内层插 40 字节 diff 值；维护 count/len。remove：缺失→panic_fmt（fmt 链 anyhow+walls+decorator）。`with_diffs`/`on_wall_*`/`DecoDiff::current`/`DecoDiffLog`/`DecoRemovalLog` 均内联（模块 1561）。Default(0x10ba60 实为 `impl$2::default<ChimneyAssemblyParams>`) 初始化 RandomState+匿名静态。
2. **chimney_assemble_bricks** 0x7f8db0(size117)：薄壳。签名 `(DerivedDecoratorInfoResources, Res<PublicWalls>, Res<AssetSsboLibrary>, ResMut<WallAttachedDecoTracker<ChimneyAssemblyParams>>, Res<TerrainHeightsData>, Res<UiSignifierStream>, EventWriter<PlayDecoratorSoundCmd>)`；推进计数→`with_diffs<ChimneyAssemblyParams,closure_env_0>`。主体内联（U）。
3. **door_stairs_assemble_bricks** 0x7ff130(size52)：同壳，`with_diffs<DoorStairsAssemblyParams,...>`。
4. **clear_out_visual_entities** 0x7f9260(size1295)：drain `HashMap<DecoratorId,Vec<Entity>>`（hashbrown 位图）逐实体 `EntityCommands::despawn`。
5. **clear_out_wall_holes** 0x7f98c0(size31)：推进两个洞缓冲计数 + `WallHoles::clear_for_next_frame`。
6. **maintain_dormer_tiles** 0x7ce920(size321)：二维瓦片网格差分（行/列 0x18 stride），删瓦片→`RawTable::remove_entry<u64,Box<dyn FnOnce>>`（rhapsody 稀疏分配注册表）+ drop `SparseInstanceBufferAllocation<RoofTileSsbo>`。

## 二、墙体/窗/门族
7. **generate_arrow_slits** 0x7bbee0(size7827)：subtype 1x1/1x2/1x3/0x17(半山墙) → `get_gothic_hat_bones` 尖拱骨骼→中点+`xz_projective_rotation_bad` 求变换；mesh `"decorators/arrow_slit_1x{1,2,3}(_halfdormer)"`（len 0x19/0x24）、shader `"nani_solid_vertex_color_window"`(0x1e)；碰撞 0x14 元素 1x1→(0.5,0.5)、1x2→(0.4,1.5)、1x3→两条(0.4,1.7)+(0.605,0.55)；0x17 追加 `DormerHoleSsbo::gothic`。
8. **generate_roof_chimneys** 0x7c0910(size4073)：subtype 0x15(21)；`as_chimney` 取高度；烟雾视觉（mesh 名 1/len0，shader len10）+ cuboid 碰撞（40 字节：`[0]=2,[1]=底座y-wall.max_y,[2..3]=xz,[4]=y_min,[5]=y_max,[6]=quat,[9]=高度+3`）。
9. **generate_wall_chimneys** 0x7c21b0(size9538)：subtype 0x13..0x14；vent_pipe 视觉(`"decorators/vent_pipe"` len0x14)；`box_trigger_volume`+`CurveSlice::extend_right_until` 求墙曲线重叠，clamp[1.5,5]，≥5 跳过；vent_pipe 三角→`new_indexed` 三角碰撞 + cuboid（`[0]=有碰撞flag,[1]=重叠量`）。
10. **generate_cottage_wall_windows** 0x7ebfd0(size11843)：窗+`add_lintels`(§14)+`flowerbed_xforms`(§13 随机花坛)+`DormerHoleSsbo::cottage`；mesh 名静态表（U）。
11. **generate_cottage_corner_windows** 0x7f66b0(size8168)：矩形角窗，`WallCornerAttachment::to_rectangle_corner_id`+`Rectangle2d::get_corner_normal/get_edge_dirs_from_corner_id`，quat `^0x8000` 符号翻转；+`add_lintels`+`has_cloth`。
12. **generate_cottage_balcony_doors** 0x7f3da0(size7226)：`DragState::is_started`、`check_for_door_stairs`(内联)、`bilinear_sample_at_world_xz<TerrainHeightFloatGrid>`(门腿接地)、`add_door_autoclutter`(内联)；mesh `"nani_balcony_door"`。
13. **flowerbed_xforms** 0x7ef7b0(size472)：`fn (subtype,height)->Vec<Transform>`；按族静态表取 mesh→positions+normals→zip→closure→Vec<Transform>。
14. **add_lintels** 0x7ef990(size2626)：`fVar20=(y+1.43)*0.5`；mesh `"decorators/setdressing_window_lintel"`(0x24)、shader nani_solid_vertex_color_window(0x1e)；scale 首尾 0.82/中部 0.64（subtype≥2），flag bit0 乘 height/0.9。
15. **generate_gothic_wall_windows** 0x7e87b0(size8889)：`WallStyle::is_halftimbered`+`get_gothic_hat_bones`+`get_window_holes`(内联)+`NameHash::compute`(mesh 名 name-hash)；shader `"nani_solid_vertex_color_bricks"`/`"nani_gothic_window_bricks"`(0x19)；+`DormerHoleSsbo::gothic`。
16. **get_gothic_hat_bones** 0x7eb500(size1181)：墙曲线采样两索引→边缘方向+法向（`1/SQRT`+`xvy`）→输出 2×2 旋转基 + 归一化边向。
17. **calc_gothic_half_dormer_profile_ws** 0x7eb160(size719)：反编译被内联打散，仅剩分配+for_each 骨架；生成半山墙剖面（U，需 P-code）。
18. **generate_gothic_balcony_doors** 0x7f0900(size9070)：`WallStyle::is_halftimbered`+`get_door_holes`(内联)+`add_door_autoclutter`+地形/`RasterBufferRef<512>` 接地；mesh `"nani_balcony_door"`。
19. **check_for_door_stairs**：内联（无独立符号；调用记录 0x7f2025→0x7f5e70）。检测门下方玩家楼梯（I）。U。

## 三、dormer 族
20. **generate_cottage_dormers** 0x7c7ac0(size9285)：subtype 2；`get_collision_mesh` 取碰撞；按形状(1/2/3)沿屋顶曲线(`as_curve3`+`get_coord_at_u`)采样 gable/hip 墙 quad（0x30），`1/SQRT` 法向+`xz_projective_rotation_bad`；`rng`+自研 hash(0xa0761d6478bd642f^0xe7037ed1a0b428db) 随机位；push 墙视觉(mesh DAT len0x2a)+窗玻璃`"decorators/window_cottage_1x1_dormer_glass"`(0x2a)+`DormerHoleSsbo::cottage`。
21. **generate_dormer_roofs** 0x7cb1a0(size9771)：差分 dormer 瓦片；subtype 掩码 `0x800186`(1,2,7,12,16,23)+0/WallStyle3；gothic(8)→`gothic_dormer_profile_curve`→变换到墙空间→`Curve::try_new_from_points`→`calc_gothic_half_dormer_profile_ws`+`get_gothic_hat_bones`→`assemble_gothic_dormer_tiles`；0x17→2 点剖面；其余→`assemble_dormer_roof_tiles`(内联)；写入 `SparseInstanceBuffer<RoofTileSsbo>`+`HashMap<Entity,InstanceHandle>`。
22. **get_collision_mesh** 0x128a0b0(size8051)：`DormerKey{i32 x,i32 z,i32 count,i8 roof_idx}`（round(dist*5) 量化）缓存 `HashMap<DormerKey,Arc<RaycastMeshData>>`；未命中→逆旋转矩阵+沿屋顶形状两侧 u 插值采样生成三角带→`new_convex`→插缓存。
23. **gothic_dormer_profile_curve** 0x7c7500(size392)：按 subtype 返回硬编码剖面（sub1 6 点、sub2 7 点、sub3 9 点，8 字节=2×f32；sub1 常量 0x3f6e147b00000000,0x3f47ae143e19999a,... ≈ (0,0.93),(0.15,0.78),(0.26,0.6),(0.32,0.4),(0.35,0.19),(0.35,0.35)）。
24. **generate_gothic_dormers** 0x7c5650(size6662)：subtype 8；高度常量 1.54(sub3)/1.2(sub2)/0.72(sub1)；mesh 名 `DAT_142a86e38`+子类型、shader `"nani_solid_vertex_color_(window)bricks"`(0x1e)；+`DormerHoleSsbo::gothic`。
25. 内联登记：`dormer_roof_height`/`is_bay_window`/`DormerKey::quantized_from`/`cottage_dormer_dims`/`get_dormer_panel_xforms` 均无独立符号。

## 四、旗帜/布条/灯笼/杂物
26. **generate_flag_connections** 0x7df470(size4852)：收集旗 CandidateInfo(0x50：+0x10 pos,+0x30 wall_id,+0x4d 可用)；case 2/3/4 朝向旋转（0.70710677 四元数+前伸 0.9/0.48/-0.7）；`greedily_connect`→每对连接求两骨骼最近点→对「A→最近点」「最近点→B」各调 `clothline`。
27. **greedily_connect** 0x7e1430(size3054)：按距离排序候选对；判据：异墙 dist²<min²、同墙 band(max²,min²)；贪心主循环：两端点容量<max_conn、`2|dz|<=水平距`、dist>0.02、`RaycastWorld::intersects_any` 无遮挡→连（`HashMap<(usize,usize),()>` 去重，计数+1）。
28. **clothline** 0x7e2510(size2552)：`n=max(2,ceil(dist/间距))` 等分插值，每次 `WindowVisualEntityBuilder::build<DecoratorTransientVisualState>`。
29. **generate_clothline_visuals** 0x7e5190(size8528)：读 `ClothInfo`(0x14/条)→CandidateInfo；mesh `"nani_cloth"`+NaniBundle/SolidVertexColorInstanceData/ChunkId 落 GPU 稀疏实例。
30. **generate_flags** 0x7ff6c0(size11875)：落位分型 mesh `"decorators/flag_terrain_support"`/`flag_wall`/`flag_roof_beam`/`flag_roof_cloth`/`flag_corner_beam(_no_connection)`；`RaycastWorld::legacy_raycast_w_params` 遮挡检测；+0x28 实例 hash。
31. **generate_lanterns** 0x78d320(size10195)：落位分型 `"decorators/lantern_wall"`/`lantern_flat_roof(_inside)`/`lantern_terrain(_support/_inside)`；shader `"nani_lantern_emissive"`；`Roof::is_flat` 判平顶。
32. **maintain_lantern_lights** 0x790400(size3533)：遍历灯笼 SSBO+屋顶瓦片位图；`rng*0.14+0.01` 闪烁阈值+时间窗(0.45..0.72)+`deny_load` 判定点亮；写 `(ColorId,bool)`；记录 `(DecoratorId,GpuLightId,ColorId,f32,Vec3)` 上限 0x8000；sort+`compare_light_lists` 双缓冲差分→`GraphicsDevice::update_buffer_bytes` 上传（SsboId→BufferState FrozenMap，名 hash 断言 0x35）。
33. **generate_trap_doors** 0x7be730(size4956)：仅平屋顶；subtype 0x0a/0x05；瓦片位图定位+`Circle2d::intersect_circle`/`Rectangle2d::contains_point_padding` 校验；cuboid 碰撞+视觉(mesh DAT 分型，len 0x1b/0x1c)+`DormerHoleSsbo::cottage`。
34. **generate_decorator_clutter** 0x7e3370(size4230)：杂物 subtype 集合(switch 0x00,07,0d,13,2b,0e,14,17,18,28,19,1a,29,22,23,24,2a)；`clutter_mesh_ty_data` 取网格+bbox→cuboid 碰撞（min 0.1）；push 视觉；每 subtype 挂高常量（case 3/4→-0.25/3.02；9→0.3；0x10→0.92/1.59；0x11/12→0.945；0x2c→1.56）。

## 五、总览
本族 = 墙/屋顶装饰的视觉+碰撞+屋顶洞差分生成集群，统一产出：①WindowVisualEntityBuilder 视觉记录（mesh/shader 名+len、位置/quat、垂直范围、等级）②DecoratorBlueprint 碰撞（new_cuboid/new_convex/new_indexed + 40 字节块存 blueprint[0x23]）③DormerHoleSsbo 屋顶洞（cottage/gothic）④WallAttachedDecoTracker 双层差分驱动 chimney/door-stairs 砖列（closure 内联）。
算法清单：差分重建 / 双层差分索引 / 砖列块装配 / dormer 剖面曲线(硬编码/采样) / DormerKey 量化缓存凸包 / 旗帜贪心连接(容量+高度+遮挡) / 布条等分插值 / 灯笼分型+闪烁双缓冲差分上传 / 门窗过梁(0.82/0.64)+花坛 / 屋顶洞 / 杂物 bbox 碰撞。

## 未知清单
1. `DecoDiff<T>` 变体名、40 字节 `Chimney/DoorStairsAssemblyParams` 布局（U）。
2. chimney/door-stairs closure_env_0 砖列主体（内联，U）。
3. `calc_gothic_half_dormer_profile_ws` 反编译打散，需 P-code（U）。
4. `check_for_door_stairs`/`add_door_autoclutter`/`get_door_holes`/`get_window_holes`/`dormer_roof_height`/`is_bay_window` 内联细节（U）。
5. 各窗/门/旗/灯笼 subtype→mesh 静态表（DAT_142a8xxxx，U）。
6. `assemble_dormer_roof_tiles` 瓦片间距常量（内联，U）。
7. `maintain_lantern_lights` 的 compare_light_lists/ColorId（内联，U）。
8. `get_gothic_hat_bones` 输出骨骼布局（U）。
9. `is_out_of_roof_bounds` 返回极性（I：u 区间是否覆盖矩形四角）。
10. `has_cloth` 触发条件（U）。
11. gothic_dormer_profile_curve 点对低/高字拆分语义（U）。

---

# C 楼梯视觉 / ui_systems / utils

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

---

## 批次 3 复核修正（登记，详见 BATCH3-LOGIC-REVIEW-decorator.md）

**系统性纠错（C1–C8）**：以下函数是**可反编译的发射函数**，ANALYSIS-B 误标为「内联/无独立符号」，须改标并反编译：
- `with_diffs`×2（WallAttachedDecoTracker）
- `construct_crossbrick_chimney`
- `construct_door_stairs`
- `assemble_dormer_roof_tiles`（+closure$3）
- `check_for_door_stairs`（实址 0x1407f3340——「无独立符号」断言直接错误）
- `compare_light_lists`×2
- `assemble_gothic_dormer_tiles`
- `is_out_of_roof_bounds`

其他：
1. 226 去重发射函数中约 166 被点名；~10 个顶级函数被误标内联、~40 闭包漏枚举。
2. `playermade_stairs_assemble` 空反编译 → 其 4 闭包（closure$8/9<true>/24/25<false>）是逻辑载体，优先反编译。
3. `WallAttachedDecoTracker` 40B params 布局可从 with_diffs/construct_* 反编译恢复。
4. 新增历史负载类型 `(DecoratorId,Vec<PreferredNodeAlignment>)`。
5. 证据一致性：A 有落盘反编译（decor-trunk/decompiled/*.c），B/C 无，建议补。
