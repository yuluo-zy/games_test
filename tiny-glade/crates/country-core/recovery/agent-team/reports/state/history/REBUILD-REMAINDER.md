# REBUILD-REMAINDER — history 模块剩余项重建（2026-10-17）

> restorer: impl-history-remainder，2026-10-17。依据 2026-10-16-logic-verify-protocol.md。
> 对 LOGIC.md/REBUILD.md 登记为「未重建」的项做 fresh 反编译（shared worker，batch 1-2，
> 落盘 `rebuild-history/decomp/` + `results/remainder-fresh.json`，含 disassembly/refs），
> 分类后按真实控制流实现或登记。红线遵守：只以 fresh 反编译为准；宏胶水不重建；
> 不删除已判等函数；不改他模块。

---

## 0. 分类结论

本批 5 项全部为**真实业务**（非宏胶水）。其中 serde 族为手写 `Serialize`/`Deserialize`
（含真实字段策略），按协议 §3 属"按业务重载"。

| 项 | 函数 | 分类 | 处置 |
|---|---|---|---|
| 1 | `StairsHistoryEdit` serde（0x140259af0/0x140a18ee0） | 真业务（手写） | **实现** |
| 2 | `history_process` 第二事件流（0x140af5b30，跳表 0x142ae5420） | 真业务 | **部分实现**（case 0-3 可解码；0xb..0x18+ U） |
| 3 | `replay_wall`/`WallReplayEditCmd::process` 17 操作（0x140951b80/0x140abfb30） | 真业务 | **登记**（跨模块，见 §3.3） |
| 4a | `replay_edit` Color/Decorator 展开（0x140af6170） | 真业务 | **登记**（跨模块，见 §3.4） |
| 4b | `replay_pop_bucket`（0x140b38250） | 真业务 | **登记**（跨模块，见 §3.2） |
| 4c | `get_param`（0x140ae5830） | 真业务 | **实现**（结构对齐；载荷面 U） |
| 4d | `missing_prefab`（0x1409e6210） | 真业务 | **实现** |
| 4e | `DecoratorHistoryState::from`（0x140b3f930） | 真业务 | **登记**（跨模块布局，见 §3.5） |
| 5 | `ColorEdit` 互异载荷类型面（0x14003c5d0） | 真业务 | **fresh 定案登记**（见 §3.6） |

---

## 1. 实现

### 1.1 `StairsHistoryEdit` 重载为原生单边枚举（0x140259af0/0x140a18ee0）

**fresh 定案**（disasm 0x140259af0，string refs @0x1429a4c82/0x1429a4c90/0x1429a4c94）：
原生为**单边枚举**，判别字节 bit0（`TEST byte ptr [RCX],0x1`），每变体持单个
`StairGraphEdge`（`ADD RSI,0x4` 定位 @+0x4）。两分支写同构 JSON：
`{"<变体>": {"edge": <StairGraphEdge>}}`：

| bit0 | 变体键（LEA 常量 / 长度） | 字段键（LEA 常量 / 长度） |
|---|---|---|
| 0 | "SupportsDelete"（0x1429a4c82，len 0xe） | "edge"（0x1429a4c90，len 4） |
| 1 | "SupportsRecreate"（0x1429a4c94，len 0x10） | "edge"（0x1429a4c90，len 4） |

deserialize（0x140a18ee0）为手写 visitor（`variant_seed` + `struct_variant`），
确认 2 变体 enum。

**实现**（`resources/history/stairs_history_edit.rs`）：
- `StairsHistoryEdit` 由 `{before: Vec<StairsEdge>, after: Vec<StairsEdge>, hash: u64}`
  平面结构重载为 `enum { SupportsDelete(StairGraphEdge), SupportsRecreate(StairGraphEdge) }`。
- 手写 `Serialize`/`Deserialize`：单 entry map `{"<变体>": {"edge": <edge>}}`。
- `StairsReplayCmd::expanded` 由 before−after 差集改为单编辑→单命令
  （`edit.edge().nodes()` → `vec![Self::new(a, b, undo, instant)]`）。
- `StairsDiff`（建模残留，native 无此型；`StairsHistoryDiff` 在 stairs_state.rs）
  删除；`StairsEdge` 别名保留。
- 测试 `stairs_history_edit_serde_matches_native_single_edge_shape`：断言
  `{"SupportsRecreate":{"edge":{"a":3,"b":4}}}` 形状。

**自检**：键名/长度与 disasm LEA 常量逐字对应；`StairGraphEdge` 内层形状由
stairs 模块 derive 持有（不改）。旧 src 的 `hash` 字段无任何读取方，删除安全。

### 1.2 `history_process` 第二事件流（0x140af5b30，跳表 0x142ae5420）——可解码分支

**fresh 定案**：`history_process` 主循环含第二事件流——16B 记录，判别字节 @+9，
`switch(*(byte *)(_Src + 9))` 跳表 0x142ae5420。可解码分支：
- case 0：队尾 `HistoryCmd` 第二字节==1（redo）→ 写事件 + 清队（code_r0x000140af5df8）
- case 1：队尾第二字节==0（undo）→ 写事件 + 清队
- case 2：无条件 → 写事件 + 清队
- case 3：`ReplayBuffer::make_animations_instant` + 写事件 + 清队

**默认路径**（未清队）：记录 @+8 的 u32（机器码 `_Src[2]`）压入
`PendingHistoryCommands`；每条记录处理后按 @+0xb 字节写事件（==3 跳过）。

**实现**（`systems/history.rs`）：
- `HistoryControlRecord { bytes: [u8; 16] }`（tag @+9、flag @+0xb）、
  `PendingHistoryControl` 资源、`HistoryControlProcessed` 事件、
  `process_history_control_stream` 系统（case 0-3 + 默认压入 + 事件写）。
- 已入 `register_history_events` 与 `HistoryCpuStage::Cursor` 链。
- 测试 `second_stream_control_records_dispatch_and_push_default_commands` /
  `second_stream_conditional_clear_only_matches_last_pending_command`。

**边界登记**：**产生端 U**（上游历史控制命令未接线）；case 0xb..0x18+（校验/动画
即时化/移除，含 DecoratorHistoryEdit Arc 计数移除、跳表外二级分派）反编译不收敛，
不编造，登记 U。

### 1.3 `missing_prefab`（0x1409e6210）——实现

**fresh 定案**：24B String 常量，机器码三 qword 字节序：
`0x5f474e495353494d`="MISSING_"，`0x4552505f45455254`="TREE_PRE"，
`0x454d414e5f424146`="FAB_NAME"；len/cap=0x18 → `"MISSING_TREE_PREFAB_NAME"`。

**实现**（`resources/history/tree_edit.rs`）：`pub fn missing_prefab() -> String`
返回 `"MISSING_TREE_PREFAB_NAME".to_string()`。测试断言常量。

### 1.4 `get_param`（0x140ae5830）——结构对齐实现

**fresh 定案**：`replay_raster::_::impl$0::get_param`——4 索引稀疏表取 4 组参数，
任一缺失 → panic `"Resource requested by <type> does not exist"`。4 组类型名常量：
`GameTimeResource` @0x142ae20a7 / `History` @0x142ae225e /
`AssetSsboLibrary` @0x142ae2287 / `ReplayBuffer` @0x142ae250d（前置串 @0x142ae20f8）。
输出 4 组 `{data, p58, p5c, flag, u1c}`（元素 0x60B；载荷 @+0x10/+0x58/+0x5c，
标志 @资源+0x130）。

**实现**（`systems/history/replay_raster.rs`）：`RasterReplayParams<'a, S>` 4 组参数面
（time/history/ssbo/replay 四资源）+ `get_param` 函数。src 回放系统经 Bevy `Res`
注入同四资源（缺失即装配错误），保留原 panic 语义面。**载荷指针 @+0x58/+0x5c
语义绑定 U**（不编造字段含义）。

### 1.5 编译状态

- **lib**：`cargo check -p country-core --lib` **0 错误**（当前树全绿）。
- **tests 构建**：`cargo check -p country-core --tests` **0 错误**。
- **测试运行**：本模块新增 4 测试全部 **PASS**
  （`systems::history::tests::*` 5/5、`replay_wall` 3/3、
  `replay_stairs` 2/3）。唯一失败
  `replay_stairs::tests::animated_redo_records_and_undo_erases_support_edge`
  （run 1 即断言边已记录）——**既有潜伏失败，与本次改动无关**：该测试只经
  `StairsReplayCmd::new(3,4,false,false)` + `replay_stairs`（二者本次均未改），
  而 `advance` 门控（judged-判等 的"先判后加"）使动画命令需 2 次 run 才处理；
  同文件 `animation_gating_advances_then_processes` 正确按 4 次 run 验证。
  属前 agent 未提交的 双路径 重写测试期望与门控不一致（其 REBUILD.md "0 错误"
  为编译断言，当时测试构建被他模块错误阻断未实际运行）。不擅改前 agent
  `replay_stairs.rs`（共享 WIP + 判等函数）。

---

## 2. fresh 反编译证据落盘

- `rebuild-history/decomp/0x140259af0-*.txt`（StairsHistoryEdit serialize）、
  `0x140a18ee0-*.txt`（deserialize）
- `rebuild-history/decomp/0x1409e6210-missing_prefab.txt`
- `rebuild-history/decomp/0x140ae5830-get_param.txt`
- `rebuild-history/decomp/0x140af5b30-history_process.txt`（第二事件流）
- `rebuild-history/decomp/0x140b38250-replay_pop_bucket.txt`
- `rebuild-history/decomp/0x14003c5d0-*.txt`（ColorEdit serialize）
- `rebuild-history/decomp/0x140b3f930-from.txt`（DecoratorHistoryState::from）
- `rebuild-history/decomp/0x140951b80-replay_wall.txt`、
  `0x140abfb30-process_*.txt`（WallReplayEditCmd::process）
- `rebuild-history/results/remainder-fresh.json`（含 disassembly + refs）
- `rebuild-history/results/remainder-strings.json`（get_param 类型名 + StairsEdit 键）

---

## 3. 登记（真业务，缺他域布局/载荷面，不编造）

### 3.1 `DecoratorHistoryState::from`（0x140b3f930）——登记

fresh 反编译已完整映射（落盘 0x140b3f930-from.txt）：
- 入口 `if (*(in_RDX+0x47) == '\0')` 才转换，否则 panic（+0x47 状态字节 ≠0 视为
  preview → "Attempting to capture a DecoratorHistoryState of a preview"）。
- style 字节 @+0x41（byte1 of src `DecoratorInfo.u40`），`(byte)(cVar2-2U)<6` 夹取，
  分支读 @+0x30 字节/u32、@+0x34/+0x38/+0x3c/+0x40、@+0x44..+0x4e 输出簇。
- **缺**：src `DecoratorInfo` 无 +0x41 style 字节字段（src u40 为整 u32 U）；
  src `DecoratorHistoryState` 仅 5 字段 vs 原生 ~0x4fB（≈10 字段）。结构扩展属
  decorator 域（resources/walls/decorator.rs），按红线不动他模块。

### 3.2 `replay_pop_bucket`（0x140b38250）——登记

fresh 流程（落盘）：前桶命令空 → pop_front → `DecoratorStorage::restore_decorators`
（closure 恢复 diff，Arc<DecoratorHistoryDiff> 递减）→ 若 stairs diff Arc 非空：
`StairsState::restore_backup` / `apply_diff` / `iter_edges_and_stubs`（closure）/
`IndexMapCore::clear<DecoratorId, Vec<(DecoratorId,CompactDirection)>>` /
`IndexMapCore::clear<(SubworldColliderHandle,SubworldColliderHandle),StructuralGraphConnection>`/
`stairs+0x178=0` / `make_backup` / Arc 递减 → drop ReplayBucket →
`DecoratorArchivist::extract_diff` + drop table → `RaycastWorld::flush_colliders`；
命令非空路径同样 `extract_diff`。

**缺**：`DecoratorArchivist::extract_diff`、`RaycastWorld::flush_colliders` 无 src
body；`ReplayBucket` 需承载 aux diff Arc（现 src 前置拒绝非零 aux 计数）。
src 已有 `DecoratorStorage::restore_decorators`（0x14155ba30）与 `StairsState`
restore_backup/apply_diff/make_backup/iter_edges_and_stubs，但 ReplayBucket
aux 面 + extract_diff + flush_colliders 均在 decorator/collision 域。

### 3.3 `replay_wall` 17 操作跳表（0x140951b80/0x140abfb30）——登记

fresh（落盘）：`WallReplayEditCmd::process` = done 字节 @+0x15；`*&1` 定时位 →
cubic_in_out + 跳表 0x142adf44c（timed）/0x142adf408（instant）；niche 解码
（`< -0x7ffffffffffffff0 → 值 -0x7fffffffffffffff`）得操作号 → 17 操作分派。
`replay_wall` 主循环按命令逐 `process`，组完成 → `is_disjoint` 相邻组墙集守卫
+ 移除。

**缺**：17 操作分支主体在 recovered_building（`WallHistoryEdit` 枚举变体——
src `advance_wall_groups_observed` 仅 EnclosureChange/StateChange(op0) 实现，
其余返 `UnrestoredOperation`）。recovered_building 为共享面，按红线不动他模块。

### 3.4 `replay_edit` Color/Decorator 展开（0x140af6170）——登记

fresh（落盘）：逐 edit `generate_replay_cmds_from_edit` 按 tag 跳表 0x142c5f284
分派 + undo 逆序 + 调度 ReplayBucket + 装饰/楼梯 diff Arc 处理（to_arc_slice）。
src `ReplayBucket::expand` 覆盖 Raster/AutoClutter/Roof/Wall/Stairs/Tree；
**缺 Color(9)/Decorator(10/11) 命令类型**（现返 `UnrestoredReplayOwner`）。
反编译遇坏指令数据，字段偏移无法逐字节核对；命令生成器主体在
recovered_building/装饰域。

### 3.5 `ColorEdit` 互异载荷类型面（0x14003c5d0）——fresh 定案登记

fresh（落盘；跳表 0x14292c628，目标字节 @+1，`bVar9 = target-3` 夹取 0..0xe）：
按 ColorTarget 写**互异载荷**（均 `{"<target>": {payload}}`，内层 serialize_entry 序
为载荷序）：

| target（@+1 字节） | 载荷类型（serialize_entry 序） |
|---|---|
| 3/4/5/9 | `{WallId, TerrainStrokeId, TerrainStrokeId}` |
| 6 | `{WallId, FlatRoofStyle, FlatRoofStyle}` |
| 7/8/13/15 | `{DecoratorId, TerrainStrokeId, TerrainStrokeId}` |
| 10/12 | `{DecoratorId, bool, bool}` |
| 11 | `{DecoratorId, f32, f32}` |
| 14 | `{DecoratorId, StairRailingType, StairRailingType}` |
| 16 | `{DecoratorId, StairSupportType, StairSupportType}` |
| 17 | `{}`（双 `}`） |

**缺**：src `ColorEdit` 为平面 `{target, from, to, target_id, chimney_h_offset,
allow_connection}`（8 变体 ColorTarget vs 原生 15），各 target 互异载荷字段
（WallId/TerrainStrokeId/FlatRoofStyle/StairRailingType/StairSupportType）由各自
域持有。重载为 15 变体 enum 会波及 save_game Colors 节 / load_game 旧版颜色 ID
升级消费端（他域），按红线不动他模块；本模块 serde 维持 target 键 + 平面字段
子集结构对齐（与 REBUILD.md §2.5 一致），互异载荷类型面以上表定案登记。

---

## 4. 汇总

- **实现（4 项）**：`StairsHistoryEdit` 原生单边枚举 serde、`missing_prefab`、
  `get_param`（结构对齐，载荷面 U）、`history_process` 第二事件流 case 0-3。
- **登记（5 项，均需他域布局/载荷面）**：`DecoratorHistoryState::from`、
  `replay_pop_bucket`、`replay_wall` 17 操作、`replay_edit` Color/Decorator、
  `ColorEdit` 互异载荷类型面（以上表定案）。
- **编译**：lib `cargo check` 0 错误；tests 构建 0 错误；本模块新增测试全 PASS
  （`replay_stairs` 1 例 `animated_redo...` 为前 agent 未提交重写测试的既有潜伏
  失败，与本改动无关，见 §1.5）。
- 新测试：`stairs_history_edit_serde_matches_native_single_edge_shape`、
  `missing_prefab_returns_observed_constant`、
  `second_stream_control_records_dispatch_and_push_default_commands`、
  `second_stream_conditional_clear_only_matches_last_pending_command`。
