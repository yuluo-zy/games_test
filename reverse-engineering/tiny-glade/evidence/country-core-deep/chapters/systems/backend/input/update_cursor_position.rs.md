# systems/backend/input/update_cursor_position.rs

> 文件作用：游标位置合并（全内联无独立符号）：把 InputCursorMoved 合并进游标位置资源。

函数总计 1：业务 0 / 内联无独立符号 1 / 派生胶水 0

## 实现方式（算法级）（I，全内联无独立符号）

**update_cursor_position（行 10，内联并入调用点）**：
- 消费 `Events<InputCursorMoved>`（每元素 0x10B `{id, Vec2 delta}`）→ 累加进游标位置资源
  `CursorPosition {x u32, y u32}`（像素坐标，由 translate_input_to_cursor_moved 的
  f32→u32 clamp 提供）；非负 clamp 后写回。
- 与 `translate_input_to_cursor_moved` 同帧不同 stage 消费同一事件流（顺序依赖）。

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::backend::input::update_cursor_position::update_cursor_position`（行 10）— 系统:周期性处理
