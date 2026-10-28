# 工具调用记录

| 子目录/文件 | 内容 |
|---|---|
| [`jobs/`](jobs/) | 可复跑的 MCP job 定义（函数、引用、内存、反编译等） |
| [`results/`](results/) | 已执行的 job 结果 |
| [`schemas/`](schemas/) | 工具参数 schema 快照 |
| [`manifest.json`](manifest.json) | 机器可读工具入口与实测参数 |
| [`manifest-macos-2026-10-11.json`](manifest-macos-2026-10-11.json) | 2026-10-11 的归档副本，保留历史路径；当前入口以 `manifest.json` 为准 |

## 使用

- 先读 `../docs/current/AI_REVERSE_TOOLS.md`，再按工具名选择 job。
- 部分 job/result 仍含迁移前路径（`games_test/`、`D:/game/`、旧发行目录）；作为历史输入可查阅，重新执行前需改为当前路径。
- 结果文件是原始工具输出，不是业务验收；判断需回到报告与源码。
- 不在此目录保存密钥或 token。

## 维护

- `../scripts/reverse_tools/update_manifest.py` 会更新 `manifest.json` 并归档旧版本。
- 新增 job 时保持参数边界（每批函数 ≤20、内存 ≤4096 字节等），详见 `AI_REVERSE_TOOLS.md`。
