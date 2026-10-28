# 新工具与小宠物工作流试跑结果

已完成工具审计、三个新技能、原重建技能更新，以及 C v1.16.0 羊抚摸业务切片试跑。三个 worker 分别负责工具/独立审计、批量原指令证据包、Rust 恢复与运行链；父 agent 集成、纠正依赖漂移、串起一键入口并复查源绑定。完整 critters 与建筑系统仍未验收。

## 工具选择

Ghidra shared 真实连接成功，14 项只读工具可列。持久查询适合已知函数、受限引用与类型合同；ReVa 留给定点分页、必要 High P-code 或隔离签名修复。REA、RE-MCP、radare2 的新复验能力有用，但本切片不需要再为相同问题冷启多个 provider。当前模型 namespace 没有直接暴露这些工具，真实调用通过现有 operator SDK 完成。

B patched 数据库提供分析上下文；C 原发行文件提供本轮原指令。metadata 没有数据库 SHA，路径加本地文件 hash 的绑定没有升级为全数据库导入身份。B pet 反编译漏五个 tag 分支，已由 C 原跳转表/ASM 裁决，保留这一实际工具边界。详情：[工具审计](tool-audit.md)。

## 技能与复用脚本

- [native-evidence-pack](/Users/yuluo/.codex/skills/native-evidence-pack/SKILL.md)：PE/PDB 身份绑定、批量 source/inline 点、定界 ASM、每指令原字节核对、不可变证据缓存。8 项机械测试及真实 B/C 冷热采集通过。
- [native-tool-batch](/Users/yuluo/.codex/skills/native-tool-batch/SKILL.md)：持久 Ghidra/ReVa 精选查询合批、选择器/数量/输出限界、双通道去重、嵌套错误与占位函数拒绝、完整原始结果留盘。
- [native-module-handoff](/Users/yuluo/.codex/skills/native-module-handoff/SKILL.md)：原模块/版本/依赖/消费者台账、源 hash、未完成边界、矛盾 verified 标记拒绝。引用有效不等于语义自动通过。
- [native-rust-reconstruction](/Users/yuluo/.codex/skills/native-rust-reconstruction/SKILL.md) 已接入前三个技能，并新增公共 Cargo 锁文件对照脚本。技能保通用方法，实际地址与字段只留项目证据。

技能结构校验通过，脚本有成功/失败检查，批量查询与交接技能经历独立 forward-test。首次 review 发现并修复：结构化/text 双通道重复、空 RETURN 占位未拒绝、查询不限界、verified 同时声明 gaps。完整技能源码与 hash 保存在 [skills-snapshot](skills-snapshot/manifest.json)。

## 实际成本

| 环节 | 本轮实测 |
| --- | --- |
| 最终一键采集与交接 | 4.38 秒，5 阶段；不含写代码、编译或指令差分 |
| 持久 MCP | 1 次 operator 进程，2 次 MCP 分析步骤，5 个函数；底层仍有各函数反编译工作 |
| C 原证据包冷请求 | 0.562 秒；3 个分析进程 + 2 个版本探测进程；4 个 source 点 |
| 相同 C 包热请求 | 0.347 秒；2 个缓存命中，0 个分析进程 + 2 个版本探测进程 |
| 完整 C 包 | 367798 字节落盘；两个原范围的定界 ASM 与 JSON，没有重复灌入上下文 |

这是有界请求的实测，不是整场逆向的提速倍数；未测精确 token 节约。哈希、版本探测和报告输出仍有成本。辅助 fixture、初次试用与最终重跑分别留档，不将其隐藏成只调用一次。机器可读结果：[pilot-summary.json](pilot-summary.json)、[最终流水账](final-workflow/receipt.json)。

## Rust 交付与证据边界

新增 [sheep_pet.rs](../../reconstruction/critters/sheep_pet.rs)、[sheep_pet_runtime.rs](../../reconstruction/critters/sheep_pet_runtime.rs) 和独立 C harness。恢复 Message membership 输入 → 五阶段状态写入 → 振幅/原 being_petted 行走门控谓词 → finish 清理；Bevy 0.17.3 App/World/Query/Message/Commands 真运行，源移除与实体删除撤回派生输出。104 字节是观察到的源投影，未编造 Sheep 构造器或 ID。`.chain()` 是 adapter 顺序，尚非原整体调度。

120 个原 C 选择行状态机/完整 getter 原指令案例全匹配，比较完整 104 字节、显式变化 tick 决策与谓词；事件 membership 作为已解析输入，不等同执行原整个 ECS/UI。4 项独立检查通过（3 个 World、1 个纯规则），包括重复消息、严格边界、原振幅超出范围保留、NaN、真实 Bevy 源变化 tick、结束/删除及稳定 NaN 位发布。[差分报告](critters-C-pet-verification.json)、[运行检查](critters-local-world-results.md)、[独立阶段 review](tool-audit-pet-stage-review.md)。

发现候选锁 114 个公共包仅 40 个匹配；以原 C 锁副本重新解析后，最终 113/113 个公共包版本/source/checksum 匹配。原锁与初次候选/报告均保留，未重写发行输入。锁匹配不接受原全部 features/Bevy fork 行为。[最终依赖报告](dependency-lock-final.json)。旧 B firefly NaN 差分失败仍保留，没有借本切片抹除。

仍缺：真实 UI 拾取/widget 到消息生产、Sheep 创建/ID、GameTime 生产者、完整行走/动画/渲染消费者、源 +60 的后续意义、原整体调度、移动/terrain/inside-building 与回放。下一步优先恢复原 UI 生产者与权威 Sheep 源，关闭显式观察输入边界。[交接台账](critters-handoff.json)。

## 单命令复用

从本项目 tiny-glade 目录执行（输出必须是新目录）：

```sh
python3 scripts/native-workflow/run_pet_pilot.py evidence/toolchain-pet-pilot-2026-10-11/pet-pilot-recipe.json --output evidence/toolchain-pet-pilot-2026-10-11/replay-new
```

这会一次完成 MCP 批量、原证据冷/热采集、公共依赖对照、交接审计；`--skip-mcp` 仅用于无需重新查询可变数据库时。业务检查复用 [既有执行命令](critters-local-world-results.md)，与工具数据采集分别计时。修改目标/范围时复制并更新项目 recipe、plan、manifest、ledger，身份错误必须失败，不自动换版本。两个版本的地址不能套用。项目内附测试脚本 `scripts/native-workflow/test_helpers.py --output <项目内新目录>`。

源代码、技能快照和试跑证据已保存到 检查点。归档排除原二进制/PDB、target 和缓存，恢复说明单独保留。
