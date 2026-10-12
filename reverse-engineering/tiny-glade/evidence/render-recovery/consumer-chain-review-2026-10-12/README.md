# 主图绑定消费链与当前功能验收复核

日期：2026-10-12。目标沿 PROJECT_MEMORY 的原作 SHA256。新目录 `reconstruction/rhapsody-rebuild` 由另一用户授权聊天维护，本轮只读，不改其源码或报告。

## 当前功能证据

以新目录 `MODULE_STATUS.md` 和 `evidence/functional-project-validation.json` 为功能进度依据。独立检查112项输入哈希无变化，25条验收命令的记录均成功，日志存在；日志对应64项测试和13个真实GPU探针。当前53个外部依赖的版本、source及checksum（git包无checksum时比较其source）逐项匹配原锁。本轮复用已有测试/GPU运行记录，没有重跑这些未变输入。见 `functional-evidence-review.json`。

已运行的功能覆盖资源、shader/descriptor生产、附件、队列同步、持续session、temporal、WSI和诊断等候选链。不能将这些范围扩展成原游戏完整帧、并行调度、所有硬件或所有资产/pass关联已验收。原55路径清单仅为结构资料。

## 主图 callback 的实际接口修正

新目录 `examples/pass_bindings_gpu.rs` 在独立counter上生成CommandState，再从已执行主图registry构建ManagedPassResources，进行第二阶段descriptor/GPU消费。新目录报告正确标记 `two_phase_callback_bridge=true`；真实ExecutionChain正在录制的命令状态尚未注入。

旧共享入口现在提供 `ExecutionChain::execute_graph_with_context`，callback收到 `PassContext { pass_index, command, compiled, resources }`。command来自本次counter/begin；compiled和资源映射来自本次compile，callback发生在真正record期间。原仅接收句柄的入口保留为兼容包装。绑定consumer可以由context.command的state/sequence/queue构造跟踪输入，无需第二个counter或人工序号。

共享链的4个场景和all-targets严格Clippy通过。新绑定场景使用两批次，观察到pass0使用sequence2、pass1使用sequence4，且句柄分别对应实际queue提交数组；不是先生成独立CommandState再匹配结果。见 `context-entry-validation.json`。这只是旧入口的可调用接线合同；新目录自动callback消费、真实shader dispatch和managed submission反馈仍由其owner完成，本轮不记成已接通。

接线时应保留以下合同：

1. compiled.registry先于首个callback可用；物理对象由本次后端provider解析。generation来自实际managed分配，不能按token或句柄推算。
2. callback中的tag/sequence/queue来自当前recording Command；queue-domain与pool-family不同，不能以family替代counter域。
3. 成功提交反馈每条command实际timeline阈值；失败时保留已注册pending/state3，不产生成功receipt。资源跟踪不能因callback或提交失败而回滚。
4. 描述符拥有者覆盖真正使用的command；最后CPU引用退役与GPU完成回收分开。原全局u64 bucket与候选domain map不能混记为同一原作ABI。

## 批量原作 producer/consumer 取证

原分析库通过已有只读SDK runner实际连接，批量导出AssetShaderLibrary四个add/get入口和rhapsody raster创建入口，共5个新目标；连接报告、伪代码和关闭结果保存在本目录。`pipeline-mcp-report.json`记录7个操作成功、close(save:false)成功，未运行原游戏。compute创建和descriptor consumers复用既有证据，不重复查询。

PDB确认上游真实生产者为country_core::asset_libraries::shader_library::AssetShaderLibrary的add_compute/get_compute_by_name与add_raster/get_raster_handle_by_name。getter消费两个不同名字索引（compute区域+0xb8，raster区域+0x80）；compute的命中继续调用+0x40区域的FrozenMap::get（PDB地址0x140969a20），add_compute进入FrozenMap::insert（0x14096a1c0）。标准map、锁和FrozenMap只作为外部容器边界，不重写其展开代码。

raster创建0x1416a3720消费CompiledShaderPipeline的+0x40反射与+0x148 push范围，以stage flags0x1f创建layout；既有compute创建0x1416a4b20对应COMPUTE flags0x20。可见原管线生产由compiled反射驱动，不能仅把手动ShaderModule+PipelineLayout创建成功当成原资产选择已恢复。新ShaderLibrary已具有反射驱动候选生产；原名字注册、资产对应、热更新/缓存失效和完整选择策略仍需与这些原上游连接。

观察到的是调用关系与字段约束。Ghidra部分getter返回值和参数类型仍是未恢复占位；不从此生成确定Rust Option/Result签名，不将static解读写成原侧运行。没有完整xref记录不作无caller证据。

## 分叉依赖边界

原锁将gpu-allocator0.26.0、vk-sync-fork0.4.0、shader-pipeline0.1.0、shader-compiler0.1.0记录为无source的local/path包，不能因此宣称与同版本公开发行版完全一致。新目录当前直接分配适配和私有包装可运行，但原分配器池/分叉仍是外部缺口；AccessTable外供仍应保留，不猜原vk-sync枚举。Bevy分叉仓库/commit沿原项目约束，未核实。

后续工作以“主图registry→当前Command→managed绑定→descriptor→首个shader callback→同一提交/回收”整条功能链交付。缓存淘汰、跨族ownership transfer、原并行调度及游戏pass关联单独保留；不按新增函数、路径数或案例数扩大原作完成范围。

## 历史报告有效性

更新总进度时发现第五批统一门禁中仅 `scripts/render-recovery/mcp-evidence-job.mjs` 的输入hash已改变。旧原码报告与结果原样保留，第五批门禁标记为历史工具输入变化，不再列为当前输入一致；没有覆盖旧hash来让检查通过。第四批记录输入仍一致。此变动不影响新目录112项输入的统一功能验收；工具runner的后续新增batch支持另由其维护方验证，不能计作原作功能完成。
