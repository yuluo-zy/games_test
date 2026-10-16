# session-DELTA — 用户批改核对（2026-10-16，对照 session-SPEC + 原机器码）

> 核对 agent（C 世界）。判定同 save-DELTA。文件：`crates/country-core/src/resources/session.rs`（20KB）。
> 原侧证据：`session-save/decomp/decomp-0x141328ac0.txt`、`session-save/ALGORITHMS.md`、`evidence/country-core-deep/ui-input/decomp/`、`sysres-procedures-v2.json`。

## 1. NextSession（160B）owner 与 kind

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| NextSession 160B 布局（Vec@0、kind@+0x18、uuid@+0x20、ignore_snapshot@+0x98） | 部分（语义 owner） | session.rs:274-279 `{on_load, kind, id, ignore_snapshot}` | Rust 语义结构，非字节对齐 160B；字段语义与 O 对应。OnLoadEntry 为枚举（非原生 0x20B 数组），已注明 |
| `NextSession::load` 0x140b46920：{空 Vec(cap=0+dangling)、kind=0x8000000000000002、ignore=false} | 真实实现 | session.rs:283-290 | `kind=LoadOnLoad=0x8000000000000002` ✓、空 Vec ✓、ignore=false ✓ |
| kind 五条 selector 路径（0 空/1 load_latest/2 SessionId/3 GameSaveData boxed/其余 create） | 部分 | session.rs:234-246 `NextSessionKind{None,LoadSavedGame,LoadOnLoad}` | 只表达 3 条 decoded 值；变体 1（load_latest）生产者、变体 3（GameSaveData boxed）未体现 |
| `create` 0x140b46960：96B 模板 + 32B 选项 + 24B 可选串/路径 + 字节旗标 | 有水分（简化） | session.rs:292-299 `create(id, kind)` | 只取 id+kind，丢弃 96B 模板/32B 尾部/24B 串路径；SPEC §1.4 已列 24B 策略 U。需补取证或标待取证 |
| `ignore_snapshot` 0x140b46a20：写 +0x98 bool + memcpy 0xa0 | 真实实现 | session.rs:301-304 | builder 语义一致 |
| `loading_background_path` 0x140b46a50（v1 名 screenshot_path 错位） | 真实实现 | session.rs:350-356 | 与 SPEC 命名纠正一致 |
| `load_latest` | 有水分（占位生成随机 id） | session.rs:307-309 | `Self::load(SessionId::new())` 生成随机 UUID，而非"最近会话"；注释标(I)诚实，但名字具误导性 |

## 2. append_on_load 两单态

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| `append_on_load<MetaCommandFence>` 0x141328ac0：push 0x20B 条目 {u32 tag=0, u32 0x80000000, u64 payload} + memcpy 0xa0 | 真实实现 | session.rs:313-318 `append_fence`；decomp-0x141328ac0 全文 | decomp：`*puVar1=0; puVar1[1]=0x80000000; puVar1[2..3]=payload` + `memcpy(param_1,param_2,0xa0)`。当前 `tag=ON_LOAD_VARIANT_MASK(0x8000000000000000)` 即 u64 折叠 {0,0x80000000}，一致 |
| `append_on_load<[MetaCommand;3]>` 0x1403b5150：三 56B MetaCommand → VecDeque → 一个 32B SessionOnLoadCmd | 部分（语义映射） | session.rs:329-332 `extend_on_load` → `OnLoadEntry::Batch{Vec<MetaCommand>}` | reviewer 修正"tag=3 批量、载荷 Box<0xa8B>"已在注释；当前以 Batch 枚举表达，原生 0xa8B 批量缓冲未复现（语义等价） |
| MetaCommand 头语义 {0, 0x80000000} | 真实实现（登记） | meta_commander.rs（共享） | 7 qword=56B 结构，与 load_game 回放一致 |
| SessionOnLoadCmd（32B）载荷字段名 | 待取证 | OnLoadEntry::Command{tag,payload} | 32B 总布局 O，各变体载荷字段 U，未编造字段名（符合 SPEC） |
| ON_LOAD_VARIANT_MASK / ON_LOAD_FENCE | 真实实现 | session.rs:267-269（0x8000000000000000 / 0x8000000000000003）；decomp-0x14146d030.txt 行 383/743 | 常量核实一致 |

## 3. GameSaveData / SessionLoadAttempt / SessionMetaSource

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| GameSaveData（save 域 owner，本域持 boxed 引用） | 待取证（见 save-DELTA §3） | — | 本域无 boxed 引用路径（kind==3 未表达），owner 缺口在 save 域 |
| SessionLoadAttempt::from_session_id（theme byte@+6） | 仍缺 | — | 未在 session.rs 实现；SPEC §2.3 |
| SessionMetaSource 枚举 | 仍缺 | — | 未实现 |
| copy_dir_all<PathBuf,PathBuf> 0x141740c00（"intro" 跳过） | 仍缺 | — | 未实现（load_game_session 启动事务的一部分） |

## 4. SavedSession / SavedSessionList / 会话切换

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| SavedSession / SavedSessionList | 真实实现 | session.rs:341-363 | 字段/排序（最近在前）合理 |
| refresh_session_list 0x140b46b00 | 真实实现 | session.rs:402-404 | 清空重建语义一致 |
| delete_session 0x140b46d60（标记文件 + rename + 两段删除；"Error deleting glade: "） | 真实实现 | session.rs:408-429 | 结构与日志一致 |
| 会话切换系统（public-session-names 第 9/11 条 SystemParam 面 + load_game_session） | 部分 | run_game.rs:71 `SessionFlowPlugin.build` + meta_commander.rs `configure_session_switch_systems` | **真实生产接线存在**：commander_update 消费 NextSession（写 on_load/kind=LoadOnLoad）已注册。但消费端 load_game 未注册（无系统应用回放结果）；load_game_session（0x14166e7d0，78KB）未实现为系统 |
| NewSessionStartedCmd / SessionLoadedCmd 事件 | 部分 | session.rs:432-435 | 结构真实+注册（add_save_load_events_and_resources）；**无任何生产者发射** |
| SessionFlowPlugin build 0x140b47840 | 真实实现 | session.rs:444-451 + run_game.rs:71 | 插入 ActiveSession/SavedSessionList/NextSession/AutoSaveState + 注册事件。生产已接 |

## 5. 真实消费者链（SPEC §5）

| 生产者 | 消费者 | 状态 |
|---|---|---|
| UI 会话选择 → NextSession::create/load | 部分（create 简化，UI 生产者未接） | — |
| commander_update case 0 → insert_resource<NextSession> | **已接**（meta_commander configure_session_switch_systems 生产注册） | 真实 |
| NextSession kind 解码 → load_game | 未接（advance_load_game 纯函数，无 ECS 系统） | 仍缺 |
| load_game_session 0x14166e7d0 | 未实现（仅 load_game.rs 纯函数近似，签名不同） | 仍缺 |
| SessionLoadedCmd / NewSessionStartedCmd → 全域 | 无生产者 | 仍缺 |

## 合成 agent 应关闭的真实剩余缺口

1. **load_game 消费端接线**：commander_update 已写 NextSession，但无系统消费回放结果（advance_load_game/load_game_session 纯函数需挂到真实 ECS 系统 + AppMode==0xe 门）。
2. **`create` 完整语义**：96B 模板 + 32B 选项 + 24B 串/路径字节旗标（SPEC §1.4 待取证字段），当前 create 只有 id+kind。
3. **`load_latest`**：应从 SavedSessionList 取最近会话 id，而非随机 UUID。
4. **SessionLoadAttempt / SessionMetaSource / copy_dir_all**：session-SPEC §2.3/§4 契约未实现。
5. **SessionLoadedCmd / NewSessionStartedCmd 生产者**：事件只注册无发射。
6. **NextSessionKind 变体 1（load_latest）与 3（GameSaveData boxed）的完整表达**：当前只有 3 个枚举值。

## 跨组/共享文件注意

- `resources/session.rs` 为本组文件；改动牵动 music（B 组读 ActiveSession）需中央仲裁——当前 ActiveSession 布局改动未见仲裁记录，需核实。
- GameSaveData 的 `sections`/`edits` 字段名见 save-DELTA §3（自定名未标待取证）。
