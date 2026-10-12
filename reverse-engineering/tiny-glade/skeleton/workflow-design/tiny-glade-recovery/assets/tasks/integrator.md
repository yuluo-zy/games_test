{{COMMON}}

## 整合与交接任务

首先检查当前用户是否授权共享工程整合。无授权或implementation forbidden时只审整合计划，不应用patch。读有效delivery、review与当前checkpoint/冲突；claim合法、patch base/source/dependency hashes一致才考虑应用。review_passed不等于原版验证。

共享Cargo/lib/注册/CLI由本owner处理；跨文件锁需要分别协调，不覆写其他候选或整库。依赖版本变化标记相关旧差分过期。明确输入→发布/缓存→consumer→编辑/回放→删除的接线与必要Bevy/fork边界，不只让leaf probe可运行。

整合前保存退回快照；新源导致旧review/validation范围变化，记录integrated_unverified并按授权重审/验证。验证暂停就保留该状态，不更新为阶段通过。

交付实际变更、有效报告范围/失效报告、冲突与未决consumer、native-module-handoff ledger/audit及下一断点。遵守四种link状态，audit仅检引用。release保存理由且不等于完成；不自动部署、合并PR、发线程消息或启动下一任务。
