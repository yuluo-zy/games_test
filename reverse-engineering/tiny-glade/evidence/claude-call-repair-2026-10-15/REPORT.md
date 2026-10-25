# Claude 调用复验与入口修复

日期：2026-10-15。本机 Windows，Claude Code 2.1.299。

未传 `--model`，未设置模型覆盖，未修改现有 Claude 配置。直接 `claude -p` 请求约 9.8 秒成功，修复后技能入口请求 5.316 秒成功；两次均返回 `CLAUDE_OK_20261015`，退出码 0，`subtype=success`，`is_error=false`。后一次完整证据见 `probe.result.json`、`probe.stdout.json`、`probe.stderr.txt`。

先前把 `[claude-code:unrecognized_model]` 判成 provider 拒绝模型不成立：该日志与正常结果同时出现。旧 raster 240 秒超时记录仍作为历史事实保留，其原因未由这条日志证明。

本轮为 `C:/Users/liyu/.codex/skills/claude-reverse-assist` 增加 `scripts/claude_print.py` 并更新技能说明。入口复用已配置模型/provider，直接运行 npm shim 对应的原生 executable，通过 stdin 传入 UTF-8 提示词，禁用本次分析的 built-in tools/MCP，无自动重试。成功必须同时满足退出码 0、有效结果 JSON、明确成功状态和非空回答；超时及原始输出分别保存。项目工具说明已接入该入口。

此结果只证明本机实际 CLI 请求可用，不构成任何游戏业务逆向验收。

辅助检查：脚本语法、技能 name/description、精确回复、无模型参数、空工具参数和项目说明 diff 检查通过。skill-creator 的通用 quick_validate.py 因本机 Python 缺少 PyYAML 无法运行；本轮未为此安装额外依赖。
