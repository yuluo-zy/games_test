# 内网 LLM 分析 worker

用途：把证据已准备好、输入输出边界清楚的重复静态分析委派给内网模型。主 agent 负责选边界、补证、复核和集成。分析结果不是原版行为验收，不直接提高游戏还原进度。

## 调用

Node.js 内置 fetch，无额外 npm 依赖。密钥只从进程环境 `TG_LLM_API_KEY` 读取；禁止写入仓库、job、证据包或日志。可选环境 `TG_LLM_BASE_URL`（默认内网 `/v1/` 地址）、`TG_LLM_MODEL`（默认实测的 `deepseek-ai/DeepSeek-V4.1-Flash`）。

```powershell
node D:/game/reverse-engineering/tiny-glade/scripts/llm-worker/run.mjs --jobs D:/game/reverse-engineering/tiny-glade/evidence/llm-worker-2026-10-19/jobs-asm-split.json --concurrency 2 --thinking template-low
```

默认 `--thinking template-low`，发送 `chat_template_kwargs={thinking:true,reasoning_effort:25}`；默认输出预算 18000。`template-disabled` 为实测有效的无思考模式，但本轮即使缩小任务仍产生关键语义错误，不用于逻辑裁决。`disabled` / `enabled-low` 为厂商标准参数对照模式，本内网实测标准 disabled 未关闭思考；`default` 不发送开关。参数是否被网关/上游尊重，以 reasoning_tokens 和完整正文为准；HTTP 200 不证明生效。来源：[DeepSeek 官方参数说明](https://api-docs.deepseek.com/guides/thinking_mode/)、[V4.1 本地服务参数](https://recipes.vllm.ai/deepseek-ai/DeepSeek-V4.1-Flash)。`--timeout-ms` 默认 240000；外部执行须异步等待并保留进度沟通。

jobs 是数组，每项含 `id`、`packet`、`prompt`、`output`、`depends_on: []`；可逐项指定 `thinking`、`max_tokens`。路径相对 manifest。默认最多 2 并发，可设 1..4；只接受无前置依赖、唯一 id 和输出路径的任务。来源/模型/提示词/模式/预算相同且此前完整返回的任务命中缓存，不重复请求。跨模块依赖必须先由 owner 提供冻结合同，不能靠并发任务猜测。

## 委派与保留职责

| 优先委派给内网 worker | 交付合同 |
|---|---|
| 独立业务闭包的长函数静态阅读 | 全部分支/谓词、状态读写、顺序、副作用、退出/释放、未知项及证据行号 |
| 多个相似 producer/consumer 的机械对照 | 同一字段/事件合同下的差异表；分别引用每个来源，不能按同址/同名推等价 |
| 现有 Rust 与原证据核对 | 逐分支映射、遗漏与额外行为、具体原证据；不凭编译成功认定等价 |
| 已有资料里的偏移/常量/事件写读整理 | 结构化事实表与缺证列表；不生成无证语义字段名 |
| 已确认依赖胶水的重复分类 | 依赖边界与游戏调用策略；保留 drop、错误和顺序后果，不重写依赖 |

主 agent 保留：真实函数归属/ABI与别名裁决、首次算法/布局歧义、跨模块权威状态与生命周期、数据库/源码写入、依赖锁和当前架构适配、原指令核验与最终验收。worker 可以提出缺证请求，主 agent 再定点补证；不赋予它全项目/共享数据库写权限。

独立任务按 consumer 驱动的完整功能边界划分。单函数测试是工具能力基准，不改变本项目以 producer → 权威状态/算法 → consumer → 交互结果恢复的工作单位。T13 反馈捕获/状态生产者仍排除。

## 提示词及结果门禁

后续任务使用 `review-production.txt`，小范围复核使用 `targeted-audit.txt`。v1/v2/v4 提示词保留为实验指纹，不重写已跑版本。证据以带行号的原文发送，避免在 JSON 字符串里转义整份源码；元数据单独发送。原文只清空行，保留原始行映射与哈希。重复容器块可合并描述，但必须引用全部出现位置，不能删游戏分支。未知 ABI、寄存器和调用地址原样保留；禁止默认值、空操作、猜枚举及完整 Rust 实现冒充恢复。

无原汇编时，反编译的 NaN/比较语义只能作候选合同；数值门控、goto/共享尾部、位标志交互有歧义时补小段原指令再派。库胶水可复用，不能把原算法分支简化掉。思考关闭不是本项目的提效默认策略：优先冻结输入、缩小交付合同、缓存结果、独立任务并发和一次性补证。

程序检查 finish_reason、JSON、必需字段、证据行号范围；错误和截断不作为完整结果。`accepted_transport` 只说明传输/格式完整，`semantically_verified` 始终默认 false，由主 agent 在独立 review 台账记录逐项裁决。仅格式正确或引用存在不证明引用支持结论。

失败不自动重复相同请求：先检查是 token 截断、模式未生效、网关错误、输入缺证还是任务过大，再做有理由的一次参数/边界修正。避免让模型反复解决同一个未知；新证据先由主 agent 提供。

2026-10-19 的长函数基准及保留失败记录位于 `evidence/llm-worker-2026-10-19/`。`prepare-stairs-benchmark.mjs` 仅准备该已知 B 函数的证据与原字节身份，不是通用反编译器，不替代 MCP/PDB，也不生成游戏实现。
