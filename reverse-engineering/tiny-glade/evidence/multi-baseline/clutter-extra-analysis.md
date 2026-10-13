# clutter 剩余项补全分析（B 主基线，反汇编级证据，第二批）

2026-10-13。本分析补全 clutter-placement-analysis.md 的剩余 unknown：
host_geometry（inter_roof_merlons / inter_shape_stitches）语义、B 独有自动杂物历史体系
（auto_clutter_edit / replay_auto_clutter / rmb_autoclutter）、add_autoclutter_on_windows
跳表分派语义、process_autoclutter_candidates 0x68 输出记录字段映射。
参考模板：clutter-placement-analysis.md（已证据化 populate/process/add_*/manage）。

全部函数 VA 基于 clutter-inventory.json 修正后的 B 地址（base 0x140001000）。
与第一批共用 AutoClutterState 布局：+0x48/+0x50/+0x58 = Vec stride 0x68（输出候选）。

## 证据来源（本次精读）

- **reva**（B 持久库 cf048e27…，唯一 worker `clutter-extra-owner`，只读 run-script）：
  - 9 个 host_geometry 函数全函数反编译（evidence/toolchain/clutter-extra/*.c）：
    construct_inter_roof_merlons / detect_inter_roof_merlons / detect_intra_shape_corners /
    dirty_walls / add_hole_at_shape_intersection / intersect_shapes×2（closure_env$4 两实例）/
    spawn_stitches / spawn_stitch_bricks；
  - 7 个 auto_clutter_edit 体系函数全函数反编译：replay_auto_clutter /
    history_record_auto_clutter / handle_rmb_autoclutter / ui_convert_autoclutter_to_decoclutter /
    AutoClutterEdit serialize×2 / deserialize。
- **radare2**（RNE 发行原件 f507a6ce…，独立会话，跳表 oracle）：
  - add_autoclutter_on_windows 全函数逐条反汇编（430 指令，含跳表分派 + 6 分支 + 写块）；
  - 跳表 0x143042370 的 0x80 字节 hexdump + 各分支目标反汇编 + 写块 0x1422cd8c0 反汇编与
    原始字节；常量 0x1429cd0c0（0,0,1,0）与 0x142925904（1.0/0.6）hexdump；
  - AutoClutterEdit serialize/deserialize 反汇编 → JSON 键字符串 0x14292c1b7("Delete")、
    0x14292c1bd("id") 的 .data 字节确认。
- **PDB 行号表**（skeleton/B/…）：construct_inter_roof_merlons.rs（L39–215+）、
  detect_intra_shape_corners.rs（L94 主，closure$0 L115/$1 L121/$2 L123/$3 L207/$5 L356/$6 L428，
  dirty_walls L69，intersect_shapes L478，add_hole_at_shape_intersection L725）、
  stitch_bricks.rs（spawn_stitches L28、spawn_stitch_bricks L92、on_exit_game_state L124、
  impl$3::default L141、InterShapeBrickStitches::clear L148）、
  country-core replay_auto_clutter.rs（L16，closure$0 L38）、auto_clutter_edit.rs（L8 impl$0）、
  defered_autoclutter_history.rs（L16–21）、rmb_autoclutter.rs（L23–44）。

## host_geometry 语义（生成什么实体、依赖哪些宿主几何）

四个规则分两组：**inter_roof_merlons**（屋顶城垛，两系统：detect + construct）与
**inter_shape_stitches**（形状缝合，两系统：detect_intra_shape_corners + stitch_bricks）。

### detect_inter_roof_merlons（0x141ce3a70，size 6751，源 L…）

- 入口先调 `WallAndRoofChanges::dirty_walls()`（0x141cd16b0，源 L69）：从 bevy 事件
  `OnWallDeleted`/`OnWallChanged`（wall_state publish）+ `RecalculateRoofCmd`（roofs commands）
  三个 EventIterator 链式 map，经 `hashbrown::set::extend` 汇成 `HashSet<WallId>` 脏墙集。
- 两个 retain：`Vec<InterRoofMerlonsDescription>::retain<closure_env_0>` 与
  `Vec<ShapeIntersectionHole>::retain<closure_env_2>`（后者来自 detect_intra_shape_corners 模块，
  跨模块共享类型）。
- 主循环：按 roof_entity（`PublicWalls::roof_entity()` 返回 Entity 打包值）查脏位集
  （wall 索引表 stride 0x14 → +4 处 u32 索引 → bit 集 `base+0x28` len `base+0x38`）；
  对脏屋顶调 `Roof::is_flat()` ×2（平顶门控），命中则
  `Roof::roof_shape_at_intersection_y()` → `local_374 != 2` 时 `ShapeParameters::expand()` ×2 +
  `intersect_shapes<closure_env$4>`（0x140edd250 实例，源 L478）在屋顶与墙形状交点处生成
  城垛断口描述。
- 收尾：遍历 `Vec`（stride 0x28）逐条 `WallHoles::add()`（wall_holes 资源）→ 把形状交点
  洞写进墙体开洞集合。即：**detect_inter_roof_merlons 产出 ShapeIntersectionHole/WallHoles
  洞**，供 construct 消费。

### construct_inter_roof_merlons（0x1422d4870，size 7070，源 L39–215+）

- 输入：宿主几何集合 Vec（stride 0x38）——每个元素：`+0x10` i32 判别（0=Circle2d，
  非 0=Rectangle2d），`+0x14/+0x1c` u64×2（形状基数据），`+0x24` u64，`+0x34` float
  （宽度基准）。
- 每个形状：`FrozenMap<SsboId,Box<BufferState>>::get` 取 `BrickTransformSsbo` 缓冲状态；
  Arc 强/弱计数克隆；`Circle2d::as_curve3` / `Rectangle2d::as_curve3` → `Curve::circular_slice<Vec3,
  CurveCoordSemantic2D>` 取曲线采样（Vec3 stride 0xc）。
- **城垛门控（local_f1）**：`PublicWalls::get()` → `WallStyle::is_halftimbered()`（半木墙）或
  `PublicWalls::roof_entity()` + `Roof::is_flat_w_no_rim()`（平顶无沿）→ local_f1=1 才生成；
  随后 `local_f1 ^= 1` 取反。
- **数量**：`ceilf(曲线长 / 0.46)`（`__real_3eeb851f`=0.46）；`长*1000.0`（`__real_447a0000`）
  截断为整型（1/1000 尺度）。
- `utils::random_splits_into()` 把曲线按均匀 u 切段；内层对每段
  `fastrand::Rng::f32()` 抖动：`((r*0.5-(1-r)*0.5)*0.21*0.8+0.368)*0.5 + (宽基+0.23)`
  （常量 0.21=3e570a3d、0.8=3f4ccccd、0.368=3ebc6a7f、0.23=3e6b851f）。
- `Curve::get_coord_at_u` ×2 取段两端坐标 → 线性插值 + 垂直方向归一化（`1/SQRT`）→
  swizzle x0y/xvy。
- **实体产出**：每段 `SparseInstanceBufferWriter::push<BrickTransformSsbo>()` 一个
  城垛砖实例。记录含：段 u×曲线长（local_144）、形状原点、(-100,-100) 常量
  （`__xmm_0x…c2c80000c2c80000`，`0xc2c80000`=-100.0）、`local_13c=4`（砖种类 4）、缩放。
  即：**城垛 = 沿屋顶形状曲线等距（0.46）的 BrickTransformSsbo 砖实例**，宿主几何 = 屋顶
  形状曲线（circle/rectangle）+ 墙样式/平顶标志。

### detect_intra_shape_corners（0x141cd1a90，size 8769，源 L94–~750）

- 系统入参：多个状态引用（in_R9 指向含多段 DirtyVec 的宿主资源；每段 stride 0x10 或 0x20，
  与 detect_inter_roof_merlons 同构的脏计数维护）。
- `dirty_walls()` 汇脏墙集；三个 retain：`Vec<Corner>::retain<closure_env_0>`（L115）、
  `HashMap<[WallId;2],Vec<Vec2>>::retain<closure_env_1>`（L121）、
  `Vec<ShapeIntersectionHole>::retain<closure_env_2>`（L123）。
- 主循环按脏墙对（hash 表 stride 0x2a8，计数 @+3 区）：对每对墙 `Roof::ty()` 判屋顶类型 →
  `PublicWallState::max_y()` 取墙顶 y → `fVar60 = max_y + flag*(-0.67)`（`__real_bf2b851f`）——
  角点高度 = 墙顶往下 0.67；经 `Roof::is_flat` 门控（flat 才做角）。
- 对角点：`ShapeParameters::expand()` ×2 + `contains_shape()`（L1179/1314 等）测形状包含，
  `ShapeParameters::perimeter()`（L1280/1286/1345/1350）取周长；`intersect_shapes<closure_env$4>`
  （0x140edbf60，源 L478）在墙对形状交线处调 `add_hole_at_shape_intersection`（0x141cd3ce0，
  L725）：`WallSpaceCoord::from_world_space` ×2 + `wall_space` + `subtract` +
  `project_clamp` → 写洞记录 stride 0x14：`+0` 世界 x、`+4` 世界 y+偏移、`+8` 高度差、
  `+0xc` Δ（fVar2-fVar1）、`+0x10` kind byte（5 或 1：在墙上/下方）。
- 收尾：`WallHoles::add()` 逐条写洞（stride 0x28，同 detect_inter_roof_merlons）。
  **产出：Corner/ShapeIntersectionHole/WallHoles 洞**；宿主几何 = 墙对 + 墙顶高度 + 形状包含。

### stitch_bricks：spawn_stitches + spawn_stitch_bricks

**spawn_stitches（0x141ce65b0，size 3025，源 L28–~89）**：
- 遍历 `Vec`（stride 0x40）缝合描述；每元素：`+0x00` i32 种类（必须==1 才处理）、
  `+0x04` f32（>=0.2 跳过）、`+0x14` f32 段起点、`+0x18` f32 段长。
- 段长检查：`u = truncate(min(1000, 2*|长|))`，`2 < u` 跳过（只缝短段）。
- 取段中点、`utils::random_splits()` 细分；按段计算单位法向（`1/SQRT`）+ 朝向
  （x0y swizzle），对每个细分点 `fastrand::Rng::f32()` 抖动
  `(r*0.5-(1-r)*0.5)*0.1+0.4`（3dcccccd=0.1、3ecccccd=0.4）、`fVar32=Δu+(-0.1)`
  （bdcccccd）。
- **实体产出**：`RawVec::grow_one<StairsFloatingBrickParams>` + 写 stride 0x40 记录：
  `+0..+0x10` 四 float（位置/朝向），`+0x10` 方向 qword（CONCAT44），`+0x18` 参数 u32，
  `+0x1c` 抖动 f32，`+0x20` Δ f32，`+0x24` 0.3（3e99999a），`+0x28` Vec2，
  `+0x38` u64。即：**缝合 = 楼梯台阶空隙处的悬浮砖参数（StairsFloatingBrickParams）**。

**spawn_stitch_bricks（0x141ce7190，size 2961，源 L92–~140）**：
- `FrozenMap<SsboId,Box<BufferState>>::get` 取 `BrickTransformSsbo` 缓冲；Arc 克隆；
  `SparseInstanceBufferWriter::clear<BrickTransformSsbo>()` 清空缓冲。
- 遍历 `Vec`（stride 0x40，= spawn_stitches 的 StairsFloatingBrickParams 输出），每条：
  `resources::render::impl$1::from()`（render 资源→颜色/变换）、
  `utils::calculate_hash<([WallId;2],usize)>`（墙对键）、
  `geometry::instanced_wall::impl$1::from()`（→ BrickTransform）、
  `SparseInstanceBufferWriter::push<BrickTransformSsbo>()`。
  **产出：把缝合砖参数烘焙进 BrickTransformSsbo 实例缓冲**。

组链（build 反编译）：stitch_bricks 与 preplaced 链按 `add_systems(...).chain()` 组合；
on_exit_game_state（L124）与 InterShapeBrickStitches::clear（L148）清理。

## auto_clutter_edit / replay_auto_clutter（B 独有历史体系）

清单 note：B 新增（A 无对应文件）`country-core systems/ui_clutter/*` 与
`systems/history/replay_auto_clutter.rs`、`resources/history/auto_clutter_edit.rs`。
本批把该体系提升为反汇编级证据。类型结构经 PDB structures：
`AutoClutterEdit` 为 **enum**（enum2$，存于 `Arc<AutoClutterEdit>`，有
drop_in_place/serde deserialize/fmt 泛型实例）。

### AutoClutterEdit serde（serialize 0x14003d300/0x140a18320，deserialize 0x140a18500）

- serialize（源 L8 impl$0）：写 `{"Delete": {"id": <u64>}}` —— 外层键字符串
  "Delete"（0x14292c1b7，len 6），内层 `serialize_entry<str,u64>` 键 "id"（0x14292c1bd，
  len 2）。外部标签式 enum 变体。
- deserialize：`variant_seed<PhantomData<__Field>>` + `struct_variant<__Visitor>`，
  期望**单键 map**（错误串 "map with a single key" @0x142ac7e80）；`visit_str`/`visit_map`
  访问器；值须为 u64。=> 可建模为：
  `pub enum AutoClutterEdit { Delete { id: u64 } }`（变体名 "Delete" 为证据，字段名 "id"）。

### history_record_auto_clutter（0x1409a9160，size 346，源 L16–21）

- 遍历 `HashSet<WallId>`（stride 0x10 双段，同 detect_* 脏集模式）；每个 WallId：
  `History::add` 新记录 `(1, 1, WallId)`（三 u64：grow=1、count=1、id=WallId，alloc 后写入）。
- 由 defered_autoclutter_history 系统调用（L16–21）：把"自动杂物被用户改动"的墙逐个记录为
  AutoClutterEdit 历史命令。

### handle_rmb_autoclutter（0x140976ca0，size 1359，源 L23–44）

- 参数：射线（in_RCX 4 float）+ 命中状态（in_RDX）。
- 命中墙时：`atan2f` 把命中点转朝向角 → `HashMap<WallId,()>::insert`（记录被编辑墙）→
  `ClutterMeshTy::from()` 定杂物网格种类 → 写编辑记录（stride 0x48，进 indexmap
  `Bucket<(SubworldColliderHandle,SubworldColliderHandle),MeshIntersectionArtifacts>`）：
  `+0` WallId、`+0x10` 网格种类、`+0x19` kind byte=3、`+0x2c` 朝向角、`+0x30` 网格类型、
  `+0x38`、`+0x3c`、`+0x40` → `History::start_input_recording()` +
  `History::add((1,1,WallId))`。
- 未命中：碰撞射线查询（Rapier raycast，见 ui_convert）沿窗/墙链找命中并算角度。
  **用户右键删自动杂物 → ClutterMeshTy + 编辑记录 + AutoClutterEdit 历史命令。**

### ui_convert_autoclutter_to_decoclutter（0x14099ca60，size 38075）

- `Rapier::raycast` 拾取 → `ClutterMeshTy::from` / `clutter_mesh_ty_data` → 转装饰物；
  最终复用 `handle_rmb_autoclutter`。UI 侧把自动杂物转 decoclutter。

### replay_auto_clutter（0x140aa4a60，源 L16–38，closure$0 L38）

- 状态读取：`fVar3 = state+0x28`（每帧生长 delta）；`state[3]==0` 早退。
- 遍历历史命令 Vec（stride 0x30）：`*cmd == '\t'`(9) 才处理（AutoClutterEdit 变体 tag 9）：
  - `+0x18` 生长值==0（未开始）→ 抹除保护：`HashMap<WallId,()>::insert`（cVar6==0）或
    hash 集遍历 `RawTable::erase<[DecoratorId,DecoratorId]>`（删除 DecoratorId 对）。
  - `+0x8` bit0==0 → 收集索引到 pending（`grow_one<usize>`）；否则
    `fVar15 = clamp( cmd+0xc / fVar3, min 0.05 )`（3d4ccccd=0.05），
    `fVar15 < cmd+0x18` → 收集索引（生长完成，删），否则 `cmd+0x18 += delta`（state+0x28，
    lVar24+0x28）。
- 反转收集的索引列表，逐条 `Vec::remove`（memmove）+ 按 kind byte drop：
  case 0/1/2 → `Arc<RasterEdit>`、3 → `Arc<TerrainEdit>`、4 → `WallReplayCmd`、
  5 → `Arc<RoofEdit>`、6/7 → `Arc<DecoratorHistoryEdit>`、8 → `Arc<TreeEdit>`、
  9 → `Arc<Mutex<CrashMiscInfo>>`、default → `Arc<Mutex<PhysicalSize<u32>>>`。
  **语义：历史命令回放时按帧推进 AutoClutterEdit 生长；完成即从命令列表移除并清其
  墙/装饰物保护标记，使该处自动杂物可重新放置。**

## add_autoclutter_on_windows 跳表语义（radare2 RNE oracle 全函数反汇编）

函数 0x1422cd7d0（size 1622）反编译被 Ghidra 截断（"bad instruction data"、
"Switch with 1 destination removed"）；以 radare2 逐条反汇编为准。

- 输入候选 Vec（stride **0x50**）：`R9 = data`、`RDX = count`，R10=count*0x50=end。
- 每候选（R9 游标，base=RSI=R9-0x50）：
  - `+0x00` byte = **种类（跳表索引）**；`MOVZX ECX byte[R9-0x50]`，
    `CMP ECX,6; JZ 0x1422cdd34`（==6 直接跳过，与 process_candidates 的强制路径同判据）。
  - `+0x01` byte → R12D；`+0x04` dword → R14D；`+0x08` qword → RAX；
    `+0x10` 16B → XMM8（候选位置）；`+0x20` qword → RBP+0x28；`+0x28` dword → RBP+0x30；
    `+0x40` qword → RDI；`+0x48` qword → R10 → RBP-0x28。
- 分派：`LEA RDX,[0x143042370]; MOVSXD RCX,dword[RDX+RCX*4]; ADD RCX,RDX; JMP RCX`
  → **target = 0x143042370 + disp32[index]**（表基相对偏移，非指令后）。
- 跳表（0x143042370，6 项 × 4B，后接 rustc 路径串 @0x14304238c）：
  | index | disp32 | target |
  |---|---|---|
  | 0 | ff28b647 | 0x1422cd9b7 |
  | 1 | ff28b868 | 0x1422cdbd8 |
  | 2 | ff28b757 | 0x1422cdac7 |
  | 3 | ff28b7e0 | 0x1422cdb50 |
  | 4 | ff28b6cf | 0x1422cda3f |
  | 5 | ff28b902 | 0x1422cdc72 |
- 6 分支共享同一向量数学（把候选位置 XMM8 绕常量 `0x1429cd0c0`=(0,0,1,0) 与
  `0x142925904`=1.0 做旋转/投影，含 `2*p.z*p` 反射项），仅两点不同：
  - **宿主源**（尾块 `mov r13,[rax]` 的 rax）：
    - index 0 → base+0xf0；index 1 → base Vec（`cmp rax,[rcx+0x10]`+`shl rax,3`+
      `add rax,[rcx+8]`，**以候选+0x08 为下标**）；index 2 → base+0xf8；
      index 3 → base+0x1a8；index 4 → base+0x1b0；index 5 → base+0x1b8。
    - base = `*in_R8`（宿主资源）：+0xf0/+0xf8/+0x1a8/+0x1b0/+0x1b8 = 各窗数据子结构
      （对应窗类型/百叶窗/开口等），+8/+0x10 = 一个 Vec。
  - **标志**：index 2 → `NOT r12b; AND r12b,1`（翻转候选+0x01）；其余 `r12b=2`、`r14d=0`。
- 尾块 0x1422cdd00→写块 0x1422cd8c0（输出候选 Vec r15+0x48/0x50/0x58，stride 0x68；
  满则 `call 0x14078c880` grow_one）：
  | 输出偏移 | 值 |
  |---|---|
  | +0x00 | 5（type，与 lantern/tree 同） |
  | +0x08 | 16B 位置（读栈槽 RBP-0x60；见 unknown ④） |
  | +0x18 | 1 |
  | +0x20 | 候选+0x48 |
  | +0x30 | byte 标志（r12b：2 或 !候选[1]） |
  | +0x31 | word RBP+0x55 |
  | +0x33 | byte RBP+0x57 |
  | +0x34 | r14d（0） |
  | +0x38 | r13（宿主子结构首 qword，墙/窗 id） |
  | +0x40 | 候选+0x40 |
  | +0x48 | 候选+0x20 |
  | +0x50 | 候选+0x28 |
  | +0x54 | XMM8 低 qword（候选位置 x/z） |
  | +0x5c | XMM8[1]（候选 y） |
  | +0x60 | 1.0（缩放，0x3f800000） |
- 循环：写后 `R9+=0x50; RSI+=0x50; cmp RSI, end; je 0x1422cdd34`（epilogue）。

**结论：on_windows 不调 add_region，而是按候选种类 byte（0–5）经跳表选不同窗宿主
子结构，用同一旋转投影计算落点，直接写 type=5 输出候选（0x68 stride）到 AutoClutterState
输出缓冲。**

## process_autoclutter_candidates 0x68 输出记录字段映射（reva 反编译）

源候选 stride 0x98（state+0x20/+0x28）。强制路径判 `source byte@+0x30 == 6`；
否则 `Rectangle3d::overlaps_rectangle3d`（140c98580）对已接受缓冲（**stride 0x20**，临时
local_70）剔除重叠。输出写入 state+0x48（stride 0x68，grow_one<WallPathSegment>）。

源→输出字段（`puVar26 = 源基`，switch on `*puVar26` 判别 3/4/5）：

| 源 0x98 偏移 | 输出 0x68 偏移 | 语义 |
|---|---|---|
| +0x0 u64 判别 | +0x0 | 种类 tag：case 3→0x3、case 4→0x4(Arc 强计数++)、case 5→0x5(裸指针=源基)、default 原值 |
| +0x8 | +0x8 | 数据/Arc 指针（case 4 `LOCK`++；case 5=plVar20） |
| +0x10 | +0x10 | 键/哈希（case 3/5 截 u32，case 4 全 u64） |
| +0x18 | +0x18 | u64（id/计数） |
| +0x20 | +0x20 | u64 |
| +0x28 | +0x28 | u64 |
| +0x34 u32 | +0x30 | u32；字节 +6→输出 +0x33 |
| +0x38 | +0x38 | u64 |
| +0x40 | +0x40 | u64 |
| +0x48 | +0x48 | u64 |
| +0x50 u32 | +0x50 | u32（uStack_98；+0x50 高 32=source+0x54 低 32） |
| +0x54 u64 | +0x58 区 | u32 uStack_8c（+0x58 高）+u32（source+0x54 高 32）；低 32 进 +0x50 高 32 |
| +0x5c u32 | +0x58 | u32（uStack_8c） |
| +0xc u32 | +0x60 低 32 | u32（uStack_88）；+0x60 高 32=uStack_84（0/debug 标志） |
| — | +0x60 | 缩放（birdnest/lantern/tree 写 1.0/0.6） |
| — | +0x34 上 4B | uVar16（调试 u16 标志，debug_viz 时设） |

debug：`*in_R9+0xf2==1 && in_R9+0x102==4` 时经 `DebugRectangle3d::new/color/show`
（140b02e10/40/ed0）可视化接受/拒绝。

## 身份验证（radare2 RNE oracle）

- on_windows 全函数 0x1422cd7d0 RNE 与 B 逐条一致（入口 push rbp/r15…rbx 序、跳表地址
  0x143042370、常量 0x1429cd0c0/0x142925904）；B patch 未改该函数与跳表。
- host_geometry 9 函数经 reva 确认名称/body 与 inventory 一致。

## 剩余 unknown（不伪造）

- ① on_windows 写块 `+0x08` 位置读栈槽 RBP-0x60，全函数反汇编未见显式写入该槽（分支只做
  寄存器数学、结果在 XMM8）——编译器栈槽别名/死槽复用疑点；位置应为分支向量数学结果，
  精确推导需 Ghidra P-code/动态分析。已记录为已知疑点。
- ② host_geometry 中 BrickTransformSsbo / StairsFloatingBrickParams 字段布局为 country-core
  instanced_wall/stairs 边界点，本批仅确认 push 记录的部分字段（缩放/种类/常量）。
- ③ AutoClutterEdit enum 除 `Delete{id:u64}` 外的其他变体未确认（deserialize __Field 支持
  ≥7 字段名："Delete/id/position/scale/rotation_degrees/prefab_name/is_preplaced/Placeindex/
  tree/Move/ip…" 为相邻 .data 字符串，可能属多类型）。
- ④ HistoryCommand（replay 的 stride 0x30）与 `(1,1,WallId)` 24B 记录的关系未完全钉死：
  replay 处理 tag==9 变体，history_record 写入 `(grow,count,id)`——两者是否同一结构不同阶段
  存疑（可能 history 层二次封装）。
- ⑤ construct 的 (-100,-100) 常量、local_13c=4 的"砖种类"与 BrickTransformSsbo 内部语义；
  spawn_stitch_bricks 的 `impl$1::from` 具体变换矩阵。
- ⑥ detect_intra_shape_corners 中 `local_374 != 2` 的"平顶模式"含义；WallHoles 洞记录
  stride 0x28 字段 +0x10/+0x18 的精确语义。
