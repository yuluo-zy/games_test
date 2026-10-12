# rhapsody 逆向进度与下一步路线

更新：2026-10-12。目标构建：Tiny Glade，二进制 SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。

当前功能工程为另一用户授权聊天维护的 [rhapsody-rebuild](../../reconstruction/rhapsody-rebuild/MODULE_STATUS.md)，以 [统一功能验收](../../reconstruction/rhapsody-rebuild/evidence/functional-project-validation.json) 为当前功能进度：64测试、严格Clippy、13个真实GPU探针，25条命令无失败。主流程只读核对112记录输入无变化、53外部依赖身份匹配原锁，未重跑这些未变输入。覆盖范围与剩余难度直接以该模块状态表为准；不能从文件数推算原作完成率。

旧共享入口已运行graph→compiled→record→counter→queue→completion，并新增 `execute_graph_with_context`，callback直接获得当前Command和图映射；4个链级场景及Clippy通过，两个批次验证sequence2/4与提交句柄关联。这为新工程自动绑定提供接口合同，尚未迁入其主图callback。见 [本轮消费链复核](consumer-chain-review-2026-10-12/README.md)。旧批次门禁是历史限定范围；旧CPU测试和新GPU验收不混计。

整体完成百分比暂不报告：原作完整实现与验收范围尚未建立可靠分母。638接口反射只是库存覆盖；M12保持 `verified_partial`。已有真实GPU候选功能验收，完整原游戏帧/画面、并行/跨族、缓存/资产关联及分叉依赖仍待原作对照。

以下表格描述旧共享工程的源码证据和缺口，不覆盖新目录已完成的候选功能实现。新目录GPU功能与原作等价性分列，仍缺首个主图shader callback自动绑定/真实CommandState注入、原并行调度、ownership transfer、缓存淘汰和游戏producer。

## 按模块看进度

| 部分 | 当前实现/接线 | 未闭合部分 | 剩余难度（工程判断） |
|---|---|---|---|
| PDB 项目结构 | 55个原文件路径可Cargo检查 | 路径骨架不代表源码恢复 | 低；填充实现另计 |
| 资源图与编译器 | ResourceInfo/Registry/compiled已迁入共享入口，record字段由其输出生成 | 原游戏graph生产者、完整compile ABI、alias生产者与原整链验证 | 高 |
| GPU 生命周期 | 原counter从1产生sequence，共享record/queue保留同值；receipt→pending→completion/回收已接通候选 | 真实ControlBlock/generation生产者、多队列可达性与碰撞、完整异常/析构 | 高 |
| barrier → record → submit | 非空buffer/image的initial/pre/export、track、callback、end、实际submit数组生产已接通 | waits/signals/prior/flush生产者、并行录制、真实锁与end-error/unwind | 高 |
| temporal 跨帧资源 | import/export、resize/parity规则；graph/temporal take/restore和frame reset叶取证 | 共享入口跨帧、unused-key清理、整体drop与失败所有权 | 高 |
| shader 接口与创建 | 404/638接口库存；layout/pool/compute layout及sparse write/update/bind消费接口 | 真实shader/interface选择生产者、共享callback、shader module/完整pipeline与游戏pass关联 | 很高 |
| 游戏pass/画面 | 尚未完成consumer接线及最终验收 | country-core最小pass、GPU执行、画面与性能对照 | 很高 |

共享源码见 `reconstruction/render/{graph_info,graph,execution}.rs`，证据见 [shared-entry-validation.json](parallel-batch-06/shared-entry-validation.json)。候选测试后端显式提供设备API和managed物理对象；这部分private生产者仍是缺口。提交错误保留pending与已分配timeline，无成功receipt，不允许假completion。Clippy/编译及入口场景通过不证明原完整链等价。

难度按剩余生产者/类型/所有权/外部关联估计，不转换为完成百分比或工期。旧2–4周/6–12周估算撤回；尚缺两轮可比较的接线吞吐与原侧链证据，不能准确回答完全逆向还需多久。

## 已交付批次

| 批次 | 主要产物 | 验证状态 |
|---|---|---|
| 01 | ResourceInfo、Registry/Pool、CompiledGraph/Batch | 512 个 ResourceInfo 快照、1,600 个 BatchCompiler 原入口案例；跨包门禁 11/11 |
| 02 | 小图 DSL → compiled snapshot、executor/temporal 候选、tracked 生命周期 | 29,692 个生命周期原指令案例；候选 27 项测试；跨包门禁 11/11 |
| 03 | barrier/record 分段、population、sequence/timeline 分域、temporal 跨帧主要路径 | 1,344 个 barrier、97 个 population 案例，6 个 snapshot 检查；候选 13 项测试；跨包门禁 13/13 |
| 04 | device counters、record→submit 扩展、全量 shader 接口目录 | 50/50统一门禁；本轮重跑2840+108原码案例、15候选及3跨包测试、全量反射检查 |
| 05 | Primary调用者、非空资源record/submit、descriptor layout/pool/compute layout | 27/27统一门禁；180调用者、774录制、5184创建边界案例零差异；5+2候选测试 |

批次 04 的具体交付：

- R6A：`aux_count+1` 个独立原子计数器全部从 1 开始；primary 使用槽 0，aux queue 按 ordinal 使用槽 n+1。CB sequence 不跨队列全局唯一，submission timeline 全局从 1 开始。原 CPU 指令片段差分 2,840 例零差异，6 项 Rust 测试与严格 Clippy 通过。完整 device 构造执行与多队列 caller 可达性未覆盖。
- R6B：108 个原指令/Rust 案例零差异，其中 28 个顺序 record、48 个 submit、20 个注入 queue 尾段、12 个 record→submit 衔接案例；3 项 Rust 测试通过。record 的资源列表为空，queue 仅验证后端调用尾段；非空资源、并行录制和完整 queue 前段仍待恢复。
- R6C：404 文件包含 170 compute 与 234 raster，后者各含 vertex/fragment，共 638 个 SPIR-V 模块；638 个全部反射成功、0 失败。统计得到 2,577 个 descriptor 变量、43 个 push-constant 变量；404 项 descriptor 合并与 234 项光栅阶段接口检查通过。4 项 Rust 测试通过，17 个依赖与原锁版本/来源/checksum 匹配；shader 到具体游戏 pass 的真实绑定仍待调用证据。

第四批实际输入480项hash和13条命令记录由主流程工具agent统一核验。R5B的fixture seed0与原counter seed1不是生产接线：整合通过只说明限定adapter合同一致。

第五批详细边界：queue包180个完整ensure_active_cb调用者入口案例，create/begin为fixture；record包774个新非空资源案例，其中726个非空pass access、48个无last-access export、330次原录制bytes→原submit，不重计旧108案例；pipeline包4438个layout完整入口、486个注入已验layout的pool完整入口、256个注入frame的compute layout片段与4个Vulkan错误入口停止，共5184例。638个shader声明只作为候选adapter输入，不证明原sparse binding或pass关联。新增独立Rust包可编译，完整GPU/画面未验收。

工具审计确认注册入口缺失、默认save:true旧脚本、xref不完整和跳表越界伪码等问题。新只读MCP runner提供save:false、会话互斥、失败隔离与输入hash缓存；没有provider manifest/数据库snapshot时默认关闭分析缓存。27个本地fake MCP测试通过，工具测试不计为原作代码恢复。进度生成器也校验第四/五批输入hash并保留渲染状态，防止更新时丢失模块进度。

## 下一步顺序

1. **shader到绑定consumer**：只围绕真实shader/interface选择producer取证，将已有sparse layout/write/update/bind消费接口接入共享record callback；不再反射全库存或新增孤立crate。
2. **完成真实执行输入**：恢复executor waits/signals/prior/flush和设备/managed资源生产者，追踪辅助队列与并行路径；原queue CPU入口证据不等于这些上游已接通。
3. **跨帧与原侧整链**：连接temporal/retire到下一帧及completion，按同一图语义对照原边界事件；针对差异调查字段和错误副作用。
4. **连接最小游戏 pass**：打通 `cull/indirect → shadow/main` 的country-core输入与资源选择，再开展GPU执行与画面对照。

恢复仍以“类型族、状态机、端到端路径”为单位。PDB/调用索引、shader 反射、接口归族与哈希检查由批量工具完成；模型集中处理语义缺口与跨模块所有权。

## 证据入口

- [总计划](BATCHED_RHAPSODY_RECOVERY_PLAN.md)
- [当前执行规则](WORKFLOW_RESET.md)与[worker指导](AGENT_EXECUTION_GUIDE.md)（覆盖总计划旧函数包分派）
- [本轮共享入口](parallel-batch-06/README.md)与[候选链验证](parallel-batch-06/shared-entry-validation.json)
- [第三批统一门禁](parallel-batch-03/integration.json)
- [第四批合同](parallel-batch-04/contract.json)
- [第四批统一门禁](parallel-batch-04/integration.json)
- [第五批交付](parallel-batch-05/README.md)与[统一门禁](parallel-batch-05/integration.json)
- [工具审计](TOOLING_AUDIT_2026-10-12.md)与[新runner说明](tooling/README.md)
- [R6A 验证](parallel-batch-04/device-counters/validation.json)
- [R6B 验证](parallel-batch-04/record-submit/validation.json)
- [R6C 验证](parallel-batch-04/pipeline-shader-interfaces/validation.json)
