# input/panic_repro_state.rs

> 文件作用：**崩溃复现状态机**。`PanicReproState::is_inactive` 检查首字节判别 == 0。
> 证据：原字节 `cmp byte [rcx],0; sete al; ret`（0x140833230）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::input::panic_repro_state::PanicReproState::is_inactive` | 0x140833230 | 24 | 是否非活跃 | **O** | 原字节 `byte[0]==0` |
