# REVIEW2 — resources/session（组 state 二轮复验）

> 审查员：reviewer-state，2026-10-15。复核 restorer RESTORE.md 新增的会话切换链数据结构
> 与 re-analyst RE-ANALYSIS 的一致性。编译/测试实测通过（session 1/1）。

## 逐项复核

| RESTORE 项 | verdict | 说明 |
|---|---|---|
| `NextSessionKind`（None/…0001/…0002；decoded=^mask） | **OK** | 与 load_game 0x14146d030 kind 解码一致；`NextSession::load` kind=…0002 与 0x140b46920 一致。 |
| `OnLoadEntry` 0x20B {tag,payload}、`ON_LOAD_VARIANT_MASK`、`ON_LOAD_FENCE` | **OK** | 与 RE-ANALYSIS §2 一致（0x20B 条目修正 DIGEST "4B tag"）。 |
| `NextSession::load/create/ignore_snapshot/append_on_load/load_latest` | OK | load/create/ignore_snapshot 与 0x140b46920/46960/46a20 一致；load_latest 为占位（I，诚实）。 |
| `append_fence`（{0x8000000000000000, 0x8000000000000003}） | **OK** | 与 append_on_load<MetaCommandFence> 0x141328ac0 一致（RE-ANALYSIS §2.1）。 |
| `extend_on_load`（tag=ON_LOAD_VARIANT_MASK\|1） | **FIX/待取证** | 本审查反汇编 append_on_load<[MetaCommand;3]> 0x1403b5150 证实批量条目 tag=**3**（非 mask\|1）。src 自洽（测试通过）但机器 tag 不符；且 src 按单条 payload 入队 vs 原版 Box<[MetaCommand;3]> 单条目。见 save/REVIEW2 §7，属同项残留。 |
| `SavedSession`/`SavedSessionList`/`load_saved_session_list`/`refresh_session_list` | PARTIAL | 实现（readdir UUID 子目录 + 按修改时间倒序）；screenshot_exists/loading_background_exists 字段为推断（rhapsody 过滤待接入），诚实。 |
| `delete_session` | OK（近似） | deleted_saves()/uuid 标记 + rename + 两段删除；src 用新 uuid 作目标（避免碰撞），原版标记文件细节 I。 |
| `SessionFlowPlugin::build` | PARTIAL(低) | 登记 SavedSessionList/NextSession/AutoSaveState/事件；**ActiveSession 未插入**（RESTORE.md 声称含 ActiveSession，代码未含——minor 不一致，ActiveSession 应由宿主/他处插入）。 |
| 事件 NewSessionStartedCmd/SessionLoadedCmd | OK | 载荷待取证（I）。 |

## 核对要点
- 会话切换链数据结构（NextSessionKind/OnLoadEntry/NextSession/SavedSession）已落地且与 RE-ANALYSIS 大体一致；on_load 应用（commander_update）未建，诚实披露，未伪造加载成功。
- round1 待取证项（meta.json "version" 键）未变，仍待取证（低）。

## 结论与 DISPATCH 建议
- **建议：session → review_passed（限定已实现范围）**。唯一待行动项 = extend_on_load tag 与 fence 双 tag 关系（与 save 同项，合并到 save 的 fix_pending 处理）。完整模块验收仍 blocked（world 切换调度 / MetaCommand 应用 / 全局接线）。
