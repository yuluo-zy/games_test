# REVIEW3 — systems/backend/input(三轮 FIX 复验)

审查员: reviewer-interaction | 日期: 2026-10-15
基线: restorer 已按 REVIEW2 修复全部 FIX;`cargo check --lib` 0 error、本模块 12 测试过。

## 三轮 FIX 复核

### 1. 捕获路径 tag 2 修正 — **PASS(与机器码一致)**

`fold_cursor_events` 捕获分支已改回:
```rust
InputCursorMoved::Relative(value) | InputCursorMoved::RelativeWhileCaptured(value) => delta += value,
InputCursorMoved::Absolute(_) => {}
```
- 与 0x140b17a80 机器码捕获路径 `cmp dword [r15+8], 1; jne 0x115(add)` 一致:tag 0 与 tag 2
  都累加,仅 tag 1 跳过 ✓。
- 非捕获路径 tag 2 跳过(`RelativeWhileCaptured(_) => {}`)✓。
- `update_cursor_position::fold_cursor_moved` 同步为 `Relative | RelativeWhileCaptured => add,
  Absolute => replace` ✓(内联 I,与同帧消费同一流的 tag 语义一致)。
- 测试 `capture_accumulates_relative_and_while_captured_events` 期望 delta=(5,5)(Relative 2 +
  RelativeWhileCaptured 3),hardware=0 ✓,覆盖该路径。

## 结论与对 DISPATCH 建议

- input 模块 REVIEW2 的 FIX(捕获 tag 2)已正确落地,**复验通过**。
- 维持 `in_progress`;待 re-analyst 补证据后闭合:
  1) translate_input 逐动作键码绑定表 + Steam(`game_key_bindings`、`systems::steam`);
  2) InputFrame 0xd0 B 布局与录制文件 serde(`try_load`);
  3) 同步点 seek(control_input_replay 的 mode 检查/恢复/panic 分支);
  4) normalize/unnormalize 精确公式。均已在 RESTORE 登记待取证,未见冒充完成。
