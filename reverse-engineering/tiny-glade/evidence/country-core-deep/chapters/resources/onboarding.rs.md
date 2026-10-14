# resources/onboarding.rs

> 文件作用：新手引导：WelcomeScreenState（is_closed = 判别==2，0x1409a9140 折叠处证实）+ NotifyHintSystemSnapping 事件 + TutorialOp 事件（tiny_glade settings 系统消费，PDB 注解证实）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::onboarding::WelcomeScreenState::is_closed` | 0x1409a9140 | 83 | WelcomeScreenState::is_closed | O（手解）：`cmp dword [rcx],2; sete al`（与 UiFocus::is_mei 同址折叠） | ui-input\decomp\decomp-0x1409a9140.txt |
