# resources/mouse_capture.rs

> 文件作用：光标捕获命令处理：读 CursorCaptureCmd（utils::get_last_event 取最后一条）；未捕获→CursorCapturedPosition={1,pos}+WindowFocusEvent{focused=0}+EscMenuState{2,2}；捕获→{0}+{focused=1}+EscMenuState{3,2}。命令族 CursorCaptureCmd/OsCursorCaptureCmd/OsCursorVisibilityCmd/CustomCursorSsboCreatedCmd。

函数总计 4：业务 1 / 内联无独立符号 2 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::mouse_capture::mouse_capture` | 0x140b17830 | 0 | 系统:事件处理（mouse_capture） | O：完整语义（DIGEST §2.6 + round2 CLOSURE）；EscMenuState 值 0x200000002/3 = {sub2,disc2}/{sub2,disc3} 布局待中央定序 | ui-input\decomp\decomp-0x140b17830.txt |

## 实现方式（算法级）（O，decomp-0x140b17830 全解）

**mouse_capture @0x140b17830（系统，消费 CursorCaptureCmd）**：
1. `get_last_event<CursorCaptureCmd>`（`country_core::utils::get_last_event` 取最后一条命令，
   0x10B 事件队列）；`flag = 命令字段`（bool）。
2. **未捕获（flag==0）**：
   - 写 `CursorCapturedPosition = {1, u64 窗口坐标}`（已捕获标志=1）；
   - push 0x18B `WindowFocusEvent {id, u32=0(focused), u64=窗口}`（失焦）；
   - push 0x10B `0x200000002` = EscMenuState `{disc 2, sub 2}`（未捕获态）。
3. **捕获（flag!=0）**：
   - 写 `CursorCapturedPosition = {0, ...}`；
   - push `{id, u32=1(focused), 窗口}`（聚焦）；
   - push `0x200000003` = EscMenuState `{disc 3, sub 2}`（捕获态）。
4. 语义终版（round2）：光标锁回 OS → disc 3；OS 光标可见 → disc 2（配合 is_open 的 {3,4} 判定）。
5. **fork 注记**：此处 0x10B push 是 `{载荷@0, id@8}`，而 translate_input_to_cursor_moved 是
   `{id@0, 载荷@8}`——两种形状并存（疑自定义记录队列），供中央裁决。

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::mouse_capture::CursorCapturedPosition::new`（行 56）— 构造
- `country_core::resources::mouse_capture::CursorCapturedPosition::is_currently_captured`（行 62）— 谓词/校验

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项
