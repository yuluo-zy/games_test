//! `resources` — WP17 资源模块根（历史回放域）。
//!
//! wp13 已恢复的 `session`/`prefab`/`history` 核心在共享 `country-core` crate；
//! 本独立门禁只含回放扩展面（`history` 编辑记录容器 + ReplayBuffer）。

pub mod history;
