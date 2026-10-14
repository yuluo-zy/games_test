# 初始整理脚本归档

本目录的 `.txt` 文件记录历史导入过程，供审计来源与策略演变。它们不是当前工程的恢复源码，也不是持续开发或增量导入入口。

`foundation-import-before-scope-pruning.py.txt` 记录初次基础库迁移以及当时非默认实验 API 的选择。该选择已被后续审计收紧：未知 IndexCondenser insert、投影轴/stride 和 2D 全局样条 query 已物理移出正式 utils。不得重跑旧脚本覆盖现有工程或重新引入这些猜测。

当前各 package README 与 `foundation-audit.json` 记录最终范围。`tools/finalize_foundation_audit.py` 只刷新审计哈希，不导入或覆盖生产源码。

其他一次性改写器也仅作历史留档：

- `write-foundation-readmes.snapshot.py.txt`：记录本轮各 package README 的生成内容；持续开发应直接维护对应 README，不重新生成覆盖。
- `remove-unrecovered-utils.once.py.txt`：记录猜测 API 的一次性物理移除与原 get/remove 测试输入初始化调整；已执行完成，不作为后续迁移入口。

活动 `tools/` 仅保留验证、来源清单与审计元数据维护工具。这些归档脚本以及它们曾采用的实验策略不计作游戏恢复源码。
