# rhapsody 攻坚工具审计

日期：2026-10-12。范围为当前 Tiny Glade 构建，SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。

## 结论与证据

需要优先优化取证编排和验收流程。当前没有证据表明需要替换反编译器或重新扫描全程序。

| 优先级 | 观察 | 影响与处理 |
|---|---|---|
| P0 | 当前工具目录无 REA/ReVa/RE-MCP 原生入口；实际执行 `rea-agents@6.3.0 doctor --client codex --json` 报 Codex registration missing/config_drift，但 Ghidra 12.1.4、JDK 和 Windows provider 可用 | 继续复用官方 MCP SDK 和现有配置连接。doctor 不等于实际连接。当前不改全局配置，不安装新引擎 |
| P0 | `scripts/mcp-job.mjs` 和 `scripts/render-architecture-mcp.mjs` 的 finally 默认 `close_database(save:true)`；脚本没有共同会话互斥协议 | 新取证应默认 save:false，协调共享数据库访问，记录实际连接/工具 catalog/关闭结果；现有脚本保留以免影响旧证据 |
| P0 | 第四批三个 validation 均已落盘，但缺少与前三批对应的 `validate_fourth_batch_contract.py` 和 integration.json | 创建统一门禁，核对目标、合同、代码/输入哈希、queue 域和 shader 接口边界。候选测试与原码差分必须分别计数 |
| P0 | batch02 gpu-lifecycle / batch03 sequence-domain 的 `get_xrefs_to` 对 begin/create/submit 仅返回1个 PDB DATA记录；workgraph 已独立解码 begin 的8个直接调用和所选 submit 的1个直接调用 | 这是当前数据库引用覆盖不完整的实证。xref 查询结果必须与原指令调用图交叉核对；空/仅DATA不能作为全程序无caller证据，不能靠反复相同查询解决 |
| P1 | 第五批 pipeline agent 实际反编译遇到跳转表越过 PE 函数边界、将 .rdata 误作指令 | 对含跳转表的函数保留原PDB/PE边界、限定反汇编和原表数据；反编译作为辅助，不据越界伪码补造分支 |
| P1 | 各包存在独立 query 脚本和 report；公共脚本没有基于 target/provider/操作参数/输入哈希的复用协议 | 为后续取证加入可审计缓存键与失效检查，失败结果不得当成成功缓存；保留完整 Evidence，不用摘要代替原输出 |
| P1 | PDB native signature 为 void()，直接调用图不解析全部间接调用；shader name candidate 仅3个 | 下一轮投入 producer/consumer、vtable/句柄和创建参数证据。不能靠扩大字符串扫描或按文件数统计来补语义 |

已有工具值得继续复用：workgraph 的地址去重/别名与 PE 边界、原指令 oracle、shader 全量反射和确定性输出。接口反射完成不表示 shader 算法或 Vulkan 画面完成。

## 并行推进与共享约束

- 工具与整合 agent：独占新增工具脚本及第四批 integration；不编辑现有候选 crate。
- 录制 agent：新增第五批 record-resources 证据及独立 crate；从原入口覆盖非空访问链与 callback 状态。
- pipeline agent：新增第五批 pipeline-descriptors 证据及独立 crate；首个持久数据库取证窗口归该 agent。
- 主 agent：第五批 queue-callers 证据、总进度和最终统一验收。共享数据库访问由主 agent 分配，等待者先用已有证据开展工作。

新工作仍以可编译 Rust、类型/控制流和原作边界验证为交付，不把工具脚本计入私有源码恢复。live GPU/完整游戏画面未验收，M12 保持 verified_partial。

## 本轮落实

三个agent均完成交付。新 `scripts/render-recovery/mcp-evidence-job.mjs` 的只读白名单、save:false、会话锁、失败隔离和缓存失效由27个本地fake MCP用例验证；provider manifest/数据库snapshot缺失时默认禁用分析缓存。旧脚本没有自动迁移，仍需窗口协调，详见[工具说明](tooling/README.md)。

第四批统一门禁50/50通过；第五批27/27通过并新增三个可编译Rust工作包，参见[第五批](parallel-batch-05/README.md)。未替换Ghidra、未重新全程序分析、未改全局注册配置。进度生成脚本增加本轮integration输入hash校验和渲染状态生成，避免报告过期或渲染进度被覆盖。

台账生成时还确认旧生成器把Cargo结构验证的 `verified_structure_only` 当成行为报告并要求 `ok:true`，导致M12更新失败。已为source_gate指定行为verification_reports，结构证据只保持其结构边界，第四/五批source/input合同独立校验；没有将结构检查冒充源码恢复。实际进度生成已通过。
