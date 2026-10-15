# systems/clutter 模块业务逻辑分析（B 主基线）

> 由批次 2 分析 agent 产出，父级代为落盘。fresh 反编译（Ghidra `--worker shared`，27 函数，2026-10-14）。

- 目标：B patched EXE `tiny-glade.exe` sha256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`。
- 标注：**O**=直接观察（反编译/原指令/骨架），**I**=推断，**U**=unknown（不猜）。
- 地址口径：VA = 骨架「编译实例 RVAs」+ 0x140000000。本报告只做模块内部实现，不展开 caller/callee、不分析跨 crate 接线。

## 0. 模块总览

`system-clutter` 是 Bevy `Plugin`（`impl Plugin::build`，0x140652050），把 autoclutter 放置链、屋顶雉堞（inter_roof_merlons）、形状接缝（inter_shape_stitches）注册为 Update/OnExit 独立系统，并插入资源、注册事件。

**autoclutter 数据流（O）**：`DirtyTiles`（tabloid）→ `populate_autoclutter_regions` 维护区域哈希表+脏格；各 `add_*` 把宿主几何转成区域或候选；`process_autoclutter_candidates` 筛选/排序/重叠剔除，重打包为输出候选；`manage_autoclutter_entities` 按集差 spawn/despawn/更新实体；`read_bush_positions_from_gpu` 从 GPU 回读灌木位置重建空间网格并发布 DirtyTiles。

**AutoClutterState 布局（O）**：

| 偏移 | 内容 |
|---|---|
| +0x08/+0x10 | DirtyTiles(lazy grid) 数据指针/容量 |
| +0x20/+0x28 | `Vec<AutoClutterCandidate>`（源候选 stride **0x98**） |
| +0x30/+0x38/+0x40 | `Vec<u16>` 候选待处理索引（+0x40 长度） |
| +0x48/+0x50/+0x58 | `Vec<输出候选>`（stride **0x68**） |
| +0x60/+0x68/+0x70/+0x78 | 区域表 hashbrown 控制字节/bucket mask/元素数/插入数 |
| +0x90 | u8 epoch（写进区域条目 +0x44） |

区域表条目 stride **0x48**：key u64 @+0，region 载荷 @+0x8（基向量/中心 float、int @+0x3c、char @+0x40/+0x41），u16 ClutterCandidateIndex @+0x42，u8 epoch @+0x44。

## 1. autoclutter 核心状态与放置

### 1.1 AutoClutterState::add_region — 0x140ee3fd0（Ghidra size 1901；mod.rs L168）
**内部逻辑（O）**：`hash_one<RandomState,&GlobalId>` 求 key；hashbrown 区域表开放寻址探测；key 命中时逐 float/char/int 比较（is_equivalent，含 NaN）。(a) 命中且不等价：`iter_world_space_rect_cells<remove_region_from_lazy_grid>` 移除旧区域 + `<add_region_to_lazy_grid>` 加入新区域（`x0y`+`Rectangle2d::new/aabb` 求世界 XZ AABB 遍历脏格），旧条目 u16 候选索引（<0xfffe）push 进 +0x30/+0x38/+0x40 Vec，覆盖字段+epoch，flush，返回。(b) 命中且等价：仅刷新 epoch + flush，判别==4 则 drop Arc。(c) 未命中：必要时 `reserve_rehash`，求 AABB → add_region_to_lazy_grid，写新条目（key/载荷/epoch/+0x78++），flush。
**算法**：hashbrown HashMap（SwissTable 开放寻址 + SIMD 控制字节）；AABB→网格单元枚举（lazy grid 空间哈希）。
**数据结构/常量**：区域条目 stride 0x48；pending Vec<u16> @+0x30..+0x40；epoch @+0x90；哨兵 0xfffe。
**未知（U）**：ClutterRegionId 精确构成；region 载荷 @+0x8..+0x40 各 float 语义。

### 1.2 AutoClutterState::mark_dirty — 0x140ee48c0（size 414；L214）
**内部逻辑（O）**：入参脏格索引；读 +0x08 表（stride 0x18，`+0x10` 取长度、`+8` 取指针）得该格关联 region key 集；逐 key `hash_one<&GlobalId>` 查区域表，命中则把条目 u16 候选索引（<0xfffe）push 进 pending Vec，并把 +0x42 置 `0xffff`（标记候选待重算）。
**算法**：脏格→region 反查 + hashbrown 查找；0xffff 哨兵。
**数据结构/常量**：反查表条目 stride 0x18；pending Vec<u16>；哨兵 0xfffe/0xffff。
**未知（U）**：+0x08 表类型；in_RDX 低 32 位 cell 线性索引语义。

### 1.3 AutoClutterState::remove_region_from_lazy_grid — 0x140ee4a70（size 378；L266）
**内部逻辑（O）**：`x0y`→`Rectangle2d::new`→`Rectangle2d::aabb`→`DirtyTiles::iter_world_space_rect_cells<remove_region_from_lazy_grid::closure_env_0>`（AABB 覆盖的每格回调删除）。
**算法**：AABB→网格枚举（移除方向）。
**未知（U）**：closure_env_0 逐格删除动作（inlinee，L274/278）。

### 1.4 AutoClutterState::flush_candidates_pending_removal — 0x140ee4bf0（size 734；L283）
**内部逻辑（O）**：读 pending Vec<u16>（+0x38/+0x40）；长度 0 返回。排序（<0x15 insertion_sort_shift_left 否则 driftsort_main，比较器 closure$0）使高索引先删。逐索引从候选 Vec（+0x20/+0x28，stride 0x98）取候选 memcpy 后 swap_remove（末元素覆盖、长度-1），判别==4 则 Arc drop_slow。若新长度!=被删索引，取被换入候选 key（@+0x88）查区域表，更新条目 +0x42=新索引（否则 assert_failed/unwrap_failed）。结束 +0x40=0。
**算法**：索引批量删除 + swap_remove（降序避免失效）；rustc driftsort/insertion_sort。
**数据结构/常量**：候选 stride 0x98、key @+0x88；哨兵 0xfffe。
**未知（U）**：closure$0 比较键（应为 u16 大小）。

### 1.5 populate_autoclutter_regions — 0x1422d6ed0（Ghidra size 7534，**PDB 44132**，碎片化 16 段；L114）
**内部逻辑（O）**：puffin 范围+begin_scope；`HashMap<ClutterRegionId,AutoClutterRegion>::retain(remove_stale_regions::closure_env_0)` 删过期区域（epoch!=state+0x90）；`flush_candidates_pending_removal`（2 处）；脏格合入：`Subscriber::take<DirtyTiles>`→复制 0x200 位集→`DirtyTiles::new_clear`/`extend`→逐位 `mark_dirty`；`filter_clutter_object`（内联 L688）：`bilinear_sample_at_world_xz<RasterBufferRef<512>>` + `terrain_rock_mask::sample_at_world_xz`，`0.0<采样`＝可用，`Aabb2::from_center_radius` + `iter_world_space_rect_cells`；按种类分派（`local_120==2` 走水位阈值 0x3c23d70a=0.01）；候选产出：`is_halloween2024` 季节门控、`rfind<str>`×2、`calculate_hash<IVec3>`/`<tuple<GlobalId,u32>>`、`RaycastWorld::raycast`、`PublicWalls::get`、`PathRastersResources::as_float_grid`、`contains_point(_padding)`、`overlaps_rectangle3d`、`set_orientation_xz`、`Rng::f32`。AutoClutterState 各方法（remove_stale_regions/add_region_to_lazy_grid/remove_region_from_lazy_grid/flush/mark_dirty/set_current_tick/reset_autoclutter_for_new_frame）由 mod.rs 经闭包实例内联进本函数。**各 add_*_around_* 是独立系统，不在本函数内**。
**算法**：lazy grid/脏格位集；地形水位栅格双线性采样；AABB→网格枚举；季节门控；hashbrown retain。
**数据结构/常量**：DirtyTiles 位集 0x200；水位阈值 0x3c23d70a=0.01；RasterBufferRef<512>。
**未知（U）**：44KB 主体碎片化（16 段、重叠指令）；filter_clutter_object/pick_clutter_mesh(L918)/align_to_nearby_wall(L944) 为内联闭包未逐行还原。

### 1.6 autoclutter_subscribe_to_tabloids — 0x1422d6930（size 1056；L80）
**内部逻辑（O）**：注册并保存订阅句柄（push 进 Vec `plVar3[1]+count*8`）：`RaycastWorld::subscribe_to_changes()` ×8、`tabloid::Publisher::subscribe<DirtyTiles>()` ×2、`WorldRaster::subscribe<512>()` ×3（经 `TweakerView::as_inner_mut`）。共 13 个。
**算法**：发布-订阅（tabloid Publisher）+ 句柄收集。
**数据结构/常量**：订阅句柄 Vec；`AutoClutterDirtySubscriptions` 资源。
**未知（U）**：8 次 RaycastWorld 订阅的变更类型；DirtyTiles/WorldRaster 订阅 2/3 次原因（疑双缓冲/分阶段）。

### 1.7 process_autoclutter_candidates — 0x1422ce260（size 1572，PDB 2375；L21；closure$0 L40、$1 L56、$2 L82、$3 L98）
**内部逻辑（O）**：读候选 Vec（+0x20/+0x28，stride 0x98）；`from_iter<OrderedCandidate, FilterMap<Enumerate<Iter<AutoClutterCandidate>>,closure$0>>`（带 u16 源下标 @+0xc）；排序（<0x15 insertion 否则 driftsort，比较器 closure$1）；debug viz 门控 `*(R9+0xf2)==1 && *(R9+0x102)==4`；遍历有序候选按 u16 下标取源候选，判 `*(char*)(源+0x30)==6`：==6 强制接受；!=6 对已接受缓冲（stride 0x20）逐条 `Rectangle3d::overlaps_rectangle3d`（0x140c98580）剔除重叠。接受路径 switch 源判别 `*puVar26`（3/4/5；case4 Arc LOCK++、case5 裸指针、case3 截 u32）复制字段写入输出候选（state+0x48/+0x50/+0x58，stride 0x68，grow_one<WallPathSegment>）；debug `DebugRectangle3d::new/color/show`。
**算法**：enumerate+filter_map → 稳定排序 → 贪心矩形重叠剔除；rustc driftsort。
**数据结构/常量**：源候选 stride 0x98（判别 @+0x30==6 强制路径）；接受缓冲 stride 0x20；输出 stride 0x68；Rectangle3d 重叠测试。
**源→输出字段（O）**：+0x0 tag(3/4/5/default)；+0x8 Arc/数据指针；+0x10 键/哈希；+0x18/+0x20/+0x28 u64；源+0x34→输出+0x30；+0x38/+0x40/+0x48 u64；源+0x50/+0x54/+0x5c→输出+0x50/+0x58；+0x60 缩放。
**未知（U）**：`byte@+0x30==6` 准确含义；输出 0x68 个别字段（+0x18/+0x2c/+0x40/+0x54）语义。

### 1.8 manage_autoclutter_entities — 0x1422d04e0（size 8178，PDB 10301；L0；closure$0 L45、$1 L56、$3 L85、$4 L95、$5 L100；update_shutter L249）
**内部逻辑（O）**：建现存集（源候选键集 `from_iter<u64,&AutoClutterObject,RandomState>` + 现存实体集 `from_iter<u64,Map<QueryIter<Entity,...>>>`）；遍历区域/候选（宿主查询 +0xf8/+0x100/+0x108；tick 索引 +0x110/+0x118/+0x120），`hash_one<&GlobalId>`×2 查成员集，`NameHash::compute`×4；需更新候选 closure_0（0x1422d2d20）+ 四元数朝向 + init_rng + `Rng::f32`×4 抖动；实体命令 `Commands::spawn_empty`、`insert<ColorId>`、`insert<ClutterMarkedForDeletion>`、`finish<MainCameraPlugin>`×5；closure 按 byte@+0x30 触发更新。
**算法**：集差驱动实体生命周期（spawn/despawn/update）；四元数；随机抖动。
**数据结构/常量**：AutoClutterObject/ClutterMarkedForDeletion/ColorId/NameHash。
**未知（U）**：ColorId 来源；update_shutter(L249) 细节；AutoClutterObject 字段布局；宿主查询 +0xf8..+0x120 各偏移。

### 1.9 read_bush_positions_from_gpu — 0x140eebaa0（size 745；L122；BushGrid subscribe L38/bushes_near_point L42/clear L67/finalize L75）
**内部逻辑（O）**：`FrozenMap<SsboId,Box<BufferState>>::get` 取灌木位置 SSBO；**类型尺寸断言** 0x5c(92) 字节（read-back 记录）；经 BufferState vtable 读回，`uVar1=*(uint*)*puVar3`=灌木数；清空本地网格（Vec stride 0x20，count @+0x10）；每条灌木（8 字节=2×u32）`Aabb2::from_center_radius`+`iter_world_space_rect_cells<closure$1>` 插入单元；`calculate_hash<&[[u32;2]]>`+`DirtyTiles::new_clear`；每单元（stride 0x20）排序灌木列表（insertion/ipnsort），hash 与已存 @+0x18 比较，变化则标脏进位集（local_258，0x200，`DecoratorRank::new`+`1<<(idx&0x3f)`）；`memcpy`+`tabloid::Publisher::publish<DirtyTiles,DirtyTiles>`。
**算法**：GPU→CPU 回读 + 空间网格（cell 哈希去重）+ 每 cell 列表哈希变更检测 → 发布脏格。
**数据结构/常量**：回读记录 92 字节；BushGrid/BushGridCell（单元 stride 0x20，hash @+0x18）；位集 0x200。
**未知（U）**：92 字节记录字段布局；BushGrid subscribe/bushes_near_point/finalize 语义；SsboId 常量。

### 1.10 load_clutter_meshes — 0x1422c0760（size 5383；L19；closure$0 L0/24、$1 L174）
**内部逻辑（O）**：约 50 个内联 `closure_0`（逐个加载一个 clutter 网格资产，硬编码调用），再 `from_iter<&ClutterMesh,Map<RangeInclusive<i32>,closure$1>>` 按索引区间收集 `Vec<&ClutterMesh>`，最后 `Commands::insert_resource<ClutterMeshes>()`。
**算法**：批量资产加载 + 索引区间 map。
**数据结构/常量**：ClutterMeshes/ClutterMesh；RangeInclusive<i32>。
**未知（U）**：约 50 个网格名/资产 id 清单；closure$1 映射细节；ClutterMeshes 结构。

### 1.11 autoclutter_animation — 0x1422ccef0（size 2238；L17）
**内部逻辑（O）**：遍历宿主实体查询（数据 +0xf8/长度 +0x100/过滤标志 +0x108）；每实体读动画状态（+8 计时、+0xc 进度、+0x10 速度、+0x14 字节、+0x2c 类型），`EasingFunction::apply` 缓动；`sinf/cosf` 构造四元数（`(eased*dir*60-(1-eased)*dir*50)*0.017453292*0.5`，绕轴转 ±60/∓50 度），写回朝向/缩放；进度归零 `EntityCommands::despawn`。
**算法**：缓动函数驱动时间动画；四元数旋转；进度钳制 [0,1]；完成 despawn。
**数据结构/常量**：动画状态 +8/+0xc/+0x10/+0x14/+0x2c；60.0/50.0；0.017453292；-0.3；缩放 1.0。
**未知（U）**：宿主实体类型（疑窗/百叶窗）；+0x2c 分支；EasingFunction 曲线种类。

### 1.12 add_autoclutter_around_gates — 0x1422cc390（size 2225，PDB 2910；L21）
**内部逻辑（O）**：遍历门宿主（+0xf8/+0x100/+0x108 双分支）；WallId+tick 索引（+0x110/+0x118）取墙条目（stride 0x38）；墙点数>2 才放置；取墙首/末点（stride 0xc=Vec3）。**2 个放置**：prefab `gate_right`(0x143042260,len10) 与 `gate_left`(len9)；位置=墙端+法向/方向偏移（`1/SQRT`+`x0y`，偏移常量 2.0/0x40000000）；键 `calculate_hash<tuple<&str,u32,WallId>>`→`add_region`（每门左右各 1）。
**算法**：宿主几何（墙端点/法向）→区域候选。
**数据结构/常量**：墙条目 stride 0x38、墙点 stride 0xc；gate_right/gate_left；偏移 2.0。
**未知（U）**：两放置精确几何（法向常量、fVar10=0.5）；第二个放置是否 Option。

### 1.13 add_autoclutter_around_windows — 0x1422cefe0（size 3606，PDB 4303；L31）
**内部逻辑（O）**：`DecoratorType::spawns_clutter` 门控 + `iter_removed_ids`；`CurveSlice::extend_right_until<Vec3,CurveCoordSemantic3D>` 沿窗曲线延伸；**3 处 add_region**（源行 31–181 多分支）；键 `calculate_hash<tuple<&str,usize,u32>>`(×2)+`<tuple<&str,i32,usize,u32>>`(×1)；位置/朝向 `xvy`/`x0y`；prefab 名含 `decorator_left`(len14)；偏移常量 4.72/0.28/-0.28/1.5/2.0。
**算法**：宿主曲线延伸 + 沿曲线偏移落位；3 类区域候选。
**未知（U）**：3 个 add_region 各自 prefab 与几何；Subtype 分支。

### 1.14 add_autoclutter_on_windows — 0x1422cd7d0（**Ghidra 反编译失败**，"bad instruction data"/跳表截断，边界仅 364；radare2 给出 size 1622；L17）
**内部逻辑（O，以 radare2 全函数反汇编为准）**：候选 Vec stride **0x50**；每候选 +0x00 byte=种类（跳表索引），==6 直接跳过；`LEA RDX,[0x143042370]; MOVSXD RCX,dword[RDX+RCX*4]; ADD RCX,RDX; JMP RCX` 跳表分派（表基相对偏移）。6 分支共享同一旋转/投影向量数学（绕 0x1429cd0c0=(0,0,1,0)、0x142925904=1.0），仅宿主源（+0xf0/+0xf8/+0x1a8/+0x1b0/+0x1b8 或按候选+0x08 下标取 Vec）与标志（index2 翻转候选+0x01，其余 r12b=2）不同。尾块写输出候选（state+0x48，stride 0x68）：+0x0=5(type)、+0x8 位置、+0x18=1、+0x30 标志、+0x38 宿主 id、+0x60=1.0。**不调 add_region**，直接写 type=5 输出候选。
**算法**：跳表分派 + 统一旋转/投影；直接写输出候选。
**未知（U）**：跳表 6 分支窗类型语义；写块 +0x08 位置读栈槽 RBP-0x60（疑栈槽别名，需 P-code/动态）。

### 1.15 add_birdnests — 0x1422c9c70（size 2532，PDB 3143；L37；closure$0 L79/0x22ca8c0、$1 L105）
**内部逻辑（O）**：`Subscriber::take<DirtyTiles>`→复制 0x200 位集→`new_clear`；遍历脏格（`cell_from_world_space`；`hash_one<&GlobalId>`×2 查已存在集）；未处理格调 closure_0（0x1422ca8c0）+`HashMap<u64,bool>::insert`（cell→已处理去重）；直接写输出候选（state+0x48，stride 0x68）：+0x0=**4**(type)、+0x8 Arc 强计数++、位置(+0.15 偏移)、+0x60=**1.0**。每脏格 1 鸟巢候选。
**算法**：脏格遍历 + 去重哈希集；AABB→cell。
**数据结构/常量**：位集 0x200；输出 stride 0x68；type=4；偏移 0.15；缩放 1.0。
**未知（U）**：closure_0 鸟巢几何/合法性判定；BirdNestCandidateLocations 用途。

### 1.16 add_lantern_water_setdresssing — 0x1422d2fa0（size 2096，PDB 2135；L22）
**内部逻辑（O）**：`TweakerView::as_inner_mut` 借出 WaterRaster（+0x298 双缓冲选择）；`is_corrupted` 门控；`DecoratorStorage::iter_terrain_decorators`；每装饰物 `Circle2d::new`+`init_rng`；**内层 5 次**（i/5.0）：`Rng::f32`×3+`fmodf` 圆周 jitter→`get_pos_at_u`→`bilinear_sample_at_world_xz` 水位采样→`0.0<采样`（入水才放）→归一化朝向+键 `calculate_hash<tuple<u32,usize,&str>>`（`lantern`@0x143042d28 len7）→`is_started` 选偏移；直接写输出候选（stride 0x68）：+0x0=**5**(type)、+0x8=lantern、+0x18=2、+0x1c 键、+0x30=5、+0x38 位置、+0x60 缩放。
**算法**：圆形均匀采样 + fastrand 抖动 + 水位栅格过滤。
**数据结构/常量**：内层 5；i/5.0；jitter *0.1；fmodf(...,1.0)；lantern 名；type=5；缩放 1.2/0.8/1.3。
**未知（U）**：+0x298 双缓冲选择语义；is_started 分支偏移向量含义。

### 1.17 add_preplaced_autoclutter — 0x141ce5900（size 1566；L29）
**内部逻辑（O）**：puffin 范围；解析 `preplaced_clutter` 串；读两 storage（`*puVar12` 当前长度 vs 记录长度）做**变更检测**（`uVar18=cur-last`），配合 Local<FramesSinceNewSession> 标志字节；变化时遍历预置列表（stride **0x1c**）：`xvy`/`x0y`+`calculate_hash<tuple<&str,usize>>`+`add_region`；未变化则仅置标志返回。
**算法**：变更检测（长度差集）+ 预置区域生成。
**数据结构/常量**：预置条目 stride 0x1c；PreplacedAutoClutter；FramesSinceNewSession；NewSessionStartedCmd/SessionLoadedCmd。
**未知（U）**：预置条目 stride 0x1c 字段布局；两 storage 类型。

### 1.18 add_tree_setdressing — 0x1422d3800（size 2566，PDB 2594；L19）
**内部逻辑（O）**：遍历树（`PlacementDataSsbo::position`×2）；`is_dragging`/`is_started`×4/`is_corrupted`×4 门控；每树 `Circle2d::new`+`init_rng`；**内层 12 次**（i/12.0）：`Rng::f32`+`fmodf` 圆周 jitter→`get_pos_at_u`→水位采样→入水→键 `calculate_hash<tuple<TreeStorageIndex,usize,&str>>`（`tree`@0x143042d91 len4）→写输出候选（stride 0x68，+0x0=**5**、+0x30=5）。与 lantern 同构（type5），count 12 vs 5，宿主=树 vs 地形装饰物。
**算法**：同 lantern。
**数据结构/常量**：内层 12；i/12.0；jitter *0.1；tree 名；type=5；缩放 1.2/0.8/1.3。
**未知（U）**：`uVar11*0x30+local_218+0x2c` 类型判别（==2）含义；TreeStorageIndex 来源。

## 2. 屋顶雉堞 / 形状接缝（host_geometry）

### 2.1 WallAndRoofChanges::dirty_walls — 0x141cd16b0（size 914；L69）
**内部逻辑（O）**：三 EventIterator 链式 map 后 `hashbrown::set::extend` 汇成 `HashSet<WallId>`：`OnWallDeleted`、`OnWallChanged`（wall_state publish）、`RecalculateRoofCmd`（roofs commands）。
**算法**：事件迭代器链 + 哈希集合并（增量脏集）。
**未知（U）**：无（逻辑确定）。

### 2.2 detect_inter_roof_merlons — 0x141ce3a70（size 5590，PDB 6751；L54；closure$0 L72、$2 L88、$3 L109、$5 L177、$6 L216）
**内部逻辑（O）**：入口 `dirty_walls()`；两 retain（Vec<InterRoofMerlonsDescription>、Vec<ShapeIntersectionHole>）；主循环按 roof_entity 查脏位集（wall 索引表 stride 0x14→+4 u32→位集）；对脏屋顶 `Roof::is_flat()`×2→`roof_shape_at_intersection_y()`→`local_374!=2` 时 `ShapeParameters::expand()`×2+`intersect_shapes<closure_env$4>`（0x140edd250,L478）生成断口；收尾（stride 0x28）逐条 `WallHoles::add()`。
**算法**：形状相交（intersect_shapes）；平顶门控；脏位集增量。
**数据结构/常量**：ShapeIntersectionHole（stride 0x28）、WallHoles；wall 索引表 stride 0x14。
**未知（U）**：local_374!=2 平顶模式含义；roof_shape_at_intersection_y 返回语义。

### 2.3 construct_inter_roof_merlons — 0x1422d4870（size 5133，PDB 7070；L39；closure$0 L72、$1 L76、$3 L124、$4 L128、$5 L129）
**内部逻辑（O）**：宿主几何集合 Vec（stride 0x38；+0x10 i32 判别 0=Circle2d/非0=Rectangle2d，+0x34 宽度基准）；每形状 `FrozenMap<SsboId,Box<BufferState>>::get` 取 BrickTransformSsbo 缓冲；`as_curve3`→`Curve::circular_slice` 取曲线采样；城垛门控 `WallStyle::is_halftimbered()` 或 `PublicWalls::roof_entity()`+`Roof::is_flat_w_no_rim()`；数量 `ceilf(曲线长/0.46)`；`random_splits_into()` 按 u 切段；每段 `Rng::f32` 抖动 `((r*0.5-(1-r)*0.5)*0.21*0.8+0.368)*0.5+(宽基+0.23)`；`Curve::get_coord_at_u`×2 插值+法向归一化；每段 `SparseInstanceBufferWriter::push<BrickTransformSsbo>()` 一城垛砖（-100 常量、种类4、缩放）。
**算法**：沿曲线等距（0.46）切分 + 均匀随机切段 + 抖动；实例缓冲写入。
**数据结构/常量**：步长 0.46；常量 0.21/0.8/0.368/0.23；-100.0(0xc2c80000)、种类4；BrickTransformSsbo。
**未知（U）**：BrickTransformSsbo 字段与真实 producer（core 卡点）；(-100,-100) 与 local_13c=4 语义。

### 2.4 detect_intra_shape_corners — 0x141cd1a90（size 7304，PDB 8769；L94；closure$0 L115、$1 L121、$2 L123、$3 L207、$4 L245/0xedbb60、$5 L356、$6 L428；intersect_shapes L478/0xedbf60；add_hole_at_shape_intersection L725）
**内部逻辑（O）**：系统入参含多段 DirtyVec（stride 0x10/0x20）；`dirty_walls()` 汇脏墙集；三 retain（Vec<Corner>、HashMap<[WallId;2],Vec<Vec2>>、Vec<ShapeIntersectionHole>）；主循环按脏墙对（hash 表 stride 0x2a8）对每对墙 `Roof::ty()` 判类型→`PublicWallState::max_y()` 取墙顶→`max_y+flag*(-0.67)`（角点高），`Roof::is_flat` 门控；`ShapeParameters::expand`×2+`contains_shape` 测包含，`perimeter()` 取周长；`intersect_shapes<closure_env$4>`（0x140edbf60）在墙对交线调 `add_hole_at_shape_intersection`；收尾 `WallHoles::add()`（stride 0x28）。
**算法**：墙对形状相交→角点/开洞；形状包含测试、周长；intersects。
**数据结构/常量**：Corner、HashMap<[WallId;2],Vec<Vec2>>、ShapeIntersectionHole（stride 0x14/0x28）；角点高偏移 -0.67(0xbf2b851f)。
**未知（U）**：local_374!=2 平顶模式；WallHoles 洞记录 +0x10/+0x18 语义。

### 2.5 add_hole_at_shape_intersection — 0x141cd3ce0（size 436；L725）
**内部逻辑（O）**：入参两点；`xvy`+`WallSpaceCoord::from_world_space`×2→`PublicWallState::wall_space`→`subtract`（Δ）；`project_clamp`；写洞记录（stride 0x14）：[0]=世界 x、[1]=插值 y、[2]=fVar5（高）、[3]=fVar2-fVar1（Δ），字节 @+0x10=kind（`5` 墙下方/`1` 墙上）。fVar5<0 时把墙厚（+0x30）折算进 y/Δ。
**算法**：世界↔墙空间坐标变换 + 投影钳制。
**数据结构/常量**：洞记录 stride 0x14（+0x10 kind byte 5/1）；墙厚 @+0x30。
**未知（U）**：`in_RCX[1]=in_XMM1_Da+fVar4*0.0*0.5`（`*0.0` 疑优化，表达式不清）；kind 5/1 精确判定。

### 2.6 spawn_stitches — 0x141ce65b0（size 2055，PDB 3025；L28；closure$0 L68）
**内部逻辑（O）**：遍历 Vec（stride **0x40**）缝合描述；每元素 +0x00 i32 种类（须==1）、+0x04 f32（>=0.2 跳过）、+0x14 f32 段起点、+0x18 f32 段长；段长 `u=truncate(min(1000,2*|长|))`，`2<u` 跳过（只缝短段）；取段中点、`random_splits()` 细分；按段算法向（1/SQRT）+朝向；每细分点 `Rng::f32` 抖动（`(r*0.5-(1-r)*0.5)*0.1+0.4`）；`grow_one<StairsFloatingBrickParams>` 写 stride 0x40（+0..+0x10 四元数、+0x10 方向 qword、+0x18 u32、+0x1c 抖动、+0x20 Δ-0.1、+0x24=0.3、+0x28 Vec2、+0x38 u64）。
**算法**：随机切分 + 抖动；短段缝合。
**数据结构/常量**：StairsFloatingBrickParams（stride 0x40）；阈值 0.2/0.1/0.4/0.3/-0.1；段长上限 1000。
**未知（U）**：StairsFloatingBrickParams 字段与真实 producer（core 卡点）。

### 2.7 spawn_stitch_bricks — 0x141ce7190（size 2040，PDB 2961；L92；closure$0 L104）
**内部逻辑（O）**：`FrozenMap<SsboId,Box<BufferState>>::get` 取 BrickTransformSsbo（断言类型名 len 0x3a=58）；`SparseInstanceBufferWriter::clear<BrickTransformSsbo>()`；遍历 Vec（stride 0x40 = spawn_stitches 输出），每条 `resources::render::impl$1::from()`、`calculate_hash<([WallId;2],usize)>`、`geometry::instanced_wall::impl$1::from()`→`push<BrickTransformSsbo>()`。
**算法**：把缝合砖参数烘焙进实例缓冲。
**未知（U）**：impl$1::from 具体变换矩阵（core 边界）。

### 2.8 request_inter_roof_merlon_audio — 0x1422d4230（size 524；L21）
**内部逻辑（O）**：`*(int*)*in_RCX==0xd`(13) 早退；`CursorTerrainRaycast::position()`；写 indexmap（stride 0x28，`Bucket<(DecoratorId,DecoratorId),AlignKind>`），kind=8/9；`spec_extend<InterRoofMerlonsDescription>` 扩展并更新标记。
**算法**：音频命令入队 + 描述扩展。
**数据结构/常量**：indexmap 条目 stride 0x28；kind 8/9；CursorTerrainRaycast。
**未知（U）**：kind 8/9 语义；0xd 状态含义；audio 后端接线（跨切面）。

## 3. 插件注册 impl$0::build — 0x140652050（size 3145；lib.rs L28）
**内部逻辑（O）**：(1) `Schedules::add_systems` 到 Update：`in_set(0x24)`、`run_if(void(*)(Res<UiSignifierStream>,Res<PreplacedAutoClutter>,ResMut<AutoClutterState>,EventReader<NewSessionStartedCmd>,EventReader<SessionLoadedCmd>,Local<FramesSinceNewSession>))`、`chain<...(InterShapeBrickStitches,IntraShapeCorners)...>`、`chain_ignore_deferred`、`in_set_inner`。(2) OnExit<AppLoadingState>：StitchBrickAlloc 清理。(3) 插入资源：AutoClutterState、InterShapeBrickStitches、IntraShapeCorners、ShapeIntersections、StitchBrickAlloc、InterRoofMerlons、InterRoofMerlonBrickCount、BushGrid、BirdNestCandidateLocations、WindowAutoClutterCandidates、AutoClutterDirtySubscriptions、DeletedAutoClutter、LastClonedClutter、DecoratorAutoClutterChildren。(4) 注册事件：StartPlaceDecoClutterInteraction、InstaCreateDeco、InstaCreateStairPoint、EvRightClickedAutoClutter、RequestRotateDecorator、RecordAutoClutterHistory、RequestMoveClutter、ShowPinnedDecoClutter、DebugSpawnAllClutterCmd、DebugSpawnClutterCmd。
**未知（U）**：8 个放置系统精确调度顺序（chain 参数被内联遮蔽）。

## 4. mod.rs 内联/派生函数（类型合同，非独立业务，O）
ClutterType::{is_window L74,is_plant L78,is_olden L82,is_window_shutter L86}（枚举判别谓词）；impl$9::clone L93、impl$14::clone L133（Clone）；impl$10::register_required_components L106/0xee4ee0（bevy 注册）；impl$13::default L133、impl$15::default L139（Default）；AutoClutterState::{set_current_tick L164,remove_stale_regions L233,add_region_to_lazy_grid L252,reset_autoclutter_for_new_frame L531}（状态维护，§1.5 说明其内联进 populate）；impl$20::eq L322、impl$22::hash L322、impl$37::eq L396（Eq/Hash）；ClutterCandidateIndex::{new L341,get L347,is_none L355}（u16 包装，哨兵 0xfffe）；AutoClutterRegion::{is_equivalent L408,centered L432,height L460,sec_xz_norm L464,basis_xz L468,sample_with_margin L477,center L516,rectangle2d L521}（区域几何访问器）；populate::_::impl$0::{apply,get_param,init_state,new_archetype,queue,validate_param} L61（SystemParam 派生）；add_birdnests::{impl$0::default L21,birdnests_subscribe_to_tabloids L31}；read_bush:::{impl$2::default L18,impl$3::clone L18,BushGrid::{subscribe L38,bushes_near_point L42,clear L67,finalize L75},impl$1::default L114}；inter_roof_merlons_audio::impl$1::default L15；detect_inter_roof_merlons::{impl$0::clone L32,impl$2::default L42}；detect_intra_shape_corners::{impl$2::default L32,impl$5::eq L84}；stitch_bricks::{on_exit_game_state L124,impl$3::default L141,InterShapeBrickStitches::clear L148}。

## 5. 逻辑总览
1. autoclutter = "区域→候选→筛选→实体"四段流水线（O）。
2. 两种候选产出路径（O）：① 经 add_region→区域→(populate)→源候选（门/窗/预置）；② 直接写输出候选 0x68（鸟巢 type4、灯笼/树 type5、on_windows type5）。两路在 process 后统一。
3. host_geometry 两组（O）：屋顶雉堞（detect 生成 ShapeIntersectionHole/WallHoles 洞→construct 沿形状曲线等距铺 BrickTransformSsbo 砖）；形状接缝（detect_intra_shape_corners 生成角点/洞→spawn_stitches 生成 StairsFloatingBrickParams→spawn_stitch_bricks 烘焙进 BrickTransformSsbo）。
4. GPU 侧（O）：read_bush_positions_from_gpu 回读→空间网格→发布 DirtyTiles（供 populate 消费）。
5. 插件注册（O）：build 接入全部系统/资源/事件。

## 6. 算法清单
| 算法 | 位置 |
|---|---|
| hashbrown HashMap（SwissTable 开放寻址+SIMD 控制字节） | add_region/mark_dirty/flush/populate |
| lazy grid / 空间哈希（AABB→网格单元枚举 iter_world_space_rect_cells） | add_region/remove_region_from_lazy_grid/populate/read_bush |
| 脏格位集（0x200 字节逐位） | populate/birdnests/read_bush |
| 稳定排序+插入排序（rustc driftsort） | process/flush/read_bush/spawn_stitches |
| 贪心矩形重叠剔除（Rectangle3d::overlaps_rectangle3d） | process_autoclutter_candidates |
| 索引批量删除（降序+swap_remove） | flush_candidates_pending_removal |
| 形状相交（intersect_shapes/contains_shape/perimeter/project_clamp） | detect/construct merlons、detect_intra_shape_corners |
| 沿曲线等距切分+随机切分（random_splits/random_splits_into）+抖动 | construct_merlons/spawn_stitches |
| 圆形均匀采样+fastrand 抖动+水位栅格双线性采样过滤 | lantern/tree setdressing |
| 跳表分派（jump table） | add_autoclutter_on_windows |
| 集差驱动实体生命周期 | manage_autoclutter_entities |
| 每单元哈希变更检测 | read_bush_positions_from_gpu |
| 缓动函数时间动画+四元数 | autoclutter_animation |
| 事件迭代器链+哈希集合并 | dirty_walls |
| 发布-订阅（tabloid） | autoclutter_subscribe_to_tabloids/read_bush |

## 7. 未知清单（不伪造）
1. populate 44KB 主体 Ghidra 碎片化（16 段、重叠指令）；filter_clutter_object/pick_clutter_mesh/align_to_nearby_wall 内联闭包未逐行还原。
2. add_autoclutter_on_windows 跳表 6 分支窗类型语义；写块 +0x08 位置读栈槽 RBP-0x60（栈槽别名疑点）。
3. process 的 `byte@+0x30==6` 含义；输出 0x68 个别字段语义。
4. gates 的 gate_left/gate_right 精确几何。
5. windows 3 个 add_region 各自 prefab 与几何。
6. manage 的 ColorId 来源、update_shutter(L249)、AutoClutterObject 布局、宿主查询 +0xf8..+0x120 各偏移。
7. read_bush 的 92 字节记录布局；BushGrid subscribe/bushes_near_point/finalize 语义。
8. load_clutter_meshes 约 50 个网格名清单；ClutterMeshes 结构。
9. autoclutter_animation 宿主实体类型、+0x2c 分支、EasingFunction 曲线。
10. BrickTransformSsbo/StairsFloatingBrickParams 字段与真实 producer（core 卡点）。
11. add_preplaced_autoclutter 预置条目 stride 0x1c 布局与两 storage 类型。
12. autoclutter_subscribe_to_tabloids 8 次 RaycastWorld 订阅变更类型；订阅次数差异原因。
13. AutoClutterState 各 Vec 字段精确关系；ClutterRegionId 内容（hash 键含 GlobalId）。
14. build 中 8 个放置系统精确调度顺序。
15. add_hole_at_shape_intersection 中 in_RCX[1] 的 `*0.0` 表达式。

## 附：最小 Rust 实现草案（仅逻辑完全确定处；依赖 core 类型，不写入工程）
```rust
// WallAndRoofChanges::dirty_walls —— 逻辑确定（O），仅依赖 core 事件类型
fn dirty_walls(deleted: EventReader<OnWallDeleted>, changed: EventReader<OnWallChanged>,
               recalc: EventReader<RecalculateRoofCmd>) -> HashSet<WallId> {
    deleted.read().map(|e| e.wall_id)
        .chain(changed.read().map(|e| e.wall_id))
        .chain(recalc.read().map(|e| e.wall_id))
        .collect()
}

// flush_candidates_pending_removal —— 索引批量删除（O）
fn flush_candidates_pending_removal(&mut self) {
    if self.pending.is_empty() { return; }
    self.pending.sort_unstable_by(|a, b| b.cmp(a)); // 高索引先删
    for &idx in self.pending.iter() {
        let _removed = self.candidates.swap_remove(idx as usize); // 判别==4 时 Arc 自然 drop
        if idx as usize != self.candidates.len() {
            let moved_key = self.candidates[idx as usize].region_key; // @+0x88
            if let Some(entry) = self.regions.get_mut(&moved_key) {
                entry.candidate_index = idx; // 修正 +0x42
            }
        }
    }
    self.pending.clear();
}
```
> 其余函数（add_region/mark_dirty/process/manage/各 add_*/host_geometry）内部逻辑已由反编译证据确定，但实现依赖 core 类型（AutoClutterRegion、AutoClutterCandidate、DirtyTiles、BrickTransformSsbo、StairsFloatingBrickParams、WallHoles、ShapeParameters 等）与字段布局，须待 core 契约闭合后方可给出可编译草案。
