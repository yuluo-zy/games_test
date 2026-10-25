# 旋转三角形多帧动画：经由 rhapsody 渲染图执行器

新增 `crates/rhapsody/tests/triangle_animation_graph.rs`（2 个测试）；`tests/draw_support/mod.rs` 只追加了 3 个公共函数
（`rotated_triangle` / `frame_clear_color` / `check_animation_frame`），既有 `triangle_draw.rs`、`triangle_animation.rs` 未改。
**`crates/rhapsody/src/**` 零改动**：执行器路径一次跑通，没有发现需要修的 bug（下面"发现但没修"列的是接口层面的坑）。
全程只走 rhapsody 自己的 rg 接口，没有接 `tiny-glade/render_host.rs`。

## 1. 调用链（每帧一张 RenderGraph）

| 步骤 | 接口 | 文件 |
|---|---|---|
| 设备/会话 | `VulkanGraphBackend::new(device, AccessTable::original())` → `VulkanGraphSession::new` → `attach_graphics_device(gd)`（挂提交引擎镜像） | `rg/vulkan.rs` |
| 节流 | `session.begin_frame()`：在飞帧 >1 时按序 `wait_frame` 排到只剩 1；账本清零 | `rg/vulkan.rs`、`rg/frame_throttle.rs` |
| 历史纹理（temporal） | `TemporalGpuResources::register("hist:0/1")` → 每帧 `TemporalRenderGraphState::get_or_create_owned_pair` → `import_into(backend,…)` → pass 后 `export_temporal`（访问码 0x37） | `rg/temporal.rs`、`rg/vulkan.rs` |
| 图内创建的颜色目标 | `backend.declare(&mut graph, "color", ResourceSpec::Image(..))`（`OriginSource::Create`，由执行器的 `Pool` 分配/复用；usage 由访问码推导，不手填） | `rg/vulkan.rs`、`rg/resource_usage.rs` |
| 外部导入 | 顶点缓冲（每帧新建）、读回缓冲：`backend.bind_import(GpuResource::Buffer)` 得 token → `graph.import_unowned(name, descriptor, token, initial_access)`；读回缓冲 `graph.export(h, 0x26)` | `rg/import_export.rs` |
| pass 1 `triangle` | `add_resource_access(vb, 0x04)`、`(color, 0x2e ColorAttachmentWrite)`；`render_with_context`：`ctx.managed.texture_for/buffer_for(逻辑下标)` 取托管资源 → `Attachment::color(texture.shared(), 0, Some(clear))` → `AttachmentCache::prepare` 得 RenderPass+Framebuffer → `RenderPassApi::begin_attachments / set_default_view_and_scissor / set_stencil_ref / bind_vertex_buffer_by_binding_point / draw / end_render_pass`（`AshCommandBackend`）；`ctx.keep_alive` 保活 framebuffer/pipeline/顶点缓冲 | `rg/pass_builder.rs`、`rg/executor.rs`(`PassContext`)、`rg/pass_bindings.rs`、`rg/attachments.rs`、`rg/pass_api.rs` |
| pass 2 `copy_out` | color 0x25 TransferRead；读回缓冲、本帧历史纹理 0x34 TransferWrite；回调里 `cmd_copy_image_to_buffer` + `cmd_copy_image` | 同上 |
| pass 3 `history_readback`（第 1 帧起） | 上一帧写的历史纹理 0x25 → 历史读回缓冲 0x34 | 同上 |
| 提交 | `session.execute_frame(graph.build())` → `SessionFrameReceipt`（含 `snapshot.pool_hits / physical_mapping / exports`） | `rg/vulkan.rs` |
| 完成/退役 | `wait_submits_done`、`wait_frame`、`collect_after_completion(retained_tokens)`、`graph.on_retire` | `rg/vulkan.rs` |

布局转换完全由执行器的屏障规划负责（UNDEFINED→COLOR_ATTACHMENT→TRANSFER_SRC、0x37 GENERAL→TRANSFER_DST 等），回调里没有手写布局转换。
唯一落到裸 ash 的调用：`cmd_bind_pipeline`（`RenderPassApi` 没有该方法）和两个 copy 命令（`pass_api` 只有 buffer copy）。
`set_default_view_and_scissor` 用负高度视口翻转 Y（原函数行为），所以顶点数据 y 取反，使画面与 `triangle_animation.rs` 的 CPU 参考一致。

## 2. 断言的多帧行为与实测数字（lavapipe，30 次循环 + `taskset -c 0` 无抖动）

主测试 `a_rotating_triangle_renders_correctly_through_the_graph_executor`（48 帧 × 15°，逐帧不同清除色）：
- 像素：与 CPU 参考光栅化最多 **0** 个像素不一致（阈值 ≤8）；质心最大误差 **0.09 px**（阈值 <0.8）；每帧三角形 512 像素；清除色不泄漏（只允许填充色或本帧清除色）。
- 在飞帧数峰值 **2**，且从未 >2；`begin_frame` 之后在飞数：首帧与排空点后首帧为 0，其余恰为 1（即第三帧起 begin_frame 真的等待并弹出最老帧，不增长）。
- 帧回执序号 1..48 单调；帧账本：每帧 execute 后 `has_ended()` 且 `submissions()>=1`；全程日志捕获（自装 `log::Log`）**没有任何"账本"告警**。
- temporal：parity 严格 0,1,0,1…；本帧读的历史纹理 token == 上一帧写的 token；首帧新建（reused=false），其后 47 帧两张全部复用，共 **94 次复用命中**，物理纹理全程 2 张；47 次"历史纹理读回 == 上一帧渲染结果"逐字节一致（每帧清除色不同，所以能区分串帧）。该链路在 2 帧在飞下跨帧依赖也正确（上一帧可能仍在飞）。
- 资源池（颜色目标）：`snapshot.pool_hits` 命中 **44/48** 帧；未命中恰是预期的 4 帧（第 0、1 帧；第 24 帧排空点清池后的第 24、25 帧）；物理颜色纹理共 4 张（2 在飞 × 2 段）。
- 退役：每帧 `on_retire` 回调共执行 48 次；第 24 帧前排空点 `wait_submits_done` + `collect_after_completion(retained)` 淘汰 26 个物理资源（24 个顶点缓冲 + 2 张池中颜色纹理），用 `Weak<Buffer>` 断言此前顶点缓冲**全部销毁**；retained 里保留 temporal 与读回槽 token，动画与历史对比在排空点前后继续正确（parity/历史不断）；结束后再次 collect，所有顶点缓冲销毁、提交引擎镜像遗留 (命令, 时间线)=(0,0)，`device.wait_idle()` 正常，会话未 poisoned。

副测试 `frame_throttle_and_wait_semantics_of_the_session`（空 pass 小图，隔离语义）：
- 空会话：`pending_frame_count()==0`；`wait_gpu_queue_trim(0)==Ok(true)`；`wait_frame` 报错（"没有待完成帧"）；`wait_submits_done` Ok。
- begin→execute：账本清零→结束并记提交；第二帧提交后在飞 2；**不 begin_frame 直接 execute_frame（会超容量）返回 Err，在飞数不增长**。
- `wait_gpu_queue_trim`：长超时 Ok(true) 且不弹出；0 超时只断言 Ok（GPU 是否已完成不确定，不能断言 true/false）；`wait_frame` 只接受队首帧（非队首 Err 且队列不变），队首成功后在飞 -1；再 begin_frame 排到只剩 1；`wait_submits_done` 后为 0。

## 3. 发现并修复的问题

无（`src/**` 未改）。过程中发现并修正的是**测试自己的竞态**：最初在 `execute_frame` 之后才校验 frame-2，而本帧会覆盖同一读回槽，frame 3 的历史对比偶发不一致。改为在 `begin_frame` 返回（frame-2 已确定完成）之后、本帧 `execute_frame` 之前读回。这不是执行器 bug。

## 4. 发现但没修的问题（接口/文档层面）

1. **temporal 与 backend 的 token 空间是同一个 u64 域却各自分配**：`TemporalGpuResources::next_token` 从 1 起，`VulkanGraphBackend::next_token` 也从 1 起；若先 `bind_import` 了别的资源再 `import_into` temporal 纹理，`bind_import_named` 会因"token已经属于其他物理资源"报错（读码推断，未单独造测试）。测试里靠"首帧先建 temporal 对、再绑读回缓冲"规避。
2. **会话里被导入的资源只在 `collect_after_completion` 才离开 `backend.resources`**：每帧新建并导入的顶点缓冲在排空点之前一直被后端持有（24 个，排空点一并淘汰）；`collect_after_completion` 还要求在飞为空并做设备级 `wait_idle`，所以不能每帧调用。要"每帧新建且随帧退役"，更合适的是图内创建（走 Pool）——本测试里颜色目标就是这条路径。是否匹配原版的跨帧导入生命周期未证实。
3. `ResourceSpec`/`Snapshot::physical_mapping[].reused` 不是池命中标志（池命中要看 `snapshot.pool_hits`）；导出的资源不会回池。
4. `RenderPassApi` 没有 bind_pipeline、没有任意 viewport 入口；`set_default_view_and_scissor` 翻转 Y。原版管线绑定在哪一层（`reports/rg-pass-entries.md`）这里未再核对。
5. `raw::buffer::Buffer::{read_bytes,write_bytes}` 的 `transfer(..., true)` 按其文档会等整设备，会破坏帧重叠；测试改用 `read_bytes_after_completion`（unsafe，调用点已等时间线）与 `write_bytes_unsynchronized`。属读文档判断，未实测对比。

## 5. 没验证到的部分

- **Vulkan 校验层**：本机只有 `VkLayer_INTEL_nullhw`、`VkLayer_MESA_overlay`，没有 Khronos validation layer，且 rhapsody 的实例创建里没有启用校验层的开关，所以**没有做"无校验层 error"的断言**。同步正确性只靠 lavapipe 上像素/历史内容逐字节一致间接证明。
- 单队列路径（`Queue::Primary`）；辅助队列/跨队列族 release-acquire、并行分块在多 pass 大图下的行为未覆盖（图里只有 3 个 pass）。
- `FrameThrottle::begin_frame_bounded`（容量 2 的第三次弹出 panic）语义在 `frame_throttle.rs` 单元测试里，会话的 `begin_frame` 用的是无上限 `while >1` 循环，两者差异未在本测试中触发（正常运行在途最多 2，不会碰到）。
- 候选优化特性（`candidate-optimizations`，ADAPTIVE 并行）在 `--all-features` 下编译并跑过全量，但本测试没有单独对比开/关两种分块。

## 6. 回归核对

全量 `cargo test --offline -p rhapsody -p tiny-glade -p country-core -p shader-pipeline --all-features`（lavapipe）：**49 个测试目标，1766 passed / 0 failed**（含 `original_oracle`、`original_oracle_barrier`、`original_oracle_descriptors`、`original_oracle_sequence`、`triangle_draw`、`triangle_animation`、新增 `triangle_animation_graph`）。相对任务给的基线（48 目标/1741）多 1 个目标（本测试）；通过数多出 25，其中本测试贡献 2，其余来自工作区里别处的已有改动，我没有逐项核对来源。
