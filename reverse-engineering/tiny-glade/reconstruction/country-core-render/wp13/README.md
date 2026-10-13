# WP13 — country-core session / prefab / history / save 域恢复

B 版本（EXE sha256 `cf048e27…`，PDB sha256 `2d9686b8…`）。方法遵循项目工作流：
**(1) 分析业务逻辑（反编译 + 调用链 + PDB 骨架证据）→ (2) 整理完整业务逻辑描述 →
(3) 按源码骨架实现 Rust**。机器码差分仅裁决关键数值/边界歧义，不作为实现手段。

## 交付内容

| 类别 | 路径 |
|---|---|
| 业务合同 | `contracts/{session,prefab,history,save-systems}.md`（O/I/U + typed gap） |
| 资源实现 | `src/resources/{session,prefab,history}/*.rs` |
| 系统实现 | `src/systems/{history,save}/*.rs` |
| 独立 crate | `Cargo.toml` + `src/lib.rs` + `src/tests.rs`（offline cargo check 门禁） |
| 证据 | `evidence.json` + `evidence/`（Ghidra job/result + 逐函数反编译 `.c` + 反汇编 `.asm` + 机器码读取） |

## 恢复要点

- **resources::session**：`GladeType` 判别字节 **1=Olden / 2=Flowery / 3=Autumn /
  4=Winter**（原机器码逐字节证实，含 WP7 冬季证据）；`ActiveSession` 字节@+0x58==4 选冬
  季水面（WP7）；`SessionId::new` = `uuid::Uuid::new_v4()`；`NextSession` 0xa0 字节、
  `ignore_snapshot` @+0x98；`SessionMeta` 季节字节扫描谓词；会话流事件/插件。
- **resources::prefab**：`PrefabLibrary` = `HashMap<PrefabName, Prefab>`（hashbrown
  RandomState，O：add 0x1408e2540）；get/has/add/iter；`PrefabName` 借用投影。
- **resources::history（S05）**：权威 `History` 缓冲布局（edit_chain Vec<MultiEdit>
  @+0x00 元素 0x28；snapshots @+0x18 元素 0x10；pending 桶 @+0x30；count @+0x58；
  nonce @+0x60；cursor @+0x68；history_start @+0x6c）；`HistoryMutationToken` 忘记变更
  drop 守卫；快照二分 `current_snapshot`/`closest_resolved_snapshot`；undo/redo 游标规则
  内联进 history_process（byte[1]==0 undo / ==1 redo，下界 history_start、上界 len）。
- **systems::history**：history_process / HistoryCmd（interrupt=0x02000200）/
  PendingHistoryCommands / history_recording / 回放族声明。
- **systems::save**：`SaveFileSection` 10 变体 json_key（Version/Identity/
  BackwardCompatibility/Camera/Time of day/Sheep/Colors/chain_index/history_start/
  snapshot）+ padding_bytes 表（0/16/0/128/32/128/64/8/8/2048）从 .rdata 表机器码恢复；
  save_game/load_game/snapshot/save_thread/save_backup_list（history_auto 前缀）/
  shadow_file/save_file_identity/load_completion_checker/snapshot_gc。

## 复用的既有工作（按 B 骨架核对）

- `reconstruction/history/`（tg-history 观察 JSON 模型；transaction.rs
  ledger/cursor/nonce 与反编译吻合）。
- `reconstruction/crates/tg-history/`。
- task-index 候选：`history_chain::HistoryChainIndex`（TG-3846e898cf43c29d09）、
  `HistoryCmd::interrupt`（TG-c4cdc0aa5e62db9266）、`GladeType` tag
  （TG-a4422b4c9680d489d4）、`asset::prefab::PrefabName`（TG-a3f23bd62617630cba）。
- `evidence/building-history/` 既有 history_process / finish_replay 反编译。

## 门禁

```
cargo check --offline     → 0 错误
cargo clippy --all-targets --offline → 0 错误 / 0 警告
cargo test --offline      → 8 passed / 0 failed
```

全部 Rust 标记 `implemented_unverified`：无 runtime/游戏验收；合同是当前可审阅依据，
不因编译通过或测试通过声称与原作等价。

## 剩余缺口

- History 完整 serde 字段序、snapshots 负载 `T`、`+0x70` 字段。
- MultiEdit 三个辅助容器精确类型。
- ActiveSession/SessionMeta/SavedSession 布局；SessionMeta 季节字节语义。
- NextSessionKind / SessionOnLoadCmd 变体集合。
- Prefab 值/部件/实例字段。
- save_game/load_game 完整资源清单与 JSON 字段序；save_thread 消息协议。
- 各 domain 回放系统（wall/roof/terrain/path/…）展开算法。
