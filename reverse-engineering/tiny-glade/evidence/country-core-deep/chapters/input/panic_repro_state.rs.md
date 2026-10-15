# input/panic_repro_state.rs

> 文件作用：**崩溃复现状态机**。`PanicReproState::is_inactive` 检查首字节判别 == 0。
> 状态机（O 原字节，本轮复核）：判别字节 @+0 == 0 → inactive（`cmp byte [rcx],0; sete al; ret`）。
> 17 调用点（ui-input DIGEST 登记），gates 调试 panic 复现输入路径（I：调用方语义）。
> 证据：原字节 `cmp byte [rcx],0; sete al; ret`（0x140833230）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::input::panic_repro_state::PanicReproState::is_inactive` | 0x140833230 | 24 | 是否非活跃 | **O** | 原字节 `byte[0]==0`（0x80 39 00 0f 94 c0 c3）；首字节判别==0 → inactive；17 调用点 |
