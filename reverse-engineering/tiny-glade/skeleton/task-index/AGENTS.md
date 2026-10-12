# 使用当前逆向索引

- 先读本目录 README 与原项目 AGENTS/AI_REVERSE_TOOLS；不从索引推断完整 ABI 或行为已知。
- 一次领取一个跨版本文件组；可缩小到某函数，但不另给 A/B/C 三个 owner。使用 task_queue.py 的独占 claim，不自行覆写他人的锁。
- C 存在时 C 为主目标，A/B 作参照；源 checksum/机器码相同仅支持明确范围的复用候选。不同版本的字段、枚举、依赖与调度不可直接套用。
- 先看现有 Rust 名字候选和保留的 B native 证据。名字匹配不是实现已验收；公共库/标准库/容器直接复用，不恢复其内部。
- 工作输出限定本任务 work/<TASK_ID>/<OWNER>。骨架只读作为导航。共享生产工程、Cargo/lib/注册/CLI 由整合 owner 应用交付。
- 查询默认只读；需要 DB overlay/ReVa 脚本用领取结果中的唯一 worker 和 --write，保留修改/原证据/保存读回；不修改共享数据库。
- 交付包括真实输入、输出/消费者、版本边界、依赖、未知字段和必要的增改删/历史传播。纯工具交付仍须对应原消费者。
- 验证按用户当前授权执行；implemented_unverified 与 verified 分开。领取/索引/审计不代表业务完成。
- 按需使用 [项目专用 skill](../workflow-design/tiny-glade-recovery/SKILL.md) 与其角色模板。workflow.py prepare 只读索引，保存包须已有匹配 claim；freeze/check 只验证显式文件指纹。review 仍独立核对原证据，不替代原版差分。
