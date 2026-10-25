# clutter 自动杂物放置链业务逻辑分析（B 主基线，反汇编级证据）

2026-10-13。本分析把 clutter 自动杂物（autoclutter）CPU 放置链升级为反汇编级证据。
覆盖链入口 populate_autoclutter_regions（44KB 主体）、候选筛选 process_autoclutter_candidates、
各放置规则 add_autoclutter_around_gates / around_windows / on_windows / add_birdnests /
add_lantern_water_setdresssing / add_tree_setdressing、实体管理 manage_autoclutter_entities，
以及 AutoClutterState 支持方法。host_geometry（inter_roof_merlons / inter_shape_stitches）
仅确认函数边界。

全部函数 VA 基于 clutter-inventory.json 修正后的 B 地址（base 0x140001000）。

## 证据来源（本次精读）

- **reva**（B 持久库 cf048e27…，worker `clutter-owner`，只读脚本）：
  - `0x1422d6ed0` 全函数反汇编 + 187KB C 反编译（populate_decompiled_full.c）；
  - 线性全 44KB 字节区间指令/CALL/Jcc/LEA 扫描（populate_full_sweep.json：28819 条含重叠、
    40 个唯一 call 目标，151 分支，47 LEA 数据引用）；
  - 8 个小函数逐一反编译与全函数反汇编（decomp/*.c、disasm/*.json）；
  - AutoClutterState::add_region（0x140ee3fd0）等支持方法反编译；
  - impl$0::build（0x140652050，plugin build）反编译（系统注册/资源插入）。
- **radare2**（RNE 发行原件 f507a6ce…，独立会话）：
  - 8 个目标函数入口逐条反汇编，与 B 完全一致（含字节），确认地址/指令未被 B patch 改动；
  - .data 字符串读取：`gate_right`/`gate_left`（0x143042260 起连续）、`lantern`（0x143042d28）、
    `tree`（0x143042d91）、on_windows 跳表（0x143042370）、常量 1.0/0.6（0x142925904）、
    0,0,1,0（0x1429cd0c0）。
- **PDB 行号表**（skeleton/B/…/autoclutter/*.rs.pdb.jsonl.gz）：各函数 offset→源行映射与
  内联闭包名/行号。

## 链结构与系统角色（证据：build 反编译 + 各函数为独立 bevy 系统）

`impl$0::build`（0x140652050）把各放置函数注册为 Update 调度链中的独立系统，并用
`add_systems(...).chain()/.chain_ignore_deferred()/.in_set()/.run_if()` 组合（build_decompiled.c）。
各目标函数自身带完整 SystemParam 样板（puffin 范围、arg 保存/恢复），互不直接调用：

- **populate_autoclutter_regions**（0x1422d6ed0，PDB size 44132=0xAC64）：主驱动，消费
  DirtyTiles 发布、维护区域哈希表与候选待处理列表、对每个脏格做 filter_clutter_object 过滤。
- **add_autoclutter_around_gates / around_windows / on_windows**：宿主几何（门/窗/墙）放置规则，
  调用 `AutoClutterState::add_region`（0x140ee3fd0）产出区域候选。
- **add_birdnests**：脏格→鸟巢候选，直接写输出候选（type 4）。
- **add_lantern_water_setdresssing / add_tree_setdressing**：地形装饰物/树周围随机落位（RNG），
  直接写输出候选（type 5）。
- **process_autoclutter_candidates**（0x1422ce260）：读取区域候选（0x98 stride），
  过滤→排序→矩形重叠剔除→重打包为 0x68 stride 输出候选。
- **manage_autoclutter_entities**（0x1422d04e0）：按输出候选与现存 AutoClutterObject 实体集合
  差集，spawn/despawn/更新（Commands::spawn_empty、insert<ColorId>、insert<ClutterMarkedForDeletion>）。

## populate_autoclutter_regions（44KB 主体，0x1422d6ed0，size 44132）

PDB 行号表覆盖源行 114–950（拆分两段 lines 事件）；内联闭包（B 库 inlinee）：
`closure$0`(L152)、`closure$1`(L488)、`filter_clutter_object`(L688, closure$0 L741)、
`pick_clutter_mesh`(L918)、`align_to_nearby_wall`(L944)。源文件
`crates/systems/clutter/src/autoclutter/populate_autoclutter_regions.rs`（B SHA 2507E518）。
参数：9 个（RCX/RDX/R8/R9 + 5 栈参，radare2 `arg_820h..arg_840h`）。Ghidra 函数体 16 段
`[[1422d6ed0,1422d7052]…[1422de9ef,1422dea04]]`（重叠/跳表致碎片化；线性扫描区间
[0x1422d6ed0, 0x1422e1b34)）。

业务步骤（reva 反编译 + 全区间 call 扫描；分段描述）：

1. **入口**：puffin `are_scopes_on`→rfind 解析范围名（"…populate_autoclutter_regions"）→
   begin_scope；保存 RCX/RDX/R8/R9 与 5 栈参。
2. **过期区域清理**：`HashMap<ClutterRegionId, AutoClutterRegion>::retain(remove_stale_regions::closure_env_0)`
   （140eecf00）删除过期区域（区域含 state+0x90 的 epoch 字节，add_region 写入）。
3. **候选待处理冲刷**：`AutoClutterState::flush_candidates_pending_removal`（140ee4bf0，2 处：
   区域 retain 后与脏位处理前）。
4. **脏格合入**：`DirtyTiles::new_clear`（140960ac0）→ 遍历取回发布（`Subscriber::take<DirtyTiles>`
   141247c90）→ `DirtyTiles::extend`（140960c30）→ 0x200 字节位集逐位 `from`+`mark_dirty`
   （140ee48c0）标记脏格。
5. **水位/地形过滤**（filter_clutter_object 内联）：`bilinear_sample_at_world_xz<RasterBufferRef<512>>`
   （140a14df0）两处 + `terrain_rock_mask::sample_at_world_xz`（B 新增水岩双采样）；
   `0.0 < 采样`＝可用；`Aabb2::from_center_radius` + `DirtyTiles::iter_world_space_rect_cells
   <bushes_near_point::closure_env_0<filter_clutter_object::closure_env_0>>`（140a14…闭包链）。
6. **按 clutter 种类分派**（switch on 局部类型字节）：`(uint)local_120 == 2` 走水位阈值
   `0x3c23d70a`(0.01) 分支；否则 puffin 范围 + 区域处理。
7. **候选产出**：`is_halloween2024`（140b46860）季节性门控；`rfind<str>`×2（1422d64c0，
   文件名解析）；`insert<String,SetValZST>`（141cad790，btree set，可能记录已处理名）；
   `calculate_hash<IVec3>`（14145c1a0，脏格键）/ `calculate_hash<tuple<GlobalId,u32>>`（140fabc80）。

**拆分说明（内部内联 add_*）**：区域管理（remove_stale_regions / add_region_to_lazy_grid /
remove_region_from_lazy_grid / flush_candidates_pending_removal / mark_dirty / set_current_tick /
reset_autoclutter_for_new_frame）在 B 库由 mod.rs 方法经闭包实例内联进本函数（A 版为顶层
ClutterState 方法 0x1409bd9d0/0x1409bdff0/0x1409be160）。各 add_*_around_* 放置规则不在本函数
内——它们是独立系统（见 add_* 节）。

## AutoClutterState 布局（证据：add_region 反编译 + mod.rs 行号）

`add_region(&mut self, key: ClutterRegionId, region: &AutoClutterRegion)`（0x140ee3fd0）：
- `self+0x60` hashbrown 控制字节 ptr、`+0x68` bucket mask、`+0x70` 计数、`+0x78` 插入计数；
  区域条目 stride **0x48**：key ClutterRegionId @0，基向量/中心 float @0x8..0x40，
  u16 @0x42（ClutterCandidateIndex），byte @0x44（epoch=state+0x90，remove_stale_regions 用）。
- `self+0x30/+0x38/+0x40`：Vec<u16> 候选待处理索引（grow_one<u16>，add_region 更新时 push）。
- `self+0x20/+0x28`：Vec stride **0x98**（AutoClutterCandidate，process_candidates 读）。
- `self+0x48/+0x50/+0x58`：Vec stride **0x68**（输出候选；add_region / birdnest / lantern / tree /
  process_candidates 共同写，+0x58 为写入计数，+0x48 容量，+0x50 数据指针）。
- `self+0x90`：byte epoch（写进区域 +0x44）。
add_region 语义：哈希 key→查区域表；命中且 is_equivalent（逐 float/byte 比较）→仅刷新
lazy grid 并 push u16 索引；命中但不等价→remove 旧格 + add 新格 + 覆盖字段 + flush 候选；
未命中→插入新条目（需 reserve_rehash）→ flush 候选。

## process_autoclutter_candidates（0x1422ce260，size 2375，源行 21–122）

闭包：closure$0=L40（filter_map）、closure$1=L56（sort_by 比较器）、closure$2=L82、
closure$3=L98。PDB 行 21（入口）→30/32→36–40（filter）→56（sort）→67–96（重叠剔除）→
105–113（输出写）→122（尾声）。

业务步骤（反编译+反汇编）：
1. 读 `AutoClutterState+0x20/+0x28` 的候选 Vec（stride 0x98）。
2. `from_iter<OrderedCandidate, FilterMap<Enumerate<Iter<AutoClutterCandidate>>, closure$0>>`
   （141cea460）枚举+过滤出有序候选（元素携带 u16 源下标；反编译按 2 字节 stride 迭代，
   OrderedCandidate 实际大小待核）。
3. 排序：`< 0x15` 用 `insertion_sort_shift_left`（141cf9b50），否则 `driftsort_main`（141cf4370）；
   比较器 closure$1=L56。
4. 遍历有序候选：按 u16 下标取源候选（stride 0x98），判 `byte@+0x30 == 6`：
   - **==6**：直接接受（强制路径），复制字段到输出；源字段含 +0x0 种类判别（switch 0x3/0x4/0x5
     按类型克隆共享字段；0x4=Arc 强计数++、0x5=裸指针、0x3=RefCell 类），
     +0x8/+0x10 等 16 字段。
   - **!=6**：`Rectangle3d::overlaps_rectangle3d`（140c98580）对已接受输出（0x68 stride）做
     空间重叠剔除；重叠→拒绝（uVar16=1）。
   - 调试：`DebugRectangle3d::new/color/show`（140b02e10/40/ed0）可视化解/拒。
5. 输出写入 state+0x48 Vec（0x68 stride；`grow_one<WallPathSegment>`，+0x58 计数）。

## add_autoclutter_around_gates（0x1422cc390，size 2910，源行 21–68）

- 遍历门（arg 宿主查询，`+0xf8/+0x100` 数据/长度，`+0x108` 过滤标志双分支）；每门经
  WallId + tick 索引（+0x110/+0x118）取墙条目（stride 0x38）。
- 墙点数 `plVar21[2] > 2` 才放置；取墙首/末点（stride 0xc=Vec3）。
- **2 个放置**：prefab `gate_right`（0x143042260，len 10）与 `gate_left`（len 9，.data 相邻）；
  位置=墙端中点 + 法向/方向偏移（`1/SQRT` 归一化、x0y swizzle），旋转用 2.0,2.0 四元数常量。
- 键：`calculate_hash<tuple<&str, u32, WallId>>`（141cf7440）→ 调 `AutoClutterState::add_region`
  （140ee3fd0，1 处）。即每门左右各 1 区域候选。

## add_autoclutter_around_windows（0x1422cefe0，size 4303，源行 31–181）

- 遍历窗（`spawns_clutter` 140833170 / `subtype` 140b26970 / `transform` 140a1aa20 /
  `iter_removed_ids` 140a1b100 组件查询）；`extend_right_until<Vec3,CurveCoordSemantic3D>`
  （1422e1e40）沿窗曲线延伸。
- **3 个 add_region**（140ee3fd0×3）；键 `calculate_hash<tuple<&str,i32,usize,u32>>`
  （141cf6d60）×1 + `<tuple<&str,usize,u32>>`（141cf77c0）×2；x0y/xvy swizzle 位置/朝向。
- 源行 31–181 覆盖多分支（窗类型/百叶窗/开口）。

## add_autoclutter_on_windows（0x1422cd7d0，size 1622，源行 17–65）

- 反编译质量差（`bad instruction data`、跳表截断）；反汇编显示：读候选字节
  `[R9-0x50]==6` 判类型，`LEA RDX [0x143042370]`（跳表，表前 6 项 + 后接 rustc 路径字符串，
  实际可用项少）+ `MOVSXD RCX dword ptr [RDX+RCX*4]` 分派；常量 1.0/0.6（0x142925904）、
  Vec 常量 0,0,1,0（0x1429cd0c0）。**语义未完全钉死**（unknown 段，见尾）。

## add_birdnests（0x1422c9c70，size 3143，源行 37–141；闭包 closure$0=L84、closure$1=L105）

- `Subscriber::take<DirtyTiles>`（141247c90）→ 复制 0x200 字节脏位集 → `new_clear`。
- 遍历脏格行（`cell_from_world_space` 140960b20；`hash_one<&GlobalId>` 1405b0d70×2 查已存在集）；
  未处理格调 `add_birdnests::closure_0`（1422ca8c0×2）+ `HashMap<u64,bool>::insert`
  （140eec230，cell→已处理去重）。
- 直接写输出候选（state+0x48，0x68 stride）：`+0x0=4`（type）、`+0x8` Arc 强计数（克隆共享字段）、
  位置（常量 0x3e19999a=0.15 偏移量）、`+0x60=1.0` 缩放、+0x58 计数。即每脏格 1 鸟巢候选。

## add_lantern_water_setdresssing（0x1422d2fa0，size 2135，源行 22–102）

- `as_inner_mut`（14067dd80）借 WaterRaster（+0x298 双缓冲选择）；`is_corrupted`（140ae8320）
  门控；`DecoratorStorage::iter_terrain_decorators`（140a1e280）遍历地形装饰物。
- 每装饰物 `Circle2d::new`（140c97540）+ `init_rng`（140c96240，fastrand RNG 初始化）；
  **内层 5 次**（`i/5.0`）：`fastrand::Rng::f32`×3 + `fmodf`（142923f10）沿圆周 jitter →
  `Circle2d::get_pos_at_u`（140c975a0）→ 水位采样 `bilinear_sample_at_world_xz`（140a14df0）→
  `0.0 < 采样`（入水才放）→ 归一化朝向 + 键 `calculate_hash<tuple<u32,usize,&str>>`
  （141cf7980，`lantern` @0x143042d28 len 7）→ `is_started`（14092e6c0）选偏移向量。
- 直接写输出候选（0x68 stride）：`+0x0=5`（type）、`+0x8=lantern`、`+0x18=2`、
  `+0x1c` 键、`+0x30=5` byte、`+0x38` 位置、`+0x60` 缩放。

## add_tree_setdressing（0x1422d3800，size 2594，源行 19–100）

- 遍历树（`PlacementDataSsbo::position` 140ad51a0×2 取位）；`is_dragging`（14092e6a0）、
  `is_started`×4、`is_corrupted`×4 多重门控（拖拽/拖拽开始/存档损坏）。
- 每树 `Circle2d::new`+`init_rng`；**内层 12 次**（`i/12.0`）：f32+fmodf 圆周 jitter →
  `get_pos_at_u` → 水位采样 → 入水 → 键 `calculate_hash<tuple<TreeStorageIndex,usize,&str>>`
  （141cf7600，`tree` @0x143042d91 len 4）→ 写输出候选（0x68 stride，`+0x0=5`、`+0x30=5`）。
- 与 lantern 同构（type 5），count 常量 12 vs 5，宿主=树 vs 地形装饰物。

## manage_autoclutter_entities（0x1422d04e0，size 10301，源行 0–247；闭包 closure$1=L56、
closure$3=L85、closure$4=L95、closure$5=L100、update_shutter=L249）

- 建现存集：`from_iter<u64, &AutoClutterObject, RandomState>`（142612aa0，源候选键集）；
  `from_iter<u64, Map<QueryIter<Entity,...>>>`（141ceaad0，现存实体集）。
- 遍历区域/候选（同 add_* 宿主查询模式 +0xf8/+0x100/+0x108；tick 索引 +0x110/+0x118/+0x120），
  `hash_one<&GlobalId>`×2（1405b0d70）查成员集。
- 每个需更新的候选：`manage_autoclutter_entities::closure_0`（1422d2d20×2）+ 四元数朝向
  （两次叉积+归一化，fVar18=1.0/fVar56=0.5 常量）+ `init_rng`（140c96240，位置抖动 f32×4）。
- 实体命令：`Commands::spawn_empty`（140d146a0）新建；`insert<ColorId>`、`insert<ClutterMarkedForDeletion>`
  经 Commands entity_command 队列（handle_error/with_entity/insert closure 链）；
  `finish<MainCameraPlugin>`×5（14003bb80，bevy Visibility 同步）；closure 按 byte@+0x30
  （type 分支：==2 或 -3 后 <4 且 !=1 触发更新命令）。
- `grow_one<EventInstance<WindowFocusEvent>>`（140562630）异常点（事件注册相关，unknown）。

## host_geometry 边界确认（仅边界，未精读）

- construct_inter_roof_merlons @0x1422d4870（size 7070，22 段）
- detect_inter_roof_merlons @0x141ce3a70（size 6751）
- detect_intra_shape_corners @0x141cd1a90（size 8769）
- spawn_stitches @0x141ce65b0（size 3025）、spawn_stitch_bricks @0x141ce7190（size 2961）
均经 reva 确认函数名/body 与 inventory 一致；未做语义精读。

## 身份验证（radare2 RNE oracle）

8 个目标函数入口（process_candidates/add_gates/add_windows/add_on_windows/add_birdnests/
add_lantern/add_tree/manage/populate）RNE 与 B 逐条字节一致（push rbp/r15…rbx 序，
lantern/tree 为 push r15…rbp/rbx 变体），确认 B patch 未改这些入口。

## 剩余 unknown（不伪造）

- populate 44KB 中 Ghidra 碎片化函数体（16 段）与重叠指令；线性扫描含重叠指令（28819 vs
  真实 ~11k），精确指令数/边界靠 radare2 fcn 分析（32075B r2 边界）仍有差异。
- add_autoclutter_on_windows 跳表分派目标与各分支语义（反编译质量差；跳表 0x143042370
  仅前 6 项有效）。
- process_autoclutter_candidates 输出 0x68 记录字段语义（+0x18/+0x2c/+0x40/+0x54 等）与
  `byte@+0x30==6` 的准确含义（强制接受 vs 种类标记）。
- add_around_gates 的 "gate_left"/"gate_right" 两放置精确几何（法向方向常量 0x40000000×2、
  fVar10=0.5 的推导）与第二个放置是否为 Option。
- manage_autoclutter_entities 中 ColorId 值来源、WindowFocusEvent grow_one 与 update_shutter
  （L249）闭包细节；AutoClutterObject 组件字段布局。
- AutoClutterState 各 Vec 字段精确偏移（+0x20/+0x48 两候选缓冲与 add_region u16 索引表的关系）；
  ClutterRegionId 内容（hash 键含 GlobalId）。
- bevy 调度链中 8 个系统精确顺序（build 反编译中 add_systems 的 chain 参数被内联遮蔽，仅见
  add_preplaced_autoclutter 与 stitch_bricks chain、in_set 0x24）。
