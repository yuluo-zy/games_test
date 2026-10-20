# G1② 交付 — 读档快照真正应用（Round 9）

> 实现 agent G1②。文件权：`crates/country-core/src/systems/save/{load_game.rs,snapshot.rs}`。
> 契约：`evidence/round9/contracts/g1-snapshot-apply-CONTRACT.md`（2026-10-19）。
> 基线：B patched EXE + 36 个发行 `.snapshot`（`assets/starting-builds/{demo,full}`）。

## 1. 接线结果（真实生产路径）

| 步骤 | 位置 | 说明 |
|---|---|---|
| 选择最近的可用快照 | `load_game.rs:246` `load_closest_valid_snapshot` | 原 `require_no_valid_snapshot` 的 `bail!` 已删除。打不开 → warning + 下一条；**打开成功 → 解析失败 fatal**（`with_context`，不回退历史链）。`ignore_snapshot` 保留原版 ignore 分支。 |
| 文件 → 解析数据 | `snapshot.rs:1170` `load_snapshot_file` | motomoto chunks：首个 `"Snapshot"` JSON → `validate_and_load_closest_snapshot_data`；命名栅格 chunk（u64 原长 + deflate）→ `rasters`；`"Terrain"` chunk（deflate）→ `terrain`（0x80000B）。栅格名按 JSON `Raster` 名表顺序消歧（8B tag 截断碰撞）。 |
| 解析后装配 | `load_game.rs:1028-1050` | 先 `apply_history_without_snapshot`（相机/时间/颜色/History/保存 owner，原版无快照分支同一基础面），再在 `snapshot Some` 时调 `init_game_state_from_snapshot`；历史 `max_wall_id` 在快照 `max_id` 之后取大值，避免回退。 |
| 域写回主链 | `snapshot.rs:1332` `init_game_state_from_snapshot` → `snapshot.rs:1515` `apply_loaded_snapshot_to_world` | World 级入口；生产调用点为 `load_game.rs:1050`。 |
| 纯资源兼容面 | `snapshot.rs` `apply_loaded_snapshot` | 仅测试/旧调用者；Walls 已同步走语义路径。 |

`gaps` 非空时 load 路径 `log::warn!` 逐条输出；不宣告完整加载。

## 2. 逐域 apply 目标

| 域 | 目标（真实写回） | 契约依据 / 状态 |
|---|---|---|
| **Walls** | `load_snapshot_walls`（`snapshot.rs:1979`）：`Vec<(WallId, WallState)>` 语义插入，复刻 0x14085ad80 的三条 empty 断言、`changes.created`、HashMap 重建、`max_id=最大 id`；点列（堆 Vec）完整保留 | G1① §4：**无 0x70 内联打包**。用 `InnerWalls::new_wall_with_id`（0x14085a4a0）完成显式 id 插入（resources/walls 属用户重构区，未改）。重复 id → `Err`（不 panic/不静默覆盖）。 |
| **Roofs** | `world.spawn(Roof)`（每个 88B 组件实体） | G1① §2.3：原版 0x40B 中间条目逐槽 U；spawn 等价 I。shape tag≥2 由 `Roof::Deserialize` 拒绝（validate 即 fatal）。 |
| **Decorators** | `DecoratorStorage::restore_snapshot_with_id_gen`（0x140a20d70） | JSON `DecoratorInfo` → `DecoratorHistoryState` 0x50B；`seed` 写 +0x2c（**不是桶键**，`snapshot.rs` `to_history_state`）；`chimney_h_offset` 作为 `h_offset` alias。 |
| **WallColors/GateColors/HalftimberColors** | `WallColorIds`/`GateColorIds`/`HalftimberColorIds` 的 `Vec<u32>` 稀疏写 | G1① §3.1：三段同构循环；越界 → `Err`（原版断言面 U）。 |
| **Terrain strokes** | `TerrainEditingData`（JSON 明文 strokes） | r7 §3 + G1① §1。 |
| **TerrainHeightRange / Terrain chunk** | `TerrainHeightsData<TerrainHeightSsbo>`：heights_mut 拷贝 512×512 u16 + `set_height_range` | 旧样本无 range 节时只写高度；长度不符 → `Err`。 |
| **Trees** | ECS 实体（`TreeComponent{index,is_preplaced}` / `TreeAnimation::preplaced()` / `TreeMoveBend`）+ `TreeStorage(index→entity)` + `SpawnPrefabCmd{prefab,entity,pos,scale,rot,variant,flags,mode}` | G1① §3.2 映射表。空 `prefab_name` 不编造名字：不发 spawn 命令，记 gap（缺失 prefab 策略属 tree-tool）。 |
| **Raster** | `AllWorldRasterWrite::write_named`（0x14083ebe0） | 6 槽整批校验；无 chunk 载荷时记 gap，不写。 |
| **Stairs / StairsRemovedSupports** | `StairsState::restore_snapshot`（0x1408ff590）/ `StairsRemovedSupports.edges` | 17 版样本无该节；14 节捕获面支持。保存 checksum 与边派生值不一致 → gap（不伪造缓存值）。 |

## 3. 契约纠偏落地

1. **Walls 语义路径**（契约 §0.1/§4/§7-1）：读档不再调用 `encode_wall_record`/`load_state`
   字节记录；`load_snapshot_walls` 直接插入 `WallState`。`parse_wall_section`/`encode_wall_record`
   作为 0x78B 字节记录 API 保留给其它调用者，文档标注“读档不使用”。
2. **Decorators 记录模型**（§0.3/§5.1/§7-8）：删除 `DecoratorSectionRecord`、
   `parse_decorator_section`、`pack_decorators_section`（0x28B 被当成内层记录的误模型）。
   常量改为 `DECORATOR_OUTER_RECORD_SIZE=0x28` / `DECORATOR_INNER_RECORD_SIZE=0x58`。
   内层按 `(DecoratorId u32 明文, DecoratorHistoryState 0x50)` 处理，**无 rotate/XOR**。
3. **ID 常量**（§5.4/§7-9）：`SNAPSHOT_ID_MASK=0` + `encode/decode_snapshot_id`（rotate32）
   已删除，替换为 `DECORATOR_HASHER_SEEDS: [u64;4]`（`somepseu|lygenera|dorandom|tedbytes`，
   0x1429cd080/090 的 aHash 固定种子）。
4. **Trees 缺省**（§3.2/§7-5）：`parse_trees_section` 在解析前补齐缺失的
   `prefab_name=""`/`is_preplaced=false`（36 样本 50/590 条），其余字段仍严格。
5. **chimney_h_offset**（§5.5/§7-10）：`#[serde(alias = "chimney_h_offset")]`，原生槽位 U 已注释。
6. **Version 门**：`validate_and_load_closest_snapshot_data` 现执行 `Version == 2` 校验
   （"Version mismatch; expected …, got …"），失败为 Err。
7. **`saved_wall_state.rs` 复核**（契约 §7-2，无需改动）：`points` 中间通道以
   `[x, _, z] → [x,z]` 丢弃且恒 0；`points_u` 原样保留不重算；`length` 直接使用缓存值；
   `has_skirts` alias 与 `style` 字符串（Stone/StoneWithSkirts/HalfTimbered）解析保持。
   36 样本 1870 点全为 `[x,0,z]`、`points/points_u` 等长（本机复核一致）。

## 4. 仍缺项（显式）

| 缺项 | 原因 / 归属 |
|---|---|
| `RoofAssociations` 未重建 | `associations.rs` 不在本组文件权；`rebuild` 需要 `PublicWallBuildingState` 行，无法从 `Roof` 组件独立构造。spawn 后每条 load 记 gap。 |
| 树缺失 prefab 回退 | `random_tree_prefab` 属 `systems/tree-tool`；空 `prefab_name` 条目只恢复实体/索引，记 gap。 |
| `DecoratorHistoryState::to_info` 对 Clutter（ty<0x38）清零扩展簇 | 缺陷在 `resources/history/decorator_edit.rs`（非本组文件权）。实际影响：Clutter 装饰的 h_offset/ladder/magic/railing/style 在 restore 时回落默认。已在 `to_history_state` 侧写全字段，待该 owner 修复 `to_info`。 |
| `tree_edit.rs` 的 serde default | canonical 修复点不在本组文件权；当前用 `parse_trees_section` 解析期补齐，等价于原版缺省行为。 |
| `GateColors` 非空取值面 | 36 样本恒空；代码与 WallColors 同构（O 结构 / I 取值）。 |
| 地形 DirtyTiles 发布 | 加载后直接写 `heights_mut` + range；未向 `Publisher<DirtyTiles>` 发布整图脏区（发布器字段私有）。下游以启动重建路径消费；登记 U。 |
| Roofs 0x40 中间条目逐槽 / Trees 0x48 目标记录逐槽 / descriptor 14 域逐名 / WallState +0x30、+0x6a..0x6f | 契约 §6 U 列表原样保留。 |
| 字节级样本计数回归 | 已在解除编译阻塞后执行并通过（见 §6）：36 文件、162 walls / 73 roofs / 134 decorators / 431 wall colors / 0 gate / 20 halftimber / 590 trees / 104 点墙 / 5 条 seed≠key / 50 条缺 prefab / 5 个 `"Stairs"` 桶。 |
| 过时文档引用 | `crates/systems/decorator/src/restore_archived_decorators.rs:13` 的模块注释仍写 “哈希 XOR 掩码 … = core SNAPSHOT_ID_MASK”。仅注释、无代码依赖；该文件不在本组文件权内，待其 owner 按 G1① §5.4 改为 aHash seeds。 |

## 5. 新增测试（`snapshot.rs` tests）

- `load_snapshot_walls_preserves_heap_curve_points`：语义插入保留 `[x,z]` 点列 / `points_u` /
  `max_id` / `changes.created`；第二次载入触发 empty 断言 Err。
- `decorator_seed_is_written_to_state_plus_0x2c_not_the_bucket_key`：key 9 / seed 18。
- `decorator_section_states_convert_to_native_history_states`：JSON ↔ state ↔ 写回侧投影。
- `decorator_hasher_seeds_are_the_shipped_a_hash_words`：32B 常量与 0x28/0x58 尺寸。
- `parse_wall_section_builds_all_proven_record_fields`：+0x60/+0x64/+0x65/+0x66/+0x67/+0x69。
- `world_apply_restores_walls_roofs_colors_trees_and_raster`：World 级真实写回。
- `shipped_snapshot_corpus_parses_expected_domain_counts`：36 样本扫描，
  162 walls / 73 roofs / 134 decorators / 431 wall colors / 0 gate / 20 halftimber /
  590 trees / 104 点墙 / 5 条 seed≠key / 50 条缺 prefab 字段 / 5 个 `"Stairs"` 桶；
  资产目录缺失时跳过。（计数用同一批发行文件独立复核：
  `assets/starting-builds/**/snapshots/*.snapshot`。）
- `decorator_addr_accepts_terrain_and_stairs_string_forms`：`"Stairs"` 单元串 + 旧对象判别（M1）。
- `load_game.rs` `state_machine_tests::{load_failure_is_terminal_and_reported_once,
  apply_counter_is_only_reached_after_a_successful_load}`：M2/M3 状态机。
- `save_thread` `unsaved_file_list_uses_the_native_0x80_byte_entry`：24B `String` 路径后断言 0x80。

## 6. G1③ 复核修复（M1/M2/M3/M7 + 编译解阻塞）

| 项 | 修复位置 | 内容 |
|---|---|---|
| **M1 阻断** | `snapshot.rs:2139` / `:2173` / `to_storage_addr` `:2211` | `DecoratorAddrJson` 新增单元变体 `Stairs`；`visit_str` 接受 `"Terrain"`/`"Stairs"`；`to_storage_addr` 映射到 `DecoratorStorageAddr::Stair`；旧对象 `{"Stair": id}` 仍兼容。样本 5/36 文件（9 条桶地址）不再使整节 fatal。 |
| **M2 高** | `load_game.rs:1010-1121` | `apply_history_without_snapshot` 不再写 `state`；只在快照与全部域写回成功后才置 `ApplyCounter`。任何 `?` 失败进入 `LoadGameState::Failed`（新终态=3）并清空 source/options/命令队列，不再走 `ApplyCounter→Loaded`。 |
| **M3 高** | 同上失败分支 | 首次失败记录 `log::error!` 并返回 Err（上报一次）；`Failed` 分支后续帧直接 `Ok(())`，不会每帧重试同一坏文件。`advance_load_game` 对 `Failed` 返回显式 Err。 |
| **M7 低** | `snapshot.rs` `apply_stairs_writeback` 文档 | 改正为：14 节捕获侧会写 `Stairs`/`StairsRemovedSupports`；17 版发行样本无这两节，内存路径仍由宿主接入。 |
| **编译阻塞** | `save_thread.rs:42-67` | `UnsavedFileEntry.path` 改为 24B `String`（`to_string_lossy().into_owned()`），`contains_file` 用 `Path::new(&candidate.path)`；`opaque: [u8;0x68]` 保留；`const _: () = assert!(size_of::<UnsavedFileEntry>() == 0x80);` **恢复且未删除**。注释说明原生 0x80 记录与 Windows std `PathBuf`（32B）差异。 |
| **补测** | `snapshot.rs` tests + `load_game.rs` `state_machine_tests` | `decorator_addr_accepts_terrain_and_stairs_string_forms`（`["Stairs", {"state":[]}]` + 旧对象）；语料回归新增 `stair_buckets == 5`；`load_failure_is_terminal_and_reported_once`（M3）；`apply_counter_is_only_reached_after_a_successful_load`（M2）。 |

### 验证结果（本机）

- `CARGO_TARGET_DIR=/d/game/tiny-glade/target cargo check -p country-core` → **通过**（仅既有 warning）。
- `cargo test -p country-core --lib systems::save -- --test-threads=4` → **49 passed, 0 failed**
  （含 `unsaved_file_list_uses_the_native_0x80_byte_entry`、`shipped_snapshot_corpus_…`、
  `world_apply_restores_…`、`decorator_addr_accepts_terrain_and_stairs_string_forms`、
  两个 load 状态机测试）。

## 7. 原侧证据引用

- `contracts/g1-snapshot-apply-CONTRACT.md` §0/§1/§2/§3/§4/§5/§7。
- `evidence/round9/tools/init-1417a9c50.c`、`dec-0x140a20d70.c`、`dec-0x14085ad80.c`、
  `dec-0x1404f4c30.c`。
- `evidence/core-recovery-20261015-adv/round5/decomp-r5-0x1417a9c50.txt`（apply 顺序）。
- 真实样本：`assets/starting-builds/**/snapshots/*.snapshot`（36 个，Version=2）；
  本机复核：`PathRast/WaterRas/GardenRa` chunk = u64 原长 + raw deflate（262144B），
  `Terrain` chunk = raw deflate 524288B。
- `crates/country-core/src/resources/walls/{saved_wall_state.rs,inner_wall_state.rs,inner_walls.rs}`
  （`WallState` Deserialize / `from_record_bytes` / `new_wall_with_id`）。
- `crates/country-core/src/resources/roofs/roof_shape.rs`（Roof Serialize/Deserialize 0x1416ea700 /
  0x1415c5460）。
