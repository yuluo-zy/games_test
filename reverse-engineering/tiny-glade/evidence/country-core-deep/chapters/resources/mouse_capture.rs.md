# resources/mouse_capture.rs

> 文件作用：光标捕获命令处理：读 CursorCaptureCmd（utils::get_last_event 取最后一条）；未捕获→CursorCapturedPosition={1,pos}+WindowFocusEvent{focused=0}+EscMenuState{2,2}；捕获→{0}+{focused=1}+EscMenuState{3,2}。命令族 CursorCaptureCmd/OsCursorCaptureCmd/OsCursorVisibilityCmd/CustomCursorSsboCreatedCmd。

函数总计 4：业务 1 / 内联无独立符号 2 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::mouse_capture::mouse_capture` | 0x140b17830 | 0 | 系统:事件处理（mouse_capture） | O：完整语义（DIGEST §2.6 + round2 CLOSURE）；EscMenuState 值 0x200000002/3 = {sub2,disc2}/{sub2,disc3} 布局待中央定序 | ui-input\decomp\decomp-0x140b17830.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::mouse_capture::CursorCapturedPosition::new`（行 56）— 构造
- `country_core::resources::mouse_capture::CursorCapturedPosition::is_currently_captured`（行 62）— 谓词/校验

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项
