# 功能进度与难度

候选已实现多条可运行功能链，原作全功能等价性尚未验收。验收仅限证据明确覆盖的行为。

以可观察功能、原二进制对照范围和实际GPU验收作为进度依据。文件数量不计入完成度。

| 子系统 | 功能验收范围 | 证据状态 / 输入 | 剩余缺口 | 难度 |
|---|---|---|---|---|
| 设备筛选与初始化 | 原Ron白名单、驱动/设备兼容判断、surface筛选与设备创建 | [passed_known_policy_and_candidate_selection](evidence/device-policy-functional-validation.json) / 原报告输入已变化 | 原完整排序、未知驱动编码及implicit layer来源待核实 | 高 |
| 基础资源与设备能力 | 真实buffer/image/view/sampler创建、映射、读回和释放；BDA与时间线提交 | [passed_live_resource_smoke](evidence/raw-resource-implementation.json) / 记录输入一致 | ray tracing完整路径未验收；硬件只有Intel UHD630 | 高 |
| 分配器私有包装 | 分配/释放、峰值、失败采样与预算回滚；实际GPU资源回收 | [private_wrapper_and_direct_adapter_partial_validated](evidence/allocator-functional-validation.json) / 记录输入一致 | gpu-allocator0.26原本地分叉来源未确认，公共池为外部缺口 | 高 |
| shader与descriptor生产 | 404原文件/638反射项布局、descriptor复用、管线缓存；原填充shader与raster读回 | [shader_production_partial_validated](evidence/shader-production-validation.json) / 记录输入一致 | 原资产/pass关联、淘汰及完整材质选择未知；638管线未全量运行 | 极高 |
| Ref与managed绑定 | Ref访问声明、主图注册表解析、generation/device校验、descriptor持有与GPU消费 | [verified_partial_boundary](evidence/pass-bindings-functional-validation.json) / 未记录统一输入 | 当前provider桥分两阶段；自动接入主图首个shader callback、原完整ABI与游戏producer待完成 | 高 |
| 附件与渲染命令 | 附件生成view/renderpass/framebuffer缓存；begin/clear/end和像素读回 | [verified_partial_boundary](evidence/attachment-production-validation.json) / 原报告输入已变化 | 多子通道、input/resolve及全量pass组合未验收 | 高 |
| 渲染图与队列同步 | 整图生成提交数组；主/辅助队列候选合同；GPU外部wait/signal与两批次读回 | [passed_known_queue_graph_scope](evidence/queue-graph-functional-validation.json) / 记录输入一致 | 原并行调度与跨族ownership transfer未恢复；辅助队列无本机实测 | 极高 |
| 持久渲染session | 连续3帧复用chain/pool/timeline，resize、命令缓冲复用、失败停止与回收 | [passed_candidate_persistent_serial_session](evidence/persistent-session-validation.json) / 原报告输入已变化 | 串行候选；原并行调度与完整失败展开未知 | 高 |
| temporal跨帧 | 原状态机、重复请求、命中规则、pair/export；6帧GPU双缓冲与resize | [passed_known_temporal_contract_scope](evidence/temporal-functional-validation.json) / 原报告输入已变化 | 原未使用key淘汰/reset触发、完整ABI与游戏consumer未验收 | 高 |
| managed生命周期 | generation、Arc、CPU退役、序号到时间线及多队列回收 | [candidate_implemented](evidence/managed-implementation.json) / 原报告输入已变化 | 原全域producer/序号桶与性能未整体对照；raw析构保守等待 | 高 |
| 窗口交换链 | 真实acquire→GPU submit→present；3帧、resize、vsync重建与Win32窗口释放 | [candidate_gpu_verified](evidence/swapchain-functional-validation.json) / 原报告输入已变化 | 原winit分叉、焦点/全屏/完整WSI策略未恢复 | 高 |
| GPU诊断 | GPU marker、assertion/debug packet读回、错误门控；原局部入口对照 | [verified_partial_boundary](evidence/crash-functional-validation.json) / 原报告输入已变化 | 原shader诊断producer、vendor dump和实际device-lost未验收 | 高 |
| 格式映射 | 3个原入口816案例对照；另提供官方249格式通用查询 | [original_format_entries_and_public_registry_validated](evidence/format-functional-validation.json) / 记录输入一致 | 原私有与Khronos语义分开，不以公共表代替原入口等价证据 | 中 |

整体复验状态：failed_or_inputs_changed。见[统一验收记录](evidence/functional-project-validation.json)。

原55个PDB路径只作为结构清单，见evidence/original-structure.json；新增候选接口、字段编码和性能策略需独立对照。
