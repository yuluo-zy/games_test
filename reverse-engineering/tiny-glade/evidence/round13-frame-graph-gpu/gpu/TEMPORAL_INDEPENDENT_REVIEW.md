# Temporal v2 独立复核

裁决：最终冻结 v2 的普通非重入调用路径可整合；未发现它破坏现 rhapsody 的 GPU pending、temporal parity、confirmed access、retire 或资源池所有权。必须收窄一处注释的保证范围，且仍须 root 编译及真实 GPU 验收。没有 v3；作者已确认 `temporal/candidate.patch`、`base.json`、`candidate/` 是最终 v2。

本复核自行读取候选和现 `VulkanGraphSession`、`TemporalRenderGraphState`、frame-import marker、FrameLedger、PreparedSessionFrame、retire callback 与 pending-frame 实现，没有以作者报告作为行为证据。只读核验五个候选文件 SHA 全部匹配冻结 `base.json`。本 worker 不运行 Cargo、不改生产、不改作者冻结 manifest。

## 确认成立的修复

| 项 | 独立核对结果 |
|---|---|
| BuiltCoreGraph Drop | guard 强持原 session 的 Rc 和原生 BuiltGraph。Drop 使用原 session 的 `cancel_frame`；后者取图内容、丢 render callbacks、逐个 catch_unwind 执行 retire callbacks、collect frame-import weak markers，再清属于该图的 declaration/named 临时状态。guard graph Option.take 保证自身不重复取消。 |
| 跨 App 拒绝 | `claim_built` 在 receiver session 执行任何 compile/submit 之前，以 Rc identity 和 GraphId 拒绝。被拒 guard 随返回 Err 析构，取消发往 origin，不污染 receiver 的 named/cache/ledger。两边普通调用均可继续构图；额外 raw API 重入限制见下文。 |
| 防二次构图 | `pending_build` 在 `begin_frame` 成功后写入；第二次 bridge build 在调用 closure 之前拒绝。builder error/escaped-host failure 的 CoreGraphFrame Drop、built Drop、显式 claim 的三条路线均释放租约；prepared 阶段接回现 `active_prepared` 门。没有复制现原生图或另建资源后端。 |
| adopt 普通忙态重试 | 同一 RefMut 和 one-shot mutex 内先借用检查，再 take。built 状态由 named.graph_id + 未结束 ledger 拒绝，prepared 由 active_prepared 拒绝，pending/导入由现验证拒绝；这些 session 本地状态不会在两步间改变。普通临时拒绝保持 worker 的 CompletedNamedResources、images、parity 和 App identity，取消 busy 图后可重试。 |
| prepared cancel 清 graph_id | 先验证 active_prepared+device generation，然后现路径 reclaim allocation/pool hits、drop callbacks、run retire、state.abandon_frame；候选只追加清声明 graph_id。FrameLedger 仍非 ended，但图已不存在，adopt 不再将这个历史诊断误当活跃 declaration。没有伪造 submission 或改 ledger.end。 |
| compile failure 清 graph_id | candidate 只清与消费图匹配的 graph_id；现 abandon_frame 回退临时 access/parity，result-cache abort、presentation cancel、退役原路径保持。错误不写成功历史。 |
| execute terminal 清 graph_id | 清除放在完整的 confirm_exports / abandon_frame / invalidate_submitted_frame 分支之后。success 的 parity/confirmed accesses 早已提交；partial 的 requested histories 已失效且真实 GraphExecution、receipt、retire 留在 pending；graph_id 只是本次 declaration identity，后续 wait/finish/reclaim 不依赖它。没有提早释放 GPU owners。 |
| 性能/架构 | 新增每 bridge 一个 Rc<Cell<Option<GraphId>>>，每个 built 一个常数尺寸 guard；不克隆图、不全量遍历 pending、不加同步 wait。adopt 的 borrowed validation 使 warmup 时重复一次已有有界资源验证，不属于每 pass/frame 热循环。 |

## 必须精确说明的边界

1. `render_bridge.rs` 候选约第163行的 “No state can change ... this RefMut remains held” 只能保证 **session 本地 adoption 前置状态**。`validate_completed_named_resources_adoption` 调用 `is_poisoned`，读取共享 device timelines 的 sticky DeviceLost atomic；另一个 session/thread 能在 preflight 与 consuming adopt 的第二次 validation 之间将它置 lost。若恰好发生这种终止性错误，第二次 validation 仍可在 `named.take()` 后返回 DeviceLost，转移值随 consuming API 被丢弃。因此应改注释/报告为“普通临时 busy 拒绝保持重试 owner；不宣称任意 Err 或 device-loss 恢复”。作者已确认此边界。此处没有可复用同 Device 的正常重试，未列为本次普通路径整合阻断；若产品以后要求任意 Err 都保留 payload，应采用借 `Option<CompletedNamedResources>` 的 take-after-final-validation commit API。

2. `with_session` 是现公开的原生逃逸面。若 caller 在 `origin.with_session(|_| ...)` 持原 session RefMut 期间 drop origin 的 BuiltCoreGraph（包括在另一 bridge 上拒绝该 foreign guard），Drop 的 `borrow_mut` 会重入 panic。现 CompiledCoreGraph Drop 也有同一限制。普通生产路径没有这类嵌套；本次不能宣传任意 raw session 重入安全。若以后要支持，需要明确 deferred cancellation 或限制逃逸面的生命周期，不能只用 `try_borrow_mut` 吞掉取消。

3. named资源被真实创建过的失败帧，其 cache 并不因取消自动变为空；`current.resources.len()!=0` 继续拒绝接管另一历史 owner，这是防止合并两个真实 cache 的既有约束，不能为了 adoption 测试删除它。

## 验收要求

root 运行冻结包新增的 `core_render_bridge` guard/cross-App/import-retire 回归、现 `compile_before_submit` 真实 copy/readback，以及 `original_scene_compute_gpu::completed_background_warmup_preserves_actual_pair_owner_parity_and_pixels`，设置 `RHAPSODY_REQUIRE_VULKAN=1`。后者保留原 AA shader、真实 worker pair Arc identity、历史/parity 和像素读回，新增的 busy→cancel→retry 不是以空图替代最终 GPU 工作。

本报告不把静态可整合裁决当作这些测试已执行，也不升级为完整游戏、WSI 所有故障注入或原版所有 temporal/reset/drop 行为等价。
