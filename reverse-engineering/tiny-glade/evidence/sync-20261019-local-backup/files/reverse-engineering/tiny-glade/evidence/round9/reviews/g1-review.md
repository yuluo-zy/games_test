# G1 存档/读档主链 — Round 9 ③ Review

- 审查对象
  - 契约：`evidence/round9/contracts/g1-snapshot-apply-CONTRACT.md`（2026-10-19 11:49）
  - 交付：`evidence/round9/deliveries/g1-snapshot-apply-DELIVERY.md`（2026-10-19 12:08 更新，含 G1③ 修复）
  - 实现：`crates/country-core/src/systems/save/load_game.rs`（内容定稿 12:06:53）、
    `crates/country-core/src/systems/save/snapshot.rs`（内容定稿 12:05:20）、
    `crates/country-core/src/systems/save/save_thread.rs`（内容定稿 12:03:57）
- 审查方式：只读；零内容修改（为绕过共享 `target/` 的旧测试二进制，复评时仅
  `touch` 了两文件 mtime 强制重编，源码字节未变）。
- 审查快照：增量复评基于 2026-10-19 12:06 的实现内容；行号以强制重编后（12:19）源码为准。

## 结论

**通过**（增量复评后）。首轮报告的 1 个阻断（`"Stairs"` 桶地址）与 3 个高/低优先级
问题（M1/M2/M3/M7）均已修复并复验；`cargo check -p country-core` 退出 0；
`cargo test -p country-core --lib systems::save` 真实执行 **49 passed / 0 failed**；
36 样本语料测试（含 `"Stairs"` 桶）真实运行并通过，计数 162/73/134/431/0/20/590、
104 点墙、5 条 seed≠key、50 条缺 prefab 全部命中。

水分计数：**2**（首轮的 W2 文档错误已修复；W1/W3 为同一个非生产旧入口的遗留问题，
均不影响读档主链）。

仍余非阻断项（不计入水分）：AutoClutter 未应用/未登记（M4）、DirtyTiles 发布缺口语料
报告未进 `gaps`（M5）、旧 `apply_loaded_snapshot` 未标 deprecated（M6）、若干零调用
helper（M8）、以及终态失败未向宿主发信号（R1，跨组接缝，见 §6.1）。

## 1. 水分检查

### 1.1 计数项（2）

| # | 位置 | 问题 | 判定 |
|---|---|---|---|
| W1 | `snapshot.rs:1354` `apply_loaded_snapshot` | 公开的“装载面”，生产调用者为 0（仅 `snapshot.rs:3006` 旧测试）；对 Roofs/WallColors/GateColors/HalftimberColors/Trees 只走 `_ => log::trace!`（`:1462`），且签名返回 `()`，失败不可见。交付 §1 已注明“仅测试/旧调用者”，但该函数仍是同一模块内第二套 apply 语义。 | 非生产路径的 log-only apply；保留须显著标 deprecated |
| W2 | ~~`snapshot.rs:1930`~~ | ~~文档仍写“快照 JSON **无 Stairs 节**”~~ **已修复**：现为 `:1146` “14 节捕获面；真实 17 版样本无此节”，与样本/写出侧一致。 | 已关闭 |
| W3 | `snapshot.rs:1462` | 旧 `apply_loaded_snapshot` 的 fallback log 写“由 init_game_state_from_snapshot (World) 处理”，但该函数**没有**调用 World 版；读者会误以为域已应用。 | log 冒充接线 |

### 1.2 “函数存在但无生产调用者”清单（不计入水分，但须登记）

| 函数 | 位置 | 生产调用者 |
|---|---|---|
| `apply_loaded_snapshot` | `snapshot.rs:1354` | 0（仅测试） |
| `try_chunk` | `snapshot.rs:1245` | 0（`load_snapshot_file` 改为直接消费 `Chunk`；仅测试） |
| `parse_wall_section` / `encode_wall_record` | `snapshot.rs:2031` / `:2052` | 0（读档已改语义路径；文档已注明） |
| `collect_trees` / `collect_ids` | `snapshot.rs:83` / `:147` | 0 |
| `write_loading_tmp` | `load_game.rs:824` | 0 |
| `save::load_game::load_game_session`（纯核心） | `load_game.rs:782` | 0（实际调度的是 `startup::startup_session::load_game_session`） |

### 1.3 取值/常量核对（未发现编造）

- `DECORATOR_HASHER_SEEDS`（`snapshot.rs:2094`）：本审查独立读取 B EXE
  `.rdata` VA `0x1429cd080` 32B，得到
  `0x736f6d6570736575,0x6c7967656e657261,0x646f72616e646f6d,0x7465646279746573`，
  与代码/契约 §5.4 完全一致。**确认它不是 ID 掩码**；`SNAPSHOT_ID_MASK`、
  `encode/decode_snapshot_id` 已从代码删除（`:2086-2099` 仅留说明）。
  （命名“aHash seeds”属 I：字节与用途 O，hasher 家族名 I。）
- `DECORATOR_OUTER_RECORD_SIZE=0x28` / `INNER=0x58`（`:2101-2106`）与契约 §5.1 一致。
- `Roof` 的 88B serde 偏移由既有 `resources/roofs/roof_shape.rs` 提供，本组未改；抽查
  真实样本 73/73 条必填字段齐全。
- 未发现固定返回值/恒 `None`/伪造字段名。`parse_trees_section` 的默认值补齐是对样本
  50/590 条缺字段的忠实处理（见 §3）。

## 2. 读档链是否真的通

实际调用链（当前工作树）：

```
Update / in_state(Game)                       tiny-glade/src/run_game.rs:463
└─ load_game_system                         load_game.rs:996
   └─ try_load_game_system                  load_game.rs:1000
      ├─ AppMode::native_tag()==14           :1001
      ├─ state==Loading
      ├─ parse_saved_game_data                :1023
      ├─ load_closest_valid_snapshot          :1039 (def :249)
      │  ├─ nearest_snapshot_index            resources/history/snapshots.rs:154
      │  ├─ File::open {id}.snapshot          :277
      │  └─ load_snapshot_file                snapshot.rs:1170
      │     ├─ motomoto::Reader chunks
      │     ├─ validate_and_load_closest_snapshot_data   snapshot.rs:1035
      │     ├─ raster chunks (u64 len + deflate)         :1195-1208
      │     └─ Terrain chunk (deflate)                   :1180-1183
      ├─ apply_history_without_snapshot       :1060 (def :847；不再改 state)
      └─ init_game_state_from_snapshot(world) :1067
         │  （闭包 Ok 后 :1101 置 ApplyCounter；Err → :1107 Failed）
         └─ apply_loaded_snapshot_to_world    snapshot.rs:1515
            ├─ Walls→load_snapshot_walls      :1533/:1979
            ├─ Roofs→world.spawn(Roof)        :1550
            ├─ Decorators→restore_snapshot_with_id_gen  :1577
            ├─ colors→write_color_table       :1622
            ├─ Terrain strokes/heights/range  :1646 附近/1658-1695
            ├─ Trees→entities+TreeStorage+SpawnPrefabCmd :1697
            ├─ colors→write_color_table       :1622
            ├─ Terrain strokes/heights/range  :1646 附近/1658-1695
            ├─ Trees→entities+TreeStorage+SpawnPrefabCmd :1697
            ├─ Raster→write_named             :1775
            └─ Stairs / StairsRemovedSupports :1823/1855
```

- **旧 bail 已消除**：`require_no_valid_snapshot` 已更名 `load_closest_valid_snapshot`
  （`load_game.rs:246`），打开成功不再 `bail!`，而是加载并返回 `Some(LoadedSnapshotData)`。
  当前 `load_game.rs` 余留 `bail!` 只在版本区间（`:378/:383`）、无 source（`:1024`）与
  未接入的 post-load 域命令（`:1142`），与快照读取无关。
- **失败路径**：快照解析失败 → `with_context` fatal，不回退历史链（符合契约 §1.1/§1.3）；
  必需资源缺失 → `anyhow::ensure!`/`Err`；可选宿主面缺失 → `SnapshotApplyReport::gaps`
  并在 `load_game.rs:1072` 附近 `log::warn!`。**不存在“只写 log 就宣告域已应用”的生产路径**
  （旧 `apply_loaded_snapshot` 除外，见 W1）。
- ~~两个失败路径缺陷~~ **已修复（12:03 实现）**：
  1. `apply_history_without_snapshot` 不再改状态；`try_load_game_system` 把整个
     Loading 分支包进闭包，`Ok(())` 后才置 `ApplyCounter`（`load_game.rs:1099-1103`）。
  2. `Err` 分支置终态 `LoadGameState::Failed`、清 source/options/post-load 命令，
     只 `log::error!` 一次并返回 Err（`:1104-1115`）；后续帧 `Failed` 分支直接 `Ok`
     （`:1118-1121`），不再重复解析同一坏文件。`advance_load_game` 对 `Failed` 返回
     明确错误（`:737-739`）。

## 3. 域覆盖（9 + Stairs）

| 域 | 状态 | 证据 | 备注 |
|---|---|---|---|
| Walls | **真实写回** | `snapshot.rs:1533` → `load_snapshot_walls:1979`：empty 断言、`new_wall_with_id`、`changes.created`、`max_id`；点列 `WallState` 直接保留（契约 §0.1） | 语义路径已替换 0x78B 字节记录，`encode_wall_record` 不再进入读档 |
| Roofs | **真实 spawn（部分）** | `:1550` `world.spawn(roof.clone())`；现有实体未清理 → gap；`RoofAssociations` 未重建 → gap | 契约 §2.3 建议的关联表缺 API（文件权外），交付已登记 |
| Decorators | **真实写回** | `:1577` `restore_snapshot_with_id_gen`；`to_history_state:2461`（seed@+0x2c）；`"Stairs"` 串已接受（`:2173`） | 5/36 文件的 Stairs 桶现可解析；语料测试通过 |
| WallColors | **真实写回** | `write_color_table:1867` / `apply_color_rows:1910`，越界 Err | 未先清表；fresh SubApp 前提成立 |
| GateColors | **真实写回（形状同构）** | 同上；36 样本全空 | 取值面未观测，交付标 I |
| HalftimberColors | **真实写回** | 同上；样本 20 行 | 契约 §3.1 |
| Terrain strokes | **真实写回** | `TerrainSectionJson::into_editing_data` → `TerrainEditingData` | 76 strokes 全部字段齐全 |
| Terrain heights/range | **真实写回** | `loaded.terrain` → `TerrainHeightsData<TerrainHeightSsbo>`；`set_height_range` | chunk 实测 524288B；`DirtyTiles` 未发布且未写入 `gaps`（仅交付 §4 记录） |
| Trees | **真实 spawn + 索引** | `parse_trees_section:103` 补默认；实体 3 组件 + `TreeStorage` + `SpawnPrefabCmd`；空 prefab → gap | 590/590 条必填字段齐全；50 条缺 prefab 字段被正确补默认 |
| Raster | **真实写回** | `AllWorldRasterWrite::write_named`；chunk 名按 JSON 名表顺序消歧 | 实测每槽 262144B；6 资源任一缺失 → gap |
| Stairs | **真实写回** | `:1823` `StairsSnapshot::new` → `restore_snapshot`；checksum 不一致 → gap | 真实样本 `{"edges":[[0,1]],"checksum":<u64>}` |
| StairsRemovedSupports | **真实写回** | `:1855` 清空后逐边 `record` | 样本 `{"edges":[]}` |
| AutoClutter | **仅解析，未应用，未登记 gap** | `SNAPSHOT_SECTION_ORDER:164`；无 apply 分支 | 契约 §1.2 原生 apply 序也未列；建议至少登记 |
| Version | **门禁** | `validate:1022-1025` 要求 ==2 | 36 样本均 2 |

## 4. 与原证据一致性（契约核对）

| 契约条目 | 实现 | 判定 |
|---|---|---|
| §0.1 无 0x70B 内联打包，语义 `Vec<(WallId,WallState)>` | `load_snapshot_walls`（`:1979`）；点列保留 | **已落地** |
| §0.2 `_UNK_1429cd080/090` 是固定 hasher seeds，非 ID 掩码 | `DECORATOR_HASHER_SEEDS`（`:2094`）；rotate/XOR 编码已删 | **已落地**（本审查独立读 EXE 复核字节） |
| §0.3 内层 0x58 不是 0x16B | 常量 `:2106`；文档修正 | **已落地** |
| §5.2 `seed` 写 +0x2c，不是桶键 | `to_history_state:2483` `color_id \| seed<<32` | **已落地**；5 条 seed≠key 由样本复核 |
| §3.2 Trees 缺 `prefab_name`/`is_preplaced` 取默认 | `parse_trees_section`（`:103`）解析期补齐（不改文件权外的 `tree_edit.rs`） | **已落地** |
| §5.5 `chimney_h_offset` alias | `DecoratorInfoJson.h_offset`（`:2256` 附近） | **已落地** |
| §1.1 打开成功即权威、解析失败 fatal、不得回退 | `load_game.rs:1039`；无 bail | **已落地** |
| §2.3 Roofs → spawn（+建议关联表） | spawn；关联表 gap | 部分（已登记） |
| §4.2 WallState 偏移表 | 语义路径不再依赖字节偏移；`encode_wall_record` 保留并标注不使用 | 一致 |
| §3.1 三色表同构 | 三分支同构 | 一致 |
| §7-12 36 样本回归计数 | 测试已落盘（`:3140` 附近） | 计数与契约一致（本审查独立复核全部命中），**但当前代码会在 `"Stairs"` 地址处 panic** |

### 4.1 独立样本复核（Python 只读，与测试同口径）

36 个 `.snapshot`（`assets/starting-builds/{demo,full}`）逐项统计：

| 指标 | 复核值 | 契约/测试期望 |
|---|---|---|
| walls | 162（canonical `WallState` 必填字段 162/162 可解析） | 162 |
| roofs | 73（必填字段 73/73） | 73 |
| decorators | 134（必填 `ty`/`dst` 134/134） | 134 |
| wall colors / gate / halftimber | 431 / 0 / 20 | 431 / 0 / 20 |
| trees | 590（必填 590/590；50 条缺 prefab 字段） | 590 |
| max curve points | 104 | 104 |
| seed ≠ bucket key | 5 | 5 |
| 缺 prefab 字段 | 50 | 50 |
| Raster chunk | u64 原长 + raw deflate，262144B | 262144B |
| Terrain chunk | raw deflate，524288B | 0x80000B |
| **Decorators 桶地址字符串** | `"Terrain"` 32、`{"Wall":id}` 129、**`"Stairs"` 5** | 首轮实现只接受前两者；**12:03 已补 `"Stairs"`（M1）** |

`"Stairs"` 影响的文件：`17642530676616200664`（空桶）、`16742947640211348660`（2 条）、
`2984846468061317468`（2 条）、`1536850669530566155`（4 条）、`13968967360472310879`（1 条）。
首轮整节反序列化失败 → `validate` fatal；增量复评时已修复，语料测试 36/36 通过。

## 5. 编译与测试（增量复评实测）

- `save_thread` 布局：`UnsavedFileEntry.path` 已改为 24B `String`（`save_thread.rs:47-55`），
  `const _: () = assert!(size_of::<UnsavedFileEntry>() == 0x80);` 已恢复（`:67`）且通过；
  Windows 0x88 阻塞解除。临时注释版断言已不存在。
- `cargo check -p country-core`：**退出 0**（仅既有 warning，无 error）。
- `cargo test -p country-core --lib systems::save -- --test-threads=4`：
  **49 passed / 0 failed / 0 ignored**（真实执行）。
  含 `decorator_addr_accepts_terrain_and_stairs_string_forms`、
  `world_apply_restores_walls_roofs_colors_trees_and_raster`、
  `load_snapshot_walls_preserves_heap_curve_points`、
  `decorator_seed_is_written_to_state_plus_0x2c_not_the_bucket_key`。
- `shipped_snapshot_corpus_parses_expected_domain_counts`：**单独 `--exact --nocapture`
  实跑通过**（未打印 skip 信息，即 36 个真实 `.snapshot` 全部解析成功），
  计数断言 162/73/134/431/0/20/590、104 点、5 条 seed≠key、50 条缺 prefab 全部命中。
- 复现注意：首次复跑时共享 `target/` 曾给出**旧测试二进制**（列出的仍是已删除的
  `snapshot_id_encode_decode_roundtrip` 等旧测试）；`touch` 源文件强制重编后，
  snapshot 模块 15 个测试（含语料测试）与 `systems::save` 49 个测试全部通过。
  说明共享 target 下以 mtime/fingerprint 判新旧可能不可靠，验收应以强制重编结果为准。
- 首轮 `world_apply_...` 的失败是旧测试二进制（改动中途编译）造成的假象；强制重编后
  单跑与全套均通过。

## 6. 缺口继承（交付 §4 对照）

| 交付声明“仍缺” | 与代码/证据一致性 |
|---|---|
| RoofAssociations 未重建 | 一致；`report.gaps` 有明确条目（`snapshot.rs:1568-1575`） |
| 树缺失 prefab 回退 | 一致；空 prefab 条目记 gap，不发 spawn |
| `DecoratorHistoryState::to_info` 对 Clutter 清零扩展簇 | 一致（文件权外）；交付已登记 |
| `tree_edit.rs` serde default | 一致；用解析期补齐规避文件权 |
| GateColors 非空取值面 | 一致（36 样本 0 行） |
| 地形 DirtyTiles 发布 | 交付登记，但**未写入 `SnapshotApplyReport::gaps`**（代码无提示） |
| Roofs 0x40 / Trees 0x48 / descriptor 逐名 / WallState +0x30、+0x6a..0x6f | 一致（契约 §6 U） |
| 样本计数回归未执行 | 首轮属实；**增量复评已真实执行并通过 36/36**（含 `"Stairs"` 桶） |

未发现把缺口悄悄标成完成的情况；`gaps` 机制整体诚实。

### 6.1 增量复评逐项（2026-10-19 12:05）

| 首轮问题 | 复评结果 | 证据 |
|---|---|---|
| **M1** `"Stairs"` 地址被拒 | **已修复** | `snapshot.rs:2173` `"Stairs" => Ok(DecoratorAddrJson::Stairs)`；`to_storage_addr` 映射 `DecoratorStorageAddr::Stair`；测试 `decorator_addr_accepts_terrain_and_stairs_string_forms` + 36 样本语料测试通过 |
| **M2** ApplyCounter 提前推进 | **已修复** | `apply_history_without_snapshot` 不再改状态（`:998-999` 注释）；成功后 `load_game.rs:1101` 置 `ApplyCounter` |
| **M3** 失败每帧重试 | **已修复** | `load_game.rs:1104-1116` 进入 `Failed` 并清请求；`:1118-1121` 终态直接返回 Ok；`advance_load_game:737` 拒绝 Failed |
| **M7** “无 Stairs 节”旧文档 | **已修复** | `snapshot.rs:1146` 改为“14 节捕获面；真实 17 版样本无此节” |
| **M9** Windows 0x80 布局 | **已修复** | `path: String`（24B）+ 断言恢复并通过；check/test 真实编译执行 |
| 语料测试 | **通过** | 49/49 `systems::save` 测试通过；语料测试 36 文件实跑 |

残留（非阻断）：
- **R1**：终态 `Failed` 只返回 Ok/日志，未发 `DisplayCorruptedSaveFileWarning` 或其它
  终端事件；宿主 `GameplayPreparation` 只等 `LoadingGameComplete`，失败时可能长时间
  空转（`tiny-glade/src/run_game.rs:880-891`，跨组接缝，G1 文件权内无法单独闭环）。
- **R2**：`LoadGameState::Failed = 3` 是 crate 侧终态扩展，原生 state 字节只证 0/1/2；
  已在枚举注释标 I，后续类型合同需保留该说明。
- M4/M5/M6/M8 见 §7，均为非阻断。

## 7. 问题清单（增量复评后状态）

| ID | 级别 | 状态 | 位置 | 问题 | 期望改法 / 复评证据 |
|---|---|---|---|---|---|
| M1 | **阻断** | **已修复** | `snapshot.rs:2173` | `visit_str` 曾拒绝 `"Stairs"`（真实样本 5 文件） | 已加 `"Stairs" => DecoratorAddrJson::Stairs` + `to_storage_addr`；新增反序列化测试与 36 样语料测试通过 |
| M2 | 高 | **已修复** | `load_game.rs:998-999`、`:1101` | 状态曾在快照 apply 前推进 | `apply_history_without_snapshot` 不再改状态；`Ok` 后统一置 `ApplyCounter` |
| M3 | 高 | **已修复** | `load_game.rs:1104-1121` | 解析失败曾每帧重试 | `Err` 转 `Failed` 终态、清请求、只报一次；`Failed` 分支直接 Ok |
| M4 | 中 | 未修（非阻断） | `snapshot.rs:164`、`apply_loaded_snapshot_to_world` | `AutoClutter` 解析但未应用，也未进 `gaps` | 若原生 init 不应用则登记；若应恢复则接 `DeletedAutoClutter` 所有者 |
| M5 | 中 | 未修（非阻断） | `snapshot.rs:1658-1695` | DirtyTiles 发布缺口只在交付文档，不在 `report.gaps` | 高度写回后 push 一条 gap，或实现发布器入口 |
| M6 | 低 | 未修（非阻断） | `snapshot.rs:1354` / `:1462` | 旧 `apply_loaded_snapshot` 零调用且日志误导 | 标 deprecated/doc(hidden) 或删除并迁移旧测试 |
| M7 | 低 | **已修复** | `snapshot.rs:1146` | 旧文档称“快照 JSON 无 Stairs 节” | 改为“14 节捕获面；真实 17 版样本无此节” |
| M8 | 低 | 未修（非阻断） | `snapshot.rs:1245`、`:2031`、`:2052`、`:83`、`:147`；`load_game.rs:824`、`:782` | 多个零生产调用 helper | 标“非读档路径/兼容 API” |
| M9 | 测试 | **已修复** | `save_thread.rs:47-67` | 测试目标被 Windows 0x80 布局阻断 | `path: String`（24B）+ 断言恢复；check/test 真实执行 |
| R1 | 中（跨组） | 待中央 | `tiny-glade/src/run_game.rs:880-891` | `Failed` 终态不发事件，宿主只等 `LoadingGameComplete`，坏档时可能空转 | 由 G1 发 `DisplayCorruptedSaveFileWarning`（或专用失败事件）并由宿主消费 |
| R2 | 低 | 已注明 | `load_game.rs:44-53` | `Failed=3` 是 crate 扩展，非原生 state 字节 | 保留 I 注释，类型合同登记 |

## 8. 给 ① 的下一步取证方向（按收益排序，≤5）

1. **Decorators 桶地址 JSON 完整枚举 + 原生 serializer 字符串表**：契约 §5 给出了
   state 偏移，但未列外层地址的 wire 形式。已实证 `"Terrain"` / `"Stairs"` / `{"Wall":id}`
   并被实现接受；仍待确认是否存在带 payload 的 Stair 地址（crate 模型是单元变体）及空桶
   语义，以判定多楼梯地址是否丢失。**影响 5/36 样本的楼梯桶归属。**
2. **RoofAssociations 0x40B 中间条目逐槽语义**：契约 §2.3 只到“spawn 等价（I）”，
   `RoofAssociations::rebuild` 的输入行与 `PublicWallBuildingState` 关系未定；补齐后可
   去掉当前每条 load 必现的 Roofs gap。**收益次高：影响所有 73 个屋顶的后续编辑/渲染。**
3. **`DecoratorHistoryState::to_info` 的 Clutter 扩展簇（0x34..0x4e）**：交付已暴露
   `ty<0x38` 时 h_offset/ladder/magic/railing/style 被清零；需 0x140a20d70 之后
   `to_info`/渲染消费侧的类型证据，判定哪些槽对 Clutter 有效。**影响 6/134 条样本装饰，
   但属真实状态丢失。**
4. **`AutoClutter` 在原生 load 链的归属**：`init-1417a9c50.c` apply 序未出现该节，
   但 capture 写出；确认它是由别的系统（clutter 删除集）消费还是原版 init 漏列。**决定
   M4 是“登记”还是“实现”。**
5. **Stairs checksum 缓存语义 + Terrain DirtyTiles 发布面**：样本 checksum 与边派生值
   是否恒等（当前不等即 gap）；以及高度写回后应发哪些 dirty tiles（字段私有，需要
   `Publisher<DirtyTiles>` 的消费者契约）。**影响地形/楼梯的增量更新正确性，非首帧可见。**

## 9. 验收门（增量复评结论）

- M1/M2/M3/M7/M9 已修复并复验；`cargo check` 退出 0，`systems::save` 49/49 测试通过，
  36 样本语料测试真实通过 → **G1 读档主链本轮验收通过**。
- 后续（不阻断本轮）：
  1. R1 跨组接缝：终态失败事件/宿主退出等待，需中央协调 G1（发事件）与 tiny-glade 宿主（消费）。
  2. M4/M5 缺口登记；M6/M8 标注或清理。
  3. ① 的 §8 五项取证继续按收益推进。
