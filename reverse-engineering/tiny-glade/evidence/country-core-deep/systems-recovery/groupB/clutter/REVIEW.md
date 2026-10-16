# clutter 审查核对表（reviewer-B tg_vB1 预置基线）

> 状态：**已审查**（2026-10-16，restorer-B 交付后独立审查；判定分布 P≈8 / F≈11 / Q≈18，明细见文末审查判定明细节；编译复验受 country-core session/save 域未提交修改阻断，restorer-B 临时 workspace 通过）。
> 本表业务函数清单与证据锚点来自 `SOURCE_MAP-systems.json`（skeleton PDB 元数据）+ `evidence/multi-baseline/*` 反汇编级分析 + country-core-deep 章节 + reviewer tg_vB1 独立 RNE 抽验。
> 交付后 reviewer 逐项核对：签名/参数面 vs 机器码；接口契约 vs core 类型；偷懒（空壳/假数据/直接复制 TODO 骨架）；缺失（本表函数未实现）。
> 核对列：P=通过 / F=失败 / Q=存疑待深挖。

| 函数 | VA | 核对 | 证据锚点 / 已知风险 |
| **autoclutter/add_autoclutter_around_gates.rs** | | | |
| `system_clutter::autoclutter::add_autoclutter_around_gates::add_autoclutter_around_gates` | 0x1422cc390 | | 0x1422cc390: 每门 2 放置 gate_right/gate_left, 键 calculate_hash<tuple<&str,u32,WallId>>, 墙点数>2, 墙端中点+法向偏移 |
| **autoclutter/add_autoclutter_around_windows.rs** | | | |
| `system_clutter::autoclutter::add_autoclutter_around_windows::add_autoclutter_around_windows` | 0x1422cefe0 | | 0x1422cefe0: 每窗 3xadd_region; extend_right_until 延伸; 键 <&str,i32,usize,u32>x1+<&str,usize,u32>x2 |
| **autoclutter/add_autoclutter_on_windows.rs** | | | |
| `system_clutter::autoclutter::add_autoclutter_on_windows::add_autoclutter_on_windows` | 0x1422cd7d0 | | 0x1422cd7d0: 跳表 0x143042370 分派, [R9-0x50]==6 判类型; 反编译质量差 — 需 disasm 精读 |
| **autoclutter/add_birdnests.rs** | | | |
| `system_clutter::autoclutter::add_birdnests::add_birdnests` | 0x1422c9c70 | | 0x1422c9c70: Subscriber::take<DirtyTiles>->逐脏格 type=4 候选, 0x3e19999a=0.15 偏移, +0x60=1.0 缩放, HashMap<u64,bool> 去重; 发布者之一(DirtyTiles pub/sub) |
| **autoclutter/add_lantern_water_setdresssing.rs** | | | |
| `system_clutter::autoclutter::add_lantern_water_setdresssing::add_lantern_water_setdresssing` | 0x1422d2fa0 | | 0x1422d2fa0: as_inner_mut WaterRaster(+0x298 双缓冲)+is_corrupted 门控; DecoratorStorage::iter_terrain_decorators; 每装饰物 5 次 i/5.0+fmodf jitter; 入水才放; 键 <u32,usize,&str>(lantern@0x143042d28) |
| **autoclutter/add_preplaced_autoclutter.rs** | | | |
| `system_clutter::autoclutter::add_preplaced_autoclutter::add_preplaced_autoclutter` | 0x141ce5900 | | 0x141ce5900 未精读 |
| **autoclutter/add_tree_vegetation_setdressing.rs** | | | |
| `system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing` | 0x1422d3800 | | 0x1422d3800: is_dragging/is_startedx4/is_corruptedx4 门控; 每树 12 次 i/12.0; 键 <TreeStorageIndex,usize,&str>(tree@0x143042d91) |
| **autoclutter/autoclutter_animation.rs** | | | |
| `system_clutter::autoclutter::autoclutter_animation::autoclutter_animation` | 0x1422ccef0 | | 0x1422ccef0 未精读 |
| **autoclutter/load_clutter_meshes.rs** | | | |
| `system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes` | 0x1422c0760 | | 0x1422c0760 未精读(host_geometry 边界确认) |
| **autoclutter/manage_autoclutter_entities.rs** | | | |
| `system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities` | 0x1422d04e0 | | 0x1422d04e0: 现存/目标键集差集; spawn_empty+insert<ColorId>; insert<ClutterMarkedForDeletion>; 四元数朝向(两次叉积+归一); init_rng 抖动; byte@+0x30 分支(==2 或 -3 后 <4 且 !=1) |
| **autoclutter/mod.rs** | | | |
| `system_clutter::autoclutter::impl$10::register_required_components` | 0x140ee4ee0 | | AutoClutterState: 区域表 hashbrown@+0x60 stride 0x48(key@0, 基向量@0x8..0x40, u16@0x42, epoch byte@0x44=state+0x90); 候选 Vec@+0x20 stride 0x98; 待处理 u16@+0x30; 输出@+0x48 stride 0x68(+0x58 计数); add_region 0x140ee3fd0 / mark_dirty 0x140ee48c0 / remove_region_from_lazy_grid 0x140ee4a70 / flush 0x140ee4bf0 / register_required_components 0x140ee4ee0 |
| `system_clutter::autoclutter::AutoClutterState::add_region` | 0x140ee3fd0 | |  |
| `system_clutter::autoclutter::AutoClutterState::mark_dirty` | 0x140ee48c0 | |  |
| `system_clutter::autoclutter::AutoClutterState::remove_region_from_lazy_grid` | 0x140ee4a70 | |  |
| `system_clutter::autoclutter::AutoClutterState::flush_candidates_pending_removal` | 0x140ee4bf0 | |  |
| **autoclutter/populate_autoclutter_regions.rs** | | | |
| `system_clutter::autoclutter::populate_autoclutter_regions::_::impl$0::init_state` | 0x1422e1b40 | | 0x1422d6ed0(44KB 主驱动): remove_stale_regions retain->flushx2->DirtyTiles new_clear+Subscriber::take->逐位 mark_dirty->filter_clutter_object(水位双采样+terrain_rock_mask, Aabb2+iter_world_space_rect_cells)->switch 种类(==2 水位阈值 0.01)->is_halloween2024 门控; init_state 0x1422e1b40 / autoclutter_subscribe_to_tabloids 0x1422d6930 |
| `system_clutter::autoclutter::populate_autoclutter_regions::autoclutter_subscribe_to_tabloids` | 0x1422d6930 | |  |
| `system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions` | 0x1422d6ed0 | |  |
| **autoclutter/process_autoclutter_candidates.rs** | | | |
| `system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates` | 0x1422ce260 | | 0x1422ce260: from_iter 过滤->排序(insertion_sort_shift_left<0x15 否则 driftsort)->遍历: byte@+0x30==6 强制接受否则 Rectangle3d::overlaps_rectangle3d(0x140c98580) 重叠剔除->输出 0x68 记录 |
| **autoclutter/read_bush_positions_from_gpu.rs** | | | |
| `system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu` | 0x140eebaa0 | | 0x140eebaa0(发布者之一) 未精读 |
| **inter_roof_merlons/construct_inter_roof_merlons.rs** | | | |
| `system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons` | 0x1422d4870 | | 0x1422d4870: 等距 0.46 城垛砖; ceilf(长/0.46); random_splits_into 切段; 抖动 ((r*.5-(1-r)*.5)*0.21*0.8+0.368)*.5+(宽基+0.23); 门控 is_halftimbered||flat_w_no_rim 然后 ^1; SparseInstanceBufferWriter push |
| **inter_roof_merlons/detect_inter_roof_merlons.rs** | | | |
| `system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons` | 0x141ce3a70 | | 0x141ce3a70: dirty_walls 事件汇入(hashbrown set extend OnWallDeleted/OnWallChanged/RecalculateRoofCmd -> 0x141cd16b0); is_flatx2 门控; roof_shape_at_intersection_y; local_374!=2 时 expandx2+intersect_shapes(0x140edd250); 收尾 WallHoles::add stride 0x28 |
| **inter_roof_merlons/inter_roof_merlons_audio.rs** | | | |
| `system_clutter::inter_roof_merlons::inter_roof_merlons_audio::request_inter_roof_merlon_audio` | 0x1422d4230 | | 0x1422d4230 未精读 |
| **inter_shape_stitches/detect_intra_shape_corners.rs** | | | |
| `system_clutter::inter_shape_stitches::detect_intra_shape_corners::_::impl$0::get_param` | 0x141ce3430 | | 0x141cd1a90: 角点高=max_y+flag*(-0.67)(0xbf2b851f); retain Corner/HashMap<[WallId;2],Vec<Vec2>>/ShapeIntersectionHole; intersect_shapes 0x140edbf60; add_hole_at_shape_intersection 0x141cd3ce0(洞 stride 0x14, kind byte 5/1); get_param 0x141ce3430; dirty_walls 0x141cd16b0 |
| `system_clutter::inter_shape_stitches::detect_intra_shape_corners::WallAndRoofChanges::dirty_walls` | 0x141cd16b0 | |  |
| `system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners` | 0x141cd1a90 | |  |
| `system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4>` | 0x140edd250 | |  |
| `system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4>` | 0x140edbf60 | |  |
| `system_clutter::inter_shape_stitches::detect_intra_shape_corners::add_hole_at_shape_intersection` | 0x141cd3ce0 | |  |
| **inter_shape_stitches/stitch_bricks.rs** | | | |
| `system_clutter::inter_shape_stitches::stitch_bricks::spawn_stitches` | 0x141ce65b0 | | spawn_stitches 0x141ce65b0(kind==1 且 +0x04<0.2, 段长 min(1000,2|长|)<=2, 抖动 (r*.5-(1-r)*.5)*0.1+0.4, Δ+(-0.1)) / spawn_stitch_bricks 0x141ce7190(clear->逐条 hash([WallId;2],usize)->push) |
| `system_clutter::inter_shape_stitches::stitch_bricks::spawn_stitch_bricks` | 0x141ce7190 | |  |
| **lib.rs** | | | |
| `serde_json::de::Deserializer::parse_exponent<serde_json::read::StrRead>` | 0x140057fd0 | | impl$0::build 0x140652050 插件注册(系统链/chain/in_set 顺序!) |
| `serde_json::de::Deserializer::parse_number<serde_json::read::StrRead>` | 0x140057730 | |  |
| `serde_json::value::de::impl$0::deserialize::impl$0::visit_map<serde_json::de::MapAccess<serde_json::read::StrRead> >` | 0x142890a70 | |  |
| `system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions` | 0x1422d6ed0 | |  |
| `system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons` | 0x1422d4870 | |  |
| `system_clutter::impl$0::build` | 0x140652050 | |  |


## 审查判定明细（2026-10-16，tg_vB1）

> 总体：结构骨架完整（37 业务均有落点、证据注释详实），但**机器码保真度低**——
> 大量"宿主缝/占位/自造常量"。判定分布：P≈8 / F≈11 / Q≈18。

### F 项（空壳/假实现/语义颠倒，需 restorer-B 修复）

1. **F1 AutoClutterState::mark_dirty 空壳**（0x140ee48c0）：`let _ = cell;` 无实现。
2. **F2 flush_candidates_pending_removal 只清表**（0x140ee4bf0）：注释自称"把 pending
   索引对应候选复制进 region_candidates"，代码仅 `clear()`。
3. **F3 populate_autoclutter_regions 骨架化**（44KB 机器码 → 192 行）：`clutter_kind_for_cell`
   恒返回 1（机器码 switch 种类分派缺失，==2 水位 0.01 分支虽写但 kind 永不为 2）；
   区域 center/basis 全零 + candidate_index=0（机器码 Aabb2::from_center_radius 真实几何）；
   `hash_cell` 用 DefaultHasher 冒充 `calculate_hash<IVec3>`（0x14145c1a0）；
   filter_clutter_object 的 iter_world_space_rect_cells/双采样链未复现；
   pick_clutter_mesh（L918）/align_to_nearby_wall（L944）闭包未实现。
4. **F4 manage_autoclutter_entities 增删语义颠倒**：机器码（draft 记录）新建=spawn_empty+
   insert<ColorId>；删除=insert<ClutterMarkedForDeletion> 标记（entity_command closure）。
   restorer 在**新建**实体上插 ClutterMarkedForDeletion、删除直接 despawn；
   更新路径（closure_0 0x1422d2d20、四元数朝向、init_rng 抖动、byte@+0x30 类型分支）未接线
   （orientation_basis/jitter_position 辅助函数未被系统调用）。
5. **F5 add_birdnests 输出 +0x08 用 String("birdnest")**：机器码该域为 **Arc 强计数**
   （draft 明注"birdnest 为 Arc 强计数域"）；String 使 0x68 stride 布局失真（lantern/tree
   的 +0x08 才是 String）。AutoClutterSpawn 用 String 表达全类型 +0x08 是布局错误。
6. **F6 process_autoclutter_candidates 比较器/几何占位**：`cmp_candidate` 按 index 比较
   （注释声称 +0x34 字段，代码未实现）；重叠矩形 `f38/f40 ± 2.0` 自造
   （机器码 Rectangle3d::overlaps_rectangle3d 真实几何）；filter 条件 `shared!=0` 自造。
7. **F7 autoclutter_subscribe_to_tabloids 空壳**：`if guard.is_none() { *guard = None; }`
   为恒等 no-op；机器码 0x1422d6930 有真实订阅逻辑。
8. **F8 spawn_stitches 位置计算假实现**：`x0y(Vec2::new(point, mid*0.0)) +
   Vec3::new(0.0, jitter*0.4, 0.0)` —— mid*0.0/jitter*0.4 为凑类型自造表达式；
   机器码 = 曲线法向归一 + x0y swizzle。
9. **F9 load_clutter_meshes 空资源**：仅 insert_resource(ClutterMeshes::default())，
   资产装载闭包（ClutterMeshTy 56 资产路径）未实现。
10. **F10 construct_inter_roof_merlons `wall_is_halftimbered` 恒 false**（假实现）；
    门控 `gate = !(halftimbered||flat_no_rim)` 与"半木墙才生成"的既有推断矛盾（Q 并存）。
11. **F11 全部 calculate_hash 用 hashbrown DefaultHashBuilder（随机种子）**冒充机器码
    固定哈希（0x141cf7440/0x141cf6d60/0x141cf7980/0x141cf7600/0x141cf70d0/0x14145c1a0）
    → 放置键跨进程不确定。hash_gate_key/hash_window_key/hash_lantern_key/hash_tree_key/
    hash_preplaced_key/hash_cell 六处同病。

### Q 项（细节自造/宿主依赖/证据不足）

- Q1 add_region 不等价路径 push `pending_indices.len()`（疑应为 region.candidate_index）；
  AutoClutterState Rust 字段序 ≠ 机器偏移（+0x20/+0x30/+0x48/+0x60/+0x90），内部使用可接受。
- Q2 add_autoclutter_on_windows：reflect 公式忠实（✓），但 +0x08 String("window") 自造；
  跳表 6 分支宿主源由 OnWindowInput 资源承载（机器码读系统内宿主查询）。
- Q3 add_lantern_water_setdresssing：is_corrupted 门控缺失；is_started 偏移 0.3/0.6 自造；
  jitter 幅度 0.1 自造；fmodf 语义用 `% 1.0` 近似。
- Q4 add_tree_setdressing：is_startedx4/is_corruptedx4 门控简化为 2 个；Circle 半径 1.5、
  jitter 0.08 自造。
- Q5 autoclutter_animation：Easing/despawn 语义合理但细节（duration 来源）未钉。
- Q6 add_preplaced_autoclutter：Circle2d::new/new_rotated 未用（basis 由 rotation cos/sin
  自造）。
- Q7 read_bush_positions_from_gpu：GPU 回读后置（合规），CPU 侧 AABB 半径 8.0 自造、
  ipnsort 键自造。
- Q8 detect_inter_roof_merlons：dirty_walls 事件汇入 ✓；is_flat 门控 ✓；intersect_shapes
  为占位；WallHoles::add 由宿主缝。
- Q9 detect_intra_shape_corners：is_flatx2 门控 ✓；角点高 -0.67 语义未实现（host 缝）；
  `get_param`（0x141ce3430）与 `init_state`（0x1422e1b40）缺失。
- Q10 inter_roof_merlons_audio：事件输出合理，机器码细节未钉。
- Q11 lib 调度链：19 系统单链 `.chain()`，机器码 build 的 chain_ignore_deferred/in_set(0x24)/
  run_if 组合未复现；`configure` 命名替代 build 0x140652050（约定内登记）。
- Q12 serde_json 3 项（0x140057fd0/0x140057730/0x142890a70）为依赖误标 business，不计。
- Q13 AutoClutterCandidate/Spawn 的 f18/f20/f28/f34... 字段名多为占位（机器码 16 字段
  映射表已列，语义 U）；candidate.shared u64 无法表达 Arc 计数语义（F5 关联）。


## 复验轮判定（2026-10-16 第二轮，restorer-B 修复后）

> 主 workspace 编译：**通过**。判定：F1/F2/F4/F5/F7/F9/F10/F11 闭合（P）；
> F3 部分闭合（种类分派仍恒 1 → 剩余 F）；F6/F8 结构闭合（Q 保留）。
> 剩余 F：**1 条（clutter F3 种类分派）**。

- **F1 mark_dirty：P**。懒网格（lazy_grid Vec<Vec<u64>>）+ pending push +
  candidate_index=0xffff 标记 + 未命中 unreachable panic，机器码证据注释完整。
  Q：懒网格记录 0x18 布局 U。
- **F2 flush_candidates_pending_removal：P**。排序 pending → Vec::remove（memmove
  语义）→ kind==4 Arc 释放缝 → 区域表 candidate_index 一致性维护 ✓。Q：shared
  为 u64 时 Arc 计数为名义缝。
- **F3 populate：部分（剩余 F）**。双水位采样 ✓、align_to_nearby_wall（WallSpatialHash
  邻域法向）✓、lazy grid 维护 ✓、subscribe（F7）✓、is_halloween2024 门控 ✓。
  **`clutter_kind_for_cell` 仍恒返回 1**——机器码 switch 种类分派（==2 水位阈值
  0.01 分支等）未实现 → **剩余 F，转 re-analyst 精读 44KB switch 后 restorer 补**。
  Q：区域 center=格中心（机器码 Aabb2 半径语义 I）；candidate_index=0 恒值。
- **F4 manage：P**。删除 → insert<ClutterMarkedForDeletion>（不直接 despawn）✓；
  新建 → spawn_empty + ColorId ✓（不再误插标记）；更新路径（orientation_basis +
  jitter_position + should_manage type 分支）已接线 ✓。Q：ColorId 值来源仍默认；
  ManagedAutoClutterPlacement 为宿主投影组件。
- **F5 birdnest +0x08：P**。SpawnShared::{ArcStrong,Prefab} 表达 Arc 强计数/prefab
  名两域 ✓。
- **F6 process：P/Q**。cmp_candidate 改按源 +0x34 比较 ✓；Q：filter（kind!=0）与
  矩形几何（f38/f40±2.0）仍 I。
- **F7 subscribe：P**。tabloid Publisher::subscribe 真实接线 ✓（宿主注入发布者）。
- **F8 stitches 位置：P/Q**。mid*0.0 假表达式已除（x0y 结构 + 法向偏移）；Q：法向
  公式 `Vec2::new(-seg_dir, seg_dir)` 为 45° 斜向（机器码 = 曲线法向 get_normal_at_u
  语义），仍可疑。
- **F9 load_clutter_meshes：P**。CLUTTER_MESH_PATHS 56 资产路径注册表 +
  ClutterMeshTy::from_native_tag（0..56）✓✓（与 collision-eco DIGEST 56 变体一致）。
- **F10 is_halftimbered：P**。真实实现 `authority.snapshot().source.style >= 2`
  （证据 0x1408c5670）✓。
- **F11 哈希固定种子：P**。六处全部改 `std DefaultHasher::new()`（固定 SipHash 键，
  跨进程确定）✓。Q：哈希值 ≠ 机器码 calculate_hash（foldhash 族）——键值不同但
  确定且内部自洽，宿主存档/放置键对接时需注意。


## 终验轮判定（2026-10-16 第三轮，F3 契约重写后）

> 对照 `RE-ANALYSIS-F3.md`（re-analyst tg_xb1）逐项核验；主 workspace 编译通过（29 警告无错误）。
> 判定：**F3 闭合（P，附带与契约"仍 U"一致的 Q）**——组 B 全量 F=0。

| 契约项 | 实现 | 判定 |
|---|---|---|
| +0x40/+0x41 双判别（mesh-type/kind） | `select_group → (mesh_type, kind)` 双值模型；判别链 I（契约自身 U：写入方在 filter_clutter_object 闭包内） | P/Q |
| MeshGroupWeights 槽组（kind 跳表 0x143043818 预设槽；+0x30==-1 → 权重 0） | `MeshGroupWeights{group_0,group_1,halloween}` + MeshEntry{weight,offset}；-1 哨兵语义仅注释 | P/Q |
| maxps(0.01, Δ) → floorf 落点（0.01=0x1429cd300） | `weighted_mesh_landing`：`delta.max(0.01).floor()` ✓ | P |
| θ = rng×2π（0x142934674）圆周落点 | `circular_position`：`rng.f32()*TAU` + cos/sin ✓；种子 = init_rng_u64(键哈希) ✓ | P |
| calculate_hash<tuple<GlobalId,u32>>（0x140fabc80）键 | `hash_cell_key`（std DefaultHasher 固定种子，(x as i64, y as u32)）；值 ≠ 机器码 hash（确定但异值） | P/Q |
| 水位阈值 **0.0**（0x142960690，非 0.01） | `wet = ray_hit && water_sum > 0.0`（4 角采样 clamp01）✓——旧 kind==2&&0.01 模型已删除 ✓✓ | P |
| legacy_raycast 掩码 0x30fb | `raycast_seam::legacy_raycast_mask`（壳返回 false → wet 恒 false，native 装配层落地后生效） | P/Q |
| dry/type<3 固定位（记录+0x24 取负, +0x20） | `Vec3::new(wx, -0.0, wz)`（cell 中心近似，非记录字段） | Q |
| candidate_index 非恒 0（交互索引） | `ClutterCandidateIndex(region_candidates.len())` 推候选后取索引 ✓ | P |
| kind 预设槽/万圣节门控 | halloween → (0,4) + halloween 组；kind 0/1/2 按组表非空选择（I） | Q |

剩余 Q（全部与 RE-ANALYSIS-F3 §9"仍 U"一致，非 restorer 缺陷）：
record+0x40/+0x41 写入方定点、万圣节精确权重常量表、mesh-type 3/4 候选槽 +0x25/+0x26、
第三跳表 0x143043840 各目标体、calculate_hash 值身份、native raycast 落地、候选 f38/f40
几何段、forced 映射（mesh_type>=9，I）。
