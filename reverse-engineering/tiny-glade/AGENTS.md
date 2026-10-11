# 逆向工作约束

先读 [PROJECT_MEMORY.md](PROJECT_MEMORY.md) 与 [TASK_BACKLOG.md](TASK_BACKLOG.md)。以原作锁定依赖与游戏自有源码为边界；直接复用依赖，不恢复其实现。当前源码优先，场景/GPU后置。依赖适配的变更必须标记旧差分报告过期，重新校验后再更新进度。

执行 [DEPENDENCY_FILTERING.md](DEPENDENCY_FILTERING.md)：每个新实现先对照三版 Cargo/PDB/定义模块，查询 `tracking/dependency-exclusions.json`。确认的库、标准库及泛型容器/Serde/ECS实例不得当作核心源码重写；local/path和私有路径不自动等于游戏业务。三版共享代码不能单凭相同就排除，自有墙体/屋顶/建筑规则仍需恢复。

当前首要交付是完整建筑状态链：Wall权威输入 → Roof创建/更新/编辑/删除 → 墙/砖/支撑/开口 → 门窗附着 → 历史事务与更新传播。复用Bevy等底层接口，不手写实体表、archetype、hashbrown/Vec/event存储布局。以业务模块联接和原指令验收衡量进展。

本轮明确的纠错必须沿用：PublicWall+0x10为raw Curve2点数，+0x30/+0x28c为raw长度；Nani tag9/7为版本对应subset kind，不是生命周期。组件derive的Self登记不构成required edge；先看原Bundle::component_ids和真实descriptor再决定组件图。新业务必须同时接原始输入、传播和删除清理，不能仅以probe通过结束。最新链证据见 evidence/building-gather/findings.md。

继续 native→Rust 恢复时应用 [native-rust-reconstruction](/Users/yuluo/.codex/skills/native-rust-reconstruction/SKILL.md)，按需读取参考；具体地址/字段仍保存在项目evidence，不复制进通用skill。
