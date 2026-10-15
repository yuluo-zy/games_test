# systems/roof 模块内部实现分析（B 版）

- 二进制：B patched EXE，sha256 cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323，程序名 tiny-glade.exe。
- 像基址 0x140000000；地址为 VA = 0x140000000 + 骨架 RVA。骨架 skeleton/B/crates/systems/roof/src/*.rs。
- 反编译来源：Ghidra get_functions（fields=signature,decompiled_code），结果落 jobs/decomp/*.c。
- 复用基础（不重复恢复）：reconstruction/roof/{surface,ridge,pipeline,tiles,edit_rules,replay}.rs；core 侧 recovered_building/roof_*.rs；evidence/roof-surface|roof-ridge|roof-tiles|building-roof-spires|building-roof-trigger。
- 标注：O=观测/原指令证实；I=结构/上下文推断；U=未知。

## 0. 共享事实：Roof 组件布局（88 字节 = 0x58）

全模块反复出现的 stride 0x58 元素即 country_core::resources::roofs::roof_shape::Roof（O）：

| 偏移 | 含义 | 依据 |
|---|---|---|
| 0x08..0x0c | shape tag u32（0=圆形，1=矩形） | O：roof[8]&1 |
| 0x0c..0x1c | 圆形 center xz / 矩形 frame(4xf32) | O |
| 0x14 | 圆形半径(tag=0) | O |
| 0x1c/0x20 | 矩形 X/Z 尺寸(tag=1) | O |
| 0x24/0x28 | tip_offset_01(Vec2) | O |
| 0x2c | profile_01 | O |
| 0x30 | height_01 | O |
| 0x34 | ridge_length_01 | O |
| 0x38 | eave_length_01 | O |
| 0x3c | ridge_dir bit(0=Width,1=Length，命名对应 U) | O |
| 0x40..0x4c | 参数块尾(Roof::new 拷入) | O |
| 0x4c | 屋顶基底 Y(=PublicWallState::max_y()) | O |
| 0x54 | is_halftimbered 缓存 | O |
| 0x55..0x58 | 未写(0x58 来自组件步长) | O |

spawn bundle(O)：tuple<Roof, RoofTileSimulation, RoofTriggerVolume, RoofAnimation, RoofTiles, RoofSpire, RoofSupportBeams, NaniBundle<StoneFloorInstanceData>>。
历史枚举 RoofEdit 判别：1=Delete(O)、2=VisualEdit(O)、3=RidgeDirection(O)、4=?(I：命名 FlatRimStyle 系推断，见下「判别值证据分级」)。回放条目 RoofReplayCmd::process 独立判别：0=Create(O)、1=Delete(O)、2=VisualEdit(O)、3=Direction(O)、4=?（行为 O，命名 FlatRimStyle 为 I）。

判别值证据分级：tag1/2/3 由 `process_invalid_roof_notifications`(写 1=Delete)、`roof_record_history`+`do_ui_edit_*`(写 2=VisualEdit)、`record_switch_roof_ridge_history`(写 3，且 `switch_roof_ridge_direction` 应用 `roof[0x3c]`)直接证实 → O。tag4 仅在 `process` case4 观测到写两个相同字节 + 0（O 行为），与 `SetFlatRoofRimStyleCmd` 事件名对应，但「tag4=FlatRimStyle」命名系 `reconstruction/roof/replay.rs::ObservedReplayEvent::FlatRimStyle` 的推断，未由消费者证实 → **I（此前报告标 O 属过度断言，此处更正）**。

---

## 1. 生命周期主干

### 1.1 create_roof — 0x1421aaea0 (rva 0x21aaea0, 2075B/432)
- 内部逻辑：ECS 系统。双层 archetype chunk 迭代（外层 stride 0x40）。每元素：PublicWalls::get()→PublicWallState::max_y()（屋顶基底 Y）、WallStyle::is_halftimbered()；Roof::new()/ty()/is_gable()；若 gable 向事件队列 push（0x10）；RoofTileSimulation::new()、RoofTriggerVolume::default()、RoofAnimation::new()；Commands::spawn<bundle>；ShapeParameters::center()+swizzlers::xvy() 算世界位置并 push indexmap bucket。
- 算法：无几何；权威状态→组件生成。墙高→roof+0x4c；Roof::new 28B 参数块见 surface.rs。
- 常量/结构：Roof 88B；chunk 0x40；uVar18=6/0；local_378=CONCAT11(RoofType,uVar18)。
- 复用：core/roof_creation.rs；reconstruction/roof/surface.rs。
- 未知 U：外层查询组件；两条事件队列资源名；local_250 来源；uVar18 枚举名；RoofType 断言编码。

### 1.2 edit_roof — 0x1421ab7c0 (rva 0x21ab7c0, 3175B/694)
- 内部逻辑：编辑后段 ECS 前段。chunk 迭代，PublicWalls::roof_entity()→实体→Roof(0x58)。按 64B 事件体应用 5 编辑，字段顺序与 edit_rules.rs::apply_edit 逐条一致(O)：profile(0x2c) mode0/1/2 clamp[0,1]；height(0x30) mode2 delta 下界0.1 else clamp[0,1]；tip(0x24 Vec2) mode2 clamp_length_max(1.0) else absolute；eave(0x38) mode2 clamp[0,1] else absolute；ridge(0x34) 调 is_gable()，mode2 clamp[0,1] else absolute。每次写设 changed tick。反馈事件 tag 2(Scalar)/10(GableEnter)/11(GableLeave)/3(Tip)。末尾 tip 距离>0 push tag3；每事件写 (entity,?,1) 记录；高度跨越 0.1 判定 push 0x20 记录。
- 算法：等价 reconstruction/roof/edit_rules.rs（SSE clamp/maxss/minss 含 NaN 语义）。
- 常量：0.1、1.0、clamp_length_max(1.0)。
- 未知 U：ECS filter(是否 Changed)；事件队列资源名；tick 语义(推断 GameTime)。

### 1.3 delete_roof 三函数
#### 1.3.1 process_invalid_roof_notifications — 0x14062eb80 (rva 0x62eb80, 2122B/530)
- 内部逻辑：两段查询迭代(flag lVar8+0x108)。核心 Vec<WallId>::from_iter(EventIterator<RoofIsInvalid> → map closure_env_0)(O mangled 名)。按 wall→roof 哈希表查实体，对受影响 roof 构造历史条目(*[2]=1 → RoofEdit::Delete) 并 History::add()。也把 roof 实体塞 0x18 记录队列。
- 常量/结构：历史 tag=1；wall→roof 索引 0xa0/0x48；事件 0x18。
- 复用：core/roof_invalid_notifications.rs。
- 未知 U：RoofIsInvalid 产生条件(wall 侧)；closure_env_0 映射细节(内联)。

#### 1.3.2 mark_roof_for_deletion — 0x14062f3d0 (rva 0x62f3d0, 1258B/323)
- 内部逻辑：收集实体 Vec<Entity>；对每个 roof Commands.entity(e).insert(RoofMarkedForDeletion)(O mangled 名)，Roof::pivot_3d() 取枢轴写 0x28 indexmap bucket。用 roof 哈希表交叉验证。
- 复用：core/roof_deletion.rs、core/roof_components.rs。
- 未知 U：标记来源；pivot 事件消费者。

#### 1.3.3 delete_roofs — 0x14062f8c0 (rva 0x62f8c0, 1008B/254)
- 内部逻辑：遍历带 RoofMarkedForDeletion 的 roof(两段)；当某浮点 fVar2==0.0 && !NaN(O) 时 EntityCommands::despawn()。该浮点即 RoofAnimation 删除进度(core RoofAnimation::deletion_finished)。
- 复用：core/roof_components.rs。
- 未知 U：despawn 前其他清理。

### 1.4 update_roof_spatial — 0x1421a7710 (rva 0x21a7710, 1459B/342)
- 内部逻辑：chunk 迭代(stride 0x10)。每 roof：PublicWalls::roof_entity()→max_y()→写 roof+0x4c；is_halftimbered()→写 roof+0x54，变化则 push tag2；墙样式 uVar4<2 时读墙 +0x25c..0x268(4xf32)+0x26c(8B) 与 roof+0x0c..0x20 比较，不同则写 roof+0x08/0x0c..0x1c 并 push tag2；Roof::ty() 非 gable 时 pivot_3d() 写 (x, y-0.69, z)，gable 清零。
- 算法：空间/尺寸缓存更新 + gable 尖顶 Y 偏移 -0.69。
- 复用：core/roof_spatial.rs。
- 未知 U：墙 +0x25c..0x26c 语义；-0.69 含义；事件队列资源名；get_shape_params(内联 0x21a7a6d)。

---

## 2. 历史与回放

### 2.1 deferred_record_roof_history::roof_record_history — 0x141a37fd0 (rva 0x1a37fd0, 402B/103)
- 内部逻辑：读 RecordRoofHistoryCmd 事件(元素 stride 0x98)。构造历史条目 *[2]=2(RoofEdit::VisualEdit)，写 +0x14=uVar5,+3=uVar4,+0x1c=uVar6,+4=uVar7,+0x24=uVar9,+5=uVar8,+0x2c=uVar10,+6=uVar11,[7],[8],[9]，来源偏移 0x10/0x18/0x20/0x24/0x28/0x2c/0x5c/0x64/0x68/0x6c/0x70；History::add()。
- 算法：延迟历史提交(打包 prev/new profile/height/ridge/eave/tip 为 VisualEdit)。
- 复用：replay.rs::visual_edit_from_observed(tag2)；core/roof_creation.rs。
- 未知 U：源 0x98 组件字段名；16B 历史前缀。

### 2.2 replay_roof — 0x141a37ad0 (rva 0x1a37ad0, 1277B/338)
- 内部逻辑：InputStream::sync_point_count()；ReplayBuffer::front_mut_bucket() 取当前帧桶(Vec 元素 0x30)；扫描桶，元素首字节=5(RoofEdit) 且 impl$0::process() 后 +0x1d==1 时收集下标；反转下标 Vec，从后往前 Vec::remove 并按 tag drop Arc(0/1/2=RasterEdit,3=TerrainEdit,4=WallReplayCmd,5=RoofEdit,6/7=DecoratorHistoryEdit,8=TreeEdit,9=CrashMiscInfo,default=PhysicalSize)(O)。
- 算法：帧回放调度 + 处理标记判定。
- 复用：reconstruction/roof/replay.rs。
- 未知 U：+0x1d 置位分支(in_RCX[0x15])；sync_point_count 作用。

### 2.3 replay_roof::impl$0::process — 0x1421b48e0 (rva 0x21b48e0, 1279B/303)
- 内部逻辑：RoofReplayCmd::process 状态机。switch(payload[0x10])：case0/1 Create/Delete 按 in_RCX[0x14](undo) 发 0x40/0x18 事件并置 in_RCX[0x15]=1；case2 VisualEdit：in_RCX[0]&1 推进 elapsed(in_RCX+0x10+=dt)，ratio=elapsed/duration(in_RCX+4) clamp[0,1]，undo 时 progress=1-progress，用 progress 线性混合 prev/new 的 profile(0x18/0x28)、height(0x14/0x24)、ridge(0x1c/0x2c)、eave(0x20/0x30)、tip(0x38/0x48)，写 0x40 事件(mode=1)；case3/4 Direction/FlatRimStyle 仅 elapsed==0 发 0x18 事件，按 in_RCX[0]==1 推进并 clamp，完成置 0x15=1。
- 算法：与 replay.rs::advance_discrete_observed/advance 逐条一致(0/0→NaN、MAXSS/MINSS、progress=1-progress)。
- 未知 U：无(内部逻辑已确定)。

---

## 3. 触发体 / 音频 / 派生

### 3.1 calculate_roof_trigger_volumes — 0x1421a3220 (rva 0x21a3220, 14692B/2771)
- 内部逻辑：读 RecalculateRoofCmd 事件(0x20)查实体→Roof。剖面细分数 local_2d8=(is_flat^1)*5+4(flat=4,gable=9)。矩形分支：get_roof_expanded_top_and_bottom、RoofBoundsDims::as_rectangle、profile_curve_normalized、Rectangle2d::new_centered_axis_aligned/new_axis_aligned、orientation_mat2、is_gable、eave_length_ws(gable 侧 -0.56, offset +0.4)、gable_sides、Rectangle2d::as_curve2；沿剖面曲线 u=i/4 采样 Curve::get_coord_at_u<Vec2>，混合 Roof::height()，构造三角条带 Vec<[Vec3;3]>+UV(2.0=0x40000000,2.2=0x40066666)+法线。圆形分支：Circle2d::new_rotated(0.05 inset)、get_pos_at_u(u=i/15, 15 段)、Roof::height()、swizzlers(cos/sin 6.2831855)。对每顶点 RoofNoise::apply_roof_noise(减 roof+0x4c)。RoofTriggerVolume::set_triangles/set_uvs/set_normals，写 changed tick。
- 算法：屋顶曲面采样→三角化(矩形 4 剖面段/圆形 15 环段)。复用 surface.rs。
- 常量：4/9；0.4；0.56；UV 2.0/2.2；15 段；0.05；0xbf3d70a4≈-0.74。
- 未知 U：RoofTriggerVolume 容器类型(core 占位)；apply_roof_noise 算法；RoofBoundsDims::as_rectangle；Roof::gable_sides()。

### 3.2 simulate_roof_tile_for_audio — 0x1421a7090 (rva 0x21a7090, 1654B/357)
- 内部逻辑：读 RoofTileSimulation(lVar8)，lVar8+0x28==0 直接返回。chunk 迭代瓦片(每瓦 0x18B：pfVar2[0]=x,[2]=z/height,[3]=angle,[4]=accum,[5]=flag)。is_flat 跳过平屋顶；pivot_3d()。每瓦：fVar24=风/参数(lVar8+0x28)；fVar27=fVar24*-45.126003+pfVar2[3]；-120.0 阈值/0.166 因子分支，clamp 到 [..,15.0]；阻尼 e=expf(fVar24*-13.300004)；fVar24=(1-e)*旧高+(1-(1-e))*新高 写 pfVar2[2]；|Δ| 与 0.1/0.01 阈值比较 push 音频 tag5(首跨0.1) 或 tag4(衰减 (accum-0.01)/0.19 clamp)，累加 pfVar2[4]。
- 算法：瓦片高度指数衰减/风驱动模拟(音频)。
- 常量：-45.126003、-13.300004、0.166、15.0、0.1、0.01、0.19。
- 复用：core/roof_components.rs::RoofTileSimulation。
- 未知 U：lVar8+0x28 来源；tag4/5 音频事件；瓦数组容器语义；simulate_roof_tile(内联 0x21a743f)。

### 3.3 wall_height_dt — 0x1421a2b60 (rva 0x21a2b60, 575B/142)
- 内部逻辑：两段查询迭代(flag lVar6+0x108)。每墙高记录(0x18B,+4=缓存高)：fVar2=tile/roof+0x4c(基底 Y)，fVar3=墙记录+4；写 *(lVar8+idx*0x18)=fVar2-fVar3(存储差值)；写 tick；写 *(lVar8+4+idx*0x18)=fVar2(更新缓存)。
- 算法：墙高变化增量传播到屋顶。无几何/曲面算法。
- 常量/结构：元素 0x18(+4 墙高缓存)；roof 0x58；墙表 0x48/0xa0。
- 未知 U：写入字段语义名(推断"墙高变化量")；两段查询差异；tick 语义。

---

## 4. 视觉族

### 4.1 visual::assemble_roof_tiles::assemble_roof_tiles — 0x141a36700 (rva 0x1a36700, 2965B/739)
- 内部逻辑：puffin scope。chunk 迭代(0x20)。每 roof：SSBO 库取 RoofTileSsbo(name 0x35)；取/分配 SparseInstanceBufferAllocation<RoofTileSsbo>；写 tick；SparseInstanceBufferWriter::clear<RoofTileSsbo>()。is_flat()；非平屋顶用 HashMap<GlobalId> hash_one+位掩码扫描判脏；命中则按 roof[8]&1 调 utils::assemble_rectangular_roof::<closure> 或 assemble_circular_roof::<closure>(CPU 铺瓦，inlined closure_env_0 逐瓦 push)。
- 算法：脏跟踪+分发 CPU 铺瓦(reconstruction/roof/pipeline.rs::assemble_roof_tiles_observed)。
- 复用：reconstruction/roof/{pipeline,tiles,rectangular_*}.rs。
- 未知 U：脏集合 key 语义与更新时机；SSBO 名常量；closure_env_0 每瓦变换。

### 4.2 visual::generate_roof_stone_floor_and_roof_bottom — 0x1421a6d40 (rva 0x21a6d40, 840B/204)
- 内部逻辑：chunk 迭代(0x20)遍历 roof。每 roof：查实体→Roof；若 (roof[8]&1)==0(圆形) 调 log2f(roof[0x14]*3.0)；DecoratorRank::new()+drawable::from()，Commands.entity(e).insert(NaniInstanceDesc)(kind 9)，记录 0x38 实例描述(含 roof handle)。
- 算法：石地板/屋顶底面实例生成；圆形细分级别 log2(radius*3)。
- 常量：3.0；kind=9；实例描述 0x38。
- 未知 U：NaniInstanceDesc kind9 含义；log2 结果用途；矩形分支去向。

### 4.3 visual::place_spires::place_roof_spires — 0x142196e70 (rva 0x2196e70, 1943B/483)
- 内部逻辑：puffin scope；SSBO 库取 country_core::resources::roof_tile_ssbo::RoofSpireSsbo(name 0x36, memcmp 校验)；Vec<RoofSpireSsbo>::from_iter 遍历 Query<(Ref<Roof>,Ref<RoofAnimation>),With<RoofSpire>>+filter_map closure_env_0(每 roof 尖顶参数内联)；GraphicsDevice::update_buffer_bytes 上传。
- 算法：尖顶数据收集→GPU 上传；每 roof 几何在闭包(内联)。
- 未知 U：closure_env_0 尖顶几何；RoofSpireSsbo 布局；SSBO 名。

### 4.4 visual::switch_roof_type::switch_roof_type — 0x1421ac4f0 (rva 0x21ac4f0, 2553B/567)
- 内部逻辑：chunk 迭代(0x20)，元素 +0x18(bool cVar3)、+0x19(char cVar2=目标类型)、+0x10(实体)。定位 Roof，Roof::ty() 并 assert 与 cVar2 相等(否则 panic)。cVar2==0：insert NaniBundle<StoneFloorInstanceData>；roof+0x34==1.0 push 事件；cVar3==1 时 InnerWalls::get_unwrapped() 且墙高 +0x54!=1.6 → update_wall_state_height()。cVar2!=0：remove DrawableMeshBundle；按名 "roof" try_get_handle_by_name+deferred_delete_mesh；insert NaniBundle；push 事件；remove Handle<Vao>。
- 算法：屋顶类型切换(视觉重建)。
- 常量：墙高默认 1.6；ridge 1.0；mesh 名 "roof"。
- 未知 U：RoofType A/B 名；其他 mesh；stone_floor_bundle 细节。

### 4.5 visual::roof_animation::roof_animation — 0x1421a2da0 (rva 0x21a2da0, 1139B/251)
- 内部逻辑：chunk 迭代遍历 roof；is_flat()。每 roof 一对 f32(pfVar14[0]/[1])按 dt(lVar7+0x28)推进：删除标记分支 fVar19=dt*-4.0,fVar18=dt*-10.0；正常分支 fVar18=(is_flat?1.0:2.0)，fVar19=dt*2.857143*fVar18，fVar18=(pfVar14[0]>0.99?dt*3.3333333:0)*fVar18；clamp[0,1] 写 pfVar14[0]/[1]，写 tick。
- 算法：屋顶动画向边界衰减。常量 -4.0/-10.0/2.857143(=1/0.35)/3.3333333(=1/0.3)/0.99。
- 复用：core/roof_components.rs::RoofAnimation::advance。
- 未知 U：两 f32 语义名；0.35/0.3 含义。

---

## 5. UI 编辑

### 5.1 ui_edit_roof
#### 5.1.1 ui_edit_roof — 0x141a18dd0 (rva 0x1a18dd0, 3254B/786)
- 内部逻辑：puffin "roof_edit_"。*(int*)*in_RCX==0 时 chunk 迭代 roof，RoofAnimation::is_being_animated() 跳过动画中 roof；format!；RoofTriggerVolume::triangles()→Vec<Triangle>(射线相交)；Roof::ty()→local_24f='\f'-ty(tag12/11)；WidgetParams::new(boop/on_hover_start)；UiState::add<String>；UiWidgetContext::hovered/intersecting；StructureType::is_fence→发 HighlightRoofCmd；slider3d::into_drag_state；EventWriter::write<RecordRidgeSwitchHistoryCmd>。
- 算法：每帧构建屋顶编辑 slider3d 控件，三角相交拾取，hover/active。
- 未知 U：local_24f=12-ty 语义；RecordRidgeSwitchHistoryCmd 发出条件；triangles() 数据源。

#### 5.1.2 do_ui_edit_roof — 0x141a19a90 (rva 0x1a19a90, 4228B/985)
- 内部逻辑：switch(clamp(payload[0x10]-2,0,3))。case1：写事件；roof 字段构造 0x40 历史；InnerWalls::get()→墙高+0x54 写 *pfVar14；rt::impl$5::from()；DecoratorArchivist::archive_current_decorators_in_storage/decorator_cull_allow_wall；*pfVar13=0.0。case2：写事件；第二查询取拖拽量 SQRT(dx²+dy²)；>0.2 发 NotifyHintSystemRoofShape；墙高 1.6 判定 update_wall_state_height()；Roof::ty()；构造 0x40 VisualEdit 事件。case3：写事件；比较 roof 0x2c/0x30/0x24/0x34/0x38/0x3c/0x3d 与控件新值；变化构造 0x98 历史(tag2)+History::add()；墙高变化 wall_history::ev_height_changed()；另发两事件。
- 常量：0.2；1.6；0.1。
- 复用：edit_rules.rs、replay.rs。
- 未知 U：action 枚举名；两条额外事件资源名；rt::impl$5::from 语义。

### 5.2 ui_edit_roof_ridge_dir 族
#### 5.2.1 ui_edit_roof_ridge_dir — 0x141a386b0 (rva 0x1a386b0, 3551B/746)
- 内部逻辑：*(int*)*in_RCX==0 时 Decorators::iter_removed_ids() 遍历，PublicWalls::get()/PublicWallState::is_rectangle()/roof_entity() 过滤；Roof::ty()(只留矩形)；RoofRidgeDims::from()+to_world_space()；swizzlers::x0y、xz_projective_rotation_bad、ui_edit_rectangle_orientation::box_trigger_volume()；WidgetParams::new；UiState::add<String>；UiWidgetContext::is_active()→发 DoUiEditRoofRidgeDir。
- 复用：reconstruction/roof/ridge.rs。
- 未知 U：xz_projective_rotation_bad；iter_removed_ids 语义。

#### 5.2.2 do_ui_edit_roof_ridge_dir — 0x141a39490 (rva 0x1a39490, 4182B/918)
- 内部逻辑：读控件盒(0x30)；roof_entity()→Roof；Roof::ty()(!=0&&ridge==1)；RoofRidgeDims::from/to_world_space；swizzlers；脊线两端中点 (a+b)*0.5；box_trigger_volume；format!("roof_ridge_dir_focused")；WidgetParams(occlude_by_world)；SignifierFactory::new+UiSignifierStream::write；slider3d::into_drag_state；UiWidgetContext::clicked() 时发 SwitchRoofRidgeDirCmd 和 RecordRidgeSwitchHistoryCmd(roof+0x3c^0x100 翻转方向位)；缩放态!=3 时 decorator_cull_allow_wall。
- 常量：0x3f99999a=1.2；ridge 判定==1。
- 未知 U：SignifierFactory 输出；^0x100 位。

#### 5.2.3 switch_roof_ridge_direction — 0x141a3a730 (rva 0x1a3a730, 1252B/273)
- 内部逻辑：读 SwitchRoofRidgeDirCmd(0x18)；roof_entity()→Roof；写 roof+0x3c=事件字节 并写 tick；push 事件(0x20/0x10)；pivot_3d()+height()→push indexmap 记录(tag7 含高度)。
- 复用：core/roof_direction.rs。
- 未知 U：tag7 记录语义；事件资源名。

#### 5.2.4 record_switch_roof_ridge_history — 0x141a3ac20 (rva 0x1a3ac20, 392B/105)
- 内部逻辑：读 RecordRidgeSwitchHistoryCmd(0x18)；每条构造历史条目 *[2]=3(RoofEdit::RidgeDirection)，写 +0x11=from,+0x12=to,[3]=uVar9；History::add()。
- 复用：replay.rs(Direction tag3)。
- 未知 U：uVar9 语义；from/to 来源。

### 5.3 ui_edit_roof_ridge_length 族
#### 5.3.1 ui_edit_roof_ridge_length — 0x1421a8210 (rva 0x21a8210, 2616B/696)
- 内部逻辑：*(int*)*in_RCX==0 时 iter_removed_ids()→roof_entity()→对每 roof 调 create_ui_widget()。模式 iVar6-1∈{0,1}(即 1 或 2)时也对选中 roof 调 create_ui_widget()。
- 复用：本模块 create_ui_widget。
- 未知 U：模式枚举(1/2)语义；候选来源。

#### 5.3.2 do_ui_edit_roof_ridge_length — 0x1421a8c50 (rva 0x21a8c50, 4765B/1006)
- 内部逻辑：读控件盒(0x40)复制端点；roof_entity()→Roof；求脊中点；2 把手循环：TweakerView::as_inner_mut；format!("roof_ridge_edit")；SignifierFactory+UiSignifierStream::write；slider3d::into_drag_state；switch(drag)：case1 project_cursor_onto_ray→距离 2*d-ridge_len 写参数+decorator_cull_allow_wall；case2 project_cursor_onto_ray→Roof::shape_dims()→inverse_ridge_01_param()→新脊长(置 emit 标志)；case3 静态值。脊长变化写 EditRoofCmd；高度/方向标志构造事件；roof 字段变化构造 0x98 历史(tag2)+History::add()。
- 复用：reconstruction/roof/ridge.rs::inverse_ridge_01_param、edit_rules.rs。
- 未知 U：project_cursor_onto_ray(core utils)；case3 静态值来源。

#### 5.3.3 create_ui_widget — 0x1421aa170 (rva 0x21aa170, 3203B/627)
- 内部逻辑：RoofRidgeDims::from/to_world_space；Roof::ty()(必须 !=0 且 ridge==1)；对脊线 2 端点各：utils::rotation_from_normal；沿脊偏移 0.75；box_trigger_volume；format!("roof_ridge_edit")；WidgetParams(boop/on_hover_start/occlude_by_world)；UiState::add；UiWidgetContext::hovered；StructureType::is_fence→into_drag_state(缩放 1.0 或 0x3f99999a=1.2)；NodeContext::id；push widget 描述(0x18)。
- 常量：0.75；1.2。
- 未知 U：rotation_from_normal；widget 描述字段。

### 5.4 ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused — 0x1421ad440 (rva 0x21ad440, 5393B/1164)
- 内部逻辑：聚焦态(模式 1/2,<6)。roof_entity()→Roof；RoofRidgeDims::from/to_world_space；求脊中点；偏移 0.75；CameraMatrices::eye_right；xz_rotation_bad；box_trigger_volume；DigitalActionState::pressed 选 UiSounds(boop/on_hover_start 或 empty)；format!("roof_edit")；WidgetParams(sounds/occlude_by_world)；UiState::add；hovered；StructureType::is_fence→into_drag_state(缩放 1.0/1.2)；format!("height ↓") 指示符；SignifierFactory+UiSignifierStream::write；switch(drag)：case1 project_cursor_onto_ray→距离差；rt::from；DecoratorArchivist::archive_current_decorators_in_storage/decorator_cull_allow_wall；InnerWalls::get() 墙高+0x54；构造 0x40 VisualEdit 事件；case2 project_cursor_onto_ray→utils::inverse_lerp→clamp[0,1] 新高度；墙高 1.6 且跨越 0.1→update_wall_state_height()；构造 0x40 事件；case3 比较 roof 字段；构造 0x98 历史(tag2)+History::add()；墙高变化→wall_history::ev_height_changed()。
- 常量：0.75；1.6；0.1；显示常量 38.7063/6.646431。
- 复用：reconstruction/roof/ridge.rs、edit_rules.rs。
- 未知 U：height ↓ 指示符字段；xz_rotation_bad；38.7063/6.646431 含义。

---

## 6. 插件 / 组件

### 6.1 lib.rs::impl$0::add_roof_systems — 0x14062fcb0 (rva 0x62fcb0, 2357B/465)
- 内部逻辑：注册 3 个 UI 编辑事件(DoUiEditRoof/DoUiEditRoofRidgeDir/DoUiEditRoofRidgeLength，各先 Components::get_resource_id 去重)，再 SubApp::add_systems 用 in_set<GameSystemSet>+chain/chain_ignore_deferred 挂系统。mangled 名暴露系统签名(O)：void(Query<(Entity,&Roof,Option<&RoofMarkedForDeletion>,&RoofAnimation)>,Commands)；void(ResMut<ReplayBuffer>,Res<GameTime>,EventWriter<CreateRoofCmd/DeleteRoofCmd/EditRoofCmd/SwitchRoofRidgeDirCmd/SetFlatRoofRimStyleCmd>)(=replay_roof)；void(ResMut<History>,EventReader<RecordRoofHistoryCmd>)(=roof_record_history)；void(EventReader<RecordRidgeSwitchHistoryCmd>,ResMut<History>)(=record_switch_roof_ridge_history)；void(Res<GraphicsDeviceResource>,ResMut<AssetSsboLibrary>)(=assemble_roof_tiles/place_spires)。
- 未知 U：系统确切顺序(chain hash)；GameSystemSet 子集。

### 6.2 lib.rs::impl$0::add_roof_events_and_resources — 0x1406305f0 (rva 0x6305f0, 1067B/237)
- 内部逻辑：insert_resource<RoofParamsCacheResource>、insert_resource<RoofHighlight>；注册事件(去重)：CreateRoofCmd/DeleteRoofCmd/EditRoofCmd/RecalculateRoofCmd/PlayRoofSound/HighlightRoofCmd/SwitchRoofTypeCmd/SwitchRoofRidgeDirCmd/RecordRoofHistoryCmd/RecordRidgeSwitchHistoryCmd/SetFlatRoofRimStyleCmd。
- 未知 U：两 resource 字段。

### 6.3 utils::components::register_required_components (impl$0 0x14258d1f0 / impl$1 0x14258d8e0, 各 1776B/420)
- 内部逻辑：derive(Component) 生成的 Component::register_required_components。hashbrown HashMap 插 TypeId→ComponentId；ComponentInfo::new(类型名长度 0x29=41 → system_roof::utils::components::RoofSpire；impl$1=RoofSupportBeams)；递归 register_required_components() 注册 required；enforce_no_required_components_recursion。
- 数据结构：utils/components.rs 定义 marker 组件 RoofSpire、RoofSupportBeams(O)。
- 复用：Bevy 0.16 derive；core/components。
- 未知 U：两组件 #[require(...)] 列表(递归调用被内联为自身)；字段(.structures 记 unknown)。

---

## 逻辑总览

编辑命令/回放：
- CreateRoofCmd → create_roof → spawn(bundle)
- EditRoofCmd → edit_roof → apply_edit(profile/height/tip/eave/ridge)+反馈事件
- DeleteRoofCmd → mark_roof_for_deletion(insert RoofMarkedForDeletion) → roof_animation(删除衰减) → delete_roofs(despawn)
- RoofIsInvalid → process_invalid_roof_notifications → History::add(Delete)

历史/回放：
- ReplayBuffer(RoofReplayCmd) → replay_roof → impl$0::process(状态机)
- RecordRoofHistoryCmd → roof_record_history → History::add(VisualEdit tag2)
- RecordRidgeSwitchHistoryCmd → record_switch_roof_ridge_history → History::add(RidgeDirection tag3)

派生/音频：
- RecalculateRoofCmd → calculate_roof_trigger_volumes → RoofTriggerVolume::{set_triangles,set_uvs,set_normals}
- RoofTileSimulation → simulate_roof_tile_for_audio → 音频 tag4/5
- 墙高变化 → wall_height_dt(缓存差值)；update_roof_spatial(max_y→roof+0x4c, halftimbered→+0x54)

视觉：
- 脏 roof → assemble_roof_tiles → assemble_{rectangular,circular}_roof → RoofTileSsbo
- With<RoofSpire> → place_roof_spires → RoofSpireSsbo
- 圆形 roof → generate_roof_stone_floor_and_roof_bottom → NaniInstanceDesc(kind9)
- RoofType 切换 → switch_roof_type(换 mesh/bundle)
- 季节/雪 → roof_animation(每 roof 两 f32 衰减)

UI 编辑：
- ui_edit_roof/ridge_dir/ridge_length/focused → 构建 slider3d 控件+触发体
- do_ui_edit_* → 拖拽反解 → EditRoofCmd/SwitchRoofRidgeDirCmd + 0x40 事件 + 0x98 历史 + History::add

## 算法清单（可复用/已恢复）

| 算法 | 出现函数 | 现成实现 |
|---|---|---|
| 参数编辑标量规则(mode 0/1/2,clamp,SSE NaN) | edit_roof, do_ui_edit_roof/ridge_length/focused | reconstruction/roof/edit_rules.rs::apply_edit |
| tip 向量 clamp_length_max(1.0) | edit_roof | edit_rules.rs::limited_tip |
| 回放状态机(progress 混合/完成标志) | replay_roof::impl$0::process | reconstruction/roof/replay.rs |
| Roof 88B 参数块布局/Roof::new | create_roof | surface.rs、roof_state.rs |
| 剖面曲线 20 点/height/eave_length_ws/Curve2 采样 | calculate_roof_trigger_volumes | surface.rs |
| 脊线几何 RoofRidgeDims::from/to_world_space/inverse_ridge_01_param | ui_edit_roof_ridge_dir/length/focused | reconstruction/roof/ridge.rs |
| CPU 铺瓦(矩形/圆形) | assemble_roof_tiles | reconstruction/roof/{pipeline,tiles,rectangular_*}.rs |
| 屋顶动画衰减 | roof_animation | core/roof_components.rs::RoofAnimation::advance |
| 删除动画完成判定 | delete_roofs | core/roof_components.rs::deletion_finished |
| 三角化(矩形 4 段/圆形 15 环) | calculate_roof_trigger_volumes | 部分(surface.rs 提供采样) |
| 高亮命令→颜色资源 | highlight_roof | core RoofHighlight/HighlightColor |
| 铺瓦 SSBO GPU 上传 | gpu_flush_buffers | rhapsody SparseInstanceBuffer::gpu_flush（依赖） |
| 石地板 bundle 构造(kind9,白底) | switch_roof_type::stone_floor_bundle / generate_roof_stone_floor_and_roof_bottom | 本报告 §4.2/§7.3.4 |

## 未知清单(U)

1. create_roof：外层查询组件；事件队列资源名；uVar18=6/0 枚举；local_250 字段。
2. edit_roof：ECS filter(是否 Changed)；事件队列资源名。
3. delete 族：RoofIsInvalid 产生条件(wall 侧)；标记来源；despawn 前清理。
4. update_roof_spatial：墙 +0x25c..0x26c 字段；-0.69 含义；get_shape_params 内联体。
5. roof_record_history：源 0x98 组件字段名；16B 历史前缀。
6. replay_roof：+0x1d 置位分支；sync_point_count。
7. calculate_roof_trigger_volumes：RoofTriggerVolume 容器类型；apply_roof_noise；RoofBoundsDims::as_rectangle；Roof::gable_sides()。
8. simulate_roof_tile_for_audio：lVar8+0x28 来源；tag4/5 音频事件；瓦数组容器。
9. wall_height_dt：写入字段语义名；两段查询差异。
10. assemble_roof_tiles：脏集合 key 语义/更新时机；closure_env_0 每瓦变换。
11. generate_roof_stone_floor_and_roof_bottom：NaniInstanceDesc kind9；log2(radius*3) 用途；矩形分支去向。
12. place_roof_spires：closure_env_0 尖顶几何；RoofSpireSsbo 布局。
13. switch_roof_type：RoofType A/B 名；其他 mesh；stone_floor_bundle 细节。
14. roof_animation：两 f32 语义名；0.35/0.3 常量含义。
15. UI 编辑族：控件 action 枚举名；rt::impl$5::from / xz_projective_rotation_bad / rotation_from_normal / project_cursor_onto_ray(utils/core 依赖)；SignifierFactory 输出；若干事件资源名；height ↓ 指示符字段。
16. register_required_components：RoofSpire/RoofSupportBeams 的 #[require(...)] 列表；字段。
17. add_roof_systems：各系统 schedule 确切顺序。
18. highlight_roof：RoofHighlight 字段布局、EditingHighlight 游标语义、HighlightColor::to_rgb 目标格式。
19. gpu_flush_buffers：无实质未知（gpu_flush 内部属 rhapsody 依赖）。
20. 内联 helper：get_shape_params(0x1408c5670 目标函数)、update_wall_height_if_needed 完整分支、get_gizmo_info 结构字段、stone_floor_bundle(StoneFloorInstanceData 字段)、simulate_roof_tile(同 §3.2)。
21. 未发射函数：ui_edit_roof_tip / place_roof_support_beams / gen_roof_snow_mesh 在 B 无发射实现（§8.1），A/C 版或替代内联路径 U。

## 依赖(直接复用，不逆向)
std/alloc、glam、fastrand(TileRng)、half(f16)、bracket-noise(apply_roof_noise)、bevy_ecs 0.16、rhapsody(GraphicsDevice::update_buffer_bytes)、utils(curve/geometry/rectangle/circle/swizzlers/rotation)、puffin。

## 证据路径
- 本报告：upper-modules-plan/systems/roof/ANALYSIS.md
- 反编译原文：upper-modules-plan/systems/roof/jobs/decomp/*.c(31 函数)；job/result：jobs/batch{1..6}.json、jobs/result{1..6}.json
- 复用实现：reconstruction/roof/*.rs、crates/country-core/src/recovered_building/roof_*.rs
- 证据：evidence/roof-surface|roof-ridge|roof-tiles|building-roof-spires|building-roof-trigger、evidence/roof-pipeline-verification.json


---

## 7. 补充：复核新增函数与 helper

### 7.1 `highlight_roof` — `0x141a284b0`（run_without_applying_deferred 实例，876B/183 指令）；`0x141a31280`（run_unsafe 实例，878B/191 指令）

- **内部逻辑**：`HighlightRoofCmd` 消费者系统（mangled 名给出签名 O：`void(ResMut<EditingHighlight>, EventReader<HighlightRoofCmd>, ResMut<RoofHighlight>)`）。用 `EventIteratorWithId<HighlightRoofCmd>` 逐条读事件：对每条取事件 payload 的颜色，`HighlightColor::to_rgb()` → 写 `RoofHighlight` 资源（`*plVar6 = color`），并写 changed tick（`*(in_RCX+0x160)=iVar2`）。同时更新 `EditingHighlight` 的读游标（`in_RCX+0x168`）。两个实例是同一系统函数的两个 monomorphization（wrapper 入口），主体一致。
- **算法**：无几何；把高亮命令颜色写入 `RoofHighlight` 资源。
- **数据结构/常量**：`RoofHighlight`（core 资源）、`EditingHighlight`（core 资源）；事件体 ≥9B（`uVar9` 8B + `uVar3` 1B）。
- **复用**：`core` 的 `RoofHighlight` / `HighlightColor`（`add_roof_events_and_resources` 注册 `HighlightRoofCmd`；`ui_edit_roof` 发出该命令）。
- **未知（U）**：`RoofHighlight` 的完整字段布局；`EditingHighlight` 游标语义；颜色 `to_rgb` 的目标格式（由 core `HighlightColor` 决定）。

### 7.2 `lib.rs::gpu_flush_buffers` — `0x140de50a0`（rva 0xde50a0，538B/108 指令）

- **内部逻辑**：签名（mangled 名 O）：`void(Res<GraphicsDeviceResource>, ResMut<AssetSsboLibrary>)`。取 `AssetSsboLibrary::get_mut<country_core::resources::roof_tile_ssbo::RoofTileSsbo>()`（不存在则 panic），`country_core::render::impl$0::deref()` 解引用 GPU 设备，`SparseInstanceBuffer::gpu_flush<RoofTileSsbo>()` 把铺瓦 SSBO 上传，写 changed tick（`*(in_RCX+0x150)=iVar2`）。
- **算法**：无；GPU 缓冲刷新（RoofTileSsbo 的 sparse instance buffer 上传）。
- **复用**：rhapsody `GraphicsDevice`、`SparseInstanceBuffer`（依赖，不逆向）。
- **未知（U）**：无实质未知；仅 `gpu_flush` 内部实现属 rhapsody 依赖。

### 7.3 内联 helper（无独立编译过程，正文来自宿主骨架的 Function-frame RVA span；主体已内联进对应宿主反编译）

#### 7.3.1 `update_roof_spatial::get_shape_params` — frame `0x1421a7a6d`（8 指令）

- **内部逻辑**：调用 `0x1408c5670`（取形状/尺寸），把结果写回宿主：`*(r8+rdx*4)=ecx`（changed-tick 数组）、`*(r15+0x54)=al`（halftimbered 缓存），读 `*(rdi)` roof 指针与栈上参数。是 `update_roof_spatial` 内取"形状参数"的小步骤。
- **算法**：形状参数读取/缓存。
- **未知（U）**：`0x1408c5670` 的具体函数（推断为 `Roof` 形状/尺寸 getter）；参数语义。

#### 7.3.2 `ui_edit_roof::update_wall_height_if_needed` — frame `0x141a1a6ef`（15 指令，另有多处内联 span）

- **内部逻辑**：读墙高 `*(rax)`，与常量 `[0x142a7fcac]`（值 1.6）比较，`<=` 时跳转执行 `InnerWalls::update_wall_state_height()`（宿主内可见）。即"墙高未达默认 1.6 时更新墙状态高度"。
- **常量**：`1.6`（0x142a7fcac）。
- **未知（U）**：该 helper 的完整分支（多内联 span：0x1a1a6ef/0x1a1a76d/0x21ae177/0x21ae19e/0x21ae345）。

#### 7.3.3 `ui_edit_roof_ridge_dir::get_gizmo_info` — frame `0x141a38bfc`（12+3+7 指令）

- **内部逻辑**：读 `roof+0x3c`（脊向位），据其用 `pxor` 取反（xmm7/xmm8 交换）；计算脊线两端中点 `xmm13 = (a+b)*0.5`（`0xe0+0xec` 单精度、`0xd8+0xe4` 双精度），写 `0x160(rbp)`。即"脊向 + 中点"的 gizmo 信息。
- **算法**：中点平均 + 方向位取反。
- **未知（U）**：gizmo 信息结构体字段。

#### 7.3.4 `visual::switch_roof_type::stone_floor_bundle` — frame `0x1421ab2ae`（14+15+14 指令）

- **内部逻辑**：`DecoratorRank::new()`(0x140833250) + `drawable::from()`(0x14082bd60)；写 `-0x18(rdi)=0`、`-0x20(rdi)=0`；`movb $0x9`（kind=9，与 `generate_roof_stone_floor_and_roof_bottom` 一致）；拷颜色常量 `0x3f8000003f800000`（两个 1.0f）与 `0x3f800000`。即构造 `NaniBundle<StoneFloorInstanceData>`（白底、kind 9）。
- **算法**：石地板 bundle 构造。
- **常量**：kind=9；颜色 (1,1,1)。
- **未知（U）**：`StoneFloorInstanceData` 字段；bundle 其余字段。

#### 7.3.5 `simulate_roof_tile_for_audio::simulate_roof_tile` — frame `0x1421a743f`（54+5 指令）

- **内部逻辑**：单瓦模拟体（已内联进宿主，见 §3.2）。核心：`xmm1=xmm2/xmm0`（tile/风比），用 `cmpnltss/cmpnless` 生成掩码在 `xmm3`(tile z) 与 `xmm14` 间选择（SSE blend，与宿主 `uVar23 & ... | ~uVar23 & ...` 一致），`xmm6 = xmm0*[0x142ff6c8c] + tile[0xc]`（-45.126 项），`xmm9 = xmm0*xmm6 + xmm5`（高度混合）。
- **算法**：同 §3.2（瓦片高度指数/风模拟的每瓦体）。
- **常量**：`[0x142ff6c8c]`（=-45.126003）。
- **未知（U）**：同 §3.2。

---

## 8. 本模块全符号清单（B 发射覆盖率）

来源：`grep -rh '^// FUNCTION system_roof::' skeleton/B/crates/systems/roof/src/`（70 条，含闭包）。下表列全部 `system_roof::` 符号并标覆盖情况。图例：✔=本报告已分析；G=derive/框架胶水（复用 derive，不计业务）；X=**B 未发射**（PDB 仅声明，无编译过程，二进制无符号）；闭包内联计入宿主。

| 符号 | 覆盖 |
|---|---|
| create_roof::create_roof | ✔ §1.1 |
| create_roof::create_roof::closure$0 | 内联入宿主 §1.1 |
| edit_roof::edit_roof | ✔ §1.2 |
| edit_roof::edit_roof::closure$0 | 内联入宿主 §1.2 |
| delete_roof::process_invalid_roof_notifications | ✔ §1.3.1 |
| delete_roof::mark_roof_for_deletion | ✔ §1.3.2 |
| delete_roof::delete_roofs | ✔ §1.3.3 |
| update_roof_spatial::update_roof_spatial | ✔ §1.4 |
| update_roof_spatial::update_roof_spatial::closure$0 | 内联入宿主 §1.4 |
| update_roof_spatial::get_shape_params | ✔ §7.3.1（内联 helper） |
| deferred_record_roof_history::roof_record_history | ✔ §2.1 |
| replay_roof::replay_roof | ✔ §2.2 |
| replay_roof::impl$0::process | ✔ §2.3 |
| roof_trigger_volume::calculate_roof_trigger_volumes | ✔ §3.1 |
| roof_trigger_volume::calculate_roof_trigger_volumes::closure$0 | 内联入宿主 §3.1 |
| roof_trigger_volume::calculate_roof_trigger_volumes::closure$1 | 内联入宿主 §3.1 |
| simulate_roof_tile_for_audio::simulate_roof_tile_for_audio | ✔ §3.2 |
| simulate_roof_tile_for_audio::simulate_roof_tile | ✔ §7.3.5（内联 helper） |
| wall_height_dt::wall_height_dt | ✔ §3.3 |
| visual::assemble_roof_tiles::assemble_roof_tiles | ✔ §4.1 |
| visual::assemble_roof_tiles::assemble_roof_tiles::closure$0 | 内联入宿主 §4.1 |
| visual::assemble_roof_tiles::assemble_roof_tiles::closure$1 | 内联入宿主 §4.1 |
| visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom | ✔ §4.2 |
| visual::place_spires::place_roof_spires | ✔ §4.3 |
| visual::place_spires::place_roof_spires::closure$0 | 内联入宿主 §4.3 |
| visual::switch_roof_type::switch_roof_type | ✔ §4.4 |
| visual::switch_roof_type::stone_floor_bundle | ✔ §7.3.4（内联 helper） |
| visual::roof_animation::roof_animation | ✔ §4.5 |
| ui_edit_roof::ui_edit_roof | ✔ §5.1.1 |
| ui_edit_roof::do_ui_edit_roof | ✔ §5.1.2 |
| ui_edit_roof::update_wall_height_if_needed | ✔ §7.3.2（内联 helper） |
| ui_edit_roof::impl$0::clone | G（derive Clone） |
| ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir | ✔ §5.2.1 |
| ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir::closure$0 | 内联入宿主 §5.2.1 |
| ui_edit_roof_ridge_dir::do_ui_edit_roof_ridge_dir | ✔ §5.2.2 |
| ui_edit_roof_ridge_dir::do_ui_edit_roof_ridge_dir::closure$0/$1 | 内联入宿主 §5.2.2 |
| ui_edit_roof_ridge_dir::switch_roof_ridge_direction | ✔ §5.2.3 |
| ui_edit_roof_ridge_dir::record_switch_roof_ridge_history | ✔ §5.2.4 |
| ui_edit_roof_ridge_dir::get_gizmo_info | ✔ §7.3.3（内联 helper） |
| ui_edit_roof_ridge_dir::impl$1::clone | G（derive Clone） |
| ui_edit_roof_ridge_length::ui_edit_roof_ridge_length | ✔ §5.3.1 |
| ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$0 | 内联入宿主 §5.3.1 |
| ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length | ✔ §5.3.2 |
| ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length::closure$0/$1/$2 | 内联入宿主 §5.3.2 |
| ui_edit_roof_ridge_length::create_ui_widget | ✔ §5.3.3 |
| ui_edit_roof_ridge_length::impl$2::clone | G（derive Clone） |
| ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused | ✔ §5.4 |
| ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0/$1 | 内联入宿主 §5.4 |
| ui_edit_roof_tip::ui_edit_roof_tip::closure$0 | X：无编译过程（见 §8.1） |
| visual::place_support_beams::place_roof_support_beams::closure$1 | X：无编译过程（见 §8.1） |
| visual::roof_snow::gen_roof_snow_mesh::closure$1/$2/$6 | X：无编译过程（见 §8.1） |
| highlight_roof::highlight_roof | ✔ §7.1 |
| impl$0::add_roof_systems | ✔ §6.1 |
| impl$0::add_roof_events_and_resources | ✔ §6.2 |
| gpu_flush_buffers | ✔ §7.2 |
| utils::components::impl$0::register_required_components | ✔ §6.3 |
| utils::components::impl$1::register_required_components | ✔ §6.3 |
| utils::assemble_rectangular_roof::assemble_rectangular_roof<closure_env$0> | ✔ 已有（§4.1/§算法清单；CPU 链已恢复） |
| utils::assemble_rectangular_roof::...::closure$0..$3 | 内联入宿主 |
| utils::assemble_circular_roof::assemble_circular_roof<closure_env$0> | ✔ 已有（§4.1） |
| utils::assemble_circular_roof::...::closure$0 | 内联入宿主 |

### 8.1 B 未发射的 3 个源文件主函数（复核点名项，实测不存在）

复核点名的 `ui_edit_roof_tip`、`place_roof_support_beams`、`gen_roof_snow_mesh` 在 B 中**未发射**，依据：
- PDB `*.pdb.json`：三者 `procedure_instances = 0`、`line_blocks = 0`、`inline_sites = 0`；骨架仅含 `closure$0/1/2/6` 的 `inlinee_declaration`（`instances=2`），标注 `Machine-code mapping: UNKNOWN (no compiled/inline host record)`。
- 二进制 `strings tiny-glade.exe`：`ui_edit_roof_tip`、`place_roof_support_beams`、`gen_roof_snow_mesh` 均 **0 命中**（对照组 `create_roof`/`switch_roof_type`/`place_roof_spires` 有命中）。
- Ghidra `find_functions`：`roof_tip`/`roof_support_beams`/`roof_snow` 查询均 **0 个 `system_roof::` 函数**（`roof_tip` 命中的 `calc_roof_tip_center_ls`/`roof_tip_offset_xz` 属 core，非本模块）。
- 结论：这三处主函数在 B 版链接期被消除/未生成独立代码；`ui_edit_roof_tip`（尖顶 UI 编辑）、`place_roof_support_beams`（支撑梁放置）、`gen_roof_snow_mesh`（积雪网格）**在 B 无对应发射实现**，其闭包仅存声明。**U**：A/C 版是否发射、或是否被其他函数内联替代。

> 说明：本清单为 `system_roof::` 命名空间全量；另有 `utils::assemble_*_roof` 的 CPU 铺瓦主体已在 `reconstruction/roof/` 完成，此处只标"已有"不重复恢复。
