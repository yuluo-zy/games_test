# Window Lab V0 验收记录

日期：2026-10-16。本文件仅登记有证据的结果；版本目标见 [DELIVERY_PLAN.md](../../DELIVERY_PLAN.md)。

| 检查 | 状态 | 证据 / 限制 |
| --- | --- | --- |
| example 构建 | PASS（首轮） | `cargo build -p rhapsody --example window_lab --offline` 成功；最终变更后复验待登记 |
| 素材单测 | PASS | `cargo test -p rhapsody --example window_lab --offline`：3/3 通过 |
| 360 帧真实窗口 + GPU 读回 | PENDING | 等待窗口运行验收 |
| 缩放/最小化/恢复 | PENDING | 待运行 `--script` |
| 暂停/单步/切换/关闭 | BLOCKED | 自动审批两次拒绝 computer-use 初始化，理由是判定缺少分支边界后的授权；已请求用户再次授权新 demo 窗口验收 |
| 最终显示图像检查 | BLOCKED | 同上；未用其他工具绕过窗口操作拒绝 |
| M0 里程碑 | IN_PROGRESS | 编译和 CPU 单测不替代实际窗口验收 |

本记录不证明新的提交事务、呈现生命周期或任何后续优化完成。
