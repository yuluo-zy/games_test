# 功能进度与难度

候选已实现多条可运行功能链，原作全功能等价性尚未验收。验收仅限证据明确覆盖的行为。

2026-10-13 shader 构建续轮已接通 include/pragma once→宏/#line→实际 DXC→独立反射→后处理/Speedy 打包，404 新包/638 阶段全部生成（393 包官方 1.9.2609，11 包官方 preview 1.10.2605.37）；最终 10 项受影响测试、严格 all-targets Clippy、新 fill 在 Intel UHD 630 上 64 word 读回、新 egui 两变体布局验证通过。当前范围见 [构建及来源交付](../../evidence/shader-build-2026-10-13/SUMMARY.md) 和 [输入哈希/验证](../../evidence/shader-build-2026-10-13/integration.json)。新编译包中 210/404 完整反射、302/638 阶段公共接口与原一致，191 包依赖集合有差异。170 compute 原宏 tuple 未独证，其中 38 非零 key 明确未知。可运行构建链已交付；完整原宏 manifest、shader 算法、原 DXC/fork、provider/异常/cache、资产库状态机、raster draw 与整帧仍未验收。前轮 schema/请求及旧 shader-production 报告保留历史输入与范围。

以可观察功能、原二进制对照范围和实际GPU验收作为进度依据。文件数量不计入完成度。

| 子系统 | 功能验收范围 | 证据状态 / 输入 | 剩余缺口 | 难度 |
|---|---|---|---|---|
| 设备筛选与初始化 | 原Ron白名单、驱动/设备兼容判断、surface筛选与设备创建 | [passed_known_policy_and_candidate_selection](evidence/device-policy-functional-validation.json) / 原报告输入已变化；候选本轮统一复验通过 | 原完整排序、未知驱动编码及implicit layer来源待核实 | 高 |
| 基础资源与设备能力 | 真实buffer/image/view/sampler创建、映射、读回和释放；BDA与时间线提交 | [passed_live_resource_smoke](evidence/raw-resource-implementation.json) / 记录输入一致；候选本轮统一复验通过 | ray tracing完整路径未验收；硬件只有Intel UHD630 | 高 |
| 分配器私有包装 | 分配/释放、峰值、失败采样与预算回滚；实际GPU资源回收 | [private_wrapper_and_direct_adapter_partial_validated](evidence/allocator-functional-validation.json) / 记录输入一致；候选本轮统一复验通过 | gpu-allocator0.26原本地分叉来源未确认，公共池为外部缺口 | 高 |
| shader与descriptor生产 | 404原文件/638反射项布局、descriptor复用、管线缓存；原填充shader与raster读回 | [shader_production_partial_validated](evidence/shader-production-validation.json) / 记录输入一致；候选本轮统一复验通过 | 原资产/pass关联、淘汰及完整材质选择未知；638管线未全量运行 | 极高 |
| Ref与managed绑定 | Ref访问声明、主图注册表解析、generation/device校验、descriptor持有与GPU消费 | [verified_partial_boundary](evidence/pass-bindings-functional-validation.json) / 未记录统一输入；候选本轮统一复验通过 | 当前provider桥分两阶段；自动接入主图首个shader callback、原完整ABI与游戏producer待完成 | 高 |
| 附件与渲染命令 | 附件生成view/renderpass/framebuffer缓存；begin/clear/end和像素读回 | [verified_partial_boundary](evidence/attachment-production-validation.json) / 原报告输入已变化；候选本轮统一复验通过 | 多子通道、input/resolve及全量pass组合未验收 | 高 |
| 渲染图与队列同步 | 整图生成提交数组；主/辅助队列候选合同；GPU外部wait/signal与两批次读回 | [passed_known_queue_graph_scope](evidence/queue-graph-functional-validation.json) / 记录输入一致；候选本轮统一复验通过 | 原并行调度与跨族ownership transfer未恢复；辅助队列无本机实测 | 极高 |
| 持久渲染session | 连续3帧复用chain/pool/timeline，resize、命令缓冲复用、失败停止与回收 | [passed_candidate_persistent_serial_session](evidence/persistent-session-validation.json) / 原报告输入已变化；候选本轮统一复验通过 | 串行候选；原并行调度与完整失败展开未知 | 高 |
| temporal跨帧 | 原状态机、重复请求、命中规则、pair/export；6帧GPU双缓冲与resize | [passed_known_temporal_contract_scope](evidence/temporal-functional-validation.json) / 原报告输入已变化；候选本轮统一复验通过 | 原未使用key淘汰/reset触发、完整ABI与游戏consumer未验收 | 高 |
| managed生命周期 | generation、Arc、CPU退役、序号到时间线及多队列回收 | [candidate_implemented](evidence/managed-implementation.json) / 原报告输入已变化；候选本轮统一复验通过 | 原全域producer/序号桶与性能未整体对照；raw析构保守等待 | 高 |
| 窗口交换链 | 真实acquire→GPU submit→present；3帧、resize、vsync重建与Win32窗口释放 | [candidate_gpu_verified](evidence/swapchain-functional-validation.json) / 原报告输入已变化；候选本轮统一复验通过 | 原winit分叉、焦点/全屏/完整WSI策略未恢复 | 高 |
| GPU诊断 | GPU marker、assertion/debug packet读回、错误门控；原局部入口对照 | [verified_partial_boundary](evidence/crash-functional-validation.json) / 原报告输入已变化；候选本轮统一复验通过 | 原shader诊断producer、vendor dump和实际device-lost未验收 | 高 |
| 格式映射 | 3个原入口816案例对照；另提供官方249格式通用查询 | [original_format_entries_and_public_registry_validated](evidence/format-functional-validation.json) / 记录输入一致；候选本轮统一复验通过 | 原私有与Khronos语义分开，不以公共表代替原入口等价证据 | 中 |

整体复验状态：passed_current_candidate_functional_scope。见[统一验收记录](evidence/functional-project-validation.json)。

原55个PDB路径只作为结构清单，见evidence/original-structure.json；新增候选接口、字段编码和性能策略需独立对照。

## 2026-10-13 显存子分配

| 项 | 状态 | 说明 |
|---|---|---|
| 子分配（gpu-allocator 0.27.0） | **候选，默认关闭** | 原作用本地分叉 0.26.0（ash 0.38 + arc-swap/egui/puffin）；公开 0.26.0 依赖 ash ≤0.37 不兼容，0.27.0 是最近且兼容的公开版；块大小策略与统计口径可能与分叉不同 |
| `AllocationBackend` 接入 | 已实现 | `SubAllocationBackend` 与 `DirectMemoryBackend` 同构，可放进 `GpuAllocator<B>` 峰值包装 |
| 统计口径 | 保持每分配一条记录 | 登记表仍是逻辑分配；真实块数另用 `memory_block_count()`；登记的内存类型/堆序号由属性反查（gpu-allocator 不暴露），是近似 |
| 非一致性内存刷新/失效 | **未验证** | 范围按分配自己的区间并对齐到 `nonCoherentAtomSize`；对齐前提有测试，但 lavapipe 只有一种一致性类型，实际刷新路径从未执行 |
| 屏障计数 | 已实现 | `ExecutionStats.barrier_commands/transitions`，串行并行一致 |

## 2026-10-13 并行录制

| 项 | 状态 | 说明 |
|---|---|---|
| `record_batch` 并行录制 | **还原原作**（PDB + PE） | 分块预算、贪心切分、双缓冲 PGO 直读自机器码；未知：命令池来源、`device+0xabb` 开关含义 |
| 每块独立命令池 | 候选 | Vulkan 要求命令池外部同步；原作的池来源未知 |
| `ADAPTIVE` 轻帧门槛 | 候选扩展 | 原作无；实测临界点约每 pass 2µs |
| `Balanced` 分块 | 候选扩展 | 解决"重 pass 在末尾则整批只有 1 块"与第一帧每 pass 一块 |
| managed 控制块无锁 | 候选 | 原作控制块即原子字段；去掉了我们自己加的互斥锁 |
| managed 槽位表 `AppendOnlyVec` + 显存预算 `ArcSwap` 快照 | 候选（依赖取自原锁，用法有 PDB 证据；槽位内部布局与空闲链表未恢复） | 见 DEPENDENCY_AUDIT.md |
| 管线创建并行 | 候选 | 依据：`VkPipelineCache` 未带 EXTERNALLY_SYNCHRONIZED 标志时内部同步（规范）；无 validation layer 复核 |
| 块边界不切断 render pass | **未强制** | 当前每图 pass 自成 render pass 实例；共享后必须填 `split_allowed` |

GPU 验证仍限于 lavapipe；并发同步正确性在真显卡与 validation layer 下**未验收**。

## 2026-10-13 缺口收敛与性能优化

| 项 | 状态 | 说明 |
|---|---|---|
| `BufferDesc` 16B | 已对齐 | `{size,usage,alignment,memory_location}`，编译期断言尺寸；`tests/buffer_desc.rs` |
| raw 释放 | 候选优化（非原作镜像） | `raw/graveyard.rs`：Drop O(1)，按 epoch 销毁；原作为按序列号排出的销毁桶 |
| setup 命令缓冲/纹理上传 | 同步候选 | 保留结尾全量屏障；原作并入下一次图提交且不等待；`tests/setup_upload.rs` 逐字节往返 |
| swapchain `retire` | 有意偏差 | 等价于下次 acquire 按 `old_swapchain` 重建；原作“取回桶+重建”未镜像 |
| `set_fullscreen_exclusive` | 未实现 | 证据不足，保持 `Unsupported` |
| `queue_submit` 签名 | 已取证 | 4 个寄存器参数 = `(&device, Option<u32>)` + 返回槽；见 P4 证据 |
| 性能追踪 | 新增 | `cargo bench --bench perf`；`tests/perf_guards.rs`；见 [PERFORMANCE_ANALYSIS.md](PERFORMANCE_ANALYSIS.md) |

GPU 验证口径：本机以 lavapipe 重跑 17 个探针中的 14 个并通过；其余 3 个受环境限制。真显卡、多队列、Win32 呈现、validation layer 下的同步正确性**仍未验收**。

## 2026-10-13 缺失函数补全

对照 `skeleton/B/crates/rhapsody` 骨架函数清单（rg/raw/managed），按用户授权按清单顺序补全缺失函数约 200 个；新增候选接口已接入真实类型，不另开 candidate crate。交付证据见 [missing-functions-completion-2026-10-13.json](evidence/missing-functions-completion-2026-10-13.json)。

- **rg**：RenderPassApi draw/dispatch 9 命令 + 调用计数（此前判定最大功能缺口）；PGO 全套（PassPgoNameId/Duration/Stats）；ExportedRenderGraphResources/ExecutedRenderGraph；Handle/ExportableResource/create_resource_handle；ResourceInfo extract_* 拆出；Parity/Temporal；RenderGraphRecorder/PassResourceRef/ensure_active_cb；VulkanGraphSession wait_submits_done/帧时长/顶层时间戳。
- **raw**：GraphicsDevice 候选包装（资源创建/屏障下发/扩展支持/内置采样器）+ create_raw_device/Queue；RasterPipelineOpts 29 setter/getter+checked；Buffer 持久映射 mapped_slice_mut；SubmitInfo/SubmissionEngine；instance 黑名单系列；descriptor/barrier/binding/command_buffer 若干。
- **managed**：GpuOwned 全族、ManagedHandle/ManagedRef/ManagedResourceChecksum；ManagedResourceMeta/owning_device/throw_out_managed_resource/get_maybe_tracked；SwapchainInner 包装；纹理视图辅助。

验证：`cargo test --locked` 69 通过 0 失败（含此前因资产路径不可编译的 device_policy 5 个）；`cargo clippy --locked --all-targets -- -D warnings` 通过。本轮未重跑 GPU 探针（需 GPU 与 D:/ 原始资产）。

**剩余 typed gap（需按原码取证才能实现）**：setup 命令缓冲体系（get_setup_cb_submit_info/with_setup_cb）、full_screen_exclusive（SwapchainInner::set_fullscreen_exclusive）、纹理暂存上传（init_texture_data/update_texture）。上述新增候选接口与原完整 ABI/行为等价性仍独立验收。


## 2026-10-13 shader 原配置与最终打包复验

38 个非零 compute 原宏 tuple 已从 native producer/caller 恢复并核原 key；实际 DXC 生成 404 个新包／638 阶段，失败 0，完整 typed 反射 397/404 相等。原 19 个 uniform 冲突包的 38 阶段经独立 SpirQ 复验全等。21 相关测试、严格 Clippy、新包 compute GPU 读回通过。原 artifact 宏与 TGV 还原源码选择宏分离。剩余 7 个 typed 反射差异、132 个 zero-key 原 tuple、325 阶段公共完整接口差异、源码依赖组织及原 DXC／离线 CLI 策略仍待恢复；不声明全 shader 语义或整帧验收。当前证据：[shader gap summary](../../evidence/shader-gap-investigation-2026-10-13/SUMMARY.md)。此前 shader-build 数字是历史验收，未覆盖本轮修复。
