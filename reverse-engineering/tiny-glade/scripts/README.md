# 脚本索引

脚本按用途分组。大多数脚本假定工作目录为项目根（`reverse-engineering/tiny-glade`），部分保留 Windows `D:/` 路径，运行前先核对。

## 维护中的入口

| 脚本 | 用途 |
|---|---|
| [`update-module-progress.py`](update-module-progress.py) | 从 `tracking/module-coverage.json` 等生成模块进度 |
| [`update-progress.py`](update-progress.py) | 更新 `tracking/project.json`；根路径按脚本位置解析 |
| [`build-package-graph.py`](build-package-graph.py) | 生成包依赖图 |
| [`chapter-audit.py`](chapter-audit.py) | 章节/源码覆盖审计 |
| [`run-decomp-batch.py`](run-decomp-batch.py)、[`gen-decomp-job.py`](gen-decomp-job.py) | 批量反编译作业 |
| [`rne-read.py`](rne-read.py)、[`rne-disasm.py`](rne-disasm.py)、[`capdis.py`](capdis.py) | RNE 读取、反汇编与能力发现 |
| [`mcp-job.mjs`](mcp-job.mjs) | MCP job 执行入口 |
| [`verify-project.py`](verify-project.py)、[`verify-roof-state.py`](verify-roof-state.py)、[`verify-window-aspect.py`](verify-window-aspect.py) | 局部验证 |
| [`multi-version/`](multi-version/) | A/B/C 多版本差分 |
| [`render-recovery/`](render-recovery/)、[`render-passes/`](render-passes/)、[`render-skeleton/`](render-skeleton/) | 渲染证据与 Pass 分析 |
| `reverse_tools/` | Ghidra/ReVa 持久 worker 入口 |
| `llm-worker/run.mjs` | 内网 LLM 批量分析（密钥只读环境变量） |
| `source-edits/`、`native-workflow/` | 源码编辑与原生工作流 |

## 一次性/历史脚本

`d1-*`、`render-architecture-probe/resolve`、`render-leaf-extract`、`run-decomp-expansion`、`run-supp-decomp`、`fill-chapters-terrain-water`、`import-module-progress-baseline`、`upgrade-hist-save-chapters` 等没有维护入口引用，保留作分析轨迹。`verify-rectangular-edges.py` 已被 `-full.py` 取代。

## 约定

- 证据输出写入 `../evidence/<topic>-<date>/`，报告写明命令、输入身份与验证边界。
- 不在脚本里写默认凭据；网络请求遵循 `AGENTS.md` 的代理与本地磁盘规则。
- `scripts/` 与 `tools/` 的分工：前者是项目恢复流程，后者是独立分析工具。
