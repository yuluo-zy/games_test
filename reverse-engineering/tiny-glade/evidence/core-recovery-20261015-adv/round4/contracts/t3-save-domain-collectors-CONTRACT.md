# T3 CONTRACT — save 域采集器 + PostGameLoadCommand::Apply 应用（身份定案）

> 逆向 agent ① 交付。证据：procs.tsv 全名 + 反编译（round4/decomp-collect-*.txt）。
> 状态：**身份+Query 面定案，可直接落地**。

---

## 1. 三采集器身份（O，procs.tsv 权威全名）

| 地址 | 原生符号 | 采集协议 |
|---|---|---|
| **0x141305170** | `from_iter<(TreeStorageIndex, TreeData)>` ∘ `Map<QueryIter<(&PrefabInstanceData, &PrefabInstance, &TreeComponent)>, capture_game_snapshot_save_data::closure_env$0>` | **Tree 采集**：`Query<(&PrefabInstanceData, &PrefabInstance, &TreeComponent)>` → `Vec<(TreeStorageIndex, TreeData)>` |
| **0x141308b40** | `from_iter<(WallId, WallState)>` ∘ `Map<Map<HashMap<WallId,WallState>::Iter, inner_walls::all::closure_env$0>, capture_game_snapshot_save_data::closure_env$1>` | **Wall 采集**：`inner_walls::all()`（HashMap<WallId,WallState>）→ `Vec<(WallId, WallState)>` |
| **0x141309840** | `from_iter<u64>` ∘ `Copied<HashSet<u64>::Iter>` | **u64 集合采集**：`HashSet<u64>` → `Vec<u64>`（decorator/stroke id 集候选） |

- 三者都属 `systems::save::snapshot::capture_game_snapshot_save_data`（存档快照捕获）。
- 采集输出 = 快照二进制载荷的对应域（Tree/Wall/u64-set）。
- ② 待办：`save/snapshot` 域实现对应采集（现 save-DELIVERY 标记"域采集器未接"）：
  - Tree：Query 遍历 → `(TreeStorageIndex, TreeData)` push。
  - Wall：`inner_walls` HashMap 全量迭代 → `(WallId, WallState)` push。
  - u64：HashSet 全量拷贝。

## 2. PostGameLoadCommand::Apply（FUN_1412e0b90）= `Commands::insert_resource::<NextSession>`

- procs.tsv 全名：**`bevy_ecs::system::commands::Commands::insert_resource<country_core::resources::session::NextSession>`**。
- 行为（decomp-0x1412e0b90，O）：命令队列 push **0xa8B 命令条目** `{ fn_ptr = FUN_1412db840, 载荷 0xa0B }`；
  `FUN_1412db840` = `FnOnce::call_once<...insert_resource<NextSession> 闭包>`（apply 时插入 NextSession）。
- **0xa0B = NextSession 尺寸（160B）**——与 load_game_session 的 0xa0 拷贝一致（T2 契约交叉验证）。
- 语义：加载完成的 NextSession 经**延迟命令**插入 World（bevy Commands::insert_resource）。
- ② 对照：现 `load_game_system`/session.rs 若已用 `Commands::insert_resource::<NextSession>`，
  语义即等价；无需另造 PostGameLoadCommand 结构。

## 3. 可实现性
- **可直接落地**：三采集器 Query 面 + 输出 Vec；`Commands::insert_resource<NextSession>` 桥。
- 需登记：采集器输出载荷的 serde/二进制格式（快照节布局）仍随 save-DELIVERY 待取证，
  不编造字段名。

## 4. 证据索引
- procs.tsv：0x141305170/0x141308b40/0x141309840（capture_game_snapshot_save_data 三采集器）、
  0x1412e0b90（Commands::insert_resource<NextSession>）、0x1412db840（insert 闭包 call_once）。
- round4/decomp-collect-*.txt（4 份反编译）。
