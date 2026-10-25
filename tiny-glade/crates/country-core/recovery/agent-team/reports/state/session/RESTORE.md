# RESTORE — resources/session（组 state）

> restorer-state，2026-10-15。按 re-analyst RE-ANALYSIS.md + reviewer REVIEW.md + chapters
> 落地会话切换链数据结构。

## 本批新增实现

| 原函数/类型 | Rust 位置 | 证据 |
|---|---|---|
| `NextSessionKind` | `session.rs`（None=0x8000..0000 / LoadSavedGame=…0001 / LoadOnLoad=…0002；`decoded()`） | load_game 0x14146d030 kind 解码 `^0x8000000000000000; min(·,2)` |
| `OnLoadEntry` + `ON_LOAD_VARIANT_MASK` + `ON_LOAD_FENCE` | `session.rs`（0x20B = {tag qword, payload qword}；fence=0x8000000000000003） | re-analyst RNE 原字节修正 |
| `NextSession::load`/`create`/`ignore_snapshot`/`load_latest` | `session.rs`（0xa0B 布局：Vec + kind niche + uuid + ignore_snapshot） | 0x140b46920 / 0x140b46960 / 0x140b46a20 |
| `append_on_load<MetaCommandFence>`/`append_on_load<[MetaCommand;3]>` | `append_fence` / `extend_on_load` | 0x141328ac0（{0x8000000000000000, fence}）/ 0x1403b5150（{count, Box}） |
| `SavedSession::loading_background_path`/`screenshot_path` | `session.rs` | 0x140b46a50（v1 名 screenshot_path 错位） |
| `SavedSessionList` + `load_saved_session_list`/`refresh_session_list` | `session.rs`（readdir UUID 子目录 + 按修改时间排序） | 0x140b46b00 / rhapsody filter_map |
| `delete_session` | `session.rs`（deleted_saves()/uuid 标记 + rename 两段删除） | 0x140b46d60 |
| `NewSessionStartedCmd`/`SessionLoadedCmd` 事件、`SessionFlowPlugin::build` | `session.rs`（资源插入 + 事件注册；bevy_app 由宿主装配） | PDB structures + 0x140b47840 |

## 依赖复用边界

- `paths` crate 的 saves()/deleted_saves() 直接复用。
- 会话文件三件套/主题谓词/元数据 schema 为既有实现（未动）。

## 仍缺失/待取证

- **完整 NextSession/world 切换生命周期**：`SessionFlowPlugin::build` 已登记资源/事件；
  实际 AppMode 门控 + 会话创建/销毁调度由宿主 App 装配，未接线。
- **on_load 变体 1（MetaCommand）应用**：`extend_on_load` 入队；应用由 meta_commander.rs
  commander_update 消费（本批只建数据结构）。
- **fence 双 tag 关系**（append 写 0x8000000000000000 vs load_game 终止 0x8000000000000003）
  与变体 2 unblock 偏移（+2 vs +0x10）：RE-ANALYSIS 标注，未伪造。
- `NextSession::load_latest` 的最近会话 id 来源（SavedSessionList 消费者）留待接线（I）。

## 二轮修正（reviewer REVIEW2）

- `SessionFlowPlugin::build` 补插入 **ActiveSession**（占位默认 `ActiveSession::new(SessionId::new(), SessionMeta::default())`，真实会话启动后替换）。
- `extend_on_load` 批量条目随 save 模块修正为 tag=3 + Box 批量结构（`OnLoadEntry::Batch`）。

## 卡2 会话切换链（RE-SESSION-SWITCH + RE-ANALYSIS）

- **commander_update case 0 两路径状态机**（meta_commander.rs）：路径 A（+0x18==0）→
  `MetaCommandFence::new_pair_blocked` + 临时 on_load（变体 0 会话加载命令，payload=fence）→
  `CommanderOutcome::SessionLoad`；路径 B（+0x18!=0）→ `SessionLoadWait`（fence +0x10 判定
  阻塞/释放）。`reenqueue_session_load` 供宿主循环。
- **fence 双 tag 定案**：append_on_load<Fence> 写 `{tag=0x8000000000000000, payload=fence}`
  = **变体 0 会话加载命令**（payload 是 fence Arc，非 0x8000000000000003）；fence 释放走
  **变体 2** 条目（unblock +0x10 + Arc drop）；0x8000000000000003 = 两遍回放终止哨兵
  （生产者写入残留待追源）。`append_fence(fence)` / `append_replay_terminator()`。
- **FUN_1412e0b90 = `Commands::insert_resource<NextSession>`**：`commander_update_system`
  按 AppMode 门控置 NextSession（kind=LoadOnLoad + on_load），`configure_session_switch_systems`
  注册。
- **load_game 衔接**：`ReplayedOnLoadCommand::FenceRelease`（变体 2）、回放终止改按
  `tag==0x8000000000000003`（非 payload）、pass 2 变体 2+ 推 `{0x8000000000000000, …}`。
- `load_game_session`（0x14146d030 可接部分）：状态机 + 两遍回放 + 版本开关 + max wall id。
  **澄清**：0x14166e7d0 是应用启动会话加载（起始 build/目录/meta/history），非游戏内切换；
  切换 = commander case0 → NextSession → load_game。

## REVIEW4（卡1 终验）吸收

- GameProfile 字段声明序按原文 ron serialize 序重排（tutorial → tutorial_cooldowns →
  game_installation_id → key_bindings → new_glade_count；player_id 尾部 skip）。
- load_completion_checker 头注释改为澄清后签名（不含 GameProfile/MetaCommander）。
- BackwardCompatibility=0u8 保持待取证登记。

## 卡X6（commander_update case 2/3/7/8/0xc）追加

- `MetaCommander::update` 补 case 语义（RE-SESSION-SWITCH §1.1 机器码登记）：
  case 2/3 = profiler_sinks HashMap 迭代（control-byte 位扫描；载荷非 0 → 重排
  {tag 3, countdown 0x12c}）；case 7 = 清零 + TimeUpdateStrategy 应用（FUN_1412e1400）；
  case 8 = 追加 0x30B 条目到 Vec；case 0xc = GPU/内存报告（alloc 0x68B → Arc →
  hashbrown insert）。**sink 载荷/资源面由宿主消费**，已登记不伪造。

## 编译/测试状态

- `cargo check -p country-core --offline`：我的文件零错误零警告。
- `cargo test -p country-core --offline --lib resources::session`：1/1 通过（既有元数据往返）。

## 合同/状态建议

- reviewer 判 session 限定范围可 review_passed；完整模块验收仍 blocked（异步捕获/
  MetaCommand 应用/全局接线）。
- inventory 建议：session.rs → `partial_implemented`（NextSession/SavedSession 链已落，
  world 切换调度待宿主）。
