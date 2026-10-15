# systems/backend/panic_reporter_fetch_active_session.rs

> 文件作用：**崩溃报告取活动会话。`panic_reporter_fetch_active_session` 全内联（PDB inline 记录），向崩溃报告器提供当前活动 session 标识（`ActiveSession` 资源）。**

函数总计 1：业务 0 / 内联无独立符号 1 / 派生胶水 0

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::backend::panic_reporter_fetch_active_session::panic_reporter_fetch_active_session`（行 7）— **I**：全内联，无独立地址；行为从名称与 PDB 记录推断（启动/崩溃时读取 `Res<ActiveSession>` 交给反馈报告器）。无机器码证据可归属，不臆造调用面。

## 派生胶水（derive/宏生成；不计独立业务）

（无）

## 备注

- 归入 backend 域内联项；`ActiveSession` 资源被 feedback_report（audio-misc 域）读取为跨域线索。

## 实现方式（算法级）

**panic_reporter_fetch_active_session（全内联，I）**：
- 无独立物理符号（PDB inline）；行为从名称+PDB 记录推断：启动/崩溃时读 `Res<ActiveSession>` 交给崩溃报告器（feedback_report_resources，audio-misc 域）。（I）
- 无机器码可归属，不臆造调用面；`ActiveSession` 资源被 feedback_report 消费（跨域线索）。（I）
