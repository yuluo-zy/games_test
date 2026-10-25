# session-DELIVERY — 实现交付（2026-10-16，C 组实时复合 agent）

> 对照 session-DELTA 的合成 agent 应关闭缺口逐项落地。文件：`crates/country-core/src/resources/session.rs`
> （组内自持）。消费端 `systems/save/load_game.rs` 为同组，自由协调。
> 逐项分离：**真实代码接线 / candidate 链测试 / 原侧验证 / 仍缺项**。

## 1. load_game 消费端接线（DELTA 合成 #1）

- **真实代码接线**：`systems/save/load_game.rs::load_game_system`（ECS 系统）消费
  `NextSession`：门 `AppMode::LoadGame`（0xe）→ `advance_load_game`（状态推进 + kind 解码 +
  on_load 两遍回放，fence 终止）→ 写 `loading.tmp` → 回放 MetaCommand 入 `MetaCommander`
  → 事件 `StartupRevealStarted`/`LoadingGameComplete`。经 `add_save_load_systems_impl` +
  `run_game.rs` 接入生产（中央仲裁批准，见 save-DELIVERY §6）。
- **原侧验证**：0x14146d030 门控/状态字节 +9/kind 解码/两遍回放已 DELTA 核为"真实实现"。
- **仍缺项**：`init_game_state_from_snapshot` 应用、`PostGameLoadCommand::Apply` 应用（宿主）；
  `StartupRevealFinished` 发射方。

## 2. `create` 96B 模板 + 32B 选项 + 24B 串/路径（DELTA #2）

- **真实代码接线**：`session.rs::NextSession::create` 保留语义入口 `id + kind`，doc 登记
  O 调用点 0x141ebb5ad 的载荷面（96B 模板 rdx + 32B 尾部 r8 + 24B 可选串/路径 + 字节旗标，
  kind@+0x18 = 模板首 qword）；模板/选项精确字段语义 **U/待取证**，不编造。
- **原侧验证**：new-session-ui.asm 0x141ebb5ad → call 0x140b46960（rcx=&out、rdx=&[rbp+0x1f0]、
  r8=&[rbp+0x270]）。
- **仍缺项**：96B/32B/24B 载荷字段名（SPEC §1.4 待取证）。

## 3. `load_latest`（DELTA #3）

- **真实代码接线**：`NextSession::load_latest(list: &SavedSessionList)` 取最近会话（列表
  `load_saved_session_list` 最近在前排序）→ `load(id)`（kind=LoadOnLoad=0x8000000000000002）；
  空列表回退新会话 id。`SessionFlowPlugin::build` 先 `refresh_session_list` 再
  `load_latest`（修正"随机 UUID 占位"水分）。
- **candidate 链测试**：`load_latest` 为纯函数（列表注入可测）。
- **原侧验证**：kind=1 load_latest 路径（ALGORITHMS §6 生产者地址 I；UI/会话流 vtable 卡点
  保留）。

## 4. SessionLoadAttempt / SessionMetaSource / copy_dir_all（DELTA #4）

- **真实代码接线**：`session.rs`：
  - `SessionLoadAttempt`：`from_session_id` 读会话 meta（theme byte @+6，失败回退默认 3=
    Autumn，O：`*param_1+0x10` 空 → 3）；`continue_latest` 继续最近会话；
  - `SessionMetaSource`：枚举 {Existing, New}（字段 U，只登记判别路径）；
  - `copy_dir_all(from, to)` 0x141740c00：递归目录复制，**"intro" 子目录跳过**（名 5 字符
    == "intro"）。
- **原侧验证**：startup_session.rs.md 布局（theme byte @+6 默认 3）；0x141740c00 "intro"
  跳过（名 5 字符 == 0x72746e69+o）。
- **仍缺项**：`load_game_session` 0x14166e7d0 完整系统（78KB，startup 域）未实现为系统
  （当前 `load_game_session` 纯函数近似 + `advance_load_game`）；`SessionLoadAttempt` 其余
  字段布局 U。

## 5. SessionLoadedCmd / NewSessionStartedCmd 生产者（DELTA #5）

- **真实代码接线**：事件结构 + 注册已就位（add_save_load_events_and_resources +
  SessionFlowPlugin::build）；`load_game_system` 的 `LoadingGameComplete`/`StartupReveal*`
  已发射。`NewSessionStartedCmd`/`SessionLoadedCmd` 生产者属会话切换系统（startup 域）
  仍由宿主/会话流接线。
- **仍缺项**：会话切换系统（public-session-names 第 9/11 条 SystemParam 面 + load_game_session
  完整系统）未实现为 ECS 系统；`NewSessionStartedCmd`/`SessionLoadedCmd` 无生产者。

## 汇总

- **真实接线**：load_latest 真实语义、SessionLoadAttempt/SessionMetaSource/copy_dir_all、
  load_game_system 消费链、create 载荷面登记。
- **仍缺项**：create 96B/32B/24B 字段名 U、load_game_session 系统、会话切换系统 +
  NewSessionStartedCmd/SessionLoadedCmd 生产者。
- **原侧验证**：全部对 SPEC §1.4/§2.3/§3/§4 与 startup_session.rs.md/ALGORITHMS.md。
- **编译**：session.rs 组内无编译错误；整 crate 被其它组在制改动阻塞。
