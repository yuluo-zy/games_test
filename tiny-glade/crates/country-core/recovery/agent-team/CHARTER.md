# country-core 智能体团队章程

> 中央智能体(主会话 + 协调 agent)+ 3 组 × 3 角色(restorer / reviewer / re-analyst)共同推进
> country-core 模块复现。本文件定义结构与协作协议;分派台账见 [DISPATCH.md](DISPATCH.md)。

## 目标

每个模块按既定工作流推进:**先分析原二进制业务逻辑并整理 → 再按 PDB 骨架实现 Rust**。
实现必须**可读、可信、功能完备**:关键逻辑标注证据来源;不得偷懒(空壳/占位/假数据/依赖胶水冒充业务);
不得缺失(原函数清单逐一对齐)。完成度声明须有原生证据(枚举原函数 + fresh 反编译复核),不得仅凭对照文档。

## 团队结构

| 角色 | 职责 | 输出落点 | 可否改 src |
|---|---|---|---|
| 中央智能体 `coordinator` | 读现状、维护分派台账、定每模块验收门槛、汇总各组报告并给出下一步分派 | `planning/`、`DISPATCH.md` | 否 |
| 每组 `restorer` | 按历史分析(章节/算法/合同/骨架)编写/补齐 Rust 实现,编译检查 | `reports/<组>/<模块>/RESTORE.md` | 是 |
| 每组 `reviewer` | 独立核对实现是否真正实现原二进制业务逻辑,揪出偷懒与缺失 | `reports/<组>/<模块>/REVIEW.md` | 否 |
| 每组 `re-analyst` | 对未挖掘/未分析的原函数定点深入逆向(Rust 专属特点) | `reports/<组>/<模块>/RE-ANALYSIS.md` | 否 |

## 协作协议

1. **分派**以 `DISPATCH.md` 为准,主会话居中动态调节:restorer 交模块后 → reviewer 独立审查 →
   FIX 项回 restorer 修复 → re-analyst 补挖证据不足处 → 再交 reviewer 复验。
2. **只读共享**:默认 Ghidra/ReVa 用 `--worker shared` 只读。ReVa 脚本 / 需要写库时用各自的唯一
   `--write` worker(worker 名 = 各自 agent 名)。不得改他人 worker 数据。
3. **src 互斥**:同一时间一个模块只由一个 restorer 编辑;reviewer / re-analyst 只写报告与证据。
4. **验收门槛**由 coordinator 在 DISPATCH.md 为每模块定义:闭合哪些原函数、接上哪些真实消费者、
   需要哪些证据。模块标 `review_passed` 前不算完成。
5. **诚实分层**:实现层面(库内有代码、可看台账)≠ 验收层面(行为等价、消费者闭合)。不得把编译通过
   或局部测试通过当作整模块完成。
