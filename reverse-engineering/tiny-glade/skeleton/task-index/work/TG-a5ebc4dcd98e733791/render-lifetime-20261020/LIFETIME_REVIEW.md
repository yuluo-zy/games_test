# 描述符寿命核查与候选交付

范围：`fa53d49a61dd7496cddc9af054c6fb1899fa1676` 当前重构后的 rhapsody 与生产 ShaderHost/FrameGlobals/RasterHost/ProductionRenderer。此文只报告本次源代码核查；没有运行 Cargo、GPU、原版程序，不将旧窗口日志当作新基线实测。

## 本次结论

尚未找到正常帧命令完成后租约无界强持有的源代码证据；也没有新复现可证明旧窗口的 240–420 总帧 StorageBuffer 耗尽已消失。不能据此宣称寿命无缺陷。旧 30 帧 headless 结果早于附件缓存和动态常量封存修复；窗口还经过真实 WSI、egui 与不同应用会话，必须重新测量。

已确认且给出源码候选的问题：`Slots::allocate` 满表错误把四类描述符合计 `live/retiring` 与失败单类 `capacity` 放在一起。该消息不足以区分 StorageBuffer 活租约增长和退役积压。候选只修此项工程诊断，并提供每类 O(1) 统计，保持容量、描述符分配、提交与完成回收策略不变。

## 实际 owner 图

1. `BindlessParameters::write` 在本次 builder 内按实际 buffer/range/上传角色或 image-view/layout 去重；`finish` 分配不可变参数 chunk，另取 StorageBuffer 描述符。`PreparedBindlessParameters` 仅在 pass/draw 回调局部持有租约。
2. `FrameGlobals` 每真实帧生成一次，`storage_lease` 和 `sampled_leases` 的 OnceLock 让整帧共享同一 FrameConstants 描述符和全局 sampled 纹理租约。sampler 租约由 ShaderHost 按名称共享。不能将这些计作每 draw 重建。
3. raster 每 draw 的 immutable 参数块和其余 local/pass resource 仍可新增描述符。compute 多次 push 使用 `PreparedBindlessParameters::with_push` 共享资源租约，但参数字节和参数描述符各自不可变。
4. `BindlessHeap::bind_pass/bind_command` 验证实际资源范围及角色，通过 `retain_recorded` 把租约 Arc 保留到实际 Vulkan 命令。lease 的独立 SyncCell、owner 的 SyncCell、heap set 的 SyncCell 全部登记到真实提交。heap 不强持有 lease，vkUpdateDescriptorSets 直接写原生表，未见 descriptor wrapper 隐式捕获 lease。
5. `VulkanSession::begin_frame` 在 pending 达设备 frames_in_flight 时等最旧会话帧；生产 renderer 的 collect 对当前和 inactive app 的 bridges 都做 collect_completed_frames。实际完成后 finish_frame observe 提交 receipt，ExecutionChain.recycle 回交命令，raw.release_completed_owners 对 TempList Vec 做 clear，真实 drop Arc。
6. 租约最后一个 Arc drop 后以独立 lease.last_use.snapshot 进入 Slots retiring；从未使用的租约 after.empty 立即回池。collect 检查所有退役项的实际多队列完成值，非队首阻塞；reserve 仅当目标类 free 耗尽时 lazy collect。低压下 retiring 可能积累而未收集，单看 retiring 绝对数不能认定泄漏。
7. 回调/提交前失败：ExecutionChain.abandon_unsubmitted 移除 CPU command snapshot，VulkanGraphBackend.abandon_command 把无提交命令标为 recycled。owner 保留至下一次该 frame slot 的整池 reset；reset 失败保留现场。此持有跨有限 frame slots，不是已证明的无界泄漏。部分已提交失败仍由 pending frame 保护，完成后同正常链释放。
8. AttachmentCache 以视图和纹理外部 Arc 计数、实际 completion 释放孤立视图，当前代码已含 ee331 修复。DynamicConstants 提交用过的页封存，完成后才从零复用，当前代码已含 a109 修复。未修改这两项。

## 候选 v1

- 文件：`candidate-v1/tiny-glade/crates/rhapsody/src/bindless.rs`
- 增量：`candidate-v1/bindless-kind-statistics.patch`
- 指纹和验证边界：`candidate-v1/SOURCE_STATE.json`
- 新 `BindlessTableStats`：capacity/live/retiring/free/exhausted/peak_live/peak_retiring。
- `BindlessStats::table(DescriptorKind)` 访问单类，旧 aggregate 字段保留，Debug 还输出四类 tables。
- 分配成功更新本类 live/peak；generation 溢出更新 exhausted；retire 更新本类 live；retiring/free 直接取已有队列长度，不添加镜像计数。collect 不扫描 live 表，不添加每帧 poll 或 wait。
- 测试 `table_statistics_distinguish_live_retiring_and_exhausted_kinds` 覆盖混类诊断、给定完成值后的 collect、unused 即回池、generation 溢出与每类容量守恒。测试由 root 运行。

## root 可立即验证

先运行新增及已有纯 Slots 单元测试（同 Cargo 构建 owner），再运行现有 `tests/bindless.rs` 中 `a_real_pending_shader_keeps_the_bindless_slot_until_actual_completion` 和 `graph_binding_rejects_undeclared_lease_bytes_and_accepts_the_complete_shader_range`。后者是当前 adapter 行为测试，不是原游戏整链 oracle。

GPU 首选生产窗口场景，必须记录每 ShaderHost/应用身份下的 StorageBuffer table live/retiring/free/peaks、session pending_frame_count、chain pending_command_count/recording_command_count/completed_command_count，与玩法 draw_list_lengths 和提交/present 计数同一时刻采样。现有 headless 入口 `original_gameplay_chain::shipped_session_steady_state_multi_frame` 已记录 heap stats、ring、pending 和内存，并在稳态对 live/peak 漂移断言；加入此候选后 Debug 自动获得 tables，host 无需改动。

判别：同 scene/draw 数且会话 pending 有界但本类 live 持续上升，查未释放 lease owner；live 平而 retiring 上升直到 reserve lazy collect 后清空，是延迟收集而不是活租约泄漏；pending 与 retiring 同涨，追真实 timeline/提交；live 随 draw 增长，核单帧需求与资源去重。不得仅增容量、周期清表、每帧 wait_idle 或新增读回来掩盖。

## 原版证据与边界

- B 原件身份由本 claim SOURCE_INPUTS 固定：EXE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03，PDB SHA256 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。
- 已读 `skeleton/B/crates/rhapsody/src/raw/binding.rs`：该文件仅 PDB 元数据，不冒称原作者源码。
- retained 入口证据 `evidence/render-recovery/rhapsody-alignment/reports/raw-core-descriptors-shader.md` 和 `wave2-samplers-binding.md`，以及 PLAN M3.4/M3.7/M3.11：原 persistent descriptors create/get 0x140bfef70/0x140bff400，update 0x1416a2320；bind_descriptor_set 0x1416a5380/0x1416a5910/0x1416a6010；DynamicConstants allocate 0x140c02740。
- 未重新发 native 查询，不扩大原 retained 证据验收边界。当前 shader heap lowering 属于用户重构后的 adapter；单类统计不是原版算法恢复，也不能计作新增业务完成。
- 跨 draw 同 buffer/range/role 活租约 Weak 复用仅为后续可评估优化，暂不实施，避免先优化遮蔽真实耗尽根因。业务路径与复杂算法无简写或空实现。

独立 reviewer 补充原证范围：B raw/binding.rs PDB 映射 0x1416a5380 的 code_size=1409；retained `parallel-batch-06/bind-enumerated.asm.txt` 只有1111字节/243指令，不能称完整函数。此候选不依赖该片段推断原版整函数。该字节核对由 reviewer 执行，本 worker 未重做。
