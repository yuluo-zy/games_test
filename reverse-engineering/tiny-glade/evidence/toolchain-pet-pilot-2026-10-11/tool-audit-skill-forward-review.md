# 更新技能独立 forward-test / review

日期：2026-10-11。审查 [native-tool-batch](/Users/yuluo/.codex/skills/native-tool-batch/SKILL.md) 与 [native-module-handoff](/Users/yuluo/.codex/skills/native-module-handoff/SKILL.md)。先独立设计合成fixture，未照root五案例答案；不修改技能/供应商源码，不重复pet查询，不执行MCP、业务构建或native oracle。

初轮结果：[首次独立fixture](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/toolchain-pet-pilot-2026-10-11/tool-audit-forward-review/summary.json)。root修订后结果：[revision3独立重测](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/toolchain-pet-pilot-2026-10-11/tool-audit-forward-review/revision3/summary.json)。全部输入、fake operator、原始输出和首次问题均保留，没有覆盖失败报告。

## 结论

此次五个独立检查在新revision均符合预期。batch脚本测试前后SHA相同：`1e1ccbd0dfed987a131754d621b737e9cd9e9ee0ee18f32a170da1741183995a`。handoff auditor SHA：`0bd23f0749abe4957699d738beda4f7cec7d9d9f6475ed39b59c8de755d236b6`。后续改版需新证据，不把本轮接受范围扩大。

| 独立检查 | 初轮实际问题/表现 | 新revision实际结果 |
|---|---|---|
| 嵌套failure，MCPisError=false/reportok=true | 正确拒绝，但双通道重复errors | exit1，嵌套错误和缺code各一次，未误标成功 |
| structuredContent/text同时带两个函数 | summary.functions=4 | valid case exit0，唯一函数摘要=2，MCPsteps=1 |
| `void pet(void) { return; }` | placeholder=true仅warning，仍exit0 | exit1，两个明确Unresolved placeholder-only错误 |
| xref limit0且缺program | 启动operator并exit0 | preflight exit1，operator_processes=0，缺selector拒绝 |
| verified link同时有`actual consumer missing`gap | exit0，unresolved_links空 | exit1，Verified link still declares gaps |

xref limit本轮组合fixture先触发selector检查；1..100limit/callercontext小范围检查另从当前源码确认，未执行真实unbounded query。复杂void函数仅warning的处理优于盲拒所有void原型：零参数/空或RETURN-only需要原ASM/caller解释，非空生命周期void可继续研究。

## 原问题与修复范围

初轮batchSHA`617d85...`下，decode重复structured/text导致计数翻倍，placeholder-only未满足SKILL的unresolved承诺，缺program/limit0未提前阻止。root已修canonical payload、具体selector/limit/callercontext、singlefunction标准化和placeholder-only判断。helper现限定持久Ghidra/ReVa；其他backend走operator，避免在轻helper里实现通用schema解释器。

旧handoff只看runtime+verification fact kind/version，允许verified与gaps矛盾。现在拒该矛盾，并明确artifact hashes是ledger声明/格式检查，不是binary-file重新哈希。`semantic_acceptance=false`与当前Rust fingerprint不等于验证接受的边界保持正确。

## 仍应保留的边界

- 本轮是假operator/合成ledger，不是各provider重新验收。真实工具入口另有本审计的sharedGhidra metadata实连证据；pet业务未测试。
- Ghidra bundle的placeholder/缺entry检测不能泛化为所有ReVa页/HighPcode已完整检查；有界分页不是全函数，尚需原ASM、真实ABI及scope人工判断。
- 身份层次如实表达：localfilehash、返回metadata字段、数据库importhash与原RNE是不同层；`ok=true`不代表原行为验收。
- ledger验证引用/路径/状态最低条件，不能证实文件内容、当前Rust与verification报告的源码依赖绑定、真实调度或业务闭环。声明artifactSHA不自动证明文件存在且匹配。
- 未支持payload形态/复杂优化需要保raw+unknown，不能用类型或默认值填补；完整库实现排除仍遵循native-rust-reconstruction。

当前未发现必须阻止本次有限pet恢复包试用的剩余问题。后续只在实际出现新payload/边界时补定向case，避免为helper重跑全部工具或业务证明。
