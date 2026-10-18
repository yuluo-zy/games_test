# REVIEW — interaction/clutter（reviewer-interaction-clutter）

评审者: reviewer-interaction-clutter | 日期: 2026-10-17
范围: `crates/systems/clutter`（package `system-clutter`）restorer 重建逐函数独立核验。
方法: 本评审**另行 fresh 反编译** 9 个关键函数（`ghidra --worker shared`，
`work/reviewer-clutter/result-review.json` + `rv-*.txt`，2026-10-17 03:09），与
restorer 证据 `work/interaction-clutter/decomp/*.txt` 逐字节比对（除证据额外多一行
`void __cdecl <名>(void)` 头外**完全一致**）→ 证据真实性确认，无陈旧/伪造。其余函数
以 restorer 证据为基底独立复述。PDB 签名交叉用 skeleton `lib.rs.enriched.json.gz` 注解。

## 判定口径（2026-10-16 硬标准）

- **判等**: 复述与原反编译逐语句等价，含副作用。
- **判等-附U分歧**: 主控制流等价；个别副作用在**登记 U 类型合同**下桩化/近似（列 FIX）。
- **未实现-需重做**: 存在明确逻辑分歧（方向错/门缺失/来源错/谓词 U），附分歧点。
- **胶水-legitimate / 归属**: 宏派生注册 / 属别 crate。

**结论先行**：restorer 的「重建 11」**高估了保真度**。独立核验后：`dirty_walls` 判等；
`is_equivalent`/`ClutterCandidateIndex` 判等；`add_region`/`mark_dirty` 判等-附U分歧；
`flush`（排序方向错）、`process`（谓词 U）、`subscribe`（11/13 未接）、`add_birdnests`
（来源/门控/Arc）、`add_lantern`（**入水门缺失**）、`remove_region_from_lazy_grid`（桩）、
`set_current_tick`/`reset_autoclutter_for_new_frame`（系统签名未还原）均为
**未实现-需重做**。「判等 2」中 `reset_autoclutter_for_new_frame` 亦非完整判等。

---

## 1. autoclutter 核心状态（mod.rs）

### 1.1 add_region — 0x140ee3fd0
**独立复述**：`hash_one<RandomState,&GlobalId>` 求 key；hashbrown 区域表（ctrl@+0x60、
mask@+0x68、growth@+0x70、items@+0x78）开放寻址。(i) 命中且 `is_equivalent` 等价 → 仅写
条目 +0x44=epoch + `flush_candidates_pending_removal`；新 region kind==4 → 对新 region+8
Arc 强计数 LOCK 递减（归零 drop_slow）后返回。(ii) 命中不等价 → 旧 region 基向量
（payload +0x24/+0x18/+0x20/+0x1c）算 `d*a+c*b`/`c*a-d*b` → x0y/Rectangle2d::new/aabb →
`iter_world_space_rect_cells<remove_region_from_lazy_grid::closure>` 移除旧几何 → 同法对新
region 走 `<add_region_to_lazy_grid::closure>` 加新几何 → 旧条目 u16@+0x42 `<0xfffe` 则
push pending（cap 满 grow_one<u16>）→ 新 region payload（+0..+0x40，8 qword）覆盖条目、
写 epoch → 旧 kind==4 时旧 Arc 递减 → flush。(iii) 未命中 → `growth(@+0x70)==0` 时
`reserve_rehash` → 加 lazy grid → 写 key/payload/epoch、items++ → flush。`is_equivalent`
逐 7×f32（payload+0x18..+0x30，`!=`∨NaN 判不等）+ i32@+0x34 + u8@+0x38/+0x39；
**不**比较 kind/candidate_index/epoch。
**逐语句对照**：三分支、比较集、pending push（<0xfffe）、条目覆盖、epoch、flush 均对应。
**分歧（FIX）**：
- (i)(ii) 两处 kind==4 的 **Arc 强计数递减被桩化**为 `let _ = region.b38;`/`let _ = old.b38;`
  ——不执行任何递减（原 LOCK 递减、归零 drop_slow 是真实副作用；受 U 的 Arc<[Vec3]>
  payload 合同约束，但必须登记而非静默）。
- `rectangle2d()` 用 center/basis 近似组合（精确 a/b/c/d 基向量→矩形映射 U）。
- 未命中分支 `reserve` 条件以 `is_empty()` 近似 `growth==0`（hashbrown 自动扩容，行为等价）。
**verdict: 判等-附U分歧**（主控流对；Arc drop 需 FIX）。

### 1.2 mark_dirty — 0x140ee48c0
**独立复述**：cell 索引（低 32 位）≥ 懒网格 len（@+0x10）→ `panic_bounds_check`；cell
（stride 0x18，ptr@+8/len@+0x10）空则返回。逐 GlobalId：区域表 items(@+0x78)!=0 才查；
`hash_one` 命中 → 条目 u16@+0x42 `<0xfffe` → push pending（grow 语义）+ 写 +0x42=0xffff
（已消费）；**未命中或表空 → `unwrap_failed` panic**（懒网格↔区域表同步不变量）。
**逐语句对照**：越界 panic、per-key 查表、`<0xfffe` push、写 0xffff 均对。
**分歧（FIX）**：现 `if self.regions.is_empty() { continue; }` 把原 `unwrap_failed`
**panic 变成静默 continue**。原在不变量破坏（表空但 cell 非空）时必 panic；现跳过。
应删该 guard（`get_mut().expect()` 已覆盖未命中 panic）。
**verdict: 判等-附U分歧**（主路径对；空表 panic 路径分歧，FIX）。

### 1.3 remove_region_from_lazy_grid — 0x140ee4a70
**独立复述**：x0y → Rectangle2d::new → aabb →
`iter_world_space_rect_cells<remove_region_from_lazy_grid::closure_env_0>`；逐格删除动作在
**内联闭包**（未见）。
**对照**：公共方法 `remove_region_from_lazy_grid(&self,_key)` **空桩**；add_region 走私有
`remove_region_from_lazy_grid_rect`（`retain(|g| *g != 0)`）——删除键应为该区域 GlobalId，
`!=0` 是近似（格内精确键来源 U）。
**verdict: 未实现-需重做**（公共方法为桩；闭包删除键 U，`!=0` 近似）。

### 1.4 flush_candidates_pending_removal — 0x140ee4bf0  ⚠ 头号 FIX
**独立复述**：pending len(@+0x40)==0 → 返回。排序：len==1 跳过；len<0x15 → 内联
`insertion_sort_shift_left`，否则 `driftsort_main<u16, sort_by::closure_env_0>`。内联插入排序
可逐行判向：`if (v[i-1] < v[i])` 进入移位，`while (v[j-1] < key)` 右移 → **[3,5]→[5,3]，
即**降序**（高索引先删）**。逐条：swap_remove（memcpy 0x98 移除、memmove 末元素进槽、
len--）；移除项 kind==4 → Arc 递减；`new_len != idx` 时取换入候选 key（@+0x88）→ 区域表
查（表空→unwrap_failed panic），命中 → **断言条目 candidate_index==Some(new_len)**
（`>=0xfffe` 或 != new_len → `assert_failed<Option<usize>,Option<usize>>` panic）→ 写
+0x42=idx。末尾 pending len=0。
**逐语句对照**：swap_remove/换入修正/断言/清空均对。**排序方向错**：现
`pending_indices.sort_by(|a, b| a.cmp(b))`（**升序**）≠ 原降序。升序+swap_remove 会
**删错候选甚至 panic**（例 pending=[2,5]、candidates len=6：原先删 5 再删 2 正常；现先删
2 后删 5 → `len(5)<=idx(5)` swap_remove assert 崩）。RESTORE.md 也误记为「升序」。
**verdict: 未实现-需重做**（FIX：`sort_by(|a, b| b.cmp(a))` 降序；Arc drop 同 U 桩）。
此 bug 同时影响所有调 flush 的路径（add_region/populate）。

### 1.5 set_current_tick（L164）/ reset_autoclutter_for_new_frame（L531）
**独立核验（无独立反编译，函数全内联进 populate 44KB 碎片）**：PDB 注解两函数为同一系统
签名 `void(*)(Res<GameTime>, ResMut<AutoClutterState>, ResMut<InterShapeBrickStitches>)`
（skeleton mod.rs L3689/L3931）。即二者是**读取 GameTime 并同时改 AutoClutterState 与
InterShapeBrickStitches 的系统**。
**对照**：现 `set_current_tick(&mut self){ epoch.wrapping_add(1) }`、
`reset_autoclutter_for_new_frame(&mut self){ spawn_candidates.clear() }`——仅为桩/清空，
**未还原 GameTime 驱动与 InterShapeBrickStitches 交互**（epoch 来源、清空是否含 stitch
状态均 U）。
**verdict: 未实现-需重做**（系统签名与宿主交互未还原；RESTORE.md「重建/判等」过强）。

### 1.6 is_equivalent（L408）/ ClutterCandidateIndex
**独立复述/对照**：is_equivalent 比较集（7×f32 + i32 + 2×u8，NaN 序，不含
candidate/epoch）与 `float_ieq` 语义完全对应；字段↔偏移映射为 U 合同。ClutterCandidateIndex
哨兵 **0xffff**、`<0xfffe` 有效——add_region/mark_dirty/flush 三处判据逐一吻合。
**verdict: 判等**（映射 U 登记）。

### 1.7 remove_stale_regions（L233）
populate 内联 `HashMap::retain(epoch != state.epoch)`——与 ANALYSIS「epoch != state+0x90
删除」一致；本身逻辑对（宿主 populate 整体仍未实现）。
**verdict: 判等（内联 retain；宿主 populate 未实现不在此函数）**。

### 1.8 register_required_components — 0x140ee4ee0
反编译 = bevy `QueuedRegistration::register`/`ComponentInfo::new`/
`enforce_no_required_components_recursion` 注册胶水。
**verdict: 胶水-legitimate**。

## 2. autoclutter 流水线

### 2.1 process_autoclutter_candidates — 0x1422ce260
**独立复述**：puffin 范围；读候选 Vec（+0x20/+0x28，stride 0x98）；
`from_iter<OrderedCandidate, FilterMap<Enumerate<Iter>, closure_env_0>>`（u16 源下标@+0xc）；
排序（<0x15 insertion 否则 driftsort，closure_env_1）；debug 门控
`*(R9+0xf2)==1 && *(R9+0x102)==4`；遍历有序：u16 下标越界 → panic_bounds_check；源
`byte@+0x30==6` → 强制接受；否则对已接受缓冲（stride 0x20 = 源+0x68..+0x88 Rectangle3d）
`overlaps_rectangle3d` 逐条剔除；接受路径 switch 源判别 `*puVar26`：case3 截 u32、
case4 **Arc LOCK++**（溢出 panic）、case5 裸指针、default 截 u32；写输出 stride 0x68
（+0x00 tag、+0x08 shared、+0x10 key、+0x18/+0x20/+0x28=源同偏移、+0x30 marker、
+0x34、+0x38/+0x40/+0x48、+0x50/+0x54/+0x58/+0x5c、+0x60）；debug viz 写。
**逐语句对照**：结构（enumerate→filter_map→稳定排序→贪心剔除→switch 3/4/5→字段写）
与 switch/剔除/写块**结构对**。
**分歧（FIX，均为 U/猜测）**：
- closure_env_0 过滤谓词未见——现 `forced==0 || forced==6` 是**猜测**（未取证）。
- closure_env_1 排序键未见——现按 `f34` 是**猜测**。
- case5 的 key（uVar23 未被 switch 复写，Ghidra 为跳表地址残留）语义 U；现 `key&0xffff_ffff` 为猜。
- 输出 +0x38/+0x40 打包为位置（low32 取 f32 x/z、y=0）为推断，非取证。
**verdict: 未实现-需重做**（结构高保真，但两个核心决策点谓词/比较器 U，不能判等）。

### 2.2 autoclutter_subscribe_to_tabloids — 0x1422d6930
**独立复述**：句柄 Vec（cap@+0/data@+8/len@+0x10）push 13 个：8×
`RaycastWorld::subscribe_to_changes` → 1×`Publisher<DirtyTiles>::subscribe` → 3×
`TweakerView::as_inner_mut`+`WorldRaster::subscribe<512>` → 1×
`Publisher<DirtyTiles>::subscribe`。顺序与 restorer 描述一致。
**对照**：现只完成 2× DirtyTiles 订阅；8+3 为宿主缝（U）。
**verdict: 未实现-需重做（部分）**——顺序文档对，11/13 未接须如实登记。

### 2.3 populate_autoclutter_regions — 0x1422d6ed0（44KB 碎片）
restorer 判未实现——**属实**。fresh 反编译确认主体碎片化（重叠指令/跳表分派无法逐行）。
现 `process_dirty_cell` 为 F3 契约近似（水位阈值、跳表、掩码均未逐行复现）。移除
mark_dirty 空表 panic 分歧后，其 retain/flush/new_clear/take 顺序与复述一致。
**verdict: 未实现-需重做**（与 restorer 一致）。

## 3. autoclutter 各放置族

### 3.1 add_birdnests — 0x1422c9c70
**独立复述**：`Subscriber::take<DirtyTiles>` → 有 reader 则复制 0x200B 位集 → 本地
`new_clear` 合入；**无回退**。遍历**宿主脏格记录 Vec（stride 0x40，位置@+0x28/+0x30）**：
每格 `hash_one<&GlobalId>` 查已处理集（HashMap<u64,bool>），未处理 → `closure_0`
（0x1422ca8c0，几何/合法性）→ 集插入；**closure_0 结果门控是否写输出**；写输出 stride
0x68：+0x00=4、+0x08 Arc 强计数 **LOCK++**、位置（+0x15 加在 record+0x28 高 32）、
+0x60=1.0。
**对照**：现实现遍历 `dirty_tiles_cells(&tiles)`（位集派生 (x,y)）+ `cell_center_world`
位置 —— **与原始遍历宿主记录 Vec 不同源**；`closure_0` 门缺失（每格恒写）；Arc 未增
（存 cell_key 而已）；**新增资源位集回退**（`if !took { tiles = dirty.inner.clone() }`，原
无回退，take 无结果即空位集）；+0.15 轴映射 U。
**verdict: 未实现-需重做**（来源/门控/Arc/回退四处分歧）。

### 3.2 add_lantern_water_setdresssing — 0x1422d2fa0  ⚠ 头号 FIX
**独立复述**：`TweakerView::as_inner_mut` 借 WaterRaster（+0x298 双缓冲）；`is_corrupted`
门控早退；`iter_terrain_decorators`；每装饰物 transform + Circle2d::new + init_rng；
内层 5 次（i/5.0）：f32 → `f32*0.1+i/5.0` → `fmodf(.,1.0)` 负值 +1.0（rem_euclid）→
`get_pos_at_u` → f32×2 → `bilinear_sample_at_world_xz` → **`0.0<采样` 才放（入水门）** →
`1/SQRT` 归一化 → `calculate_hash<tuple<u32,usize,&str>>`("lantern"@0x143042d28 len7) →
`is_started` 三路选偏移（local_1d8/1e8/1d0，值 -0.8/0.8/1.3 等）→ 写输出 stride 0x68
（+0x00=5、+0x18=2、+0x30=5、+0x38 偏移、+0x60=`fVar23*(r*1.2+(1-r)*0.8)`）。
**逐语句对照**：rem_euclid、5 次、i/5.0、get_pos_at_u、key 结构对。
**分歧（FIX）**：
- **入水门缺失**：现代码 `let _water=...; let _water2=...;` 只消费 RNG 序列，**没有
  `0.0<采样` 判断就无条件 push** → 每装饰物 5 个恒放（原仅在入水处放）。这是行为级差异。
- 归一化方向 `1/SQRT` 算后**未应用到位置**（现 `let _ = nd`）。
- 偏移三路 → 现二值 `if started.0 {0.0}else{0.6}` 近似，且映射到 position.y 为猜。
- 缩放 `fVar23*(r*1.2+(1-r)*0.8)` → 现常量 0.6。
- key 哈希 tuple 顺序/类型（u32 装饰物 id, usize 迭代, &str）与现 `(i as u32, u64, str)`
  不一致（U/近似）。
**verdict: 未实现-需重做**（入水门缺失为最明确分歧）。

### 3.3 add_autoclutter_around_gates / around_windows / on_windows / add_preplaced / add_tree
restorer 均判未实现——**属实**（fresh 复述支持：gates=墙宿主+2 add_region+2.0 偏移、
windows=3 键/4.72/0.28 偏移、on_windows=跳表分派写 type5、preplaced=变更检测缺失（现
无条件遍历，正是 restorer 所述分歧）、tree=12 次圆周+水位入水）。逐条核对现 Rust 确为
近似/跳表近似。
**verdict: 未实现-需重做**（与 restorer 一致）。

## 4. 屋顶雉堞 / 形状接缝

### 4.1 WallAndRoofChanges::dirty_walls — 0x141cd16b0
**独立复述**：三事件迭代器（OnWallDeleted/OnWallChanged/RecalculateRoofCmd）链式 map →
`hashbrown::set::extend` 汇成**新** `HashSet<WallId>` 写回结构（stack 新建 + size_hint
预留 + reserve_rehash + fold）。
**逐语句对照**：现 `dirty_walls(ResMut, EventReader×3)` 新建空 HashSet 三连 extend → 整体
替换 `changes.dirty_walls`。事件全消费、新集合语义、非清空复用均对应。
**verdict: 判等**（集合构造方式等价；事件→`e.0` 字段映射未取证但 PDB map 输出为
`(WallId,())` 一致）。
**注**：`detect_inter_roof_merlons` 内调 `dirty_walls_ev`（no-op 桩）而非本函数——链上若
未独立注册 `dirty_walls` 系统则宿主刷新断裂（lib.rs `configure` 未注册 dirty_walls 为独立
系统，仅在 detect_* 内联调用脏墙集）。此为**接线缺口**，应登记（detect 系统入参缺
EventReader）。

### 4.2 detect_inter_roof_merlons / construct_inter_roof_merlons / request_audio / detect_intra_shape_corners / add_hole / spawn_stitches / spawn_stitch_bricks
restorer 均判未实现——**属实**（复述确认：detect 的 dirty_walls+retain+脏位集+expand×2+
roof_shape_at_intersection_y 主循环未逐行复现；construct 的 BrickTransformSsbo 字段 core
卡点；audio 的 `*(int)*RCX==0xd`+indexmap Bucket kind 8/9 与现距离<1.0 事件**结构完全不
同**；intra_shape 的 stride 0x2a8 脏墙对表+max_y-0.67 角点+intersect_shapes 未复现（现为
64 采样近似）；add_hole 现桩直通 intersect_shapes；spawn_stitches 的四元数/抖动公式未逐
语句复现；spawn_stitch_bricks 的 SSBO 烘焙未接）。各现 Rust 均确为近似/桩。
**verdict: 未实现-需重做**（与 restorer 一致）。

## 5. 插件与归属

- `impl Plugin::build` — 0x140652050：restorer 判 glue-legitimate（add_systems/
  insert_resource/register_event 注册近似，链序被内联遮蔽 U）——**属实**；但 lib.rs
  `configure` 手写 `add_systems(...).chain()` 未含 dirty_walls 独立系统（见 4.1 接线缺口）。
- `history_record_auto_clutter` 0x1409a9160：fresh 反编译符号 =
  `country_core::systems::ui_clutter::defered_autoclutter_history::history_record_auto_clutter`
  ——**属 country-core**，restorer 归属判定对。
- `replay_auto_clutter` 0x140aa4a60：反编译符号 =
  `country_core::systems::history::replay_auto_clutter::replay_auto_clutter` ——**属
  country-core**，判定对。
- `auto_clutter_edit.rs` 重复定义 → 建议移除：**属实**（两函数 B 侧均归 country-core；
  system-clutter 无对应二进制函数）。

## 6. FIX 清单（按优先级）

| # | 函数 | FIX |
|---|---|---|
| 1 | flush_candidates_pending_removal | **排序改降序** `sort_by(\|a,b\| b.cmp(a))`（现升序会删错/panic）；RESTORE.md「升序」改「降序」 |
| 2 | add_lantern_water_setdresssing | **补入水门 `0.0<采样`**（现无条件写）；补 1/SQRT 归一化应用；缩放 `fVar23*(r*1.2+(1-r)*0.8)`；is_started 三路偏移 |
| 3 | add_birdnests | 删资源位集回退（原无）；遍历源=宿主记录 Vec（U）；补 closure_0 门；Arc LOCK++ |
| 4 | mark_dirty | 删 `is_empty→continue`（原空表 panic unwrap_failed） |
| 5 | add_region / flush | kind==4 Arc 强计数递减（现 `let _` 桩）——须在 Arc payload 合同内落实或明确登记 |
| 6 | process_autoclutter_candidates | closure_env_0 过滤谓词 / closure_env_1 排序键取证（现猜测）；case5 key 语义；不得标判等 |
| 7 | subscribe | 11/13 订阅未接（Raycast×8/WorldRaster×3）如实登记，勿计完成 |
| 8 | set_current_tick / reset_autoclutter_for_new_frame | 系统签名（Res<GameTime>,ResMut<AutoClutterState>,ResMut<InterShapeBrickStitches>）未还原 |
| 9 | remove_region_from_lazy_grid | 公共方法空桩；闭包删除键（GlobalId）U |
| 10 | detect_inter_roof_merlons | dirty_walls 接线：链上无独立 dirty_walls 系统 → 脏墙集刷新断裂（登记接线缺口） |

## 7. 汇总

| 判定 | 函数数 | 名单 |
|---|---|---|
| 判等 | 3 | dirty_walls 0x141cd16b0、is_equivalent、ClutterCandidateIndex |
| 判等-附U分歧 | 2 | add_region 0x140ee3fd0（Arc 桩/rect 近似）、mark_dirty 0x140ee48c0（空表 panic 路径） |
| 未实现-需重做 | 25 | restorer 自报 17（populate/manage/read_bush/load_meshes/animation/gates/windows/on_windows/preplaced/tree/detect+construct merlons/request_audio/detect_intra_shape/add_hole/spawn_stitches/spawn_stitch_bricks）+ 本评审补 8（flush/process/subscribe/birdnests/lantern/remove_region_from_lazy_grid/set_current_tick/reset_autoclutter_for_new_frame） |
| 胶水-legitimate | 1 | register_required_components 0x140ee4ee0 |
| 归属 | 2 | history_record_auto_clutter 0x1409a9160、replay_auto_clutter 0x140aa4a60（均 country-core）；auto_clutter_edit.rs 建议移除 |

**过度断言核查**：RESTORE.md「重建 11」中 ≥7 个未达判等（flush 排序方向错、
lantern 入水门缺失、birdnests 来源/门控、process 谓词 U、subscribe 部分、remove_region
桩、set_current_tick 签名）；「判等 2」中 reset_autoclutter_for_new_frame 仅部分对。
无「伪造证据」——证据文件与评审 fresh 反编译逐字节一致；分歧均集中在**解读/落码**层面。

---

## 8. FIX 落实复验（restorer v2，2026-10-17 03:20）

| FIX | 落实 | 复验结论 |
|---|---|---|
| 1 flush 降序 | `sort_by(\|a,b\| b.cmp(a))` | **fix_verified**——与反汇编降序判向一致（原 [3,5]→[5,3]） |
| 2 lantern 入水门 | `if !(0.0<sample) { continue; }` + 缩放 `fVar23*(r*1.2+(1-r)*0.8)` + 1/SQRT | **fix_verified**（门/缩放到位）；残余：1/SQRT 方向仍 `let _ = dir` 未应用到位置，且归一所取 RNG 为门后 _03/_04（原用门前 _01/_02）——**仍 U** |
| 3 birdnests 回退 | 删 `dirty.inner.clone()` 回退，take 无结果即空位集 | **fix_verified**；closure_0 门/Arc 仍登记 U |
| 4 mark_dirty panic | 删 `is_empty→continue`，`get_mut().expect()` 兜底 | **fix_verified**——空表/未命中均 panic |
| 5 Arc drop | 仍 `let _` 桩 | **仍 U**（Arc payload 合同未闭） |
| 6 process U | filter/comparator/case5 显式标 U（不得判等） | **fix_verified**（如实登记） |
| 7 subscribe | 11/13 宿主缝登记 | **fix_verified**（如实登记） |
| 8 set_current_tick/reset | 登记 PDB 系统签名 | **fix_verified**（如实登记） |
| 9 remove_region_from_lazy_grid | 标 U | **fix_verified**（如实登记，公共方法仍空桩） |
| 10 detect 接线 | detect_inter_roof_merlons 自带三 EventReader 内联刷新脏墙集，删 no-op `dirty_walls_ev` 桩 | **fix_verified**（接线缺口闭合；standalone `dirty_walls` 现未注册，判等但为死代码——非缺口） |

- `auto_clutter_edit.rs` 已删；全 crate 无引用残留（grep 空）。
- `cargo check -p system-clutter` exit 0（30 警告，死代码/未用导入）。
- 状态：**10/10 FIX 落实**；其中 6 项为「如实登记 U」而非逻辑闭合。**剩余 U 不阻断**
  crate 编译，但构成 `system-clutter` 的**未实现部分**（判等判定不变：判等 3、
  判等-附U分歧 2、未实现 25、胶水 1、归属 2）。
