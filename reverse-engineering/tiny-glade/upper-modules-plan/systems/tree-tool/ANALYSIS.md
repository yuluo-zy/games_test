# systems/tree-tool 业务逻辑分析（B 主基线）

版本：B patched EXE（sha256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`），
程序名 `tiny-glade.exe`，crate 命名空间 `system_tree`（package `system-tree`）。
组织：按 B 源码 `crates/systems/tree-tool/src/*.rs` 层级，每函数一节。
只分析**模块内部实现**，不展开 caller/callee 接线（core 的 TreeStorage/PrefabInstanceData 等仅确认数据布局）。

证据标记：**O** = 原指令/反编译直接观察；**I** = 由观察合理推断；**U** = unknown（不猜）。

证据来源（本轮 fresh）：
- **Ghidra 批量反编译**（`ghidra --worker shared`，job `job1.json`/`job2.json`）：下列全部函数 fresh 反编译；
  含 `TreeEdit` serde deserialize（0x17c2f60 / 0x17c2960，返回空 void() 占位，未取到变体串）。
- **PDB 骨架**（`skeleton/B/crates/systems/tree-tool/src/*.rs` + `.enriched.json.gz` + `country-core/.../tree_edit.rs`）。
- **既有草稿**（`evidence/multi-baseline/tree-process-cmds-analysis.md`、`tree-tool-rest-analysis.md`、
  `tree-tool-extra-analysis.md`）的 reva/radare2 交叉证据——与本轮 fresh 反编译互证；冲突处标注并更正。

## 函数地址总表（RVA→VA）

| 函数 | B 源文件 | VA | RVA | PDB 源行 |
|---|---|---|---|---|
| process_tree_cmds | process_tree_cmds.rs | 0x142267430 | 0x2267430 | 0 |
| despawn_trees | process_tree_cmds.rs | 0x1422686d0 | 0x22686d0 | 203 |
| convert_preplaced_trees | convert_preplaced_trees.rs | 0x14226be70 | 0x226be70 | 32 |
| replay_tree | replay_tree.rs | 0x141bc34f0 | 0x1bc34f0 | 0 |
| recalc_tree_position_on_terrain_height_change | recalc_tree_position_on_terrain_height_change.rs | 0x142268dc0 | 0x2268dc0 | 12 |
| tree_move_bend | tree_move_bend.rs | 0x1422689c0 | 0x22689c0 | 8 |
| hide_trees | hide_trees.rs | 0x14226a260 | 0x226a260 | 0 |
| random_tree_prefab | ui_place_tree.rs | 0x14226a170 | 0x226a170 | 0 |
| ui_place_tree | ui_place_tree.rs | 0x142269170 | 0x2269170 | 30 |
| place_tree | ui_place_tree.rs | 0x142269970 | 0x2269970 | 157 |
| ui_move_tree | ui_move_tree.rs | 0x14226b1a0 | 0x226b1a0 | 32 |
| ui_delete_tree | ui_delete_tree.rs | 0x141bd2510 | 0x1bd2510 | 21 |
| reset_tree_tool（startup_tree_tool.rs） | startup_tree_tool.rs | 内联 | — | 4 |
| TreePlugin::build | lib.rs | 0x140650520 | 0x650520 | 38 |

注：`ui_place_tree.rs` 另有 `closure$0/$1`（源行 73/86，VA 0x2269720）、`seed_cursor_icon`（源行 115，内联）；
`ui_move_tree.rs` 另有 `TreeMoveCache::default`（源行 26）、`closure$0/$1`（源行 93/112，0x1bd4120）、
`remap_pos_y_if_in_water`（源行 150，内联）；`hide_trees.rs` 有 `closure$0`（源行 51，内联）；
`convert_preplaced_trees.rs` 有 `closure$0/$1`（源行 102/129，内联）。

---

## 卡点结论（本模块两个已知卡点）

### 卡点 A：`TreeEdit` 跨系统判别值 —— 已定位（0=Insert / 1=Move / 2=Delete）

`country-core::resources::history::tree_edit::TreeEdit` 是 serde 枚举，PDB 无字段布局
（`tree_edit.rs.structures.json.gz` 全部 `name_and_relations_only`）。判别值由**写入侧（History::add 载荷）**
与**读取侧（replay_tree switch）**双侧一致确定：

- 写入侧：`Arc<TreeEdit>` 分配布局 `[strong u64 @+0][weak u64 @+8][TreeEdit @+0x10]`，
  TreeEdit 的**枚举判别式 u32 落在 Arc 载荷 +0x10**（O）：
  - `place_tree`（0x142269970）：`*(u32*)(ptr+0x10) = 0` → **Insert=0**
  - `ui_move_tree`（0x14226b1a0）：`*(u32*)(ptr+0x10) = 1` → **Move=1**
  - `ui_delete_tree`（0x141bd2510）：`*(u32*)(ptr+0x10) = 2` → **Delete=2**
  - `hide_trees`（0x14226a260）：`*(u32*)(ptr+0x10) = 2` → **Delete=2**
- 读取侧：`replay_tree`（0x141bc34f0，O）对 `TreeEdit+0x10` 判别：
  `==2` 分支写 **DeleteTreeCmd**（in_R9 写者，stride 0x10）；`==1` 写 **MoveTreeCmd**（in_R8 写者，stride 0x20）；
  其余（0）写 **InsertTreeCmd**（in_RDX 写者，stride 0x48）。

**更正**：既有草稿 `tree-tool-rest-analysis.md` 记 replay 为「1=Move、2=Insert、其余=Delete」，与 fresh 反编译
（`==2` 走 Delete 写者）**不符**；正确为 **0=Insert、1=Move、2=Delete**（与 `tree-tool-extra-analysis.md`
「place=0/delete=2/move=1」一致）。接线时以本结论为准。

### 卡点 B：`PlaceTreeCmd` 载荷结构 —— 已定位

`PlaceTreeCmd` 是 `system_tree::ui_place_tree` 本地类型（非 core）。事件槽 **stride 0x18**（O，写入侧
`ui_place_tree` 0x142269170 与读取侧 `place_tree` 0x142269970 一致）：

| 槽偏移 | 内容 | 证据 |
|---|---|---|
| +0x00 | bevy `EventInstance` 计数/id（u64，框架字段，复用不还原） | O：写者从 writer state[+0x40] 取 |
| +0x08 | position **Vec3**：f32 x @+8、f32 y @+0xc、f32 z @+0x10 | O：写者 `CONCAT44(fStack_a0,local_a4)` + `local_9c`；来源 `CursorTerrainRaycast::position()` |
| +0x10 | （同上 z，Vec3 尾） | O：读侧 `place_tree` 按 `u64@+8` + `u32@+0x10` 整体读 |

即 **PlaceTreeCmd = { 位置 Vec3（世界 xz + 地形高度 y）}**（I：写者 y 分量做过水面高度重映射
`y<0 时 y ← (y/-2.5)*-1.2`）。精确 x/y/z 顺序以 `CursorTerrainRaycast::position()` 返回为准（I）。

---

## src/process_tree_cmds.rs

### process_tree_cmds
- **地址**：VA 0x142267430（RVA 0x2267430），size 0x1296(4758)。
- **系统签名（O，run_unsafe 包装器泛型 + 本轮反编译）**：
  `ResMut<TreeStorage>, Query<&mut PrefabInstanceData>, EventReader<MoveTreeCmd>, EventReader<DeleteTreeCmd>,
  EventReader<InsertTreeCmd>, EventWriter<SpawnPrefabCmd>, Commands, EventWriter<PlayTreeSoundCmd>,
  Res<ActiveSession>, Res<WaterRaster>, EventWriter<DespawnPrefabCmd>`。
- **内部逻辑（O，处理顺序 Move→Insert→Delete，与源码行 43–74 / 74–176 / 182–201 一致）**：
  1. **Move 分支**（事件槽 stride 0x20）：逐事件读 `index u64 @+8`、`position Vec3 @+0x10`；
     `TreeStorage::get(index)` → `None` 记日志 `"MoveTreeCmd references a non-existing tree"`；
     `Some` 时按值 `{prefab表下标 u32, entity u32}` 定位 `PrefabInstanceData`（Query 表 stride 0x14），
     写新位置 Vec3 到组件 +0；`inverse_lerp` + `bilinear_sample_at_world_xz<RasterBufferRef<512>>` 判树入水；
     发 `PlayTreeSoundCmd`（kind=1）。
  2. **Insert 分支**（事件槽 stride 0x48）：`is_dragging` 门控；树名 SIMD 匹配已知 prefab 名
     （"clearing/naked_tree/…"，`.data` 0x143034d30 起）；`deny_load` + `is_corrupted` 门控；
     `Commands::spawn_empty` → 经 Commands 插 `TreeComponent`、`TreeAnimation`、`TreeMoveBend`、`TreeAmbience`
     （TreeAnimation 值 = `r*0.75+(1-r)*0.55` 随机 scale∈[0.55,0.75]；高度 `max(h,0.7)`）；
     prefab 名由 `random_tree_prefab` 或 `PrefabName::new`（兜底 `"MISSING_TREE_PREFAB_NAME"`）；
     发 `SpawnPrefabCmd`（槽 stride 0x50）；`TreeStorage::insert(index@+0x38)`；`bilinear_sample`；发 `PlayTreeSoundCmd`（kind=0）。
     `insert` 返回高 32 非零（槽位已占用）时发 `DespawnPrefabCmd`（槽 stride 0x10）。
  3. **Delete 分支**（事件槽 stride 0x10）：逐事件读 `index u64 @+8`；`TreeStorage::remove(index)`；
     `Entities::contains(entity)` 才继续；经 Commands 推入 `TreeMarkedForDeletion` 标记；发 `PlayTreeSoundCmd`（kind=2）。
     **Delete 分支不直接发 DespawnPrefabCmd**（实际销毁由同文件 `despawn_trees` 负责）。
- **算法**：事件双缓冲迭代（bevy Events `+0x8/+0x10`、`+0x28/+0x30` 两 Vec，`+0x40` 计数）；哈希表查询；
  地形/水面双线性采样；SIMD 字符串比较（树名匹配）。
- **数据结构与常量（O）**：
  - 事件槽 stride：Move 0x20 / Delete 0x10 / Insert 0x48 / Spawn 0x50 / Despawn 0x10 / Sound 0x20。
  - `index` 为 **u64**（get 内 `cmp qword [entry], rsi`）。
  - TreeStorage：+0x18 非空标记、+0x20 hashbrown map（键 u64→值 8B）、+0x30 next_index（u64）；
    值 = {低32 prefab 表下标, 高32 entity}。
  - PrefabInstanceData：position Vec3 @+0（12B）、f32 @+0xc（inverse_lerp 读）。
  - WaterRaster：双缓冲栅格 + @+0x298 读缓冲选择字节；`0.0 < 采样值`＝入水。
  - 常量：130.0（水面采样参数）、inverse_lerp 阈值 0.5/0.9。
- **未知（U）**：InsertTreeCmd 槽 @+0x40..0x48 尾字段语义；SpawnPrefabCmd 除 name/position 外字段
  （@+0x10 u64、@+0x30=1.0、@+0x44 u32）；`ActiveSession[+0x58]` 对象实际 Rust 类型（三方法同址）；
  Insert 分支 DespawnPrefabCmd 触发条件语义；TreeStorage::insert 内部 hash 插入调用语义。

### despawn_trees
- **地址**：VA 0x1422686d0（RVA 0x22686d0），size 0x2EF(751)，源行 203–214。
- **系统签名（O）**：`Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)>, EventWriter<DespawnPrefabCmd>`。
- **内部逻辑（O）**：逐实体读 `TreeAnimation` f32 **@+4**（组件 stride 0x10）；当 `f < 0.01`
  （`f <= 0.01 && f != 0.01`，严格小于、排除 NaN）时发 `DespawnPrefabCmd`：
  事件槽 stride 0x10，`slot+0`=bevy id，`slot+8` = **entity（u64）**（非树 index）。
  两处查询路径（archetype 分派）：一条走 `TreeMarkedForDeletion` 列（@+0xf8/+0x100/+0x108 标志），
  一条走墙/decorator 表（@+0x1a8）。
- **算法**：bevy 查询 archetype 列查找；固定阈值比较（0.01）。
- **数据结构与常量（O）**：TreeAnimation size 0x10，f32 @+4（疑似淡出/进度计时，<0.01 触发销毁）；
  0.01f = 0x3c23d70a；payload = entity。
- **未知（U）**：TreeAnimation f32@+4 字段名/语义；entity 是否即 prefab 根实体。

## src/convert_preplaced_trees.rs

### convert_preplaced_trees（+ 内联 closure$0/$1）
- **地址**：VA 0x14226be70（RVA 0x226be70），size 0x12DB(4827)，源行 32–137；
  closure$0 源行 102（`WallSpatialHash::query_with_circle_callback` 回调）、closure$1 源行 129（`Drain<TreeEdit>`→`EditType` 映射）。
- **签名（I，PDB 无完整包装器名）**：arg1 RCX=EventReader<SpawnPreplacedTrees>；arg2 RDX=EventWriter（stride 0x10，payload 树 u64 @slot+8）；
  arg3 R8=ResMut<{u64 计数 @+0, u8 标志 @+8}>；arg4 R9=Res（水面栅格，`PathRastersResources::as_float_grid`）；
  arg5=Res<TweakerView>（`as_inner_mut`）；arg6=Res<WallSpatialHash>；arg7=资源（History 写入面）。
  （本轮反编译确认调用 `TweakerView::as_inner_mut`、`FrozenMap::get<SsboId,BufferState>`（岩石掩码 ssbo，assert 名 len 0x3f）、
  `Decorators::iter_removed_ids`、`Circle2d::new`、`WallSpatialHash::query_with_circle_callback`、
  `from_iter<EditType, Map<Drain<TreeEdit>,closure$1>>`、`History::insert_starting_edit`。）
- **内部逻辑（O/I）**：把装载期 `SpawnPreplacedTrees` 声明的"预放置树"（静态资产树）转为运行时树流程：
  延迟数帧等事件停止（计数自增，>2 清标志执行转换块）→ 遍历墙空间哈希内树 → 水/岩石掩码门控
  （6 次 `bilinear_sample_at_world_xz<RasterBufferRef<512>>` + `terrain_rock_mask::sample_at_world_xz`；
  水中或岩石上跳过）→ 通过者组 stride-0x48 条目并经 arg2 发转换事件（payload=树 u64）+
  `History::insert_starting_edit` 记历史起始编辑。
- **算法**：延迟帧计数门控；栅格双线性采样（水/岩石掩码）；空间哈希圆查询；迭代器 fold/from_iter 映射为 EditType。
- **数据结构与常量（O）**：arg3 状态 {u64,u8}；arg2 事件 stride 0x10；岩石掩码 ssbo id 名 len 0x3f；
  常量 1.0（0x142925904）、≈0.6（0x142925908）。
- **未知（U）**：convert 完整 FunctionSystem 签名（包装器名被 ICF 合并吞并）；arg2 事件确切类型（stride 0x10/u64 载荷，
  疑似 DespawnPrefabCmd）；arg3 {u64,u8} 资源归属；arg7 资源归属；`<0.6/1.0` 常数精确语义。

## src/replay_tree.rs

### replay_tree
- **地址**：VA 0x141bc34f0（RVA 0x1bc34f0），size 0xAA2(2722)，源行 17–121。
- **系统签名（O，lib.rs 注册泛型）**：`ResMut<ReplayBuffer>, EventWriter<InsertTreeCmd>, EventWriter<MoveTreeCmd>,
  EventWriter<DeleteTreeCmd>, Res<GameTime>`（注册于 `Update`，`in_set<GameSystemSet>` + `chain_ignore_deferred`）。
- **内部逻辑（O）**：
  1. `ReplayBuffer::front_mut_bucket()` 取前端桶（Vec<ReplayCmd>，stride 0x30）；空则返回；读 `ReplayBuffer+0x28`（f32 延时除数）。
  2. 逐 `ReplayCmd`：tag 字节 **@+0 == 8** 才是 TreeEdit（其余 0–9 为其它编辑类型），否则跳过。
  3. `Arc<TreeEdit>` 指针 @ReplayCmd+0x10；`TreeEdit+0x10` 判别：**2=Delete / 1=Move / 0=Insert**（见卡点 A）。
  4. 仅当 `ReplayCmd.elapsed`(f32 @+0x18) == 0.0（非 NaN）时重发命令事件：
     - Delete（2）：index = TreeEdit+0x18；DeleteTreeCmd 槽 stride 0x10。
     - Move（1）：flag(`ReplayCmd+0x1c`)==0 → position @TreeEdit+0x20/+0x28、index @+0x30；flag!=0 → @+0x14/+0x1c；MoveTreeCmd 槽 stride 0x20。
     - Insert（0）：String::clone 名字（@TreeEdit+0x18 起），读 +0x38/+0x40/+0x44/+0x4c/+0x50；InsertTreeCmd 槽 stride 0x48。
  5. 时序记账：`ReplayCmd` flags 字节 @+8 bit0==0 → 立即记入待移除；否则延时：
     `delay = ReplayCmd[+0xc] / ReplayBuffer[+0x28]`（下钳 0.05=0x3d4ccccd）；`delay < elapsed` → 待移除；否则 `elapsed += GameTime[+0x28]`。
  6. 循环后逆序 `Vec::remove` 出桶，按 tag switch（0–9）释放负载（**8 = `drop_slow<Arc<TreeEdit>>`**）。
- **算法**：回放桶；时间累加 + 延时阈值判定；按索引逆序删除（O：`Vec::remove` + `memmove`）；tag 驱动负载析构（switch 0–9）。
- **数据结构与常量（O）**：
  - ReplayCmd stride 0x30：tag u8 @+0；flags u8 @+8（bit0：立即/延时）；f32 delay @+0xc；Arc 指针 @+0x10；f32 elapsed @+0x18；flag u8 @+0x1c。
  - ReplayBuffer +0x28 = f32 除数（回放倍速）。
  - 命令事件槽：MoveTreeCmd 0x20、DeleteTreeCmd 0x10、InsertTreeCmd 0x48（本函数从 slot+0 直写 payload）。
  - 常量 0.05（delay 下限）。
- **未知（U）**：TreeEdit 各变体完整字段布局（尤其 Insert 的 name/position/height/flags 组合与 `ReplayCmd+0x1c` flag 语义）；
  ReplayBuffer+0x28 除数语义（回放倍速）；front/back 双缓冲细节（ReplayBuffer 内部）。

## src/recalc_tree_position_on_terrain_height_change.rs

### recalc_tree_position_on_terrain_height_change
- **地址**：VA 0x142268dc0（RVA 0x2268dc0），size 0x386(902)，源行 12–35。
- **系统签名（O）**：`Query<&mut PrefabInstanceData>, Res<TerrainHeightsData<TerrainHeightSsbo>>, EventReader<OnTerrainChange>`。
- **内部逻辑（O）**：EventReader 有新事件才继续；逐 `PrefabInstanceData`（stride 0x30）写变更 tick；
  边界门控 `|x|<=65.0 && |z|<=65.0`；`PlacementDataSsbo::position()` 取 x/z →
  `TerrainHeightsData::sample_at_world_xz<TerrainHeightSsbo>` 取高度；若高度 != 已存 y → 写 `max(高度, 0.0)` 到 +4。
- **算法**：地形高度栅格采样（世界 xz → 高度）；固定边界裁剪（±65 = 世界 130×130 半宽，I）。
- **数据结构与常量（O）**：PrefabInstanceData stride 0x30，y @+4；阈值 65.0 = 0x42820000；abs 掩码 0x7fffffff。
- **未知（U）**：±65 边界语义；"高度<=0 钳 0" 是否法线方向约定；TerrainHeightsData<TerrainHeightSsbo> 内部布局。

## src/tree_move_bend.rs

### tree_move_bend
- **地址**：VA 0x1422689c0（RVA 0x22689c0），size 0x3FB(1019)，源行 8–42。
- **系统签名（O）**：`Query<(&mut TreeMoveBend, &mut PrefabInstanceData)>, Res<GameTime>`。
- **内部逻辑（O）**：逐实体，`position()` 取 PrefabInstanceData 位置 x/z 作目标；
  `utils::time_continuous_lerp` 两次把 `TreeMoveBend.x/.y` 向目标缓动（delta=GameTime[+0x28]）；
  距离 `dist>=0.1` 时 `step = min(dist,5.0) * (delta/dist)`，`bend = target + step`（写回 TreeMoveBend）；
  变更检测：step 分量与 PrefabInstanceData+0x24/+0x28 比较，不同则写 tick + `[+0x24] = step_x | step_y<<32`（打包 u64）。
- **算法**：指数时间连续缓动 `time_continuous_lerp`（= `target+(current-target)*expf(-delta*rate)`）；
  距离钳制（0.1 阈值、5.0 上限）+ 归一化位移。
- **数据结构与常量（O，本轮 fresh 反编译）**：TreeMoveBend Vec2 stride 8（+0 x、+4 y）；
  PrefabInstanceData +0x24 f32 step_x、+0x28 f32 step_y（打包 u64）；
  常量 **0.1**（dist 阈值）、**5.0**（step 上限）；`time_continuous_lerp` 的 rate 常数（草稿 radare2 记 5.0/3.22，
  本轮反编译未显式暴露 XMM rate 实参，标 **U 待 radare2 复核**）。
- **未知（U）**：`time_continuous_lerp` rate 精确值（5.0 或 3.22，与既有草稿存在不一致）；3.22 常数来源；
  PrefabInstanceData+0x24/+0x28 视觉语义（疑似移动滑移/倾斜量 + GPU 变更量）。

## src/hide_trees.rs（本轮新分析，此前未覆盖）

### hide_trees（+ 内联 closure$0）
- **地址**：VA 0x14226a260（RVA 0x226a260），size 0xD15(3349)，源行 0；closure$0 源行 51（内联）。
- **系统签名（O，`.enriched.json.gz` file_interface_candidate）**：
  `Query<(&mut TreeAnimation, &PrefabInstanceData, &PrefabInstance, &TreeComponent)>,
  Res<TreeStorage>, EventReader<HideTreesinAreaCmd>, EventReader<HideTreesNextToWallCmd>,
  Res<WallSpatialHash>, EventWriter<DeleteTreeCmd>, ResMut<History>`。
- **内部逻辑（O/I）**：
  1. 迭代两个命令事件读者：`HideTreesinAreaCmd`（事件槽 stride 0x18）与 `HideTreesNextToWallCmd`（事件槽 stride 0x30）；
     每次 `Decorators::iter_removed_ids()` 取"已移除/待隐藏"树 id 位图（FixedBitSet）作为候选树集合。
  2. 对每个候选树：经 `WallSpatialHash`（哈希桶 @+0x8 stride 0x14，+0x1a8、+0x110/+0x118/+0x120 实体表）
     定位树实体 → `PlacementDataSsbo::position()` 取世界位置。
  3. **Area 命中**：命令 shape 字节 @+0 选择 `Circle2d::with_expanded_diameter`（=0）或
     `Rectangle2d::with_expanded_size`（=1）判 `contains_point(tree_pos)`。
  4. **NextToWall 命中**：`Circle2d::new` + `WallSpatialHash::query_with_circle_callback<closure$0>` 判树是否近墙。
  5. 命中后：写隐藏标志位（位图 bit@+0xc = 1），并在 `PrefabInstance`/条件字节 @+0x24==1（或局部标志）时
     发 **`DeleteTreeCmd`**（槽 stride 0x10：id @+0、树 index @+8）+ `History::add`（TreeEdit **Delete 变体，判别=2**，见卡点 A）。
- **算法**：位图（FixedBitSet）迭代 + 位扫描（`~bits & (bits-2)` 取最低有效位）；圆形/矩形点包含测试；
  空间哈希圆查询；命令形状判别（circle/rectangle）。
- **数据结构与常量（O）**：
  - `HideTreesinAreaCmd` 事件槽 stride 0x18；`HideTreesNextToWallCmd` 事件槽 stride 0x30。
  - 命令 shape 字节 @+0（0=circle，1=rectangle）+ 中心 + 半径/尺寸。
  - 树隐藏标志位图：每树 1 位，另有 byte@+0xc 隐藏状态。
  - `DeleteTreeCmd` 槽 stride 0x10；History TreeEdit Delete 载荷 = `Arc` 头 + disc u32@+0x10=2 + index@+0x18 + 名字 String。
  - `PrefabInstance`/命令 @+0x24 字节门控是否发删除命令。
- **未知（U）**：`HideTreesinAreaCmd` / `HideTreesNextToWallCmd` 载荷字段完整语义（中心/尺寸/半径偏移）；
  隐藏标志位图归属资源（`iter_removed_ids` 的 Decorators 位图如何区分移除 vs 隐藏）；
  @+0x24 门控字节确切含义；两个命令是否可同时命中（去重）。

## src/ui_place_tree.rs

### random_tree_prefab
- **地址**：VA 0x14226a170（RVA 0x226a170），size 0xE1(225)，源行 0。
- **内部逻辑（O，process_tree_cmds Insert 分支同款）**：门控 `deny_load` / `is_halloween_any` / `is_corrupted` /
  `DragState::is_started` 后，经 `fastrand` 抽取随机树 prefab 名，返回 `PrefabName`（末尾调 `PrefabName::new`）。
- **算法**：fastrand RNG（wyrand）+ `<0.6` 门控选普通/特殊 prefab（I，与 place_tree 中 `10*rand<6` 一致）。
- **数据结构与常量（O）**：RNG 状态（fastrand 1.8.0）；prefab 名表（`.data`，见 process_tree_cmds）。
- **未知（U）**：具体 prefab 名集合与选择概率分布（需与原 fastrand 种子序列逐位对照）；门控各方法的对象实际类型。

### ui_place_tree
- **地址**：VA 0x142269170（RVA 0x2269170），size 0x5A1(1441)，源行 30–113。
- **系统签名（O，enriched candidate）**：`Res<AppMode>, Res<UiState>, Res<UiSignifierStream>,
  Res<CursorTerrainRaycast>, EventWriter<PlaceTreeCmd>, Res<RaycastWorld>, Res<CameraCursorRay>,
  Res<CursorPositionSS>, Res<WindowSize>, Res<GameConfig>, (Res<GladeBorder>, EventWriter<DisplayOutOfBorder>,
  EventWriter<FeedbackInputOutOfBorder>)`。
- **内部逻辑（O）**：`*AppMode == 0x10`（种树工具态）且 `DEV_OVERRIDE_IS_DEMO_MODE==0` 才运行；
  构建 widget（`WidgetParams::new/occlude_by_world/sounds`，`tree_canvas`）；`RaycastWorld::legacy_raycast` +
  `geo_src_to_edit_subject`；`CursorTerrainRaycast::position()`；`GladeBorder::is_pos_inside`。
  glade 内点击 → 发 **`PlaceTreeCmd`**（槽 stride 0x18，payload=位置 Vec3 @+8..+0x10，见卡点 B）；
  glade 外点击 → 发 2 个空载荷事件（UI 反馈）。
- **算法**：UI 工具状态机；地形光标 raycast；边界（glade）内/外判定；widget/signifier 生成。
- **数据结构与常量（O）**：AppMode 值 0x10；PlaceTreeCmd 槽 stride 0x18；`DEV_OVERRIDE_IS_DEMO_MODE` 全局字节 0x14362c098。
- **未知（U）**：glade 外 2 个空载荷事件身份；水面高度重映射 `y<0 → (y/-2.5)*-1.2` 的物理语义。

### place_tree
- **地址**：VA 0x142269970（RVA 0x2269970），size 0x7F6(2038)，源行 157–190。
- **系统签名（O，enriched candidate）**：`ResMut<TreeStorage>, EventReader<PlaceTreeCmd>,
  EventWriter<InsertTreeCmd>, ResMut<History>, Res<ActiveSession>`。
- **内部逻辑（O）**：消费 `PlaceTreeCmd`（槽 stride 0x18）→ `TreeStorage::next_index()` 取新索引 →
  `fastrand::Rng::f32()` 两次生成随机量：scale = `r*0.9+(1-r)*0.5`（∈[0.5,0.9]）、
  rotation = `r*360.0`（∈[0,360)）；`random_tree_prefab` 选名；`PrefabName::as_name_ref` 复制名；
  组装 **`InsertTreeCmd`**（槽 stride 0x48：name/position/height/scale/rotation/flags/index）并发事件；
  组装 **`TreeEdit` Insert 变体**（Arc 头 + disc u32@+0x10=**0**）并 `History::add`。
- **算法**：fastrand RNG 生成 scale/rotation；`10*rand<6`（rand<0.6）门控特殊 prefab（O）。
- **数据结构与常量（O）**：PlaceTreeCmd 槽 0x18；InsertTreeCmd 槽 0x48；TreeEdit Insert 判别=0；
  常量 0.9/0.5（scale 区间）、360.0（角度）、0.6（门控）。
- **未知（U）**：InsertTreeCmd / TreeEdit Insert 变体各字段精确偏移（本轮反编译受内联干扰，部分字段顺序为 I）。

### ui_place_tree.rs 其它符号
- `closure$0`（源行 73，VA 0x2269720）、`closure$1`（源行 86）、`seed_cursor_icon`（源行 115，内联）：UI 辅助/闭包，未逐条展开（U）。

## src/ui_move_tree.rs

### ui_move_tree（+ TreeMoveCache::default / closure$0/$1 / remap_pos_y_if_in_water）
- **地址**：VA 0x14226b1a0（RVA 0x226b1a0），size 0xCC4(3268)，源行 32–148。
- **系统签名（O，enriched candidate）**：`Res<AppMode>, Res<UiState>, Res<TreeStorage>,
  Query<(&PrefabInstanceData, &PrefabInstance)>, Res<CursorTerrainRaycast>,
  Res<TerrainHeightsData<TerrainHeightSsbo>>, EventWriter<MoveTreeCmd>, ResMut<History>,
  Local<TreeMoveCache>, (Res<GladeBorder>, EventWriter<DisplayOutOfBorder>, EventWriter<FeedbackInputOutOfBorder>)`。
- **内部逻辑（O）**：工具状态 `AppMode-5 ∈ {11(移动), 0}` 且非 demo；`iter_removed_ids` + 空间哈希遍历树；
  `CursorTerrainRaycast::position()`（y<0 做水面重映射）；`slider3d::into_drag_state` 分派
  （case 1 悬停/按下、case 2/3 拖拽）→ 拖拽 glade 内发 **`MoveTreeCmd`**（槽 stride 0x20：
  id@+0、index u64@+8、position Vec3 @+0x10/+0x14/+0x18），`TerrainHeightsData::sample_at_world_xz` 取高度，
  `GladeBorder::is_pos_inside`；松开记录 **TreeEdit Move 变体**（disc u32@+0x10=**1**）`History::add`；
  glade 外发 2 个 8B 事件（UI 反馈）。
- **算法**：拖拽状态机（slider3d）；地形高度采样；glade 边界判定；空间哈希遍历。
- **数据结构与常量（O）**：MoveTreeCmd 槽 stride 0x20；TreeEdit Move 判别=1（+0x14/+0x1c 旧位、+0x20/+0x24 新位、+0x28/+0x30）；
  TreeMoveCache（Local，源行 26 定义）；工具状态值 11。
- **未知（U）**：glade 外 2 个 8B 事件身份；TreeMoveCache 字段布局；`remap_pos_y_if_in_water`（内联）语义。

## src/ui_delete_tree.rs

### ui_delete_tree
- **地址**：VA 0x141bd2510（RVA 0x1bd2510），size 0x74F(1871)，源行 21–103。
- **系统签名（O，enriched candidate）**：`Res<AppMode>, Res<UiState>,
  Query<(&PrefabInstanceData, &PrefabInstance, &TreeComponent)>, EventReader<PathDrawEvent>,
  EventWriter<DeleteTreeCmd>, Res<TreeStorage>, Res<CursorTerrainRaycast>, ResMut<History>,
  Res<UiSignifierStream>, Res<EraserSizeMultiplier>`。
- **内部逻辑（O）**：`DEV_OVERRIDE_IS_DEMO_MODE!=0` 直接返回；擦除工具态（`*AppMode==9` 且子态==6）；
  `clearing_canvas_w_layer` + `slider3d::into_drag_state`；`CursorTerrainRaycast::position()`；
  `iter_removed_ids` 遍历候选树，空间哈希定位树位置；对距光标半径（`radius*3.0`，@+0x14 双倍）内的树发
  **`DeleteTreeCmd`**（槽 stride 0x10：id@+0、index u64@+8）；组 **TreeEdit Delete 变体**（disc u32@+0x10=**2**）`History::add`。
- **算法**：擦除半径点包含（`sqrt(dx²+dz²) < radius`）；位图迭代；空间哈希遍历。
- **数据结构与常量（O）**：DeleteTreeCmd 槽 stride 0x10；TreeEdit Delete 判别=2（+0x18 index、名字 String @+0x20..）；
  擦除半径 ×3.0、@+0x14 尺寸双倍；工具状态值 9/子态 6。
- **未知（U）**：`EraserSizeMultiplier` 语义；擦除子态 6 的含义；PathDrawEvent 在本系统的作用。

## src/startup_tree_tool.rs

### reset_tree_tool（全内联）
- **地址**：无独立 compiled_instance（全内联进 bevy `run_unsafe<FunctionSystem<ResMut<TreeStorage>>>`，
  run_unsafe = 0x141bb19d0）。
- **注册（O）**：`add_systems<OnExit<AppLoadingState>, fn(ResMut<TreeStorage>)>`——装载状态退出时重置 TreeStorage。
- **内部逻辑（O）**：`*tree_storage = TreeStorage::default()` = 清空 hashbrown map（+0x0/+0x10 头清零）+
  写随机 KEYS（+0x20）+ next_index=0（+0x30），随后置变更 tick。
- **算法**：无（默认值覆写）。
- **数据结构与常量（O）**：TreeStorage 默认 0x40 字节；`[0x143034a10]`/`[0x143034a20]` 为清零常量。
- **未知（U）**：0x143034a00 指向内容；KEYS 随机键与 hashbrown 头字段切分。

## src/lib.rs

### TreePlugin::build
- **地址**：VA 0x140650520（RVA 0x650520），源行 38。
- **内部逻辑（O）**：
  1. `add_systems<OnExit<AppLoadingState>, fn(ResMut<TreeStorage>)>` → 注册 `reset_tree_tool`。
  2. `add_systems`（`Update` 计划）内 `in_set<GameSystemSet>` + `chain_ignore_deferred`；配置含
     `replay_tree` 签名 `fn(ResMut<ReplayBuffer>, EventWriter<InsertTreeCmd>, EventWriter<MoveTreeCmd>,
     EventWriter<DeleteTreeCmd>, Res<GameTime>)`；同批 add_systems 传入系统元组（含 process_tree_cmds / despawn_trees /
     recalc / tree_move_bend / hide_trees / ui_place_tree / place_tree / ui_move_tree / ui_delete_tree /
     convert_preplaced_trees 等）。
  3. `insert_resource<TreeStorage>()`（默认值）。
  4. `register_event`：**PlaceTreeCmd、MoveTreeCmd、DeleteTreeCmd、InsertTreeCmd、SpawnPreplacedTrees、
     HideTreesinAreaCmd、HideTreesNextToWallCmd**（共 7 个，O）。
- **算法**：无（bevy 插件注册）。
- **数据结构与常量（O）**：注册顺序如上；资源 id 哈希常量（`0x8e1bd251962ba784` 等为 TypeId）。
- **未知（U）**：`Update` add_systems 元组内系统精确排列顺序（反编译中元组位于栈槽，顺序未完全可读）；
  各系统是否全部落在同一 SystemSet。

---

## 逻辑总览

tree-tool 是"种树工具"系统包，围绕 `TreeStorage`（index→{prefab 下标, entity} 的 hashbrown 表）与
一套命令事件（`PlaceTreeCmd`/`InsertTreeCmd`/`MoveTreeCmd`/`DeleteTreeCmd`/`HideTrees*Cmd`/`SpawnPreplacedTrees`）组织。数据流：

1. **UI 输入层**（`ui_place_tree` / `ui_move_tree` / `ui_delete_tree` / `hide_trees`）：读工具状态 + 光标地形
   raycast + glade 边界，把用户意图编码为命令事件；同时把编辑写入 `History`（`TreeEdit`，判别 0/1/2）。
2. **命令分发层**（`process_tree_cmds`）：Move→Insert→Delete 顺序消费命令，维护 `TreeStorage`、`PrefabInstanceData`
   位置、生成/标记 prefab 实体，播放音效；Delete 只打 `TreeMarkedForDeletion` 标记。
3. **销毁层**（`despawn_trees`）：逐帧检查标记树的 `TreeAnimation` 进度（<0.01）后发 `DespawnPrefabCmd`（payload=entity）。
4. **回放层**（`replay_tree`）：从 `ReplayBuffer` 按 `GameTime` 推进，重发历史命令事件并回收桶。
5. **地形/物理跟随**：`recalc_tree_position_on_terrain_height_change`（地形变化重算 y）、
   `tree_move_bend`（拖动弯曲缓动）、`convert_preplaced_trees`（装载期预置树转运行时树）。
6. **插件**（`lib.rs`）：注册资源 + 7 个事件 + 系统集；`reset_tree_tool` 于装载退出时清空 TreeStorage。

## 算法清单

| 算法 | 出现函数 | 说明 |
|---|---|---|
| 命令事件双缓冲迭代 | process_tree_cmds / hide_trees / ui_* / replay_tree | bevy Events 两 Vec + 计数 |
| 哈希表查询（index→{prefab,entity}） | process_tree_cmds / despawn_trees / ui_* | TreeStorage（hashbrown） |
| 地形/水面双线性采样 | process_tree_cmds / recalc / convert / ui_move_tree | `bilinear_sample_at_world_xz<RasterBufferRef<512>>`、`TerrainHeightsData::sample_at_world_xz` |
| 时间连续指数缓动 | tree_move_bend | `utils::time_continuous_lerp` = `target+(current-target)*expf(-delta*rate)` |
| 距离钳制 + 归一化位移 | tree_move_bend | min(dist,5.0)、0.1 阈值 |
| fastrand RNG 选取（wyrand） | random_tree_prefab / place_tree / process_tree_cmds | scale/rotation/名选择，`10*rand<6`（<0.6）门控 |
| serde 枚举判别 + 回放重编码 | replay_tree | ReplayCmd tag=8(TreeEdit) + TreeEdit disc 0/1/2 |
| 回放时间累加/延时阈值 | replay_tree | `delay=cmd[+0xc]/buf[+0x28]`（下钳 0.05）vs elapsed |
| 位图迭代 + 位扫描 | hide_trees / ui_move_tree / ui_delete_tree | `Decorators::iter_removed_ids`（FixedBitSet） |
| 圆形/矩形点包含 | hide_trees | `Circle2d`/`Rectangle2d::contains_point` |
| 空间哈希圆查询 | hide_trees / convert / ui_move_tree | `WallSpatialHash::query_with_circle_callback` |
| SIMD 字符串比较（树名匹配） | process_tree_cmds | 已知 prefab 名匹配 + "MISSING_TREE_PREFAB_NAME" 兜底 |
| 延迟帧计数门控 | convert_preplaced_trees | 事件停止后计数>2 触发转换 |

## 未知清单（汇总，不伪造）

1. `TreeEdit` 各变体完整字段布局（Insert 的 name/position/height/flags 组合、Move 的旧/新位与 `ReplayCmd+0x1c` flag）。
2. `PlaceTreeCmd` 精确 x/y/z 顺序（本轮为 I）；glade 外 2 个空载荷事件、ui_move_tree glade 外 2 个 8B 事件身份。
3. `HideTreesinAreaCmd` / `HideTreesNextToWallCmd` 载荷字段语义；隐藏标志位图与 `iter_removed_ids` 的关系；
   hide_trees @+0x24 门控字节含义。
4. `convert_preplaced_trees` 完整 FunctionSystem 签名 / arg2 事件类型 / arg3、arg7 资源归属；`<0.6/1.0` 常数语义。
5. `tree_move_bend` 的 `time_continuous_lerp` rate 精确值（5.0 vs 3.22，与既有草稿不一致）+ 3.22 常数来源。
6. `ActiveSession[+0x58]` 对象实际 Rust 类型（三方法同址）；`TreeAnimation` f32@+4 字段语义；
   `TreeMoveCache` 布局；`EraserSizeMultiplier` 语义。
7. SpawnPrefabCmd / InsertTreeCmd 部分字段语义；`TreeStorage::insert` 内部 hash 插入调用语义。
8. `Update` add_systems 元组内系统精确排列顺序。
9. `recalc` ±65 边界语义；`reset` 默认 TreeStorage 的 0x143034a00 内容。

## 完全确定、可附最小 Rust 实现草案的函数

以下内部逻辑已完全确定且不需要新 core 接口（仅依赖已定位的数据布局）：`despawn_trees`、
`recalc_tree_position_on_terrain_height_change`、`tree_move_bend`（`time_continuous_lerp` rate 除外）。
草案见下（仅示意，不写进工程）：

```rust
// despawn_trees（src/process_tree_cmds.rs）
// 签名：Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)>, EventWriter<DespawnPrefabCmd>
fn despawn_trees(
    q: Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)>,
    mut despawn: EventWriter<DespawnPrefabCmd>,
) {
    for (entity, anim, _marked) in &q {
        if anim.progress < 0.01 {          // f32 @+4，严格小于（O）
            despawn.write(DespawnPrefabCmd { entity });
        }
    }
}
```

```rust
// recalc_tree_position_on_terrain_height_change（src/recalc_tree_position_on_terrain_height_change.rs）
// 签名：Query<&mut PrefabInstanceData>, Res<TerrainHeightsData<TerrainHeightSsbo>>, EventReader<OnTerrainChange>
fn recalc_tree_position_on_terrain_height_change(
    mut q: Query<&mut PrefabInstanceData>,
    heights: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    mut events: EventReader<OnTerrainChange>,
) {
    if events.is_empty() { return; }
    for mut data in &mut q {
        let p = data.position();
        if p.x.abs() <= 65.0 && p.z.abs() <= 65.0 {     // O
            let h = heights.sample_at_world_xz(p.x, p.z);
            if data.y != h { data.y = h.max(0.0); }      // O
        }
    }
}
```

```rust
// tree_move_bend（src/tree_move_bend.rs）—— rate 常数待 radare2 复核（U）
// 签名：Query<(&mut TreeMoveBend, &mut PrefabInstanceData)>, Res<GameTime>
fn tree_move_bend(
    mut q: Query<(&mut TreeMoveBend, &mut PrefabInstanceData)>,
    time: Res<GameTime>,
) {
    let delta = time.delta();                            // f32 @+0x28
    for (mut bend, mut data) in &mut q {
        let target = data.position();                    // x/z
        bend.x = time_continuous_lerp(bend.x, target.x, delta, RATE);  // RATE=U
        bend.y = time_continuous_lerp(bend.y, target.y, delta, RATE);
        let dx = bend.x - target.x;
        let dz = bend.y - target.z;
        let dist = (dx * dx + dz * dz).sqrt();
        if dist >= 0.1 {                                 // O
            let step = dist.min(5.0);
            let sx = step * dx / dist;
            let sz = step * dz / dist;
            bend.x = target.x + sx;
            bend.y = target.z + sz;
            if data.step_x != sx || data.step_y != sz {  // +0x24/+0x28（O）
                data.mark_changed();
                data.step = pack(sx, sz);
            }
        }
    }
}
```

（其余函数受内联干扰或仍含 U 项，暂不附草案。）
