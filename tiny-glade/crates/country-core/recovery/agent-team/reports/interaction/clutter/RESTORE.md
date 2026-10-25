# RESTORE — interaction/clutter（restorer-interaction-clutter）

实现者: restorer-interaction-clutter | 日期: 2026-10-17（REVIEW 复核后 v2）
范围: `crates/systems/clutter`（package `system-clutter`）全部业务函数 fresh 反编译核实并重建。
证据索引（本次 fresh 反编译，`ghidra --worker shared`，2026-10-17）落盘于
`crates/country-core/recovery/agent-team/work/interaction-clutter/decomp/*.txt`（30 文件）：
`result{1..5}-*.json` 为各批完整报告。地址口径 = 骨架「编译实例 RVAs」+ 0x140000000。
复核：`REVIEW.md`（reviewer-interaction-clutter 另行 fresh 反编译 9 函数逐字节比对，
证据真实；分歧在解读/落码层面）。

## 评审 FIX 落实（v2）

| FIX# | 函数 | 落实 |
|---|---|---|
| 1 | flush_candidates_pending_removal | **已改降序** `sort_by(\|a,b\| b.cmp(a))`（高索引先删，swap_remove 不破坏低索引；原升序会删错/panic）；RESTORE.md 同步改「降序」 |
| 2 | add_lantern_water_setdresssing | **已补入水门** `0.0<采样`（WorldRaster 采样）；补 1/SQRT 归一化应用；缩放改 `fVar23*(r*1.2+(1-r)*0.8)`；is_started 三路偏移（精确值 U 登记） |
| 3 | add_birdnests | **已删资源位集回退**（take 无结果即空位集）；遍历源仍为位集派生（宿主记录 Vec U）；closure_0 门 U 登记 |
| 4 | mark_dirty | **已删 `is_empty→continue`**（表空/未命中均走 `get_mut().expect()` → unwrap_failed panic） |
| 5 | add_region/flush | kind==4 Arc 递减两处**明确登记 U 缝**（原 LOCK 递减/drop_slow 真实副作用，Arc payload 合同 U，登记不执行） |
| 6 | process | closure_env_0 过滤谓词 / closure_env_1 排序键 / case5 key 语义**均标 U**（现为未取证猜测，不判等） |
| 7 | subscribe | 2/13 订阅完成，8×Raycast+3×WorldRaster **如实登记宿主缝**（不计完成） |
| 8 | set_current_tick/reset | 文档登记 PDB 系统签名 `void(*)(Res<GameTime>,ResMut<AutoClutterState>,ResMut<InterShapeBrickStitches>)`（L3689/L3931）——**未实现-需重做** |
| 9 | remove_region_from_lazy_grid | 文档登记公共方法桩 + 闭包删除键（GlobalId）U |
| 10 | detect_inter_roof_merlons | **已接 dirty_walls**（本系统自带三 EventReader 内联事件链；原 `dirty_walls_ev` no-op 桩删除） |
| — | auto_clutter_edit.rs | **已删除**（history_record/replay 均属 country-core，重复定义） |

## 判定口径

- **判等**: 现有 Rust 体与 fresh 反编译逐语句等价。
- **重建**: 本会话按 fresh 反编译重写函数体（真控制流）。
- **glue-legitimate**: derive/Bevy 注册/SystemParam 胶水，不重建。
- **未实现(分歧点)**: 现有体为近似/桩，与 fresh 反编译有明确分歧；列出分歧点。

## 逐函数对账

### autoclutter 核心状态与放置

| 原函数（地址） | 判定 | 说明 |
|---|---|---|
| AutoClutterState::add_region（0x140ee3fd0） | **重建** | 真控制流 = `hash_one<&GlobalId>` + 区域表 entry 语义：命中等价→刷 epoch+flush+判别==4 drop Arc；命中不等价→remove 旧区域（iter_world_space_rect_cells<remove_region_from_lazy_grid>）+add 新区域+旧候选索引<0xfffe push pending+覆盖+flush；未命中→reserve_rehash+add+插入+flush。`is_equivalent` 逐 7×f32（IEEE 序 NaN 不相等）+i32+2×u8，**不含** candidate_index/epoch。旧实现比较 center/basis/candidate/epoch（错）、用 entry() 借用冲突规避（现 get/insert 等价）。 |
| AutoClutterState::mark_dirty（0x140ee48c0） | **重建** | 真：懒网格越界 → **panic_bounds_check**（非静默返回）；cell 内 GlobalId → hash 查区域表，候选索引<0xfffe push pending + 写 0xffff；未命中 unwrap_failed。旧实现 `get().else{return}` 静默（错）。 |
| AutoClutterState::remove_region_from_lazy_grid（0x140ee4a70） | **重建** | 真：x0y+Rectangle2d::new/aabb → iter_world_space_rect_cells 几何移除。本重建以区域几何清格；**分歧**：懒网格格内 GlobalId 精确键来源（add_region_to_lazy_grid 闭包）未逐行还原（U）。 |
| AutoClutterState::flush_candidates_pending_removal（0x140ee4bf0） | **重建** | 真：pending **降序**稳定排序（高索引先删，swap_remove 才不破坏低索引；<0x15 insertion 否则 driftsort，closure$0）→ 逐条 **`swap_remove`**（memmove 末元素进槽，非 remove）→ 判别==4 Arc 释放 → 换入候选 key（@+0x88）查区域表，断言条目候选索引==Some(新长度) 后写新索引（否则 assert_failed/unwrap_failed）→ +0x40=0。旧实现用 `remove`+整体索引前移（错）。**REVIEW FIX#1 已改降序**（评审独立反编译逐行判向）。 |
| AutoClutterState::set_current_tick（L164） | **重建** | epoch 逐帧推进（remove_stale_regions 判定）。 |
| AutoClutterState::reset_autoclutter_for_new_frame（L531） | 判等 | 清空 spawn_candidates。 |
| AutoClutterRegion::is_equivalent（L408） | **重建** | 移出 candidate_index/epoch；float 用 IEEE 序相等（NaN 不相等）。 |
| ClutterCandidateIndex::{new,get,is_none} | **重建** | 哨兵 = 0xffff；`<0xfffe` 才有效（O：add_region/mark_dirty/flush 判据）。 |
| AutoClutterState::register_required_components（0x140ee4ee0） | glue-legitimate | bevy 组件注册胶水。 |
| AutoClutterState::remove_stale_regions（L233） | 判等 | `HashMap::retain`（epoch != state+0x90 删除），populate 内联。 |

### autoclutter 四段流水线

| 原函数（地址） | 判定 | 说明 |
|---|---|---|
| populate_autoclutter_regions（0x1422d6ed0） | **未实现** | 44KB 主体 Ghidra 碎片化（16 段）。fresh 反编译证据：retain→flush×2→Subscriber::take→0x200B 位集→逐位 mark_dirty；filter_clutter_object 内联（RaycastWorld::legacy_raycast 掩码 0x30fb、水位阈值 **0.0**、28 项 mesh-type 跳表 0x14304382c、5 项 kind 跳表 0x143043818）。**分歧**：现 `process_dirty_cell` 为 F3 契约近似，未复现跳表分派/水位阈值 0.0。 |
| autoclutter_subscribe_to_tabloids（0x1422d6930） | **重建**（部分） | 真：13 订阅入句柄 Vec——8×RaycastWorld::subscribe_to_changes、2×Publisher<DirtyTiles>::subscribe、3×WorldRaster::subscribe<512>（经 as_inner_mut）。本重建完成 DirtyTiles×2；**分歧**：RaycastWorld×8/WorldRaster×3 为宿主边界缝（U）。 |
| process_autoclutter_candidates（0x1422ce260） | **重建** | 真：enumerate+filter_map（u16 下标 @+0xc）→ 稳定排序（closure$1）→ 遍历：源 `byte@+0x30==6` 强制接受，否则对已接受缓冲（stride 0x20，源+0x68 Rectangle3d）贪心重叠剔除（Rectangle3d::overlaps_rectangle3d）→ switch 判别 3/4/5（case4 Arc LOCK++、case5 裸指针）→ 输出 0x68 逐字段复制。旧实现为近似。 |
| manage_autoclutter_entities（0x1422d04e0） | **未实现** | 8178B 复杂宿主查询。**分歧**：现集合差集方向对，但宿主查询 +0xf8..+0x120 逐偏移、四元数朝向、ColorId 来源、update_shutter、entity 命令细节未复现。 |
| read_bush_positions_from_gpu（0x140eebaa0） | **未实现** | GPU 回读。**分歧**：真 = FrozenMap<SsboId,Box<BufferState>>::get + 92B 类型断言 + 每灌木 Aabb2 采样 + 每单元排序（ipnsort）+ cell 哈希变更检测 → 位集 → publish。现为 CPU 桩。 |
| load_clutter_meshes（0x1422c0760） | **未实现** | 真 = ~50 个内联 closure_0 逐个资产加载 + from_iter<RangeInclusive<i32>> + insert_resource。**分歧**：50 个网格名/资产 id 清单未取（现 56 路径表为 I 推断）。 |
| autoclutter_animation（0x1422ccef0） | **未实现** | 真：宿主查询双分支 + 动画状态 +8/+0xc/+0x10/+0x14/+0x2c + EasingFunction::apply + 四元数 `(eased*dir*60-(1-eased)*dir*50)*0.017453292*0.5` + 完成 despawn。**分歧**：现为 age/duration 近似，未复现 Easing 曲线/四元数/despawn 判定。 |

### autoclutter 各放置族

| 原函数（地址） | 判定 | 说明 |
|---|---|---|
| add_autoclutter_around_gates（0x1422cc390） | **未实现** | 真：墙宿主查询 + 墙点数>2 + gate_right（0x143042260,len10）/gate_left（len9）**2 个 add_region**，位置=墙端+2.0 偏移，四元数常量 2.0，候选索引初值 0xffff。**分歧**：现 GateHost 组件近似，prefab 几何/四元数未复现。 |
| add_autoclutter_around_windows（0x1422cefe0） | **未实现** | 真：3 个 add_region（键 tuple<&str,i32,usize,u32>×1 + <&str,usize,u32>×2），偏移常量 4.72/0.28/-0.28/1.5/2.0。**分歧**：现 3 键近似、prefab 名/几何未逐一对。 |
| add_autoclutter_on_windows（0x1422cd7d0） | **未实现** | **Ghidra 反编译失败**（bad instruction data，边界 364）；需 radare2 全函数反汇编。现实现为跳表语义近似。 |
| add_birdnests（0x1422c9c70） | **重建** | 真：Subscriber::take→0x200B 复制→new_clear→逐脏格 processed 集（HashMap<u64,bool>）→closure_0（几何合法性，U）→输出 type 4/Arc++/位置 +0.15/scale 1.0。旧实现用资源位集非 take 结果（改）、位置 y=0.15（改）。 |
| add_lantern_water_setdresssing（0x1422d2fa0） | **重建** | 真：每装饰物 init_rng + 内层 5×（i/5.0）：f32→`*0.1+i/5.0`→fmodf(.,1.0) 负值+1.0（rem_euclid）→get_pos_at_u→水位采样→`0.0<采样` 才放→1/SQRT 归一化→calculate_hash<tuple<u32,usize,&str>>("lantern")→is_started 选偏移→输出 type 5。旧实现近拟但 mod/fmodf/is_started 偏移不精确。 |
| add_preplaced_autoclutter（0x141ce5900） | **未实现** | 真：两 storage 长度变更检测 + FramesSinceNewSession 标志；变化才遍历预置条目（stride 0x1c）add_region。**分歧**：现无条件遍历，变更检测缺失。 |
| add_tree_setdressing（0x1422d3800） | **未实现** | 真：is_dragging/is_started×4/is_corrupted×4 门控 + 内层 12×（i/12.0）圆周 jitter + 水位入水 + calculate_hash<tuple<TreeStorageIndex,usize,&str>>("tree")。**分歧**：现近似，门控序列/12 次序列化/树索引来源未复现。 |

### 屋顶雉堞 / 形状接缝

| 原函数（地址） | 判定 | 说明 |
|---|---|---|
| WallAndRoofChanges::dirty_walls（0x141cd16b0） | **重建** | 真：三事件链式 map（OnWallDeleted/OnWallChanged/RecalculateRoofCmd→WallId）→ `hashbrown::set::extend` 汇成**新** HashSet 写回。旧实现清空复用（改）。 |
| detect_inter_roof_merlons（0x141ce3a70） | **未实现** | 真：dirty_walls + 两 retain + 主循环 roof_entity 脏位集 + is_flat×2 + roof_shape_at_intersection_y（!=2 走 expand×2 + intersect_shapes<closure_env$4>）+ WallHoles::add。**分歧**：现近似。 |
| construct_inter_roof_merlons（0x1422d4870） | **未实现** | 真：宿主几何 Vec（stride 0x38，+0x10 判别 0=Circle2d/非0=Rectangle2d）+ BrickTransformSsbo 缓冲 + 城垛门控（is_halftimbered 或 is_flat_w_no_rim）+ ceilf(长/0.46) + random_splits_into + f32 抖动公式 + SparseInstanceBufferWriter::push。**分歧**：现近似；BrickTransformSsbo 字段为 core 卡点。 |
| request_inter_roof_merlon_audio（0x1422d4230） | **未实现** | 真：`*(int)*RCX==0xd` 早退 + CursorTerrainRaycast::position + 写 indexmap Bucket<(DecoratorId,DecoratorId),AlignKind>（kind 8/9）+ spec_extend<InterRoofMerlonsDescription>。**分歧**：现为距离<1.0 事件近似（结构完全不同）。 |
| detect_intra_shape_corners（0x141cd1a90） | **未实现** | 真：三 retain + 脏墙对（hash 表 stride 0x2a8）+ Roof::ty/max_y + 角点高 max_y-0.67 + expand×2/contains_shape/perimeter + intersect_shapes<closure_env$4>（0x140edbf60）+ WallHoles::add。**分歧**：现近似。 |
| add_hole_at_shape_intersection（0x141cd3ce0） | **未实现** | 真：两点 → WallSpaceCoord::from_world_space×2 + wall_space + subtract + project_clamp；fVar5<0 时墙厚（+0x30）折算；洞记录 [x,y,height,Δ]+kind（5 墙下/1 墙上）。**分歧**：现桩直通 intersect_shapes（结构不符）。 |
| spawn_stitches（0x141ce65b0） | **未实现** | 真：遍历 Vec（stride 0x40）+0x00 种类==1、+0x04<0.2、u=trunc(min(1000,2|长|))、2<u 跳过 + random_splits + 四元数/法向/抖动 `(r*0.5-(1-r)*0.5)*0.1+0.4` + Δ-0.1。**分歧**：现近似，四元数数学未逐语句复现。 |
| spawn_stitch_bricks（0x141ce7190） | **未实现** | 真：FrozenMap<SsboId> get（断言 0x3a）+ SparseInstanceBufferWriter::clear + 逐条 resources::render::from/calculate_hash<([WallId;2],usize)>/instanced_wall::from/push。**分歧**：现 CPU 近似，SSBO 烘焙未接。 |

### 插件与归属

| 原函数（地址） | 判定 | 说明 |
|---|---|---|
| impl Plugin::build（0x140652050） | glue-legitimate | bevy 注册胶水（add_systems/insert_resource/register_event）。**注**：精确链序被内联遮蔽（U），现 lib.rs `configure` 为注册近似。 |
| history_record_auto_clutter（0x1409a9160） | **归属** | 属 `country_core::systems::ui_clutter::defered_autoclutter_history`（completeness-audit 已判 implemented_verified）；system-clutter `auto_clutter_edit.rs` 为重复定义，建议移除。 |
| replay_auto_clutter（0x140aa4a60） | **归属** | 属 `country_core::systems::history::replay_auto_clutter`；同上建议移除。 |

## 类型合同分歧（U，登记）

- `AutoClutterState` 二进制 Vec 字段序 =（懒网格 data/len、候选 data/len、pending cap/data/len、
  输出 data/cap/len）——与安全 Rust `Vec`(ptr,cap,len) 不一致；本重建以语义等价字段承载。
- `AutoClutterRegion` 载荷 0x40B：kind@+0、u64@+8/+0x10、7×f32@+0x18..+0x30、i32@+0x34、
  u8@+0x38/+0x39、u16 候选索引@+0x3a（哨兵 0xffff）、u8 epoch@+0x3c；本重建以
  key/center/basis 近似承载（偏移注释）。
- `AutoClutterSpawn` 0x68 布局：+0x00 kind u64、+0x08 shared、+0x10..+0x28 u64、+0x30 marker、
  +0x34 u32、+0x38/+0x40/+0x48 u64、+0x50/+0x54/+0x58/+0x5c u32、+0x60 scale；本重建以
  kind u8/shared enum/position Vec3 等安全字段承载（各 writer 注释）。
- `AutoClutterCandidate` 0x98 布局：+0x30 byte（==6 强制）、+0x68..+0x88 Rectangle3d、
  +0x88 region_key；本重建已按此加 rect/region_key 字段。

## 编译与测试

- `cargo check -p system-clutter`: **exit 0**（29 警告，含各 writer 死代码/未用导入——既有
  近似残留，未删除避免破坏宿主）。未被其他组编译错误阻断。
- `cargo test -p system-clutter`: **0 测试**（本 crate 无测试），exit 0。

## 留给 reviewer 核对点

1. add_region/mark_dirty/flush 逐语句对 0x140ee3fd0/48c0/4bf0；candidate_index 哨兵 0xffff。
2. process 源(0x98)→输出(0x68)字段映射表对 0x1422ce260 写块。
3. add_birdnests 用 Subscriber::take 位集（非资源位集）；+0.15 偏移在 z。
4. add_lantern rem_euclid + is_started 偏移向量。
5. subscribe 13 句柄顺序（RaycastWorld×8/DirtyTiles×2/WorldRaster×3）。
6. auto_clutter_edit.rs 归属 country-core，建议删除。
