# REVIEW5 — 会话切换链（组 state 卡2 终验：commander_update + fence + load_game 衔接）

> 审查员：reviewer-state，2026-10-15。复核 restorer 卡2 落地与 RE-SESSION-SWITCH.md 一致性。
> 编译基线实测：`cargo check -p country-core --offline` 通过；本组 27 通过 0 败（覆盖 20/20 声明）；
> 全 crate **1606/1**（唯一失败 = `geometry::sphere::tests::uv_formula_matches_the_closure`，
> components-geometry 组，非 state）。

## 逐项复核

### 1. commander_update 分派（MetaCommander::update）
| 项 | verdict | 说明 |
|---|---|---|
| case 0 两路径状态机 | **OK**（FIX-1 见下） | 路径 A（+0x18==0）：new_pair_blocked + 临时 on_load 变体 0 会话加载命令（payload=fence）→ `CommanderOutcome::SessionLoad`；路径 B（+0x18!=0）→ `SessionLoadWait`。与 RE-SESSION-SWITCH §1 一致。 |
| case 5 延后重入队 | OK | 载荷非 0 → 重排 {tag5, count-1}。 |
| case 6 FloatEvent / case 9 CreateDirectory / default Unknown | OK | 行为与机器码登记一致（case 9 路径载荷编码待取证，诚实）。 |
| **case 0 AppMode 门控** | **FIX** | RE-SESSION-SWITCH §1：门控 = `mode-6 < 0xf && mode-0xd < 2` → **mode ∈ {13, 14} = {Replay, LoadGame}**。src `commander_update_system` 用 `app_mode.native_tag() != AppMode::LoadGame.native_tag()`（仅 14）。**与 load_game 门（==0xe 仅 LoadGame）不同**——src 注释"与 load_game 同门"判断错误。建议改 `!matches!(app_mode, AppMode::Replay | AppMode::LoadGame)`。 |
| case 2/3（profiler_sinks）/7/8/0xc 载荷 | PARTIAL | Handled 占位（宿主/域待接），诚实登记。 |

### 2. fence 双 tag
| 项 | verdict | 说明 |
|---|---|---|
| append_fence = Command{tag=0x8000000000000000, payload=fence}（变体 0 非终止） | **OK** | `fence.as_payload()`；与 RE-SESSION-SWITCH §2 一致（fence 是变体 0 载荷）。 |
| append_replay_terminator = {tag=0x8000000000000003, payload 0} | **OK** | 两遍回放终止哨兵单独追加。 |
| 回放终止 `tag == 0x8000000000000003` | **OK** | load_game 两遍 `if *tag == ON_LOAD_FENCE break`（不再用 payload==FENCE，修正前轮）。 |
| 释放走变体 2（unblock +0x10 + Arc drop） | **OK**（PARTIAL 动作） | `ReplayedOnLoadCommand::FenceRelease { fence_payload }` 输出；实际 unblock/drop 由宿主按 payload 执行（fence.as_payload 为不透明 u64，src 不逆向重建 Arc——诚实）。fence 标志偏移定案 +0x10（RE-SESSION-SWITCH §1 纠正"变体 2 +2 伪影"）。 |
| MetaCommandFence::unblock/is_blocked/as_payload | OK | 语义与 RE-SESSION-SWITCH（+0x10 标志）一致。 |

### 3. insert_resource<NextSession>
| 项 | verdict | 说明 |
|---|---|---|
| commander_update_system 按 AppMode 门控置 NextSession | OK（FIX-1 门控值见上） | 置 kind=LoadOnLoad + on_load（含变体 0 + fence）；configure_session_switch_systems 注册。 |
| 实现形态：ResMut<NextSession> vs Commands::insert_resource | PARTIAL(低) | RE-SESSION-SWITCH §0 定案原版 = `Commands::insert_resource<NextSession>`（FUN_1412e0b90 deferred）；src 用 `ResMut<NextSession>` 直接改写（即时）。效果等价，机制不同；建议注明或改 Commands 形态。 |

### 4. load_game 衔接
| 项 | verdict | 说明 |
|---|---|---|
| ReplayedOnLoadCommand::FenceRelease（变体 2） | OK | 一遍变体 2 → FenceRelease（非 `_ => {}` 跳过）。 |
| pass2 变体 2+ push {0x8000000000000000, elem[+8]} | OK | `Apply { tag: ON_LOAD_VARIANT_MASK, payload_a, ... }`。 |
| load_game_session（状态机+两遍回放+版本开关+max wall id） | OK | advance_load_game + version_gates + extract_max_wall_id_from_history 组合；MetaCommand 应用 / init_game_state_from_snapshot 留宿主（不伪造加载成功）。 |
| 0x14166e7d0 = 应用启动加载（非游戏内切换） | OK（命名提示） | RE-SESSION-SWITCH §3 澄清；src `load_game_session` 函数名与 0x14166e7d0 相近但内容是游戏内切换核心（advance_load_game），非启动 copy_dir_all 流。建议注释/命名区分，避免混淆。 |

## 结论与 DISPATCH 建议
- **卡2（会话切换链库内部分）→ 闭合**：commander_update 分派、fence 双 tag、NextSession 接力、
  load_game 两遍回放衔接均与 RE-SESSION-SWITCH 对齐；20/20 测试通过。
- **1 个 FIX（低-中）**：`commander_update_system` 的 AppMode 门控应为 {Replay(13), LoadGame(14)}，
  src 仅 LoadGame(14)。建议 `!matches!(app_mode, AppMode::Replay | AppMode::LoadGame)`。
- 仍阻塞项（登记）：case 2/3/7/8/0xc 载荷与 profiler_sinks、run_game_thread 3 命令逐 cmd tag/载荷、
  MetaCommand 变体 1 应用（commander_update case 1 重排/宿主）、0x8000000000000003 终止哨兵生产者的
  完整追源（RE-SESSION-SWITCH §5 #2/#3）、save_game 本体 ECS 资源组装 / capture_game_snapshot 域 Query /
  全局接线。
- 模块状态建议：save/session → **review_passed（限定已实现范围）**，卡2 闭合登记（附 FIX-1 门控）；
  完整验收仍 blocked（上述阻塞项）。
