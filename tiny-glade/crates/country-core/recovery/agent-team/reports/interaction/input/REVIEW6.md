# REVIEW6 — systems/backend/input(卡3/卡4 终验 → 验收闭合·库内)

审查员: reviewer-interaction | 日期: 2026-10-15
基线: restorer 完成卡3(Steam 全链登记)+卡4(录制往返);`cargo check --lib` exit 0(独立复验)、
本模块 15/15。

## 核对

### ① InputFrameSerde 字段序 — PASS
`InputFrameSerde { translation_state: [u64;11], prev_normalized_cursor_pos: [f32;2],
time_delta_seconds: f32, main_action_set_value: Vec<[u8;12]> }` —— 字段序与 RE-ANALYSIS2
serde 视图 4 键 `translation_state/prev_normalized_cursor_pos/time_delta_seconds/
main_action_set_value` 对齐 ✓;与运行时帧布局是不同子结构(ALGORITHMS §6.2)已注明 ✓。

### ② try_load 路径 — PASS
读路径 → `RecordingFile{replay_initial_state, first_frame}` serde 反序列化 → 首帧入流
`add_frame(..., false)`;调用方 `control_input_replay` 成功后 `mode=Replay`、`index=u64::MAX`
(=-1)。与 0x1409423d0 / trailer Replaying 语义(index=-1 → advance 到 0 取首帧)一致 ✓。
`replay_initial_state` 10 字段语义 U、文件路径层与 save/session 组格式约定待协调(本实现
接受 JSON 路径)——诚实登记。

### ③ seek_to_sync_point — PASS
`index = frame_index−1` ✓;`"No sync points found in replay state"` panic ✓;`"Can only seek
to sync point if replaying but mode is {mode}"` 带 mode 格式化 panic ✓(与 0x140941a30 decomp
一致)。clone toolbar(sync+0x20)+ load_parameters(sync+0x30)+ immediately_finish_interpolating
依赖状态块子字段类型化恢复,**待取证**(诚实登记)。

### ④ 卡3 登记诚实 — PASS
`systems::steam` 不在本 crate:SteamInputActive 文档已登记 Arc Clone ClientManager →
SteamAPI_SteamUtils_v010(0x142923570)→ IsSteamRunningOnSteamDeck(0x1429234f0) 门控事件源;
controllers→InputTranslationState 在 `systems::steam::update_steam_inputs`(22 同构块,跨域
ui-input/steam)。键绑定表 40 槽×12B + 0xc3 哨兵已定,**动作↔槽精确映射仍 U**(诚实)。

### 潜在待取证(不阻塞)
Replay 分支 `index >= frame_count` panic 与 index=u64::MAX 的初始时序依赖调度顺序
(advance 先于 Replay 读),全局调度未重建,待接线时核对。

## 结论与 DISPATCH 建议

- 卡3/卡4 复验通过:**input 卡3/卡4 → 验收闭合(库内部分)**,库外项(Steam 接线、文件路径
  与 save/session 协调、动作↔槽映射)登记待协调。
- 建议:input 模块维持 **review_passed(限定已实现范围)**,卡3/卡4 库内部分计入已实现;
  交互组三模块全部限定通过。库外项由宿主/跨组接线时闭合,不据此宣布整模块"100% 完成"。
