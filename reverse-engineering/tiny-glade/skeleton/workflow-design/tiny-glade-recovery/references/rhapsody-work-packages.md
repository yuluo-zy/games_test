# rhapsody 工作包模式

用户要求恢复 `rhapsody` 或 Tiny Glade 私有渲染框架时，先读：

- `evidence/render-recovery/BATCHED_RHAPSODY_RECOVERY_PLAN.md`
- `evidence/render-recovery/workgraph/summary.json`
- `evidence/render-recovery/workgraph/work-packages.json`

先运行确定性工作图生成器：

```powershell
& workspace\python\Scripts\python.exe scripts\render-recovery\build_rhapsody_workgraph.py
```

生成成功只证明函数、调用边、共享地址冲突和七个工作包被确定性索引，不等于恢复 ABI、类型或实现。

进入图编译器工作包时生成 R1 合同：

```powershell
& workspace\python\Scripts\python.exe scripts\render-recovery\build_graph_compiler_contract.py
```

只把合同和对应原始证据片段交给模型，不把完整 `calls.json` 放入上下文。

生成并验证三批并行合同：

```powershell
& workspace\python\Scripts\python.exe scripts\render-recovery\build_second_batch_contract.py
& workspace\python\Scripts\python.exe scripts\render-recovery\validate_parallel_graph_contract.py
& workspace\python\Scripts\python.exe scripts\render-recovery\validate_second_batch_contract.py
& workspace\python\Scripts\python.exe scripts\render-recovery\build_third_batch_contract.py
& workspace\python\Scripts\python.exe scripts\render-recovery\validate_third_batch_contract.py
```

第三批固定 R5A barrier/record、R5B command sequence/queue timeline、R5C temporal payload 三个接口。三个工作包必须写入互斥目录；生产模块由主流程在门禁通过后整合。

门禁只验证账本、状态机和候选报告的一致性，不把 CPU candidate 升级成 Vulkan/GPU 等价证明。输入变化后必须重建 `integration.json`。

## 执行边界

1. 以工作包为恢复单位，不为每个函数单独建立实现任务。
2. 先汇总共享类型字段和状态不变量，再读反编译结果。
3. AI 取证必须对应 `unknown` 或未解析边；确定性脚本能回答的问题不重复调用模型。
4. 泛型实例先按 semantic family 归组；只有行为或布局差异有证据时才拆分。
5. ICF/shared-address collision 保留所有名字，不自动选择 canonical 源码语义。
6. 公开依赖、std、ash、Bevy、allocator/sync 库直接复用。
7. 图编译器比较 compiled graph 快照；生命周期比较事件轨迹；Vulkan 包装比较公开 API 参数；纯叶规则才优先机器码逐例差分。

当前三批 CPU 合同已形成 `consistent_partial` 闭环。下一恢复单位是 device counter 构造、完整 pass recording/submit 边界，然后进入 F pipeline/binding 与 country-core 最小 pass 接线。

必须保留以下不变量：

- ordinary initial transition 处理 `reused=false`；无 predecessor 的 pre-transition 只处理 `reused=true && initial=0`。
- persistent `ResourcePool` 命中不等于 graph-local alias；已观察两个 marker=1 消费路径，但上游生产者仍未知。
- command-buffer sequence 与 queue timeline 是不同编号域。
- timeline 达标后 command buffer 可回池；sequence gap 只延后 resource bucket 的顺序释放。
- `ExecutedRenderGraph::retire` 是立即 CPU 所有权/归池转换；真实析构由 tracked handles 和 completion buckets 延迟。
- temporal raw texture=0、raw buffer=1、exported=2；同帧重复 lookup 报错，下一帧从 exported import。
- texture extent 变化重建；其余 texture descriptor mismatch panic；buffer reuse 不比较 descriptor。
- temporal parity 按 key 保存，pair 在两次 single lookup 成功后翻转；`export_temporal` 不翻转 parity。

当同一机械步骤重复两次时，优先加入 `scripts/render-recovery/` 并让本参考指向该入口；不要把大型生成清单复制进 `SKILL.md`。
