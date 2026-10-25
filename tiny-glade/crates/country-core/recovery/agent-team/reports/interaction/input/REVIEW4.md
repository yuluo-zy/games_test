# REVIEW4 — systems/backend/input(终验)

审查员: reviewer-interaction | 日期: 2026-10-15
基线: restorer 终轮落地 + RE-ANALYSIS2 证据;`cargo check --lib` 0 error、本模块 12/12。

## 终验核对

1. **捕获路径 tag 0/2 累加**:三轮已 PASS,机器码 `cmp dword [r15+8],1; jne add` 一致(未变)。
2. **Shape B(槽 20..25)**:与 fresh 0x1416feeb0 一致(未变)。
3. **键绑定表结构**:RE-ANALYSIS2 定案 40 槽 ×12 B `{key u32@0, value u64@4}` + 0xc3 哨兵已
   入文档;`BindingsTable` 默认序列来自 load_game_profile 证据。**动作↔槽映射仍待取证**
   (诚实登记)。代码仍未逐槽对映射,维持"结构对、映射待取证"。
4. **InputFrame +0x68..+0xb8 对账**:帧状态块 @+0x68..0xb8 = 状态回写目标 +0x58..0xa8 + 0x10,
   二轮 0x68 vs 0x58 疑点已消解(帧内偏移 vs 资源偏移)。serde 视图序
   [translation_state@0, prev@0xc0, time@0xc8, main_action_set_value@0x68] 已文档化;
   **实际 serde impl / try_load 仍待取证**(文件无 serde 实现,已登记)。
5. **待取证项登记诚实**:translate_input 动作↔槽映射、Steam 接线(IsSteamRunningOnSteamDeck
   已登记跨域)、InputFrame serde、try_load、SyncPoint 子字段类型化、normalize/unnormalize
   精确公式——全部在 RESTORE/RE-ANALYSIS2 显式登记,无冒充完成。

## 结论与 DISPATCH 建议

- 已实现主体(calculate_input_delta Shape B、translate_input_to_cursor_moved 方向/夹取/捕获、
  input_record_replay 状态机分派、advance、reset buffer 置换)与机器码/PDB **对得上**;
  待取证项诚实登记、未冒充。
- 建议:input 模块给 **review_passed(限定已实现范围)**。边界明确:动作↔槽映射、Steam、
  帧 serde/try_load、SyncPoint 子字段不在本次验收内,由 re-analyst 补证后单独闭合;
  不据此宣布整模块"100% 完成"。
